use std::env;
use std::thread;
use std::time::Duration;
use std::str;

use reqwest::{Client, header::{CONTENT_TYPE, HeaderMap, HeaderValue}};
use serde_json::{json, to_string_pretty, Value};
use systemstat::{System, Platform};

fn sys_stats() -> Result<Value, std::io::Error> {
  let sys = System::new();

  let memory = sys.memory()?;
  let uptime = sys.uptime()?;
  let boot_time = sys.boot_time()?;
  let cpu_temp = sys.cpu_temp()?;
  let cpu_load_average = sys.load_average()?;

  let cpu_load_aggregate = sys.cpu_load_aggregate()?;
  thread::sleep(Duration::from_secs(1));
  let cpu_load_aggregate = cpu_load_aggregate.done()?;

  Ok(json!({
    "cpu_temp": cpu_temp,
    "cpu_load_average": {
      "one":     cpu_load_average.one,
      "five":    cpu_load_average.five,
      "fifteen": cpu_load_average.fifteen,
    },
    "cpu_load_aggregate": {
      "user":      cpu_load_aggregate.user * 100.0,
      "nice":      cpu_load_aggregate.nice * 100.0,
      "system":    cpu_load_aggregate.system * 100.0,
      "interrupt": cpu_load_aggregate.interrupt * 100.0,
      "idle":      cpu_load_aggregate.idle * 100.0,
    },
    "memory": json!({
      "used": (memory.total - memory.free).to_string(true),
      "free": memory.free.to_string(true)
    }),
    "boot_time": boot_time,
    "uptime": json!({
      "hours":   uptime.as_secs() / 3600,
      "minutes": (uptime.as_secs() % 3600) / 60,
      "seconds": (uptime.as_secs() % 3600) % 60,
    }),
  }))
}

fn post_topic(kafka_url: &str, topic: &str, value: &Value) -> Result<Value, reqwest::Error> {
  let mut headers = HeaderMap::new();
  headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/vnd.kafka.json.v2+json"));

  let mut res = Client::new()
    .post(&format!("{}/topics/{}", kafka_url, topic))
    .headers(headers)
    .body(json!({"records": [{"value": value}]}).to_string())
    .send()?;

  res.json()
}

fn main() {
  let kafka_host = env::var("KAFKA_HOST").expect("KAFKA_HOST is not set");
  let kafka_port = env::var("KAFKA_PORT").expect("KAFKA_PORT is not set");

  let kafka_url = format!("http://{}:{}", kafka_host, kafka_port);

  loop {
    println!("KAFKA: {}", kafka_url);

    match sys_stats() {
      Ok(stats) => {
        println!("INFO: {}", to_string_pretty(&stats).unwrap());
        let json_response = post_topic(&kafka_url, "sensor", &stats).unwrap();
        println!("RESPONSE: {}", to_string_pretty(&json_response).unwrap());
      },
      Err(err) => eprintln!("ERROR: {}", err.to_string()),
    }

    thread::sleep(Duration::from_secs(15));
  }
}
