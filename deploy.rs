#!/usr/bin/env run-cargo-script

//! ```cargo
//! [dependencies]
//! clap = "~2.32.0"
//! rand = "~0.5.0"
//! curl = "~0.4.19"
//! which = "2.0.0"
//! ```

use std::{
  env,
  error::Error,
  fs::{self, File},
  io::prelude::*,
  process::{exit, Command, Stdio},
  thread,
};

#[macro_use]
extern crate clap;
use clap::{App, Arg};

extern crate rand;
use rand::{distributions::Alphanumeric, prelude::*};

extern crate curl;
use curl::easy::Easy;

extern crate which;
use which::which;

macro_rules! docker {
  () => {{
    Command::new("docker")
  }};
  ($e:expr) => {{
    docker!().arg($e)
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
    if !docker_success!("secret", "inspect", $name) {
      let mut process = docker!("secret", "create", $name, "-")
        .stdin(Stdio::piped())
        .spawn()?;

      process.stdin.as_mut().unwrap()
        .write_all($secret.as_bytes())?;

      process.wait()?;
    }
  }};
}

macro_rules! curl_download {
  ($url:expr, $target:expr) => {{
    let mut easy = Easy::new();
    easy.url($url)?;

    easy.write_function(|data| {
      File::create($target).expect("could not create file")
        .write_all(data).expect("could not write to file");
      Ok(data.len())
    })?;

    easy.perform()?;
  }}
}

fn main() -> Result<(), Box<Error>> {
  let matches = App::new("Deploy")
                  .arg(Arg::with_name("no-auth")
                    .short("n")
                    .long("no-auth")
                    .help("Deploys swarm without authentication"))
                  .arg(Arg::with_name("restart")
                    .short("r")
                    .long("restart")
                    .takes_value(true)
                    .min_values(1)
                    .multiple(true)
                    .help("Restarts individual services"))
                  .get_matches();

  if which("docker").is_err() {
    eprintln!("Cannot find `docker` command, please install Docker (https://www.docker.com/) and retry.");
    exit(1);
  }

  if which("faas-cli").is_err() {
    if cfg!(target_os = "macos") {
      Command::new("brew").args(&["install", "faas-cli"]).status().unwrap();
    } else if cfg!(target_os = "windows") {
      Command::new("choco").args(&["install", "faas-cli", "-y"]).status().unwrap();
    } else {
      let mut easy = Easy::new();
      easy.url("https://cli.openfaas.com")?;

      easy.write_function(move |data| {
        let mut process = Command::new("sudo").args(&["-E", "sh"]).stdin(Stdio::piped()).spawn().unwrap();
        process.stdin.as_mut().unwrap()
          .write_all(data).unwrap();
        process.wait().unwrap();
        Ok(data.len())
      })?;

      easy.perform()?;
    }
  }

  docker!("swarm", "init").output()?;

  let user = "admin";
  let password: String = thread_rng().sample_iter(&Alphanumeric).take(16).collect();

  docker_create_secret!("basic-auth-user", user);
  docker_create_secret!("basic-auth-password", password);
  println!("secret is: {}", password);

  if let Ok(services) = values_t!(matches, "restart", String) {
    let threads: Vec<_> = services.iter()
      .map(|service| {
        println!("Restarting {} …", service);

        let service_clone = service.clone();

        thread::spawn(move || {
          let output = docker!("service", "inspect", "--format", "{{ .ID }}", &service_clone).output().unwrap();
          let id = String::from_utf8_lossy(&output.stdout).trim_right().to_owned();
          let output = docker!("service", "update", "--force", &id).output().unwrap();
          (service_clone, id, output.status.to_owned())
        })
      })
      .collect();

    for t in threads {
      let (service, id, status) = t.join().unwrap();

      if status.success() {
        println!("Restarted {} ({}).", service, id);
      } else {
        eprintln!("Failed to restart {} ({}).", service, id);
      }
    }

    return Ok(())
  }

  if matches.is_present("no-auth") {
    println!("Disabling basic authentication…");
    env::set_var("BASIC_AUTH", "false");
  } else {
    println!("Enabling basic authentication…");
    env::set_var("BASIC_AUTH", "true");
  }

  fs::create_dir_all("faas/prometheus")?;
  fs::copy("deploy.yml", "faas/deploy.yml")?;

  curl_download!("https://raw.githubusercontent.com/openfaas/faas/master/prometheus/alertmanager.yml", "faas/prometheus/alertmanager.yml");
  curl_download!("https://raw.githubusercontent.com/openfaas/faas/master/prometheus/alert.rules.yml", "faas/prometheus/alert.rules.yml");
  curl_download!("https://raw.githubusercontent.com/openfaas/faas/master/prometheus/prometheus.yml", "faas/prometheus/prometheus.yml");

  docker!("stack", "deploy", "func", "--compose-file", "deploy.yml")
    .current_dir("faas")
    .status()
    .unwrap();

  Ok(())
}
