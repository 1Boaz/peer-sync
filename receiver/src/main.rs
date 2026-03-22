mod args;

use std::fs::{File, OpenOptions};
use std::io::{Error, Read, Write};
use clap::Parser;
use args::ReceiverArgs;
use std::net::TcpListener;
use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread;
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
            match read_network_stream(&tx, &mut conn) {
                Ok(_) => {}
                Err(_) => break
            }
        }
    }
}

fn read_network_stream(tx: &SyncSender<AlignedVec>, conn: &mut impl Read) -> Result<(), Error> {
    let mut len_buf = [0u8; 8];
    conn.read_exact(&mut len_buf)?;

    let msg_len = u64::from_be_bytes(len_buf) as usize;

    let mut buff: AlignedVec = AlignedVec::new();
    buff.resize(msg_len, 0);

    conn.read_exact(&mut buff[..])?;

    tx.send(buff).expect("Write the error enum with thiserror");
    Ok(())
}

fn write_thread(rx: Receiver<AlignedVec>) {
    let mut current_file: Option<File> = None;

    loop {
        let buff = match rx.recv() {
            Ok(b) => b,
            Err(_) => break
        };

        let message = match access_buffer(&buff) {
            Ok(val) => val,
            Err(_) => todo!("Write the error enum with thiserror")
        };

        match message {
            ArchivedSyncMessage::NewFile { path, perm } => {
                let base_dir = std::env::current_dir().expect("Failed to get current dir");

                let mut safe_relative_path = std::path::PathBuf::new();
                for component in Path::new(path.as_str()).components() {
                    if let std::path::Component::Normal(c) = component {
                        safe_relative_path.push(c);
                    }
                }
                let final_path = base_dir.join(safe_relative_path);

                current_file = match create_parent_and_file(&final_path, &u32::from(*perm)) {
                    Ok(file) => Some(file),
                    Err(e) => {
                        eprintln!("Failed to create file {:?}: {}", final_path, e);
                        None
                    }
                };
            }
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

fn create_parent_and_file(path: &Path, perm: &u32) -> Result<File, Error> {
    let path = Path::new(path);

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        OpenOptions::new()
            .mode(*perm)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
    }

    #[cfg(not(unix))]
    {
        File::create(path)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;
    use clap::CommandFactory;
    use common::{serialize, SyncMessage};

    #[test]
    fn args_parsing_test() {
        let cli = ReceiverArgs::parse_from(["my_app", "--port", "5"]);
        assert_eq!(cli.port, 5);

        let cli = ReceiverArgs::parse_from([""]);
        assert_eq!(cli.port, 31415);
    }

    #[test]
    fn verify_cli() {
        ReceiverArgs::command().debug_assert();
    }

    #[test]
    fn test_read_network_stream() {
        let message = SyncMessage::NewFile {
            path: "hello.txt".to_string(),
            perm: 0o644,
        };

        let serialized = serialize(&message).unwrap();

        let mut fake_net_buffer = Vec::new();
        fake_net_buffer.extend_from_slice(&(serialized.len() as u64).to_be_bytes());
        fake_net_buffer.extend_from_slice(serialized.as_slice());

        let mut mock_stream = Cursor::new(fake_net_buffer);
        let (tx, rx) = mpsc::sync_channel::<AlignedVec>(1);

        assert!(read_network_stream(&tx, &mut mock_stream).is_ok());

        let received_buff = rx.recv().unwrap();

        let decoded = access_buffer(&received_buff).unwrap();

        match decoded {
            ArchivedSyncMessage::NewFile { path, perm} => {
                assert_eq!(path.as_str(), "hello.txt");
                assert_eq!(u32::from(*perm), 0o644)
            },
            _ => panic!("Decoded the wrong message type!")
        }
    }

    #[test]
    fn file_creation_test() {
        let temp_dir = tempfile::tempdir().unwrap();
        let target_path = temp_dir.path().join("dir/then/file.rs");

        let mut file = create_parent_and_file(&target_path, &0o644).unwrap();

        assert!(file.write_all(b"Inserting data").is_ok());
        assert!(target_path.exists())
    }
}