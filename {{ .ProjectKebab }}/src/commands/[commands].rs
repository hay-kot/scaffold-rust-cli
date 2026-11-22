//! The `{{ .Each.Item }}` command.

use std::io::Write;
{{- if .Computed.feature_json_output }}

use serde::Serialize;
{{- end }}

use super::Context;

#[derive(Debug, clap::Args)]
pub(crate) struct Args;
{{- if .Computed.feature_json_output }}

#[derive(Debug, Serialize)]
struct Output {
    message: &'static str,
}
{{- end }}

pub(crate) fn run(_args: &Args, {{ if .Computed.feature_json_output }}{{ else }}_{{ end }}ctx: &Context, out: &mut impl Write) -> anyhow::Result<()> {
    tracing::info!("running {{ .Each.Item }} command");
{{- if .Computed.feature_json_output }}
    if ctx.json {
        serde_json::to_writer(&mut *out, &Output { message: "Hello World!" })?;
        writeln!(out)?;
        return Ok(());
    }
{{- end }}

    writeln!(out, "Hello World!")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_greeting() {
        let mut out = Vec::new();
        run(&Args, &Context::default(), &mut out).unwrap();
        insta::assert_snapshot!(String::from_utf8(out).unwrap(), @"Hello World!");
    }
{{- if .Computed.feature_json_output }}

    #[test]
    fn prints_json() {
        let mut out = Vec::new();
        run(&Args, &Context { json: true }, &mut out).unwrap();
        insta::assert_snapshot!(String::from_utf8(out).unwrap(), @r#"{"message":"Hello World!"}"#);
    }
{{- end }}
}
