use std::error::Error;

use http::{HeaderMap, Method, Uri, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};

use openfaas;

const SUPPORTED_DATA_TYPES: [&'static str; 15] = [
  "cpu_frequency",
  "proximity",
  "acceleration",
  "cpu_temperature",
  "gravity",
  "humidity",
  "illuminance",
  "magnetic_field",
  "magnetic_field_uncalibrated",
  "orientation",
  "pressure",
  "rotation",
  "rotation_rate",
  "rotation_rate_uncalibrated",
  "temperature",
];

#[derive(Deserialize, Serialize)]
struct Data {
  device_id: String,
  #[serde(rename = "type", skip_serializing)]
  data_type: String,
  time: String,
  value: Value,
}

pub async fn handle(method: Method, _uri: Uri, _headers: HeaderMap, body: String) -> Result<(StatusCode, String), Box<dyn Error + Send + Sync>> {
  if method != Method::POST {
    return Ok((StatusCode::METHOD_NOT_ALLOWED, format!("Method '{}' is not allowed.", method)))
  }

  let data = match serde_json::from_str::<Data>(&body) {
    Ok(data) => data,
    _ => {
      log::error!("Invalid JSON format:\n{}", body);
      return Ok((StatusCode::BAD_REQUEST, "Invalid format.".to_string()))
    },
  };

  if !SUPPORTED_DATA_TYPES.iter().any(|&t| t == data.data_type) {
    log::error!("Data type '{}' is not supported.", data.data_type);
    return Ok((StatusCode::BAD_REQUEST, format!("Data type '{}' is not supported.", data.data_type)));
  }

  if data.data_type == "illuminance" {
    log::debug!("Illuminance for device '{}': {:?}", data.device_id, data.value);
  }

  let res = openfaas::call("database", json!({
    "collection": "devices",
    "action": "update",
    "query": {
      "_id": data.device_id,
    },
    "update": {
      "$addToSet": { "data_types": data.data_type },
    }
  }).to_string()).await;

  match res {
    Ok((code, err)) if !code.is_success() => {
      log::error!("Error updating device '{}': {}", data.device_id, err);
      return Ok((code, err))
    },
    Err(err) => {
      log::error!("Error updating device '{}': {}", data.device_id, err);
      return Err(err)
    }
    _ => {},
  }

  let res = openfaas::call("database", json!({
    "collection": data.data_type,
    "action": "insert",
    "doc": data,
  }).to_string()).await;

  match res {
    Ok((code, err)) if !code.is_success() => {
      log::error!("Error inserting '{}' data for device '{}': {}", data.data_type, data.device_id, err);
      return Ok((code, err))
    },
    Err(err) => {
      log::error!("Error inserting '{}' data for device '{}': {}", data.data_type, data.device_id, err);
      Err(err)
    },
    ok => ok,
  }
}
