//! Command-line interface definition.
{{- if or .Computed.feature_file_logging .Computed.feature_config_file }}

use std::path::PathBuf;
{{- end }}

use clap::{Args, Parser, Subcommand, ValueEnum};
{{- if .Computed.feature_config_file }}
use serde::Deserialize;
{{- end }}
use tracing_subscriber::filter::LevelFilter;

use crate::{build_info, commands};

#[derive(Debug, Parser)]
#[command(name = "{{ .ProjectKebab }}", version = build_info::VERSION, about, long_about = None)]
pub(crate) struct Cli {
    #[command(flatten)]
    pub(crate) global: GlobalArgs,

    #[command(subcommand)]
    pub(crate) command: Command,
}

/// Flags shared by every command.
#[derive(Debug, Args)]
pub(crate) struct GlobalArgs {
    /// Log level
    #[arg(long, env = "LOG_LEVEL", value_enum, global = true)]
    pub(crate) log_level: Option<LogLevel>,

    /// Disable colored output
    #[arg(long, env = "NO_COLOR", global = true)]
    pub(crate) no_color: bool,
{{- if .Computed.feature_file_logging }}

    /// Path to the log file [default: data directory]
    #[arg(long, env = "LOG_FILE", global = true)]
    pub(crate) log_file: Option<PathBuf>,
{{- end }}
{{- if .Computed.feature_config_file }}

    /// Path to the config file [default: config directory]
    #[arg(long, env = "CONFIG_FILE", global = true)]
    pub(crate) config: Option<PathBuf>,
{{- end }}
{{- if .Computed.feature_json_output }}

    /// Write output as JSON
    #[arg(long, global = true)]
    pub(crate) json: bool,
{{- end }}
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
{{- range .Scaffold.commands }}
    /// Run the {{ . }} command
    {{ . | toPascalCase }}(commands::{{ . | toSnakeCase }}::Args),
{{- end }}
    // +scaffold:command:variant
    /// Print a shell completion script
    Completions(commands::completions::Args),
}

{{- if .Computed.feature_config_file }}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, ValueEnum, Deserialize)]
#[serde(rename_all = "lowercase")]
{{- else }}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, ValueEnum)]
{{- end }}
pub(crate) enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => Self::TRACE,
            LogLevel::Debug => Self::DEBUG,
            LogLevel::Info => Self::INFO,
            LogLevel::Warn => Self::WARN,
            LogLevel::Error => Self::ERROR,
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory as _;

    use super::*;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }
}
