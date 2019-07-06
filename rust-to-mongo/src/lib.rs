use std::env;
use std::fs;
use std::str::FromStr;

use http::{HeaderMap, Method, Uri};
use mongodb::{doc, bson, Client, ThreadedClient, db::ThreadedDatabase};

use lazy_static::lazy_static;

fn get_secret(name: &str) -> String {
  if let Ok(secret) = fs::read_to_string(&format!("/var/openfaas/secrets/{}", name)) {
    return secret
  }

  if let Ok(secret) = fs::read_to_string(&format!("/run/secrets/{}", name)) {
    return secret
  }

  panic!("Could not find secret '{}'.", name)
}

lazy_static! {
  static ref MONGO_HOST: String = env::var("MONGO_HOST").expect("MONGO_HOST is not set");
  static ref MONGO_PORT: u16 = env::var("MONGO_PORT").ok().and_then(|p| u16::from_str(&p).ok()).unwrap_or(27017);
  static ref MONGO_DB: String = env::var("MONGO_DB").expect("MONGO_DB is not set");
  static ref MONGO_USERNAME: String = get_secret("mongo-root-username");
  static ref MONGO_PASSWORD: String = get_secret("mongo-root-password");
}

pub fn handle(_method: Method, _uri: Uri, _headers: HeaderMap, _body: String) -> String {
  let client = Client::connect(&MONGO_HOST, *MONGO_PORT).expect("Failed to connect to database");

  let admin_database = client.db("admin");
  admin_database.auth(&MONGO_USERNAME, &MONGO_PASSWORD).expect("Failed to authenticate with database");

  let database = client.db(&MONGO_DB);
  let collection = database.collection("temperatures");

  let doc = doc!{"temperature": 42.0};
  let result = format!("Inserted {} into {} database.\n", doc, *MONGO_DB);
  collection.insert_one(doc, None).unwrap();

  result
}
