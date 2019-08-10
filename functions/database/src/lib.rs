#![feature(async_await)]

use std::env;
use std::error::Error;
use std::str::FromStr;

use chrono::{DateTime, offset::Utc};
use http::{HeaderMap, Method, Uri, StatusCode};
use lazy_static::lazy_static;
use mongodb::{doc, bson, Document, Bson, Client, ThreadedClient, db::ThreadedDatabase};
use serde_derive::Deserialize;
use serde_json::{self, Value};

use openfaas;

lazy_static! {
  static ref MONGO_HOST: String = env::var("MONGO_HOST").expect("MONGO_HOST is not set");
  static ref MONGO_PORT: u16 = env::var("MONGO_PORT").ok().and_then(|p| u16::from_str(&p).ok()).unwrap_or(27017);
  static ref MONGO_DB: String = env::var("MONGO_DB").expect("MONGO_DB is not set");
  static ref MONGO_USERNAME: String = openfaas::secret("mongo-root-username").unwrap();
  static ref MONGO_PASSWORD: String = openfaas::secret("mongo-root-password").unwrap();
}

#[derive(Debug, Deserialize)]
struct MongoArgs {
  collection: String,
  action: String,
  doc: Option<Document>,
}

pub async fn handle(_method: Method, _uri: Uri, _headers: HeaderMap, body: String) -> Result<(StatusCode, String), Box<dyn Error + Send>> {
  let mut args = match serde_json::from_str::<MongoArgs>(&body) {
    Ok(json) => json,
    Err(err) => return Err(Box::new(err) as _),
  };

  let client = Client::connect(&MONGO_HOST, *MONGO_PORT).expect("Failed to connect to database");

  let admin_database = client.db("admin");
  admin_database.auth(&MONGO_USERNAME, &MONGO_PASSWORD).expect("Failed to authenticate with database");

  let database = client.db(&MONGO_DB);
  let collection = database.collection(&args.collection);

  let doc = args.doc.take();

  match args.action.as_ref() {
    "insert" => {
      let mut doc = match doc {
        Some(doc) => doc,
        None => return Ok((StatusCode::BAD_REQUEST, "No document found.".to_string())),
      };

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
    "insert_or_replace" => {
      let doc = match doc {
        Some(doc) => doc,
        None => return Ok((StatusCode::BAD_REQUEST, "No document found.".to_string())),
      };

      let id: String = match doc.get("_id").and_then(|id| id.as_str()) {
        Some(id) => id.to_string(),
        _ => return Ok((StatusCode::BAD_REQUEST, "No ID found.".to_string())),
      };

      let filter = doc! { "_id": id };

      match collection.replace_one(filter, doc.clone(), None) {
        Ok(ref update_result) if update_result.modified_count == 1 => {
          Ok((StatusCode::CREATED, "Replaced.".to_string()))
        },
        Ok(ref update_result) if update_result.matched_count == 1 && update_result.modified_count == 0 => {
          Ok((StatusCode::CREATED, "No change.".to_string()))
        },
        Ok(_) => match collection.insert_one(doc, None) {
          Ok(_) => Ok((StatusCode::CREATED, "Inserted.".to_string())),
          Err(err) => Err(Box::new(err) as _),
        },
        Err(err) => Err(Box::new(err) as _),
      }
    },
    "find" => {
      match collection.find(None, None) {
        Ok(mut cursor) => {
          let mut items = Vec::new();

          while match cursor.has_next() {
            Ok(b) => b,
            Err(err) => return Ok((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err))),
          } {
            match cursor.drain_current_batch() {
              Ok(batch) => items.extend(batch),
              Err(err) => return Ok((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err))),
            };
          }

          Ok((StatusCode::OK, serde_json::to_string(&items).unwrap()))
        },
        Err(err) => Ok((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err))),
      }
    },
    method => Ok((StatusCode::METHOD_NOT_ALLOWED, format!("Action '{}' is not allowed.", method)))
  }
}
