use time::{format_description, UtcOffset};
use tracing::Level;
use tracing_subscriber::{
    fmt::{self, time::OffsetTime, writer::MakeWriterExt},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

/// init the log
pub fn init_log() {

    // format the log time output, and handle the time zone problem
    let format = "[year]-[month]-[day][hour]:[minute]:[second].[subsecond digits:3]";
    let time_format = OffsetTime::new(
        UtcOffset::current_local_offset().unwrap(),
        format_description::parse(format).unwrap(),
    );

    // info config
    let info_log_file_appender = tracing_appender::rolling::daily("./logs/info", "app-info.log")
        .with_max_level(Level::INFO)
        .with_min_level(Level::INFO);

    // warn config
    let warn_log_file_appender = tracing_appender::rolling::daily("./logs/warn", "app-warn.log")
        .with_max_level(Level::WARN)
        .with_min_level(Level::WARN);

    // error config
    let error_log_file_appender = tracing_appender::rolling::daily("./logs/error", "app-error.log")
        .with_max_level(Level::ERROR)
        .with_min_level(Level::ERROR);

    let all_files_appender = info_log_file_appender
        .and(warn_log_file_appender)
        .and(error_log_file_appender);

    // file output
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(all_files_appender)
        .with_line_number(true)
        .with_timer(time_format.clone())
        .compact();

    // console output
    let formatting_layer = fmt::layer()
        .with_writer(std::io::stdout.with_max_level(tracing::Level::DEBUG).with_min_level(tracing::Level::ERROR))
        .with_line_number(true)
        .with_timer(time_format.clone());

    tracing_subscriber::registry()
        .with(file_layer)
        .with(formatting_layer)
        .init();
}
