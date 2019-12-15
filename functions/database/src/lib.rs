use std::env;
use std::error::Error;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use http::{HeaderMap, Method, Uri, StatusCode};
use lazy_static::lazy_static;
use mongodb::{doc, bson, Bson::{self, UtcDatetime}, Document, Client, ThreadedClient, db::ThreadedDatabase, coll::options::UpdateOptions};
use serde_derive::Deserialize;
use itertools::Either;

use openfaas;

mod duration_steps;
use duration_steps::IntoDurationSteps;

lazy_static! {
  static ref MONGO_HOST: String = env::var("MONGO_HOST").expect("MONGO_HOST is not set");
  static ref MONGO_PORT: u16 = env::var("MONGO_PORT").ok().and_then(|p| u16::from_str(&p).ok()).unwrap_or(27017);
  static ref MONGO_DB: String = env::var("MONGO_DB").expect("MONGO_DB is not set");
  static ref MONGO_USERNAME: String = openfaas::secret("mongo-root-username").unwrap();
  static ref MONGO_PASSWORD: String = openfaas::secret("mongo-root-password").unwrap();
}

#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
enum Action {
  Update { query: Document, update: Document },
  Insert { doc: Document },
  InsertOrUpdate { doc: Document },
  Find,
  Aggregate {
    pipeline: Vec<Document>,
    begin: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    steps: Option<i32>,
  },
}

#[derive(Debug, Deserialize)]
struct MongoArgs {
  collection: String,
  #[serde(flatten)]
  action: Action,
}

trait BsonExt {
  fn map(self, f: &dyn Fn(Bson) -> Bson) -> Bson;
}

impl BsonExt for Bson {
  fn map(self, f: &dyn Fn(Bson) -> Bson) -> Bson {
    use Bson::*;

    match self {
      Array(vec) => Array(vec.into_iter().map(|b| b.map(f)).collect()),
      Document(doc) => {
        Document(doc.into_iter()
          .map(|(key, value)| (key.to_owned(), value.to_owned().map(f)))
          .collect())
      },
      v => f(v),
    }
  }
}

fn simplify_bson(bson: Bson, round: bool) -> Bson {
  use Bson::*;

  bson.map(&|v| match v {
    ObjectId(id) => String(id.to_hex()),
    UtcDatetime(datetime) => String(datetime.to_rfc3339()),
    FloatingPoint(v) => FloatingPoint(if round { v.round() } else { v }),
    bson => bson,
  })
}

fn json_to_bson(bson: Bson) -> Bson {
  use Bson::*;

  bson.map(&|v| match v {
    String(s) => if let Ok(datetime) = s.parse::<DateTime<Utc>>() {
      UtcDatetime(datetime)
    } else {
      String(s)
    },
    bson => bson,
  })
}

pub async fn handle(_method: Method, _uri: Uri, _headers: HeaderMap, body: String) -> Result<(StatusCode, String), Box<dyn Error + Send + Sync>> {
  use Action::*;

  let args = match serde_json::from_str::<MongoArgs>(&body) {
    Ok(json) => json,
    Err(err) => return Err(Box::new(err) as _),
  };

  let client = Client::connect(&MONGO_HOST, *MONGO_PORT).expect("Failed to connect to database");

  let admin_database = client.db("admin");
  admin_database.auth(&MONGO_USERNAME, &MONGO_PASSWORD).expect("Failed to authenticate with database");

  let database = client.db(&MONGO_DB);
  let collection = database.collection(&args.collection);

  match args.action {
    Insert { mut doc } => {
      if let Some(time) = doc.get_mut("time") {
        if let Some(s) = time.as_str() {
          if let Ok(date) = DateTime::<Utc>::from_str(s) {
            *time = date.into()
          }
        }
      }

      return match collection.insert_one(doc.clone(), None) {
        Ok(result) => Ok((StatusCode::CREATED, format!("Inserted {:?} into collection '{}' in database '{}': {:?}.", doc, args.collection, *MONGO_DB, result))),
        Err(err) => {
          log::error!("Failed to insert {:?} into collection '{}': {}", doc, args.collection, err);
          Ok((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)))
        },
      }
    },
    InsertOrUpdate { doc } => {
      let id: String = match doc.get("_id").and_then(|id| id.as_str()) {
        Some(id) => id.to_string(),
        _ => return Ok((StatusCode::BAD_REQUEST, "No ID found.".to_string())),
      };

      let filter = doc! { "_id": id };

      let mut update_options = UpdateOptions::new();
      update_options.upsert = Some(true);

      match collection.update_one(filter, doc! { "$set": doc.clone() }, Some(update_options)) {
        Ok(_) => Ok((StatusCode::CREATED, "Updated.".to_string())),
        Err(err) => Err(Box::new(err) as _),
      }
    },
    Find => {
      let items = collection.find(None, None).and_then(|cursor| {
        cursor.into_iter().collect::<Result<Vec<_>, _>>()
      });

      match items {
        Ok(items) => Ok((StatusCode::OK, serde_json::to_string(&items).unwrap())),
        Err(err) => Ok((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err))),
      }
    },
    Aggregate { pipeline, begin, end, steps } => {
      let pipeline = pipeline.into_iter()
        .map(|doc| json_to_bson(doc.into()).as_document().unwrap().to_owned())
        .collect::<Vec<_>>();

      let items: Result<Vec<Bson>, mongodb::Error> = if let (Some(begin), Some(end), Some(steps)) = (begin, end, steps) {
        let steps = (begin, end).into_duration_steps(steps);
        let stream = steps.map(|(begin, end)| {
          let mut pipeline = pipeline.clone();
          pipeline.insert(0, doc! { "$match": { "time": { "$gte": UtcDatetime(begin), "$lte": UtcDatetime(end) } } });
          pipeline.push(doc! { "$set": { "time": begin } });

          match collection.aggregate(pipeline, None) {
            Ok(cursor) => Either::Left(cursor.map(|res| res.map(|doc| simplify_bson(doc.into(), false)))),
            Err(err) => Either::Right(std::iter::once(Err(err))),
          }
        }).flatten();

        stream.collect()
      } else {
        let round = pipeline.len() < 2;

        collection.aggregate(pipeline, None).and_then(|cursor| {
          cursor.map(|res| res.map(|doc| simplify_bson(doc.into(), round))).collect()
        })
      };

      match items {
        Ok(items) => {
          let items = filter_data(items);
          Ok((StatusCode::OK, serde_json::to_string(&items).unwrap()))
        },
        Err(err) => Ok((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err))),
      }
    },
    Update { query, update } => {
      match collection.update_one(query, update, None) {
        Ok(_) => Ok((StatusCode::CREATED, "Inserted.".to_string())),
        Err(err) => Ok((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err))),
      }
    },
  }
}

fn filter_data(documents: Vec<Bson>) -> Vec<Bson> {
  documents
    .iter()
    .enumerate()
    .filter(|(i, c)| documents.get(i - 1).map(|p| !value_eq(&p, &c)).unwrap_or(true))
    .map(|(_, v)| v.clone())
    .collect()
}

fn value_eq(d1: &Bson, d2: &Bson) -> bool {
  match (d1.as_document().and_then(|d| d.get("value")), d2.as_document().and_then(|d| d.get("value"))) {
    (Some(v1), Some(v2)) => v1 == v2,
    _ => false,
  }
}
