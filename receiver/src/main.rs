mod args;

use std::fs::File;
use std::io::{Read, Write};
use clap::Parser;
use args::ReceiverArgs;
use std::net::TcpListener;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use rkyv::util::AlignedVec;
use common::{deserialize, SyncMessage};

fn main() {
    let args = ReceiverArgs::parse();

    let listener = match TcpListener::bind(format!("0.0.0.0:{}", args.port)) {
        Ok(lis) => lis,
        Err(_) => todo!("Write the error enum with thiserror")
    };

    println!("Listening on port: {}", args.port);

    let mut buff: AlignedVec = AlignedVec::new();
    let (tx, rx) = mpsc::sync_channel::<SyncMessage>(10);

    thread::spawn(move || {write_thread(rx)});

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

            let message = match deserialize(&buff) {
                Ok(val) => val,
                Err(_) => todo!("Write the error enum with thiserror")
            };

            tx.send(message).expect("Write the error enum with thiserror");
        }
    }
}

fn write_thread(rx: Receiver<SyncMessage>) {
    let mut current_file: Option<File> = None;

    loop {
        let message = rx.recv().expect("Write the error enum with thiserror");

        match message {
            SyncMessage::NewFile { path, perm } => current_file = Some(File::create(path).expect("Write the error enum with thiserror")),
            SyncMessage::Chunk(buff) => {
                if let Some(ref mut file) = current_file {
                    if let Err(e) = file.write_all(&buff) {
                        eprintln!("Failed to write chunk: {}", e);
                    }
                }
            }
            SyncMessage::EndFile => current_file = None
        }
    }
}
