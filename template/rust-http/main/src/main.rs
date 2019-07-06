use futures::{future, Future, Stream};
use gotham;
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::{FromState, State};
use hyper::Body;
use http::{Method, HeaderMap, Uri, StatusCode};
use gotham::helpers::http::response::create_response;
use mime;

pub fn handler(mut state: State) -> Box<HandlerFuture> {
  let f = Body::take_from(&mut state)
    .concat2()
    .then(|res| match res {
      Ok(body) => {
        match String::from_utf8(body.to_vec()) {
          Ok(content) => future::ok(content),
          Err(err) => future::err(err.into_handler_error()),
        }
      },
      Err(err) => future::err(err.into_handler_error()),
  })
  .then(|res| match res {
    Ok(body) => {
      let method = http::Method::borrow_from(&state).to_owned();
      let headers = HeaderMap::borrow_from(&state).to_owned();
      let uri = Uri::borrow_from(&state).to_owned();

      let res = create_response(
          &state,
          StatusCode::OK,
          mime::TEXT_PLAIN,
          handle(method, uri, headers, body),
      );

      future::ok((state, res))
    },
    Err(err) => future::err((state, err)),
  });

  Box::new(f)
}

use handler::handle;


fn router() -> Router {
  build_simple_router(|route| {
    route.request(vec![Method::GET, Method::PUT, Method::POST, Method::DELETE], "/").to(handler);
    route.request(vec![Method::GET, Method::PUT, Method::POST, Method::DELETE], "/*").to(handler);
  })
}

fn main() {
  gotham::start("127.0.0.1:7878", router())
}
