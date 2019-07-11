#![feature(async_await)]

use std::env;
use std::error::Error;
use std::fs;
use std::io;

use futures01::{future::Future, stream::Stream};
use futures::compat::Future01CompatExt;
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

pub async fn call(function: &str, body: String) -> Result<(StatusCode, String), Box<dyn Error + Send>> {
  let response = Client::new()
                   .post(&format!("{}/function/{}", gateway_url(), function))
                   .body(body)
                   .send()
                   .map_err(|err| Box::new(err) as _)
                   .compat()
                   .await?;

  let status = response.status();

  let body = response.into_body()
               .concat2()
               .map_err(|err| Box::new(err) as _)
               .compat()
               .await?;

  match String::from_utf8(body.to_vec()) {
    Ok(content) => Ok((status, content)),
    Err(err) => Err(Box::new(err) as Box<dyn Error + Send>),
  }
}
