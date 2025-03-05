use crate::config::LogConfig;
use anyhow::Result;
use chrono::Utc;
use tracing::{level_filters::LevelFilter, subscriber::set_global_default};
use tracing_appender::{
    non_blocking,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{
    Layer,
    filter::Targets,
    fmt::{self, time::FormatTime},
    layer::SubscriberExt,
};

struct CustomizeTime;

impl FormatTime for CustomizeTime {
    fn format_time(&self, w: &mut fmt::format::Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Utc::now().format("%H:%M:%S"))
    }
}

pub fn init(config: &LogConfig) -> Result<()> {
    let mut layers = Vec::new();

    if !config.targets.is_empty() {
        let layer = Targets::new().with_targets(
            config
                .targets
                .iter()
                .map(|target| (target, LevelFilter::TRACE)),
        );
        layers.push(layer.boxed());
    }

    if config.console {
        let (writer, guard) = non_blocking(std::io::stdout());
        std::mem::forget(guard);
        let layer = fmt::layer()
            .with_writer(writer)
            .with_ansi(config.color)
            .with_target(config.target)
            .with_file(false)
            .with_level(true)
            .with_line_number(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_timer(CustomizeTime);
        layers.push(layer.boxed());
    }

    if config.file {
        let rolling_file = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .filename_suffix("log")
            .build(&config.filedir)?;
        let (writer, guard) = non_blocking(rolling_file);
        std::mem::forget(guard);
        let layer = fmt::layer()
            .with_writer(writer)
            .with_ansi(false)
            .with_target(config.target)
            .with_file(false)
            .with_level(true)
            .with_line_number(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_timer(CustomizeTime);
        layers.push(layer.boxed());
    }

    set_global_default(tracing_subscriber::registry().with(layers))?;
    Ok(())
}
