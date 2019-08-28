use std::error::Error;

use http::{HeaderMap, Method, Uri, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};

use openfaas;

#[derive(Deserialize, Serialize, Debug)]
struct Args {
  device_id: String,
  collection: String,
  begin: Option<String>,
  end: Option<String>,
  interval: Option<usize>,
}

pub async fn handle(method: Method, _uri: Uri, _headers: HeaderMap, body: String) -> Result<(StatusCode, String), Box<dyn Error + Send>> {
  if method != Method::POST {
    return Ok((StatusCode::METHOD_NOT_ALLOWED, format!("Method '{}' is not allowed.", method)))
  }

  let args: Args = match serde_json::from_str(&body) {
    Ok(args) => args,
    _ => return Ok((StatusCode::BAD_REQUEST, "Invalid format.".to_string())),
  };

  if let (Some(begin), Some(end), Some(interval)) = (args.begin, args.end, args.interval) {
    let pipeline = vec![
      json!({ "$match": { "device_id": args.device_id } }),
      json!({ "$group": {
        "_id": null,
        "avg": { "$avg": "$value" },
        "avg_x": { "$avg": "$value.x" },
        "avg_y": { "$avg": "$value.y" },
        "avg_z": { "$avg": "$value.z" },
      }}),
    ];

    return aggregate(&args.collection, pipeline, Some(begin), Some(end), Some(interval)).await
  }

  let pipeline = vec![json!({ "$match": { "device_id": args.device_id } })];
  Ok(aggregate(&args.collection, pipeline, None, None, None).await?)
}

async fn aggregate(collection: &str, pipeline: Vec<Value>, begin: Option<String>, end: Option<String>, steps: Option<usize>) -> Result<(StatusCode, String), Box<dyn Error + Send>> {
  openfaas::call("database", json!({
    "collection": collection,
    "action": "aggregate",
    "pipeline": pipeline,
    "begin": begin,
    "end": end,
    "steps": steps,
  }).to_string()).await
}
