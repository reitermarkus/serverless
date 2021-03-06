use std::error::Error;

use http::{HeaderMap, Method, Uri, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{self, json};

use openfaas;

#[derive(Deserialize, Serialize, Debug)]
struct Device {
  #[serde(alias = "_id")]
  id: String,
  name: String,
  data_types: Vec<String>,
}

pub async fn handle(method: Method, _uri: Uri, _headers: HeaderMap, _body: String) -> Result<(StatusCode, String), Box<dyn Error + Send + Sync>> {
  if method != Method::GET {
    return Ok((StatusCode::METHOD_NOT_ALLOWED, format!("Method '{}' is not allowed.", method)))
  }

  let devices = openfaas::call("database", json!({
    "collection": "devices",
    "action": "find",
  }).to_string()).await?;

  let devices: Vec<Device> = match devices {
    (StatusCode::OK, devices) => serde_json::from_str(&devices).unwrap(),
    res => return Ok(res),
  };

  Ok((StatusCode::OK, json!(devices).to_string()))
}
