//! Tracing setup. Logs go to stderr so stdout stays free for command output.
{{- if .Computed.feature_file_logging }}

use std::fs;
use std::path::Path;
{{- end }}

use anyhow::Context as _;
{{- if .Computed.feature_file_logging }}
use tracing_appender::non_blocking::WorkerGuard;
{{- end }}
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

use crate::cli::LogLevel;
{{- if .Computed.feature_file_logging }}

/// Installs the global subscriber, logging to stderr and to `log_file`.
///
/// Keep the returned guard alive until exit: dropping it flushes the file writer.
pub(crate) fn init(level: LogLevel, color: bool, log_file: &Path) -> anyhow::Result<WorkerGuard> {
    let dir = log_file.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(dir).with_context(|| format!("creating log directory {}", dir.display()))?;
    let file_name = log_file
        .file_name()
        .with_context(|| format!("log file path {} has no file name", log_file.display()))?;

    let (file_writer, guard) =
        tracing_appender::non_blocking(tracing_appender::rolling::never(dir, file_name));

    tracing_subscriber::registry()
        .with(LevelFilter::from(level))
        .with(fmt::layer().with_writer(std::io::stderr).with_ansi(color))
        .with(fmt::layer().with_writer(file_writer).with_ansi(false))
        .try_init()
        .context("installing the tracing subscriber")?;

    Ok(guard)
}
{{- else }}

/// Installs the global subscriber, logging to stderr.
pub(crate) fn init(level: LogLevel, color: bool) -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(LevelFilter::from(level))
        .with(fmt::layer().with_writer(std::io::stderr).with_ansi(color))
        .try_init()
        .context("installing the tracing subscriber")
}
{{- end }}
