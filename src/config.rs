use log::warn;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;
use std::fs;
extern crate xdg;

static SETTINGS_FILE: &str = "settings.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
  #[serde(default = "default_host")]
  pub host: String,
  #[serde(default = "default_port")]
  pub port: String,
  pub app: AppConfig,
}
#[derive(Serialize, Deserialize)]
pub struct AppConfig {
  #[serde(rename = "fontDirs")]
  pub font_dirs: Vec<String>,
}

fn default_host() -> String {
  "127.0.0.1".to_string()
}
fn default_port() -> String {
  "18412".to_string()
}

impl fmt::Debug for Config {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut print = vec!["Config (".to_owned()];

    print.push(format!("host: {}, ", self.host));
    print.push(format!("port: {}, ", self.port));
    print.push(format!(
      "directories: [{}])",
      &self.app.font_dirs.join(", ")
    ));

    write!(f, "{}", print.join(""))
  }
}

impl Config {
  pub fn new() -> Self {
    let config = Config::parse_config_file();

    Config {
      host: config.host,
      port: config.port,
      app: AppConfig {
        font_dirs: config.app.font_dirs,
      },
    }
  }

  fn parse_config_file() -> Config {
    let mut config = Config {
      host: "127.0.0.1".to_owned(),
      port: "18412".to_owned(),
      app: AppConfig {
        font_dirs: vec![String::from("/usr/share/fonts")],
      },
    };

    let xdg_dirs = xdg::BaseDirectories::with_prefix("figma-linux").unwrap();
    let path = xdg_dirs.find_config_file(SETTINGS_FILE);

    match path {
      None => {
        let json = serde_json::to_string(&config).unwrap();
        let config_path = xdg_dirs
          .place_config_file(SETTINGS_FILE)
          .expect(&format!("Cannot create {}", SETTINGS_FILE));

        fs::write(&config_path, json.as_bytes()).expect(&format!(
          "Cannot create file {}",
          &config_path.to_str().unwrap()
        ));
      }
      Some(path) => {
        match fs::read(&path) {
          Err(err) => warn!("Cannot read the config file, ERROR: {}", err),
          Ok(file) => {
            config = serde_json::from_str(&String::from_utf8(file).unwrap()).unwrap();
          }
        };
      }
    }

    config
  }
}
