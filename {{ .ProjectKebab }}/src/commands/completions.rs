//! The `completions` command.

use std::io::Write;

use clap::CommandFactory as _;
use clap_complete::Shell;

use crate::cli::Cli;

#[derive(Debug, clap::Args)]
pub(crate) struct Args {
    /// Shell to generate completions for
    shell: Shell,
}

pub(crate) fn run(args: &Args, out: &mut impl Write) {
    let mut command = Cli::command();
    let name = command.get_name().to_owned();
    clap_complete::generate(args.shell, &mut command, name, out);
}
