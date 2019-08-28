use std::error::Error;

use http::{HeaderMap, Method, Uri, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};
use chrono::{DateTime, Utc};
use futures::future::join_all;

use openfaas;

#[derive(Deserialize, Serialize, Debug)]
struct Args {
  device_id: String,
  collection: String,
  begin: Option<DateTime<Utc>>,
  end: Option<DateTime<Utc>>,
}

mod duration_steps;
use duration_steps::IntoDurationSteps;

pub async fn handle(method: Method, _uri: Uri, _headers: HeaderMap, body: String) -> Result<(StatusCode, String), Box<dyn Error + Send>> {
  if method != Method::POST {
    return Ok((StatusCode::METHOD_NOT_ALLOWED, format!("Method '{}' is not allowed.", method)))
  }

  let args: Args = match serde_json::from_str(&body) {
    Ok(args) => args,
    _ => return Ok((StatusCode::BAD_REQUEST, "Invalid format.".to_string())),
  };

  if let (Some(begin), Some(end)) = (args.begin, args.end) {
    let steps = (begin, end).into_duration_steps(20);
    let stream = steps.map(|(begin, end)| fetch_timeframe(&args.device_id, &args.collection, begin, end));
    let res = join_all(stream).await.into_iter().collect::<Result<Vec<_>, _>>()?;
    dbg!(&res);
    let values = res.into_iter().map(|(_, s)| serde_json::from_str::<Vec<Value>>(&s).unwrap().into_iter()).flatten().collect::<Vec<_>>();
    return Ok((StatusCode::OK, serde_json::to_string(&values).unwrap()))
  }

  let pipeline = vec![json!({ "$match": { "device_id": args.device_id } })];
  Ok(aggregate(&args.collection, pipeline).await?)
}

async fn fetch_timeframe(device_id: &str, collection: &str, begin: DateTime<Utc>, end: DateTime<Utc>) -> Result<(StatusCode, String), Box<dyn Error + Send>> {
  let pipeline = vec![
    json!({ "$match": { "device_id": device_id } }),
    json!({ "$match": { "time": { "$gte": begin, "$lte": end } } }),
    json!({ "$group": {
      "_id": null,
      "avg": { "$avg": "$value" },
      "avg_x": { "$avg": "$value.x" },
      "avg_y": { "$avg": "$value.y" },
      "avg_z": { "$avg": "$value.z" },
    }}),
    json!({ "$set": { "time": begin }}),
  ];

  aggregate(collection, pipeline).await
}

async fn aggregate(collection: &str, pipeline: Vec<Value>) -> Result<(StatusCode, String), Box<dyn Error + Send>> {
  openfaas::call("database", json!({
    "collection": collection,
    "action": "aggregate",
    "pipeline": pipeline,
  }).to_string()).await
}
