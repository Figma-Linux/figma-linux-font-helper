use crate::config::Config;

pub fn is_valid_file(config: &Config, file_path: &String) -> bool {
  if file_path.contains("..") {
    return false;
  }

  for dir in &config.directories {
    if file_path.starts_with(dir) {
      return true;
    }
  }

  false
}
