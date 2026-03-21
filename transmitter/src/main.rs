mod args;

use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use args::TransmitterArgs;
use common::{serialize, SyncMessage};
use std::net::TcpStream;
use clap::Parser;

fn main() {
    let args = TransmitterArgs::parse();

    let mut stream = match TcpStream::connect(format!("{}:{}", args.ip, args.port)) {
        Ok(stream) => stream,
        Err(_) => todo!("Write the error enum with thiserror")
    };

    for path in args.files {
        let mut file = File::open(&path).expect("File not Found");

        send_message(&mut stream, &SyncMessage::NewFile {
            path,
            perm: get_file_perm(&file)
        }).expect("Failed sending message");

        let mut buffer = vec![0u8; 64 * 1024];

        loop {
            let bytes_read = match file.read(&mut buffer) {
                Ok(0) => { break }
                Ok(n) => { n }
                Err(_) => { break }
            };
            let chunk_data = buffer[..bytes_read].to_vec();

            send_message(&mut stream, &SyncMessage::Chunk(chunk_data)).expect("Failed to send message");
        }

        send_message(&mut stream, &SyncMessage::EndFile).expect("Failed to send message");
    }
}

fn get_file_perm(file: &File) -> u32 {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(md) = file.metadata() {
            md.permissions().mode()
        } else {
            0o644
        }
    }
    #[cfg(not(unix))]
    {
        0o644
    }
}

fn send_message(stream: &mut TcpStream, message: &SyncMessage) -> Result<(), Box<dyn Error>> {
    let serialized = serialize(&message)?;

    let sent = serialized.as_slice();
    println!("Sending payload of {} bytes", sent.len());

    let length_prefix = sent.len().to_be_bytes();
    stream.write_all(&length_prefix)?;

    stream.write_all(sent)?;

    Ok(())
}