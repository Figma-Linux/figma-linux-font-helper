use std::{env, process};

pub fn init() {
  let mut args = env::args();
  args.next();

  if let Some(version) = args.next() {
    if version == "-v" {
      println!("{}", env!("CARGO_PKG_VERSION"));
      process::exit(0);
    }
  }
}
