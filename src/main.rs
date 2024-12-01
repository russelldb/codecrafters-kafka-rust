#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                let mut preamble = [0u8; 12];
                let req = stream.read(&mut preamble).unwrap();
                assert!(req == 12);
                // correlation id is last 4 bytes as u32
                let mut cid = [0u8; 4];
                cid.copy_from_slice(&preamble[preamble.len() - 4..]);
                let response = [0u32.to_be_bytes(), cid].concat();
                stream.write_all(&response).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
