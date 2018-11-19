#!/usr/bin/env run-cargo-script

//! ```cargo
//! [dependencies]
//! clap = "~2.32.0"
//! rand = "~0.5.0"
//! curl = "~0.4.19"
//! which = "2.0.0"
//! dockworker = { git = "git://github.com/reitermarkus/dockworker.git" }
//! failure = "0.1"
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
use clap::{App, Arg, SubCommand};

extern crate curl;
use curl::easy::Easy;

extern crate dockworker;
use dockworker::Docker;

extern crate failure;
use failure::ResultExt;

extern crate rand;
use rand::{distributions::Alphanumeric, prelude::*};

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

macro_rules! curl_download {
  ($url:expr, $target:expr) => {{
    let mut handle = Easy::new();
    handle.url($url)?;

    let mut buf = Vec::new();

    {
      let mut transfer = handle.transfer();
      transfer.write_function(|data| {
        buf.extend_from_slice(data);
        return Ok(data.len())
      })?;
      transfer.perform()?;
    }

    let mut file = File::create($target)?;

    file.write_all(&buf)?;
    file.sync_all()?;
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
                  .subcommand(SubCommand::with_name("func")
                    .about("Interacts with \"faas-cli\"")
                    .arg(Arg::with_name("deploy")
                      .short("d")
                      .long("deploy")
                      .help("Deploy a function")
                      .takes_value(true)
                      .min_values(1)
                      .max_values(1)
                      .multiple(true))
                    .arg(Arg::with_name("build")
                      .short("b")
                      .long("build")
                      .help("Build a function")
                      .takes_value(true)
                      .min_values(1)
                      .max_values(1)
                      .multiple(true)))
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

  let docker = Docker::from_env().compat()?;
  let swarm_state = docker.system_info().compat()?.swarm.local_node_state;

  if swarm_state != "active" {
    docker.swarm_init(None, None, None, None).compat()?;
  }

  if matches.is_present("restart") {
    if let Ok(services) = values_t!(matches, "restart", String) {
      let threads: Vec<_> = services.iter()
        .map(|service| {
          println!("Restarting {} …", service);

          let service_clone = service.clone();
          let id = docker.service_inspect(&service_clone, None).unwrap().id;

          thread::spawn(move || {
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
  }

  if let Some(sub_matches) = matches.subcommand_matches("func") {
    if sub_matches.is_present("build") {
      Command::new("faas-cli").args(&["build", "-f", &value_t!(sub_matches, "build", String).unwrap()]).status().unwrap();
      return Ok(())
    } else if sub_matches.is_present("deploy") {
      Command::new("faas-cli").args(&["deploy", "-f", &value_t!(sub_matches, "deploy", String).unwrap()]).status().unwrap();
      return Ok(())
    }
  }

  let user = "admin";
  let password: String = thread_rng().sample_iter(&Alphanumeric).take(16).collect();

  docker.secret_create("basic-auth-user", user).compat()?;
  docker.secret_create("basic-auth-password", &password).compat()?;

  println!("secret is: {}", password);

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
