extern crate flexi_logger;
extern crate xdg;

use flexi_logger::{opt_format, Logger};

pub fn init() {
  let xdg_dirs = xdg::BaseDirectories::with_prefix("figma-fonthelper").unwrap();

  Logger::with_env_or_str(
    "font_helper=debug, finder=debug, libfonthelper=debug, simple_server=info",
  )
  .log_to_file()
  .directory(xdg_dirs.get_cache_home())
  .format(opt_format)
  .start()
  .unwrap();
}
