use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use kvs::store::Store;

#[tokio::main]
pub async fn main() {
    // let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    //
    // stream.write("Hello, World!".as_bytes()).unwrap();
    //
    // let mut buffer = [0; 13];
    //
    // stream.read(&mut buffer).unwrap();
    //
    // println!("Response from server: {:?}", std::str::from_utf8(&buffer).unwrap());

    let db = Arc::new(Mutex::new(Store::new()));

    let db = db.clone();
    thread::scope(|s| {
        s.spawn(|| {
            let mut db = db.lock().unwrap();
            db.insert("1".to_string(), "one".to_string());
            // sleep(Duration::new(1, 0));
        });

        s.spawn(|| {
            let db = db.lock().unwrap();
            println!("value for key 1 is {:?}", db.get_by_key(&"1".to_string()));
            // sleep(Duration::new(1, 0));
        });

        s.spawn(|| {
            let mut db = db.lock().unwrap();
            db.remove("1".to_string());
            // sleep(Duration::new(1, 0));
        });

        s.spawn(|| {
            let db = db.lock().unwrap();
            println!("value for key 1 is {:?}", db.get_by_key(&"1".to_string()));
            // sleep(Duration::new(1, 0));
        });
    })
}
