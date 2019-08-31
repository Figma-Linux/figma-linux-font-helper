use log::warn;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{env, fs, path::Path};

pub struct Config {
  pub host: String,
  pub port: String,
  pub directories: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct ConfigFile {
  port: String,
  directories: Vec<String>,
}

impl Config {
  pub fn new() -> Self {
    let config = Config::parse_config_file();

    Config {
      host: "127.0.0.1".to_owned(),
      port: config.port,
      directories: config.directories,
    }
  }

  fn parse_config_file() -> ConfigFile {
    let home = String::from(&env::var("HOME").unwrap());
    let path = Path::new(&home).join(".config/figma-linux/FontHelper");

    let mut config = ConfigFile {
      port: "18412".to_owned(),
      directories: vec![
        String::from("/usr/share/fonts"),
        String::from(format!("{}/.local/share/fonts", &home)),
      ],
    };

    if path.exists() {
      match fs::read(&path) {
        Err(err) => warn!("Cannot read the config file, ERROR: {}", err),
        Ok(file) => {
          config = serde_json::from_str(&String::from_utf8(file).unwrap()).unwrap();
        }
      };
    } else {
      let json = serde_json::to_string(&config).unwrap();
      fs::create_dir_all(&path.parent().unwrap()).expect(&format!(
        "Cannot create all path {}",
        &path.parent().unwrap().to_str().unwrap()
      ));
      fs::write(&path, json.as_bytes())
        .expect(&format!("Cannot create file {}", &path.to_str().unwrap()));
    }

    config
  }
}
