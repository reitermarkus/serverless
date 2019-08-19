#![feature(async_await)]

use std::error::Error;

use http::{HeaderMap, Method, Uri, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{self, json};

use openfaas;

#[derive(Deserialize, Serialize, Debug)]
struct Args {
  device_id: String,
  collection: String,
}

pub async fn handle(method: Method, _uri: Uri, _headers: HeaderMap, body: String) -> Result<(StatusCode, String), Box<dyn Error + Send>> {
  if method != Method::POST {
    return Ok((StatusCode::METHOD_NOT_ALLOWED, format!("Method '{}' is not allowed.", method)))
  }

  let args: Args = match serde_json::from_str(&body) {
    Ok(args) => args,
    _ => return Ok((StatusCode::BAD_REQUEST, "Invalid format.".to_string())),
  };

  Ok(openfaas::call("database", json!({
    "collection": args.collection,
    "action": "find",
    "filter": {
      "device_id": args.device_id,
    },
  }).to_string()).await?)
}
