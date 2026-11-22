//! Subcommand implementations. Each command writes its output to the writer
//! it is given, so tests can capture it.

pub(crate) mod completions;
{{- range .Scaffold.commands }}
pub(crate) mod {{ . | toSnakeCase }};
{{- end }}
// +scaffold:command:module

/// Settings that shape how every command writes output.
#[derive(Debug, Default)]
{{- if .Computed.feature_json_output }}
pub(crate) struct Context {
    /// Write JSON instead of text.
    pub(crate) json: bool,
}
{{- else }}
#[expect(
    clippy::empty_structs_with_brackets,
    reason = "commands build it with `Context::default()`, which keeps working once fields exist"
)]
pub(crate) struct Context {}
{{- end }}
