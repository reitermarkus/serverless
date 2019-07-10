use futures::future::{self, Future, Either};
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

pub fn handle(_method: Method, _uri: Uri, _headers: HeaderMap, body: String) -> impl Future<Item = (StatusCode, String), Error = StatusCode> {
  let body1 = body.clone();
  let body2 = body.clone();
  let body3 = body.clone();

  let f1 = future::lazy(move || {
    if let Ok(illuminance_data) = serde_json::from_str::<IlluminanceData>(&body1) {
      println!("ILLUMINANCE");

      Either::A(openfaas::call("database", json!({
        "collection": "illuminances",
        "action": "insert",
        "doc": {
          "illuminance": illuminance_data.illuminance,
        },
      }).to_string()).map_err(|e| e.unwrap_or(StatusCode::NOT_FOUND)))
    } else {
      Either::B(future::err(StatusCode::NOT_FOUND))
    }
  });

  let f2 = future::lazy(move || {
    if let Ok(pressure_data) = serde_json::from_str::<PressureData>(&body2) {
      println!("PRESSURE");

      Either::A(openfaas::call("database", json!({
        "collection": "pressures",
        "action": "insert",
        "doc": {
          "pressure": pressure_data.pressure,
        },
      }).to_string()).map_err(|e| e.unwrap_or(StatusCode::NOT_FOUND)))
    } else {
      Either::B(future::err(StatusCode::NOT_FOUND))
    }
  });

  let f3 = future::lazy(move || {
    if let Ok(temperature_data) = serde_json::from_str::<TemperatureData>(&body3) {
      println!("TEMPERATURE");

      Either::A(openfaas::call("database", json!({
        "collection": "temperatures",
        "action": "insert",
        "doc": {
          "temperature": temperature_data.temperature,
        },
      }).to_string()).map_err(|e| e.unwrap_or(StatusCode::NOT_FOUND)))
    } else {
      Either::B(future::err(StatusCode::NOT_FOUND))
    }
  });

  f1.join3(f2, f3).map(|(r1, r2, r3)| (r1.0, format!("{}\n{}\n{}", r1.1, r2.1, r3.1)))
}
