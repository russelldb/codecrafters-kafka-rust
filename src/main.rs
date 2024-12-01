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
                println!("read {} bytes from req", req);
                // len is first 4 bytes, then next 4 are api key and version, then the next four cid
                let mut cid = [0u8; 4];
                cid.copy_from_slice(&preamble[8..]);
                println!("cid is {:?}", cid);
                let response = [0u32.to_be_bytes(), cid].concat();
                println!("writing response");
                let r = stream.write_all(&response);

                println!("did we send it? {:?}", r);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
