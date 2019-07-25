#![feature(async_await)]

use futures::TryStreamExt;
use hyper::{Body, Server, Request, Response, StatusCode, service::{make_service_fn, service_fn}};

use handler::handle;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
  let addr = ([127, 0, 0, 1], 7878).into();

  let make_service = make_service_fn(|_| async {
    Ok::<_, hyper::Error>(service_fn(|request: Request<Body>| async {
      let method = request.method().clone();
      let uri = request.uri().clone();
      let headers = request.headers().clone();

      let bytes = request.into_body().try_concat().await?.to_vec();

      let body = match String::from_utf8(bytes) {
        Ok(body) => body,
        Err(_) => return Ok::<_, hyper::Error>(Response::builder()
          .status(StatusCode::BAD_REQUEST)
          .body(Body::from(""))
          .unwrap()),
      };

      match handle(method, uri, headers, body).await {
        Ok((status_code, response)) => Ok(Response::builder()
          .status(status_code)
          .body(Body::from(response))
          .unwrap()),
        Err(err) => Ok(Response::builder()
          .status(StatusCode::INTERNAL_SERVER_ERROR)
          .body(Body::from(err.to_string()))
          .unwrap())
      }
    }))
  });

  let server = Server::bind(&addr)
    .serve(make_service);

  server.await
}
