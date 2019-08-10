#![feature(async_await)]

use std::collections::HashMap;
use std::error::Error;

use http::{HeaderMap, Method, Uri, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};

use openfaas;

#[derive(Deserialize, Serialize)]
struct Device {
  id: String,
  name: String,
}

pub async fn handle(method: Method, _uri: Uri, _headers: HeaderMap, _body: String) -> Result<(StatusCode, String), Box<dyn Error + Send>> {
  if method != Method::GET {
    return Ok((StatusCode::METHOD_NOT_ALLOWED, format!("Method '{}' is not allowed.", method)))
  }

  let mut devices: Vec<Device> = Vec::new();

  devices.push(Device {
    id: String::from("mydevice"),
    name: String::from("My Device"),
  });

  Ok((StatusCode::OK, json!(devices).to_string()))
}
