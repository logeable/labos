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
            match record.level() {
                Level::Error => println!("\x1b[31m{} - {}\x1b[0m", record.level(), record.args()),
                Level::Warn => println!("\x1b[93m{} - {}\x1b[0m", record.level(), record.args()),
                Level::Info => println!("\x1b[36m{} - {}\x1b[0m", record.level(), record.args()),
                Level::Debug => println!("\x1b[37m{} - {}\x1b[0m", record.level(), record.args()),
                Level::Trace => println!("\x1b[90m{} - {}\x1b[0m", record.level(), record.args()),
            };
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    let log_level = option_env!("LOG").unwrap_or("INFO");
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::from_str(log_level).unwrap()))
}
