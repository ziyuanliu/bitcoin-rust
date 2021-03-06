use std::io;
use hash::H32;
use ser::{Deserializable, Reader, Error as ReaderError};
use chain::IndexedBlock;

#[derive(Debug, PartialEq)]
pub struct Block {
    pub magic: H32,
    pub block_size: u32,
    pub block: IndexedBlock,
}

impl Deserializable for Block {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, ReaderError> where T: io::Read {
        let block = Block {
        	magic: reader.read()?,
        	block_size: reader.read()?,
        	block: reader.read()?,
        };

        Ok(block)
    }
}
