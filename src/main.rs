#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut ms = [0u8; 4];
                let _ = stream.read(&mut ms).unwrap();
                let len = u32::from_be_bytes(ms);
                let mut buff = vec![0; len as usize];
                stream.read_exact(&mut buff).unwrap();
                let _api = u16::from_be_bytes([buff[0], buff[1]]);
                let _req_ver = u16::from_be_bytes([buff[2], buff[3]]);
                let cid = u32::from_be_bytes([buff[4], buff[5], buff[6], buff[7]]);
                let mut resp = Vec::with_capacity(8);
                resp.extend(6u32.to_be_bytes());
                resp.extend(cid.to_be_bytes());
                resp.extend(35u16.to_be_bytes());
                stream.write_all(&resp).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
