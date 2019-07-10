use std::env;
use std::fs;
use std::io;

use futures::{future::Future, stream::Stream};
use http::StatusCode;
use reqwest::r#async::Client;

pub fn secret(name: &str) -> Result<String, io::Error> {
  match fs::read_to_string(&format!("/var/openfaas/secrets/{}", name)) {
    Err(ref e) if e.kind() == io::ErrorKind::NotFound => fs::read_to_string(&format!("/run/secrets/{}", name)),
    res => res,
  }
}

pub fn gateway_url() -> String {
  env::var("gateway_url").unwrap_or_else(|_| "http://gateway:8080".to_string())
}

pub fn call(function: &str, body: String) -> impl Future<Item = (StatusCode, String), Error = Option<StatusCode>> {
  Client::new()
    .post(&format!("{}/function/{}", gateway_url(), function))
    .body(body)
    .send()
    .map_err(|err| err.status())
    .and_then(|response| {
      let status = response.status();

      response.into_body()
              .concat2()
              .map_err(|err| err.status())
              .and_then(move |body| {
                match String::from_utf8(body.to_vec()) {
                  Ok(content) => Ok((status, content)),
                  Err(_) => Err(None),
                }
              })
    })
}
