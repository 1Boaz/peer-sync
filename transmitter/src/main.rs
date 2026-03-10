mod args;

use std::io::Write;
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

    let very_existing_file = SyncMessage::NewFile {
        path: String::from("very_existing_dir/very_existing_file"),
        perm: 0
    };

    let serialized = match serialize(&very_existing_file) {
        Ok(vec) => vec,
        Err(_) => todo!("Write the error enum with thiserror")
    };

    let sent = serialized.as_slice();
    println!("Sending payload of {} bytes", sent.len());

    let length_prefix = sent.len().to_be_bytes();
    stream.write_all(&length_prefix).expect("Failed to write length");

    stream.write_all(sent).expect("Failed to write payload");
}