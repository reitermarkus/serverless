use http::{HeaderMap, Method, Uri};

pub fn handle(method: Method, uri: Uri, headers: HeaderMap, body: String) -> String {
  format!("{:?} {:?} {:?} {:?}", method, uri, headers, body)
}
