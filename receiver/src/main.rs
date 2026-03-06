mod args;
mod error;

use std::io::{Read, Write};
use clap::Parser;
use crate::args::ReceiverArgs;
use std::net::TcpListener;

fn main() {
    let args = ReceiverArgs::parse();
    println!("Serving on port: {}", args.port);
    let listener = match TcpListener::bind(format!("0.0.0.0:{}", args.port)) {
        Ok(lis) => { lis }
        Err(_) => { todo!("Finish the error file/enum") }
    };

    loop {
        let mut client = match listener.accept() {
            Ok(client) => { client.0 }
            Err(_) => { todo!("Finish the error file/enum") }
        };

        println!("client connected");

        let mut buf: [u8; 4096] = [0; 4096];
        loop {
            let message = match client.read(&mut buf) {
                Ok(val) => {val}
                Err(_) => {break}
            };
            if message == 0 {
                break
            }
            client.write(&buf[0..message]).expect("Finish the error file/enum");
        }
    }
}