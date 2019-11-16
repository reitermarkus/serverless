use std::env;
use std::thread;
use std::time::{Duration, SystemTime};
use std::str;
use std::error::Error;
use std::fmt;

use chrono::{DateTime, offset::Utc};
use mac_address::get_mac_address;
use reqwest::{Client, header::{CONTENT_TYPE, HeaderMap, HeaderValue}};
use serde::Deserialize;
use serde_json::{json, to_string_pretty, Value};
use systemstat::{System, Platform};

mod bmp180;
mod photoresistor;

fn now() -> DateTime<Utc> {
   DateTime::from(SystemTime::now())
}

fn sys_stats() -> Result<Vec<Value>, std::io::Error> {
  let mut stats = Vec::new();

  let sys = System::new();

  if let Ok(memory) = sys.memory() {
    stats.push(json!({
      "type": "memory",
      "time": now(),
      "value": {
        "used": memory.total.as_u64() - memory.free.as_u64(),
        "free": memory.free.as_u64(),
      }
    }));
  }

  // if let Ok(uptime) = sys.uptime() {
  //   stats.insert("uptime", json!({
  //     "hours":   uptime.as_secs() / 3600,
  //     "minutes": (uptime.as_secs() % 3600) / 60,
  //     "seconds": (uptime.as_secs() % 3600) % 60,
  //   }));
  // }

  if let Ok(boot_time) = sys.boot_time() {
    stats.push(json!({
      "type": "boot_time",
      "time": boot_time,
    }));
  }

  if let Ok(cpu_temp) = sys.cpu_temp() {
    stats.push(json!({
      "type": "cpu_temperature",
      "time": now(),
      "value": cpu_temp,
    }));
  }

  if let Ok(cpu_load_average) = sys.load_average() {
    stats.push(json!({
      "type": "cpu_load_average",
      "time": now(),
      "value": {
        "one":     cpu_load_average.one,
        "five":    cpu_load_average.five,
        "fifteen": cpu_load_average.fifteen,
      },
    }));
  }

  let cpu_load_aggregate = sys.cpu_load_aggregate()
    .and_then(|cpu_load_aggregate| {
      thread::sleep(Duration::from_secs(1));
      cpu_load_aggregate.done()
    });

  if let Ok(cpu_load_aggregate) = cpu_load_aggregate {
    stats.push(json!({
      "type": "cpu_load_aggregate",
      "time": now(),
      "value": {
        "user":      cpu_load_aggregate.user * 100.0,
        "nice":      cpu_load_aggregate.nice * 100.0,
        "system":    cpu_load_aggregate.system * 100.0,
        "interrupt": cpu_load_aggregate.interrupt * 100.0,
        "idle":      cpu_load_aggregate.idle * 100.0,
      },
    }));
  }

  if let Ok(pressure) = bmp180::pressure() {
    stats.push(json!({
      "type": "pressure",
      "time": now(),
      "value": pressure,
    }));
  }

  if let Ok(temperature) = bmp180::temperature() {
    stats.push(json!({
      "type": "temperature",
      "time": now(),
      "value": temperature,
    }));
  }

  if let Ok(illuminance) = photoresistor::lux() {
    stats.push(json!({
      "type": "illuminance",
      "time": now(),
      "value": illuminance,
    }));
  }

  let i2c_dev = linux_embedded_hal::I2cdev::new(env::var("I2C_DEVICE").expect("I2C_DEVICE is not set")).unwrap();
  let mut am2320 = am2320::Am2320::new(i2c_dev, linux_embedded_hal::Delay);

  if let Ok(measurement) = am2320.read() {
    stats.push(json!({
      "type": "temperature",
      "time": now(),
      "value": measurement.temperature,
    }));

    stats.push(json!({
      "type": "humidity",
      "time": now(),
      "value": measurement.humidity,
    }));
  }

  Ok(stats)
}

#[derive(Deserialize, Debug)]
struct KafkaRestError {
  error_code: Option<usize>,
  message: String,
}

impl fmt::Display for KafkaRestError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.description())
  }
}

impl Error for KafkaRestError {
  fn description(&self) -> &str {
    &self.message
  }

  fn cause(&self) -> Option<&dyn Error> {
    None
  }
}

struct KafkaRestClient {
  host: String,
  port: usize,
}

impl KafkaRestClient {
  pub fn new(host: impl AsRef<str>, port: usize) -> Self {
    Self { host: host.as_ref().to_string(), port }
  }

  pub fn url(&self) -> String {
    format!("http://{}:{}", self.host, self.port)
  }

  pub fn post(&self, topic: &str, records: &[Value]) -> Result<Value, KafkaRestError> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/vnd.kafka.json.v2+json"));

    let map_err = |err: reqwest::Error| KafkaRestError { error_code: err.status().map(|s| s.as_u16() as usize), message: err.to_string() };

    Client::new()
      .post(&format!("{}/topics/{}", self.url(), topic))
      .headers(headers)
      .body(json!({"records": records}).to_string())
      .send()
      .map_err(map_err)
      .and_then(|mut res| {
        let json = res.json::<Value>().map_err(map_err)?;

        if let Ok(error) = serde_json::from_value::<KafkaRestError>(json.clone()) {
          Err(error)
        } else {
          Ok(json)
        }
      })
  }
}

fn register_device(kafka_client: &KafkaRestClient, name: &str) -> String {
  let mac_address = get_mac_address().expect("Cannot retrieve MAC address").expect("No MAC address found").to_string();
  println!("MAC address: {}", mac_address);

  kafka_client.post("register-device", &[json!({
    "value": {
      "id": mac_address,
      "name": name,
    },
  })]).expect("Failed to register device");

  mac_address
}

fn main() {
  let kafka_host = if cfg!(debug_assertions) {
    env::var("KAFKA_HOST").unwrap_or_else(|_| "localhost".to_string())
  } else {
    env::var("KAFKA_HOST").expect("KAFKA_HOST is not set")
  };
  let kafka_port = if cfg!(debug_assertions) {
    env::var("KAFKA_PORT").map(|s| s.parse::<usize>().expect("Cannot parse KAFKA_PORT")).unwrap_or(8082)
  } else {
    env::var("KAFKA_PORT").expect("KAFKA_PORT is not set")
      .parse::<usize>().expect("Cannot parse KAFKA_PORT")
  };

  let kafka_client = KafkaRestClient::new(kafka_host, kafka_port);

  let device_id = register_device(&kafka_client, "Raspberry Pi");

  loop {
    println!("KAFKA: {}", kafka_client.url());

    match sys_stats() {
      Ok(mut stats) => {
        for v in &mut stats {
          v.as_object_mut().unwrap().insert("device_id".into(), json!(device_id));
        }

        println!("INFO: {}", to_string_pretty(&json!(stats)).unwrap());

        let records = stats.into_iter().map(|s| json!({"value": s})).collect::<Vec<_>>();

        match kafka_client.post("sensor", &records) {
          Ok(json_response) => println!("RESPONSE: {}", to_string_pretty(&json_response).unwrap()),
          Err(err) => eprintln!("ERROR: {}", err.to_string()),
        }
      },
      Err(err) => eprintln!("ERROR: {}", err.to_string()),
    }

    thread::sleep(Duration::from_secs(15));
  }
}
