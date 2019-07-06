use std::io::{self, stdin, Read};

use handler::handle;

fn main() -> io::Result<()> {
  let mut buffer = String::new();

  stdin().lock().read_to_string(&mut buffer)
    .map(|_| println!("{}", handle(buffer)))
}
