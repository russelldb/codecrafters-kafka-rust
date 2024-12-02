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
                let _ = stream.read_exact(&mut ms).unwrap();
                let len = i32::from_be_bytes(ms);
                let mut buff = vec![0; (len - 4) as usize];
                stream.read_exact(&mut buff).unwrap();

                let _api = i16::from_be_bytes([buff[0], buff[1]]);
                let _req_ver = i16::from_be_bytes([buff[2], buff[3]]);

                let cid = [buff[4], buff[5], buff[6], buff[7]];
                let error_code = 0i16.to_be_bytes();
                let key_count = 1i32.to_be_bytes();
                let api_key =
                    [18i16.to_be_bytes(), 4i16.to_be_bytes(), 4i16.to_be_bytes()].concat(); //is min 4 or zero?
                let mess_len = cid.len()
                    + error_code.len()
                    + key_count.len()
                    + api_key.len()
                    + size_of::<i32>() // the message len header
                    + size_of::<i8>(); // the no tags tag buffer

                let mut resp = Vec::with_capacity(mess_len);

                resp.extend((mess_len as i32).to_be_bytes());
                resp.extend(cid);
                resp.extend(error_code);
                resp.extend(key_count);
                resp.extend(api_key);
                resp.extend(0i8.to_be_bytes()); // tag buf, yes/no??
                println!("{:?}", resp);
                let _ = stream.write_all(&resp);

                let res = stream.flush();

                println!("res {:?}", res);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
