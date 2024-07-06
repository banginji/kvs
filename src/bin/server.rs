use std::{fs::remove_file, io::Error, sync::Arc};

use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use kvs::{actor::kvs_actor_handle::KvsActorHandle, cli::{kvs_command::KvsCliCommand, kvs_server_codec::KvsServerCodec}};
use tokio::net::UnixListener;
use tokio_util::codec::Framed;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let socket_path = "/tmp/kvs_daemon.sock";
    let _ = remove_file(socket_path);

    let listener = UnixListener::bind(socket_path).unwrap();

    println!("daemon is running...");

    let handle = Arc::new(KvsActorHandle::new());

    loop {

        let handle = handle.clone();

        let (socket, _) = listener.accept().await.unwrap();

        let mut framed_stream = Framed::new(socket, KvsServerCodec::new());

        tokio::spawn(async move {
            if let Some(response) = framed_stream.next().await {
                match response {
                    Ok(KvsCliCommand::Get { key }) => {
                        let value_opt = handle.get_by_key(Bytes::from(key.clone())).await;
                        let _ = match value_opt {
                            Some(value) => framed_stream.send(Bytes::from(value)).await,
                            None => framed_stream.send(Bytes::from("None")).await,
                        };
                    },
                    Ok(KvsCliCommand::Set { key, value }) => {
                        handle.set_value(Bytes::from(key), Bytes::from(value)).await;
                        let _ = framed_stream.send(Bytes::from("Ok")).await;
                    },
                    Ok(KvsCliCommand::Delete { key }) => {
                        handle.remove_value(Bytes::from(key)).await;
                        let _ = framed_stream.send(Bytes::from("Ok")).await;
                    },
                    Ok(KvsCliCommand::Unknown) => {
                        println!("received an unknown command");
                    }
                    Err(e) => {
                        let _ = anyhow::anyhow!("failed to read from the socket: {:?}", e);
                        let _ = framed_stream.send(Bytes::from("Unknown")).await;
                    }
                }
            }
        });
    }
}
