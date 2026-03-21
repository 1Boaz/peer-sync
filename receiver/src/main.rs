mod args;

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use clap::Parser;
use args::ReceiverArgs;
use std::net::TcpListener;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use rkyv::primitive::ArchivedU32;
use rkyv::string::ArchivedString;
use rkyv::util::AlignedVec;
use common::{access_buffer, ArchivedSyncMessage};

fn main() {
    let args = ReceiverArgs::parse();

    let listener = match TcpListener::bind(format!("0.0.0.0:{}", args.port)) {
        Ok(lis) => lis,
        Err(_) => todo!("Write the error enum with thiserror")
    };

    println!("Listening on port: {}", args.port);

    let (tx, rx) = mpsc::sync_channel::<AlignedVec>(10);

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

            let mut buff: AlignedVec = AlignedVec::new();
            buff.resize(msg_len, 0);

            if let Err(e) = conn.read_exact(&mut buff[..]) {
                eprintln!("Failed to read payload: {}", e);
                break;
            }

            tx.send(buff).expect("Write the error enum with thiserror");
        }
    }
}

fn write_thread(rx: Receiver<AlignedVec>) {
    let mut current_file: Option<File> = None;

    loop {
        let buff = rx.recv().expect("Write the error enum with thiserror");

        let message = match access_buffer(&buff) {
            Ok(val) => val,
            Err(_) => todo!("Write the error enum with thiserror")
        };

        match message {
            ArchivedSyncMessage::NewFile { path, perm } => current_file = Some(create_parent_and_file(path, perm).expect("Write the error enum with thiserror")),
            ArchivedSyncMessage::Chunk(buff) => {
                if let Some(ref mut file) = current_file {
                    if let Err(e) = file.write_all(&buff) {
                        eprintln!("Failed to write chunk: {}", e);
                    }
                }
            }
            ArchivedSyncMessage::EndFile => current_file = None
        }
    }
}

fn create_parent_and_file(path: &ArchivedString, perm: &ArchivedU32) -> Result<File, std::io::Error> {
    let path = std::path::Path::new(path.as_str());
    let prefix = path.parent();

    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;

        if prefix.is_none() {
            return OpenOptions::new().mode(u32::from(*perm)).create(true).open(path)
        }

        std::fs::create_dir_all(prefix.unwrap())?;

        Ok(OpenOptions::new().mode(u32::from(*perm)).create(true).open(path)?)
    }

    #[cfg(not(unix))]
    {
        if prefix.is_none() {
            return File::create(path)
        }

        std::fs::create_dir_all(prefix.unwrap())?;

        Ok(File::create(path)?)
    }
}