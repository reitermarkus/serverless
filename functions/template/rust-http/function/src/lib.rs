use std::error::Error;

use http::{HeaderMap, Method, Uri, StatusCode};

pub async fn handle(method: Method, uri: Uri, headers: HeaderMap, body: String) -> Result<(StatusCode, String), Box<dyn Error + Send>> {
  Ok((StatusCode::OK, "It works!".to_string()))
}
