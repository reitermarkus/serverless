#![feature(async_await)]

use std::collections::HashMap;
use std::error::Error;

use http::{HeaderMap, Method, Uri, StatusCode};
use serde_derive::{Deserialize, Serialize};
use serde_json::{self, json, Value};

use openfaas;

const SUPPORTED_DATA_TYPES: [&'static str; 5] = [
  "pressure",
  "cpu_temperature",
  "illuminance",
  "temperature",
  "humidity",
];

pub async fn handle(method: Method, uri: Uri, headers: HeaderMap, body: String) -> Result<(StatusCode, String), Box<dyn Error + Send>> {
  if method != Method::POST {
    return Ok((StatusCode::METHOD_NOT_ALLOWED, format!("Method '{}' is not allowed.", method)))
  }

  let mut map = match serde_json::from_str::<HashMap<String, Value>>(&body) {
    Ok(map) => map,
    _ => return Ok((StatusCode::BAD_REQUEST, "Invalid format.".to_string())),
  };

  let data_type = match map.remove("type") {
    Some(value) => match serde_json::from_value::<String>(value) {
      Ok(s) => s,
      _ => return Ok((StatusCode::BAD_REQUEST, "Invalid format.".to_string())),
    },
    _ => return Ok((StatusCode::BAD_REQUEST, "No 'type' found.".to_string())),
  };

  if !SUPPORTED_DATA_TYPES.iter().any(|&t| t == data_type) {
    return Ok((StatusCode::BAD_REQUEST, format!("Data type '{}' is not supported.", data_type)));
  }

  openfaas::call("database", json!({
    "collection": data_type,
    "action": "insert",
    "doc": map,
  }).to_string()).await
}
