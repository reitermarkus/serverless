#![feature(async_await)]

use std::error::Error;

use http::{HeaderMap, Method, Uri, StatusCode};
use serde_derive::{Deserialize, Serialize};
use serde_json::{self, json};

use openfaas;

#[derive(Debug, Deserialize, Serialize)]
struct IlluminanceData {
  pub illuminance: f64,
  pub time: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct PressureData {
  pub pressure: f64,
  pub time: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TemperatureData {
  pub temperature: f64,
  pub time: String,
}

pub async fn handle(method: Method, _uri: Uri, _headers: HeaderMap, body: String) -> Result<(StatusCode, String), Box<dyn Error + Send>> {
  if method != Method::POST {
    return Ok((StatusCode::METHOD_NOT_ALLOWED, "".to_string()))
  }

  let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;
  let mut response = "".to_string();

  if let Ok(illuminance_data) = serde_json::from_str::<IlluminanceData>(&body) {
    if let Ok((s, r)) = openfaas::call("database", json!({
      "collection": "illuminances",
      "action": "insert",
      "doc": illuminance_data,
    }).to_string()).await {
      status_code = s;
      response.push_str(&r);
      response.push('\n');
    }
  }

  if let Ok(pressure_data) = serde_json::from_str::<PressureData>(&body) {
    if let Ok((s, r)) = openfaas::call("database", json!({
      "collection": "pressures",
      "action": "insert",
      "doc": pressure_data,
    }).to_string()).await {
      status_code = s;
      response.push_str(&r);
      response.push('\n');
    }
  }

  if let Ok(temperature_data) = serde_json::from_str::<TemperatureData>(&body) {
    if let Ok((s, r)) = openfaas::call("database", json!({
      "collection": "temperatures",
      "action": "insert",
      "doc": temperature_data,
    }).to_string()).await {
      status_code = s;
      response.push_str(&r);
      response.push('\n');
    }
  }

  Ok((status_code, response))
}
