use crate::config::LogConfig;
use anyhow::Result;
use tracing::{level_filters::LevelFilter, subscriber::set_global_default};
use tracing_appender::{
    non_blocking,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{Layer, filter::Targets, fmt, layer::SubscriberExt};

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
            .json()
            .with_writer(writer)
            .with_ansi(false)
            .with_span_list(false)
            .with_current_span(false)
            .with_file(true)
            .with_level(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_thread_names(false);
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
            .with_target(false)
            .with_file(true)
            .with_level(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_thread_names(false);
        layers.push(layer.boxed());
    }

    set_global_default(tracing_subscriber::registry().with(layers))?;
    Ok(())
}
