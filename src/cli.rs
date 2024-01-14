use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_CRATE_NAME"))]
#[command(flatten_help = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    Show { git_show_args: Vec<String> },
    Log { git_log_args: Vec<String> },
}
