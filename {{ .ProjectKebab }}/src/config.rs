//! YAML config file support.

use std::fs;
use std::io::ErrorKind;
{{- if .Computed.feature_file_logging }}
use std::path::{Path, PathBuf};
{{- else }}
use std::path::Path;
{{- end }}

use anyhow::Context as _;
use serde::Deserialize;

use crate::cli::LogLevel;
use crate::paths;

/// Settings read from the config file. Command-line flags take precedence.
#[derive(Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct Config {
    pub(crate) log_level: Option<LogLevel>,
{{- if .Computed.feature_file_logging }}
    pub(crate) log_file: Option<PathBuf>,
{{- end }}
}

impl Config {
    /// Reads `config.yaml` from the config directory, or the defaults when it
    /// does not exist.
    pub(crate) fn read() -> anyhow::Result<Self> {
        let path = paths::config_dir()?.join("config.yaml");
        match fs::read_to_string(&path) {
            Ok(contents) => Self::parse(&contents, &path),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(Self::default()),
            Err(err) => Err(err).with_context(|| format!("reading config {}", path.display())),
        }
    }

    /// Reads the config file at `path`, which must exist.
    pub(crate) fn read_from(path: &Path) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("reading config {}", path.display()))?;
        Self::parse(&contents, path)
    }

    fn parse(contents: &str, path: &Path) -> anyhow::Result<Self> {
        if contents.trim().is_empty() {
            return Ok(Self::default());
        }
        serde_saphyr::from_str(contents)
            .with_context(|| format!("parsing config {}", path.display()))
    }
}

#[cfg(test)]
mod tests {
{{- if not .Computed.feature_file_logging }}
    use std::path::PathBuf;

{{- end }}
    use super::*;

    fn write_config(contents: &str) -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.yaml");
        fs::write(&path, contents).unwrap();
        (dir, path)
    }

    #[test]
    fn reads_log_level() {
        let (_dir, path) = write_config("log_level: debug\n");
        let config = Config::read_from(&path).unwrap();
        assert_eq!(config.log_level, Some(LogLevel::Debug), "log_level should parse");
    }

    #[test]
    fn empty_file_gives_defaults() {
        let (_dir, path) = write_config("");
        assert_eq!(Config::read_from(&path).unwrap(), Config::default(), "empty file");
    }

    #[test]
    fn rejects_unknown_fields() {
        let (_dir, path) = write_config("not_a_setting: true\n");
        assert!(Config::read_from(&path).is_err(), "unknown fields should be an error");
    }

    #[test]
    fn missing_explicit_file_is_an_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("missing.yaml");
        assert!(Config::read_from(&path).is_err(), "an explicit path must exist");
    }
}
