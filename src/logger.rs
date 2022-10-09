use fast_log::appender::*;
use log::{Level, LevelFilter, Log, Metadata, Record};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Method {
    None,
    Stdout,
    File,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum MaxLevel {
    Debug,
    Info,
    Error,
    Trace,
}

impl MaxLevel {
    pub fn to_level_filter(&self) -> LevelFilter {
        match self {
            Self::Debug => LevelFilter::Debug,
            Self::Info => LevelFilter::Info,
            Self::Error => LevelFilter::Error,
            Self::Trace => LevelFilter::Trace,
        }
    }
}

pub struct Logger;

impl LogAppender for Logger {
    fn do_logs(&self, records: &[FastLogRecord]) {}
}
