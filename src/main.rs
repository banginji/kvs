use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use kvs::cli::{Cli, KvsCliCommand};
use kvs::store::Store;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let store = Store::new();

    use clap::Parser;
    let cli = Cli::parse();

    match &cli.command {
        KvsCliCommand::GET => {
            store.get_by_key(&cli.key);
        },
        KvsCliCommand::SET => {
            store.insert(cli.key, "one".to_string(), 2);
            store.print_all_elements();
        },
        KvsCliCommand::DELETE => {
            store.remove(cli.key);
        }
    }

    Ok(())
}
