use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Cli {
    pub command: KvsCliCommand,
    pub key: String,
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum KvsCliCommand {
    GET,
    SET,
    DELETE,
}
