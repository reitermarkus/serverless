#!/usr/bin/env run-cargo-script

//! ```cargo
//! [dependencies]
//! clap = "~2.32.0"
//! ```

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
    docker!($($t),*)
      .output()
      .expect("failed to execute `docker`")
      .status
      .success()
  }}
}

macro_rules! docker_create_secret {
  ($name:expr, $secret:expr) => {{
    if ! docker_success!("secret", "inspect", $name) {
      let process = docker!("secret", "create", $name, "-")
                      .stdin(Stdio::piped())
                      .spawn()
                      .expect("failed to execute `docker`");
      process.stdin.unwrap().write_all($secret.as_bytes());
    }
  }};
}

fn main() {
  docker!("swarm", "init")
    .output()
    .expect("failed to execute `docker`");

  docker_create_secret!("basic-auth-user", "admin");
  docker_create_secret!("basic-auth-password", "password");
}
