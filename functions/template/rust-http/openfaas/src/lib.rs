use std::env;
use std::error::Error;
use std::fs;
use std::io;

use futures::{TryFutureExt, TryStreamExt};
use hyper::{Client, Request, Body, StatusCode};

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
  let request = Request::post(format!("{}/function/{}", gateway_url(), function))
    .body(Body::from(body))
    .unwrap();

  let response = Client::new()
    .request(request)
    .map_err(|err| Box::new(err) as _)
    .await?;

  let status = response.status();

  let bytes = response.into_body()
    .try_concat()
    .map_err(|err| Box::new(err) as _)
    .await?.to_vec();

  match String::from_utf8(bytes) {
    Ok(content) => Ok((status, content)),
    Err(err) => Err(Box::new(err) as _),
  }
}
