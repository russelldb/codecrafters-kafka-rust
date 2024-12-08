#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::TcpListener,
};

use bytes::BufMut;

fn main() {
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut ms = [0u8; 4];
                let _ = stream.read_exact(&mut ms).unwrap();
                let len = i32::from_be_bytes(ms);
                let mut buff = vec![0; (len - 4) as usize];
                stream.read_exact(&mut buff).unwrap();

                let _api = i16::from_be_bytes([buff[0], buff[1]]);
                let _req_ver = i16::from_be_bytes([buff[2], buff[3]]);

                let mut data: Vec<u8> = Vec::new();

                data.put(&buff[4..8]); //cid
                data.put_i16(0); //error code
                data.put_i8(2); //num keys
                data.put_i16(18); // api key
                data.put_i16(0); // min ver
                data.put_i16(4); //max ver
                data.put_i8(0); // tag buf
                data.put_i32(420); // throttle ms
                data.put_i8(0); // tag buff

                let len = data.len() as u32;
                stream.write_all(&len.to_be_bytes()).unwrap();
                let res = stream.write_all(&data);

                println!("res {:?}", res);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
