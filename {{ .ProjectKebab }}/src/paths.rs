//! XDG directories for the application. These follow XDG on every platform,
//! including macOS, so `~/.config/{{ .ProjectKebab }}` works everywhere.

use std::path::PathBuf;

use anyhow::Context as _;
use etcetera::BaseStrategy as _;
use etcetera::base_strategy::Xdg;

const APP_NAME: &str = "{{ .ProjectKebab }}";

fn strategy() -> anyhow::Result<Xdg> {
    Xdg::new().context("finding the home directory")
}
{{- if .Computed.feature_config_file }}

/// `$XDG_CONFIG_HOME/<app>`, or `~/.config/<app>`.
pub(crate) fn config_dir() -> anyhow::Result<PathBuf> {
    Ok(strategy()?.config_dir().join(APP_NAME))
}
{{- end }}
{{- if .Computed.feature_file_logging }}

/// `$XDG_DATA_HOME/<app>`, or `~/.local/share/<app>`.
pub(crate) fn data_dir() -> anyhow::Result<PathBuf> {
    Ok(strategy()?.data_dir().join(APP_NAME))
}
{{- end }}
