use std::error::Error;

use http::{HeaderMap, Method, Uri, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};
use chrono::{DateTime, Duration, Utc};
use futures::future::join_all;

use openfaas;

#[derive(Deserialize, Serialize, Debug)]
struct Args {
  device_id: String,
  collection: String,
  begin: Option<DateTime<Utc>>,
  end: Option<DateTime<Utc>>,
}

pub async fn handle(method: Method, _uri: Uri, _headers: HeaderMap, body: String) -> Result<(StatusCode, String), Box<dyn Error + Send>> {
  if method != Method::POST {
    return Ok((StatusCode::METHOD_NOT_ALLOWED, format!("Method '{}' is not allowed.", method)))
  }

  let args: Args = match serde_json::from_str(&body) {
    Ok(args) => args,
    _ => return Ok((StatusCode::BAD_REQUEST, "Invalid format.".to_string())),
  };

  if let (Some(begin), Some(end)) = (args.begin, args.end) {
    let steps = (begin, end).into_steps(20);
    let stream = steps.map(|(begin, end)| fetch_timeframe(&args.device_id, &args.collection, begin, end));
    let res = join_all(stream).await.into_iter().collect::<Result<Vec<_>, _>>()?;
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

struct DurationSteps {
  begin: DateTime<Utc>,
  end: DateTime<Utc>,
  step: Duration,
}

impl Iterator for DurationSteps {
  type Item = (DateTime<Utc>, DateTime<Utc>);

  fn next(&mut self) -> Option<Self::Item> {
    if self.begin >= self.end {
      return None
    }

    let end = if let Some(end) = self.begin.checked_add_signed(self.step) {
      end
    } else {
      return None
    };

    let v = if end >= self.end {
      (self.begin.clone(), self.end.clone())
    } else {
      (self.begin.clone(), end)
    };

    self.begin = end;

    Some(v)
  }
}

trait IntoSteps {
  fn into_steps(self, steps: i32) -> DurationSteps;
}

impl IntoSteps for (DateTime<Utc>, DateTime<Utc>) {
  fn into_steps(self, steps: i32) -> DurationSteps {
    let (begin, end) = self;
    let duration = end.signed_duration_since(begin);
    let step = duration / steps;

    DurationSteps { begin, end, step }
  }
}
