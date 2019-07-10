use futures::future::{self, Future};
use http::{HeaderMap, Method, Uri, StatusCode};

pub fn handle(method: Method, uri: Uri, headers: HeaderMap, body: String) -> impl Future<Item = (StatusCode, String), Error = StatusCode> {
  future::ok((StatusCode::OK, "It works!".to_string()))
}
