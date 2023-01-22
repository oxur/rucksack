// The flow for databases pre-v0.7.0 was this for serialisation (writing to disk):
// *
// *
// *
// *
// *
// *
// *
// *
//
// And this for deserialisation (reading from disk):
// *
// *
// *
// *
// *
// *
// *

pub struct OldDB {
    bytes: Vec<u8>,
}

impl OldDB {
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}
