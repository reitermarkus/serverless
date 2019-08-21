#![feature(async_await)]

use std::env;
use std::error::Error;
use std::str::FromStr;

use chrono::{DateTime, offset::Utc};
use http::{HeaderMap, Method, Uri, StatusCode};
use lazy_static::lazy_static;
use mongodb::{doc, bson, Bson, Document, Client, ThreadedClient, db::ThreadedDatabase, coll::options::UpdateOptions};
use serde_derive::Deserialize;

use openfaas;

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
  Find { filter: Option<Document> },
}

#[derive(Debug, Deserialize)]
struct MongoArgs {
  collection: String,
  #[serde(flatten)]
  action: Action,
}

fn simplify_bson(bson: Bson) -> Bson {
  use Bson::*;

  match bson {
    Array(vec) => Array(vec.into_iter().map(simplify_bson).collect()),
    Document(doc) => {
      Document(doc.iter()
        .map(|(key, value)| (key.to_owned(), simplify_bson(value.to_owned())))
        .collect())
    },
    ObjectId(id) => String(id.to_hex()),
    UtcDatetime(datetime) => String(datetime.to_rfc3339()),
    bson => bson,
  }
}

pub async fn handle(_method: Method, _uri: Uri, _headers: HeaderMap, body: String) -> Result<(StatusCode, String), Box<dyn Error + Send>> {
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
        Err(err) => Ok((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err))),
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
    Find { filter } => {
      match collection.find(filter, None) {
        Ok(mut cursor) => {
          let mut items = Vec::new();

          while match cursor.has_next() {
            Ok(b) => b,
            Err(err) => return Ok((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err))),
          } {
            match cursor.drain_current_batch() {
              Ok(batch) => items.extend(batch.into_iter().map(|doc| simplify_bson(doc.into()))),
              Err(err) => return Ok((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err))),
            };
          }

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
