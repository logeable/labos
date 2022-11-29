use core::option_env;
use core::str::FromStr;

use log::{Level, LevelFilter, SetLoggerError};

static LOGGER: Logger = Logger;
struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        let log_level = option_env!("LOG").unwrap_or("INFO");
        let level = Level::from_str(log_level).unwrap();
        metadata.level() <= level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug))
}
