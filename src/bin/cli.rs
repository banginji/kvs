use std::io::Error;

use futures::{SinkExt, StreamExt};
use kvs::cli::{kvs_cli_codec::KvsCliCodec, kvs_command::Cli};
use tokio::net::UnixStream;
use tokio_util::codec::Framed;

#[tokio::main]
async fn main() -> Result<(), Error> {
    use clap::Parser;

    let cli = Cli::parse();

    let socket_path = "/tmp/kvs_daemon.sock";

    let stream = UnixStream::connect(socket_path).await?;

    let mut framed_stream = Framed::new(stream, KvsCliCodec::new());

    framed_stream.send(cli.command).await?;

    if let Some(frame) = framed_stream.next().await {
        match frame {
            Ok(bytes) => {
                println!("received : {:?}", bytes);
            }
            Err(e) => eprintln!("Error reading frame: {:?}", e),
        }
    }

    Ok(())
}
