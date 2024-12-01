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
                let mut ms = [0u8; 4];
                let _ = stream.read(&mut ms).unwrap();
                // len is first 4 bytes, then next 4 are api key and version, then the next four cid
                let len = u32::from_be_bytes(ms);
                let mut buff = vec![0; len as usize];
                stream.read_exact(&mut buff).unwrap();
                // api-key 2bytes
                // req-ver 2bytes
                // cid 4bytes
                let api = u16::from_be_bytes([buff[0], buff[1]]);
                let req_ver = u16::from_be_bytes([buff[2], buff[3]]);
                let cid = &buff[4..8];
                let cid_u32 =
                    u32::from_be_bytes(cid.try_into().expect("slice with incorrect length"));
                println!("api-key {} req_ver {} cid is {:?}", api, req_ver, cid_u32);

                stream
                    .write_all(&[4u32.to_be_bytes(), cid_u32.to_be_bytes()].concat())
                    .unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
