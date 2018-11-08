#!/usr/bin/env run-cargo-script

//! ```cargo
//! [dependencies]
//! clap = "~2.32.0"
//! ```

use std::error::Error;

use std::io::prelude::*;
use std::process::{Command, Stdio};

macro_rules! docker {
  () => {{
    Command::new("docker")
  }};
  ($e:expr) => {{
    docker!().arg(stringify!($e))
  }};
  ($($es:expr),+) => {{
    docker!().args(&[$($es),+])
  }};
}

macro_rules! docker_success {
  ($($t:tt),*) => {{
    docker!($($t),*).output()?.status.success()
  }}
}

macro_rules! docker_create_secret {
  ($name:expr, $secret:expr) => {{
    if ! docker_success!("secret", "inspect", $name) {
      docker!("secret", "create", $name, "-")
        .stdin(Stdio::piped())
        .spawn()?.stdin.unwrap().write_all($secret.as_bytes());
    }
  }};
}

fn main() -> Result<(), Box<Error>>  {
  docker!("swarm", "init").output()?;

  docker_create_secret!("basic-auth-user", "admin");
  docker_create_secret!("basic-auth-password", "password");

  Ok(())
}
