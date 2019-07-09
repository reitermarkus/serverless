use std::fs;
use std::io;

pub fn secret(name: &str) -> Result<String, io::Error> {
  match fs::read_to_string(&format!("/var/openfaas/secrets/{}", name)) {
    Err(ref e) if e.kind() == io::ErrorKind::NotFound => fs::read_to_string(&format!("/run/secrets/{}", name)),
    res => res,
  }
}
