use std::env;
use std::str::FromStr;

use futures::future::{self, Future, Either};
use http::{HeaderMap, Method, Uri, StatusCode};
use lazy_static::lazy_static;
use mongodb::{doc, Bson, Client, ThreadedClient, db::ThreadedDatabase, coll::results::InsertOneResult};
use serde_derive::Deserialize;
use serde_json::{self, json};

use openfaas;

#[derive(Debug, Deserialize)]
struct IlluminanceData {
  pub illuminance: f64,
}

#[derive(Debug, Deserialize)]
struct PressureData {
  pub pressure: f64,
}

#[derive(Debug, Deserialize)]
struct TemperatureData {
  pub temperature: f64,
}

lazy_static! {
  static ref MONGO_HOST: String = env::var("MONGO_HOST").expect("MONGO_HOST is not set");
  static ref MONGO_PORT: u16 = env::var("MONGO_PORT").ok().and_then(|p| u16::from_str(&p).ok()).unwrap_or(27017);
  static ref MONGO_DB: String = env::var("MONGO_DB").expect("MONGO_DB is not set");
  static ref MONGO_USERNAME: String = openfaas::secret("mongo-root-username").unwrap();
  static ref MONGO_PASSWORD: String = openfaas::secret("mongo-root-password").unwrap();

  static ref MONGO_CLIENT: Client = {
    println!("Initializing database connection …");

    let client = Client::connect(&MONGO_HOST, *MONGO_PORT).expect("Failed to connect to database");
    let admin_database = client.db("admin");
    admin_database.auth(&MONGO_USERNAME, &MONGO_PASSWORD).expect("Failed to authenticate with database");

    println!("Database connection authenticated.");

    client
  };
}


use serde_json::Value;
#[derive(Debug, Deserialize)]
struct MongoArgs {
  db: String,
  collection: String,
  action: String,
  doc: Value,
}

fn mongodb(body: String) -> impl Future<Item = (StatusCode, String), Error = StatusCode> {
  let args = match serde_json::from_str::<MongoArgs>(&body) {
    Ok(json) => json,
    Err(_) => return future::err(StatusCode::BAD_REQUEST),
  };

  match args.action.as_ref() {
    "insert" => {
      let database = MONGO_CLIENT.db(&args.db);
      let collection = database.collection(&args.collection);

      if let Some(doc) = Bson::from(args.doc).as_document().cloned() {
        if let Ok(InsertOneResult { inserted_id: Some(inserted_id), .. }) = collection.insert_one(doc, None) {
          return future::ok((StatusCode::CREATED, format!("Inserted {} into collection {} in database {}.", inserted_id, args.collection, args.db)))
        }
      }

      future::err(StatusCode::BAD_REQUEST)
    },
    _ => future::err(StatusCode::METHOD_NOT_ALLOWED)
  }
}

pub fn handle(_method: Method, _uri: Uri, _headers: HeaderMap, body: String) -> impl Future<Item = (StatusCode, String), Error = StatusCode> {
  println!("Body: {:?}\n", body);

  if body.is_empty() {
    return Either::B(future::ok((StatusCode::OK, "RECURSIVE REQUEST SUCCESSFUL\n".to_string())))
  }

  let f = future::lazy(move || {
    if let Ok(illuminance_data) = serde_json::from_str::<IlluminanceData>(&body) {
      mongodb(json!({
        "db": *MONGO_DB,
        "collection": "illuminances",
        "action": "insert",
        "doc": {
          "illuminance": illuminance_data.illuminance,
        },
      }).to_string());
    }

    if let Ok(pressure_data) = serde_json::from_str::<PressureData>(&body) {
      mongodb(json!({
        "db": *MONGO_DB,
        "collection": "pressures",
        "action": "insert",
        "doc": {
          "pressure": pressure_data.pressure,
        },
      }).to_string());
    }

    if let Ok(temperature_data) = serde_json::from_str::<TemperatureData>(&body) {
      mongodb(json!({
        "db": *MONGO_DB,
        "collection": "temperatures",
        "action": "insert",
        "doc": {
          "temperature": temperature_data.temperature,
        },
      }).to_string());
    }

    future::ok(())
  });

  Either::A(f.then(|_: Result<_, ()>| openfaas::call("rust-to-mongo", "".to_string())
    .map(|res| (StatusCode::OK, format!("Nested response: {:?}\n", res.1)))
    .map_err(|status_code| status_code.unwrap_or(StatusCode::IM_A_TEAPOT))))
}
