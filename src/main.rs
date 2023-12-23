use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use kvs::cli::{Cli, KvsCliCommand};
use kvs::store::Store;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();

        // handle_connection(stream);
    }

    let mut store = Store::new();
    // let mut mem: &mut HashMap<String, String> = store.get_store();

    use clap::Parser;
    let cli = Cli::parse();

    match &cli.command {
        KvsCliCommand::GET => {
            store.get_by_key(cli.key);
        },
        KvsCliCommand::SET => {
            store.insert(cli.key, "one".to_string());
            store.print_all_elements();
        },
        KvsCliCommand::DELETE => {
            store.remove(&cli.key);
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
