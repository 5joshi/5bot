use flexi_logger::{
    filter::{LogLineFilter, LogLineWriter},
    Age, Cleanup, Criterion, DeferredNow, Duplicate, FileSpec, Logger, LoggerHandle, Naming,
};
use log::Record;
use once_cell::sync::OnceCell;
use std::io::{Result as IoResult, Write};
use time::{format_description::FormatItem, macros::format_description};

static LOGGER: OnceCell<LoggerHandle> = OnceCell::new();

lazy_static! {
    static ref FORMAT: &'static [FormatItem<'static>] =
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
}

pub struct GoodCratesOnly;
impl LogLineFilter for GoodCratesOnly {
    fn write(
        &self,
        now: &mut DeferredNow,
        record: &log::Record,
        log_line_writer: &dyn LogLineWriter,
    ) -> std::io::Result<()> {
        if !(record.target().contains("songbird") || record.target().contains("tracing")) {
            log_line_writer.write(now, record)?;
        }
        Ok(())
    }
}

pub fn initialize() {
    let file_spec = FileSpec::default().directory("logs");

    let logger_handle = Logger::try_with_str("fivebot")
        .unwrap()
        .filter(Box::new(GoodCratesOnly))
        .log_to_file(file_spec)
        .format(log_format)
        .format_for_files(log_format_files)
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogAndCompressedFiles(5, 20),
        )
        .duplicate_to_stdout(Duplicate::Info)
        .start_with_specfile("logconfig.toml")
        .expect("Failed to make logger.");

    let _ = LOGGER.set(logger_handle);
}

pub fn log_format(w: &mut dyn Write, now: &mut DeferredNow, record: &Record) -> IoResult<()> {
    write!(
        w,
        "[{}] {} {}",
        now.now().format(&FORMAT).unwrap(),
        record.level(),
        &record.args()
    )
}

pub fn log_format_files(w: &mut dyn Write, now: &mut DeferredNow, record: &Record) -> IoResult<()> {
    write!(
        w,
        "[{}] {:^5} [{}:{}] {}",
        now.now().format(&FORMAT).unwrap(),
        record.level(),
        record.file_static().unwrap_or_else(|| record.target()),
        record.line().unwrap_or(0),
        &record.args()
    )
}
