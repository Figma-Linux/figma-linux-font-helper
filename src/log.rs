extern crate flexi_logger;

use flexi_logger::{Logger, opt_format};

pub fn init() {
    Logger::with_env_or_str("font_helper=debug, simple_server=info")
        .log_to_file()
        .directory("/var/log/fonthelper")
        .format(opt_format)
        .start()
        .unwrap();
}
