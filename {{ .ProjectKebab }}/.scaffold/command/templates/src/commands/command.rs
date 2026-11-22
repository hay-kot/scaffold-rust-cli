//! The `{{ .Scaffold.command_name }}` command.

use std::io::Write;

use super::Context;

#[derive(Debug, clap::Args)]
pub(crate) struct Args;

pub(crate) fn run(_args: &Args, _ctx: &Context, out: &mut impl Write) -> anyhow::Result<()> {
    tracing::info!("running {{ .Scaffold.command_name }} command");

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
}
