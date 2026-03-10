mod args;

use std::io::Read;
use clap::Parser;
use args::ReceiverArgs;
use std::net::TcpListener;
use rkyv::util::AlignedVec;
use common::access_message;

fn main() {
    let args = ReceiverArgs::parse();

    let listener = match TcpListener::bind(format!("0.0.0.0:{}", args.port)) {
        Ok(lis) => lis,
        Err(_) => todo!("Write the error enum with thiserror")
    };

    println!("Listening on port: {}", args.port);

    let mut buff: AlignedVec = AlignedVec::new();

    loop {
        let mut conn = match listener.accept() {
            Ok(conn) => conn.0,
            Err(_) => todo!("Write the error enum with thiserror")
        };

        println!("Client connected!");

        loop {
            let mut len_buf =[0u8; 8];
            match conn.read_exact(&mut len_buf) {
                Ok(_) => {}
                Err(_) => {
                    println!("Client disconnected.");
                    break;
                }
            }

            let msg_len = u64::from_be_bytes(len_buf) as usize;

            buff.clear();
            buff.resize(msg_len, 0);

            if let Err(e) = conn.read_exact(&mut buff[..]) {
                eprintln!("Failed to read payload: {}", e);
                break;
            }

            let message = match access_message(&buff) {
                Ok(val) => val,
                Err(_) => todo!("Write the error enum with thiserror")
            };

            println!("Received: {:?}", message);
        }
    }
}