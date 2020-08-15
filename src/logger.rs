use colored::*;
pub use log::{debug, error, info, trace, warn};
use log::{LevelFilter, Metadata, Record};

pub struct Logger;
impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Trace
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                log::Level::Error => eprintln!("{} - {}", "Error".red(), record.args()),
                log::Level::Warn => eprintln!("{} - {}", "WARN".red(), record.args()),
                log::Level::Info => eprintln!("{} - {}", "INFO".cyan(), record.args()),
                log::Level::Debug => eprintln!("{} - {}", "DEBUG".blue().bold(), record.args()),
                log::Level::Trace => eprintln!("{} - {}", "TRACE".blue(), record.args()),
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                log::Level::Error => gwg::warn!("[Error] {}", record.args()),
                log::Level::Warn => gwg::warn!("[Warn] {}", record.args()),
                log::Level::Info => gwg::info!("[Info] {}", record.args()),
                log::Level::Debug => gwg::debug!("[Debug] {}", record.args()),
                log::Level::Trace => gwg::debug!("[Trace] {}", record.args()),
            }
        }
    }
    fn flush(&self) {}
}

pub fn init_logger(logger: &'static Logger, log_level: &str) {
    log::set_logger(logger).unwrap();
    if log_level == "trace" || log_level == "all" {
        log::set_max_level(LevelFilter::Trace);
    } else if log_level == "debug" {
        log::set_max_level(LevelFilter::Debug);
    } else if log_level == "info" {
        log::set_max_level(LevelFilter::Info);
    } else if log_level == "warn" {
        log::set_max_level(LevelFilter::Warn);
    } else {
        log::set_max_level(LevelFilter::Warn);
    }
}
