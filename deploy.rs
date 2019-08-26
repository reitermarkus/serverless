#!/usr/bin/env cargo eval --

//! ```cargo
//! [dependencies]
//! clap = "2.33"
//! rand = "0.5"
//! curl = "0.4"
//! which = "2"
//! hostname = "0.1"
//! failure = "0.1"
//! dockworker = { git = "https://github.com/reitermarkus/dockworker" }
//! ```

use std::{
  env,
  error::Error,
  fs::{self, File},
  io::prelude::*,
  process::{exit, Command, ExitStatus, Stdio},
};

use clap::{App, Arg, SubCommand, AppSettings, values_t};

use curl::easy::Easy;

use dockworker::{Docker, models::UpdateStatus};

use failure::ResultExt;

use rand::{distributions::Alphanumeric, prelude::*};

use which::which;

use hostname::get_hostname;

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

fn main() -> Result<(), Box<dyn Error>> {
  let matches = App::new("Deploy")
                  .arg(Arg::with_name("no-auth")
                    .short("n")
                    .long("no-auth")
                    .help("Deploys swarm without authentication")
                  )
                  .arg(Arg::with_name("restart")
                    .short("r")
                    .long("restart")
                    .takes_value(true)
                    .min_values(1)
                    .multiple(true)
                    .help("Restarts individual services")
                  )
                  .subcommand(SubCommand::with_name("func")
                    .setting(AppSettings::TrailingVarArg)
                    .arg(Arg::with_name("restart")
                      .short("r")
                      .long("restart")
                    )
                    .arg(Arg::with_name("functions")
                      .multiple(true)
                    )
                  )
                  .get_matches();

  if which("docker").is_err() {
    eprintln!("Cannot find `docker` command, please install Docker (https://www.docker.com/) and retry.");
    exit(1);
  }

  if which("faas-cli").is_err() {
    if cfg!(target_os = "macos") {
      Command::new("brew").args(&["install", "faas-cli"]).status().unwrap();
    } else if cfg!(target_os = "windows") {
      if which("scoop").is_ok() {
        Command::new("choco").args(&["install", "faas-cli", "-y"]).status().unwrap();
      } else if which("choco").is_ok() {
        Command::new("scoop").args(&["install", "faas-cli"]).status().unwrap();
      }
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
    docker.swarm().init().finish().compat()?;
  }

  if matches.is_present("restart") {
    if let Ok(services) = values_t!(matches, "restart", String) {
      for service in services {
        println!("Restarting {} …", service);

        let service_clone = service.clone();

        let service_info = docker.service_inspect(&service_clone, None).unwrap();

        let version = service_info.version.index;
        let mut spec = service_info.spec;

        spec.task_template.force_update += 1;

        let res = docker.service_update(&service_info.id, version, None, None, &spec);

        if res.is_ok() {
          loop {
            let service_info = docker.service_inspect(&service_clone, None).unwrap();

            if let Some(status) = service_info.update_status {
              match status {
                UpdateStatus::Completed { .. } => break,
                _ => continue,
              }
            }
          }

          println!("Restarted {}.", service);
        } else {
          eprintln!("Failed to restart {}.", service);
        }
      }

      return Ok(())
    }
  }

  if let Some(sub_matches) = matches.subcommand_matches("func") {
    let functions = values_t!(sub_matches, "functions", String).unwrap();

    let exit_on_error = |exit_status: ExitStatus| {
      if !exit_status.success() {
        exit(exit_status.code().unwrap_or(1));
      }
    };

    for function in functions {
      let yml = format!("{}.yml", function);

      exit_on_error(Command::new("faas-cli")
        .current_dir("functions")
        .args(&["build", "-f", &yml, &function])
        .status().unwrap());

      if sub_matches.is_present("restart") {
        println!("Restarting function '{}' …", function);

        Command::new("faas-cli")
          .current_dir("functions")
          .args(&["remove", "-f", &yml, &function])
          .status().unwrap();
      }

      exit_on_error(Command::new("faas-cli")
        .current_dir("functions")
        .args(&["deploy", "-f", &yml, &function])
        .status().unwrap());
    }

    return Ok(())
  }

  let user = "admin";
  let password: String = thread_rng().sample_iter(&Alphanumeric).take(16).collect();

  docker.secret_create("basic-auth-user", user).compat()?;
  docker.secret_create("basic-auth-password", &password).compat()?;

  println!("secret is: {}", password);

  let mongo_username = "admin";
  let mongo_password = "password";

  docker.secret_create("mongo-root-username", mongo_username).compat()?;
  docker.secret_create("mongo-root-password", mongo_password).compat()?;

  env::set_var("ME_CONFIG_MONGODB_ADMINUSERNAME", mongo_username);
  env::set_var("ME_CONFIG_MONGODB_ADMINPASSWORD", mongo_password);

  if matches.is_present("no-auth") {
    println!("Disabling basic authentication…");
    env::set_var("BASIC_AUTH", "false");
  } else {
    println!("Enabling basic authentication…");
    env::set_var("BASIC_AUTH", "true");
  }

  let hostname = get_hostname().unwrap();
  println!("Setting Kafka hostname to “{}”…", hostname);
  env::set_var("KAFKA_PUBLIC_HOSTNAME", hostname);

  fs::create_dir_all("faas/prometheus")?;
  fs::copy("deploy.yml", "faas/deploy.yml")?;

  let database_dir = env::current_dir()?.join("faas").join("db-data");
  fs::create_dir_all(&database_dir)?;
  env::set_var("DATABASE_DIR", database_dir);

  curl_download!("https://raw.githubusercontent.com/openfaas/faas/master/prometheus/alertmanager.yml", "faas/prometheus/alertmanager.yml");
  curl_download!("https://raw.githubusercontent.com/openfaas/faas/master/prometheus/alert.rules.yml", "faas/prometheus/alert.rules.yml");
  curl_download!("https://raw.githubusercontent.com/openfaas/faas/master/prometheus/prometheus.yml", "faas/prometheus/prometheus.yml");

  println!("Deploying stack…");

  docker.stack_deploy_with_compose_file("func", "./faas/deploy.yml", None).compat()?;

  Ok(())
}
