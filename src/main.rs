use std::io::{Error};

use bytes::Bytes;

use kvs::cli::{Cli, KvsCliCommand};
use kvs::store::Store;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let store = Store::new();

    use clap::Parser;
    let cli = Cli::parse();

    match &cli.command {
        KvsCliCommand::GET => {
            store.get_by_key(Bytes::from(cli.key.clone()), 1).await;
        },
        KvsCliCommand::SET => {
            store.insert(Bytes::from(cli.key), Bytes::from("one".to_string()), 2).await;
            store.insert(Bytes::from("2"), Bytes::from("two".to_string()), 2).await;
            store.print_all_elements();
        },
        KvsCliCommand::DELETE => {
            store.remove(Bytes::from (cli.key), 1).await;
        }
    }

    Ok(())
}
