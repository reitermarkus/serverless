use std::error::Error;

use futures::{future::{self, Future, IntoFuture, Either}, stream::Stream};
use http::{HeaderMap, Method, Uri, StatusCode};
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

pub fn handle(_method: Method, _uri: Uri, _headers: HeaderMap, body: String) -> impl Future<Item = (StatusCode, String), Error = Box<Error + Send + 'static>> {
  let body1 = body.clone();
  let body2 = body.clone();
  let body3 = body.clone();

  let f1 = future::lazy(move || {
    match serde_json::from_str::<IlluminanceData>(&body1) {
      Ok(illuminance_data) => Either::A(openfaas::call("database", json!({
        "collection": "illuminances",
        "action": "insert",
        "doc": {
          "illuminance": illuminance_data.illuminance,
        },
      }).to_string())),
      Err(_) => Either::B(future::ok((StatusCode::NOT_FOUND, "".to_string()))),
    }
  });

  let f2 = future::lazy(move || {
    match serde_json::from_str::<PressureData>(&body2) {
      Ok(pressure_data) => Either::A(openfaas::call("database", json!({
        "collection": "pressures",
        "action": "insert",
        "doc": {
          "pressure": pressure_data.pressure,
        },
      }).to_string())),
      Err(_) => Either::B(future::ok((StatusCode::NOT_FOUND, "".to_string()))),
    }
  });

  let f3 = future::lazy(move || {
    match serde_json::from_str::<TemperatureData>(&body3) {
      Ok(temperature_data) => Either::A(openfaas::call("database", json!({
        "collection": "temperatures",
        "action": "insert",
        "doc": {
          "temperature": temperature_data.temperature,
        },
      }).to_string())),
      Err(_) => Either::B(future::ok((StatusCode::NOT_FOUND, "".to_string()))),
    }
  });

  future::lazy(|| {
    f1.into_stream().chain(f2.into_stream()).chain(f3.into_stream())
      .filter(|(status, _)| *status == StatusCode::CREATED)
      .fold((StatusCode::BAD_REQUEST, "".to_string()), |(_, r1), (s2, r2)| {
        future::ok((s2, format!("{}{}\n", r1, r2)))
      })
      .into_future()
  })
}
