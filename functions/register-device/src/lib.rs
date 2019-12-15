use std::error::Error;

use http::{HeaderMap, Method, Uri, StatusCode};
use serde_derive::{Deserialize, Serialize};
use serde_json::{self, json};

use openfaas;

#[derive(Deserialize, Serialize)]
struct Device {
  id: String,
  name: String,
}

pub async fn handle(method: Method, _uri: Uri, _headers: HeaderMap, body: String) -> Result<(StatusCode, String), Box<dyn Error + Send>> {
  if method != Method::POST {
    return Ok((StatusCode::METHOD_NOT_ALLOWED, format!("Method '{}' is not allowed.", method)))
  }

  let device = match serde_json::from_str::<Device>(&body) {
    Ok(map) => map,
    _ => return Ok((StatusCode::BAD_REQUEST, "Invalid format.".to_string())),
  };

  openfaas::call("database", json!({
    "collection": "devices",
    "action": "insert_or_update",
    "doc": {
      "_id": device.id,
      "name": device.name,
    },
  }).to_string()).await
}
