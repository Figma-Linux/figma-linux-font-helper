extern crate flexi_logger;
extern crate xdg;

use flexi_logger::{opt_format, FileSpec, Logger};

pub fn init() {
  let xdg_dirs = xdg::BaseDirectories::with_prefix("figma-fonthelper").unwrap();

  Logger::try_with_env_or_str(
    "font_helper=debug, finder=debug, libfonthelper=debug, simple_server=info",
  )
  .unwrap()
  .log_to_file(
    FileSpec::default()
      .directory(xdg_dirs.get_cache_home())
      .basename("font-helper")
      .suppress_timestamp()
      .suffix("log"),
  )
  .format(opt_format)
  .start()
  .unwrap();
}
