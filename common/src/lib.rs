use rkyv::{rancor::Error, Archive, Deserialize, Serialize};
use rkyv::util::AlignedVec;

// The enum that holds the protocol
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub enum SyncMessage {
    // Indicates new file
    NewFile {
        path: String,
        perm: u32
    },

    // Chunk from a file
    Chunk(Vec<u8>),

    // End of the file
    EndFile
}

// Helper function to serialize
pub fn serialize(sync_message: &SyncMessage) -> Result<AlignedVec, Error> {
     rkyv::to_bytes::<Error>(sync_message)
}

// Helper function to access(look at the data without changing it)
pub fn deserialize(serialized: &AlignedVec) -> Result<SyncMessage, Error> {
    let archived = rkyv::access::<ArchivedSyncMessage, Error>(serialized)?;
    rkyv::deserialize::<SyncMessage, Error>(archived)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ser_access_newfile_test() {
        let demo_sync_message: SyncMessage = SyncMessage::NewFile {
            path: "/hi".to_string(),
            perm: 5
        };
        let serialized = serialize(&demo_sync_message).unwrap();
        let accessed = deserialize(&serialized).unwrap();
        assert_eq!(accessed, demo_sync_message);
    }

    #[test]
    fn ser_access_chunk_test() {
        let demo_sync_message: SyncMessage = SyncMessage::Chunk(vec![1, 2, 5, 6, 7, 2, 8, 1]);
        let serialized = serialize(&demo_sync_message).unwrap();
        let accessed = deserialize(&serialized).unwrap();
        assert_eq!(accessed, demo_sync_message);
    }

    #[test]
    fn ser_access_endfile_test() {
        let demo_sync_message: SyncMessage = SyncMessage::EndFile;
        let serialized = serialize(&demo_sync_message).unwrap();
        let accessed = deserialize(&serialized).unwrap();
        println!("{:?}", accessed);
        assert_eq!(accessed, demo_sync_message);
    }
}
