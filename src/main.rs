use std::io::Error;

use bytes::Bytes;

use kvs::{actor::kvs_actor_handle::KvsActorHandle, cli::{Cli, KvsCliCommand}};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handle = KvsActorHandle::new();

    loop {
        use clap::Parser;
        let cli = Cli::parse();
        
        match cli.command {
            KvsCliCommand::GET => {
                handle.get_by_key(Bytes::from(cli.key.clone())).await;
            },
            KvsCliCommand::SET => {
                handle.set_value(Bytes::from(cli.key), Bytes::from("one".to_string())).await;
                handle.set_value(Bytes::from("2"), Bytes::from("two".to_string())).await;
            },
            KvsCliCommand::DELETE => {
                handle.remove_value(Bytes::from (cli.key)).await;
            }
        }
    }
}
