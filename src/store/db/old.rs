// The flow for databases pre-v0.7.0 was this for serialisation (writing to disk):
// * bincode encoded the hashmap to bytes
// * checked the hash of the bincoded data to see if it was necessary to write
// * encrypted the bincode-encoded bytes
// * wrote those bytes to a file
//
// And this for deserialisation (reading from disk):
// * read the db file from disk
// * decrypted the read bytes
// * performed a hash CRC on the decrypted bytes
// * bincode-decoded the decrypted bytes to a hashmap (DashMap)
//
pub struct OldDB {
    bytes: Vec<u8>,
}

// For the new database format, the point at which reading an older format would
// fail would be right after the decryption. The hash is computed earlier in the
// newer version, so the only thing the OldDB needs to do is bincode-decode
impl OldDB {
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}
