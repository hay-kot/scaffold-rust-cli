//! {{ .Scaffold.description }}

mod build_info;
mod cli;
mod commands;
{{- if .Computed.feature_config_file }}
mod config;
{{- end }}
mod logging;
{{- if .Computed.feature_paths }}
mod paths;
{{- end }}

use std::io::{self, IsTerminal as _};
use std::process::ExitCode;

use clap::{CommandFactory as _, FromArgMatches as _};

use crate::cli::{Cli, Command};

fn main() -> ExitCode {
    let matches = Cli::command().long_version(build_info::long_version()).get_matches();
    let cli = match Cli::from_arg_matches(&matches) {
        Ok(cli) => cli,
        Err(err) => err.exit(),
    };

    let color = !cli.global.no_color && io::stderr().is_terminal();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            report(&err, color);
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> anyhow::Result<()> {
{{- if .Computed.feature_config_file }}
    let config = match &cli.global.config {
        Some(path) => config::Config::read_from(path)?,
        None => config::Config::read()?,
    };
    let log_level = cli.global.log_level.or(config.log_level).unwrap_or_default();
{{- else }}
    let log_level = cli.global.log_level.unwrap_or_default();
{{- end }}
{{- if .Computed.feature_file_logging }}
{{- if .Computed.feature_config_file }}
    let log_file = match cli.global.log_file.or(config.log_file) {
{{- else }}
    let log_file = match cli.global.log_file {
{{- end }}
        Some(path) => path,
        None => paths::data_dir()?.join("{{ .ProjectKebab }}.log"),
    };
    let _log_guard = logging::init(log_level, !cli.global.no_color, &log_file)?;
{{- else }}
    logging::init(log_level, !cli.global.no_color)?;
{{- end }}

{{- if .Computed.feature_json_output }}
    let ctx = commands::Context { json: cli.global.json };
{{- else }}
    let ctx = commands::Context::default();
{{- end }}
    let mut out = io::stdout().lock();

    match cli.command {
{{- range .Scaffold.commands }}
        Command::{{ . | toPascalCase }}(args) => commands::{{ . | toSnakeCase }}::run(&args, &ctx, &mut out),
{{- end }}
        // +scaffold:command:dispatch
        Command::Completions(args) => {
            commands::completions::run(&args, &mut out);
            Ok(())
        }
    }
}

/// Writes `err` and its causes to stderr as `error: <message>: <cause>`.
fn report(err: &anyhow::Error, color: bool) {
    // Nothing useful remains to do if stderr itself is gone.
    let _ignored = write_report(&mut io::stderr().lock(), err, color);
}

fn write_report(out: &mut impl io::Write, err: &anyhow::Error, color: bool) -> io::Result<()> {
    let (red, reset) = if color { ("\x1b[1;31m", "\x1b[0m") } else { ("", "") };
    writeln!(out, "{red}error:{reset} {err:#}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_prints_the_error_and_its_causes() {
        let err = anyhow::anyhow!("file not found").context("loading config");
        let mut out = Vec::new();
        write_report(&mut out, &err, false).unwrap();
        insta::assert_snapshot!(
            String::from_utf8(out).unwrap(),
            @"error: loading config: file not found"
        );
    }
}
