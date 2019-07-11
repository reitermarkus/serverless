#![feature(async_await)]

use std::fmt::{self, Display, Debug};
use std::error::Error;

use futures01::{future::{self, Future}, stream::Stream};
use futures::future::{FutureExt, TryFutureExt};
use gotham::{self, handler::{HandlerFuture, IntoHandlerError}, helpers::http::response::create_response, router::{builder::*, Router}, state::{FromState, State}};
use hyper::Body;
use http::{Method, HeaderMap, Uri};
use mime;

use handler::handle;

struct HandlerErrorWrapper {
  cause: Box<dyn Error + Send>,
}

impl Display for HandlerErrorWrapper {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    Display::fmt(&*self.cause, f)
  }
}

impl Debug for HandlerErrorWrapper {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    Debug::fmt(&*self.cause, f)
  }
}

impl Error for HandlerErrorWrapper {
  fn description(&self) -> &str {
    ""
  }

  fn cause(&self) -> Option<&dyn Error> {
    Some(&*self.cause)
  }
}

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
    .then(|res| {
      let f = async {
        match res {
          Ok(body) => {
            let method = http::Method::borrow_from(&state).to_owned();
            let headers = HeaderMap::borrow_from(&state).to_owned();
            let uri = Uri::borrow_from(&state).to_owned();

            match handle(method, uri, headers, body).await {
              Ok((status_code, response)) => {
                let res = create_response(
                    &state,
                    status_code,
                    mime::TEXT_PLAIN,
                    response,
                );

                Ok((state, res))
              },
              Err(err) => Err((state, HandlerErrorWrapper { cause: err }.into_handler_error())),
            }
          },
          Err(err) => Err((state, err.into_handler_error())),
        }
      };

      f.boxed().compat()
    });

  Box::new(f)
}

fn router() -> Router {
  build_simple_router(|route| {
    route.request(vec![Method::GET, Method::PUT, Method::POST, Method::DELETE], "/").to(handler);
    route.request(vec![Method::GET, Method::PUT, Method::POST, Method::DELETE], "/*").to(handler);
  })
}

fn main() {
  gotham::start("127.0.0.1:7878", router())
}
