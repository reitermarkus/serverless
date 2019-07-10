use std::env;
use std::str::FromStr;

use futures::future::{self, Future};
use http::{HeaderMap, Method, Uri, StatusCode};
use lazy_static::lazy_static;
use mongodb::{doc, Bson, Client, ThreadedClient, db::ThreadedDatabase};
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
  doc: Value,
}

pub fn handle(_method: Method, _uri: Uri, _headers: HeaderMap, body: String) -> impl Future<Item = (StatusCode, String), Error = StatusCode> {
  let args = match serde_json::from_str::<MongoArgs>(&body) {
    Ok(json) => json,
    Err(err) => return future::err(StatusCode::BAD_REQUEST),
  };

  let client = Client::connect(&MONGO_HOST, *MONGO_PORT).expect("Failed to connect to database");

  let admin_database = client.db("admin");
  admin_database.auth(&MONGO_USERNAME, &MONGO_PASSWORD).expect("Failed to authenticate with database");

  match args.action.as_ref() {
    "insert" => {
      let database = client.db(&MONGO_DB);
      let collection = database.collection(&args.collection);

      if let Some(doc) = Bson::from(args.doc).as_document().cloned() {
        return match collection.insert_one(doc, None) {
          Ok(result) => future::ok((StatusCode::CREATED, format!("Inserted {:?} into collection '{}' in database '{}'.", result, args.collection, *MONGO_DB))),
          Err(err) => future::ok((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err))),
        }
      }

      future::ok((StatusCode::BAD_REQUEST, "Invalid document.".to_string()))
    },
    _ => future::err(StatusCode::METHOD_NOT_ALLOWED)
  }
}
