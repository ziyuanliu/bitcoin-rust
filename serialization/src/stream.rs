//! Stream used for serialization of Bitcoin structures
use std::io::{self, Write};
use std::borrow::Borrow;
use compact_integer::CompactInteger;
use bytes::Bytes;

// Do not serialize witness data
pub const SERIALIZE_TRANSACTION_WITNESS: u32 = 0x40000000;

pub fn serialize<T>(t: &T) -> Bytes where T: Serializable {
    let mut stream = Stream::default();
    stream.append(t);
    stream.out()
}

pub fn serialize_with_flags<T>(t: &T, flags: u32) -> Bytes where T: Serializable {
    let mut stream = Stream::with_flags(flags);
    stream.append(t);
    stream.out()
}

pub fn serialize_list<T, K>(t: &[K]) -> Bytes where T: Serializable, K: Borrow<T> {
    let mut stream = Stream::default();
    stream.append_list(t);
    stream.out()
}

pub fn serialized_list_size<T, K>(t: &[K]) -> usize where T: Serializable, K: Borrow<T> {
    CompactInteger::from(t.len()).serialized_size() + t.iter().
    	map(Borrow::borrow).map(Serializable::serialized_size).sum::<usize>()
}

pub fn serialized_list_size_with_flags<T, K>(t: &[K], flags: u32) -> usize where T: Serializable, K: Borrow<T> {
    CompactInteger::from(t.len()).serialized_size() + t.iter().
    	map(Borrow::borrow).map(|i| Serializable::serialized_size_with_flags(i, flags)).sum::<usize>()
}

pub trait Serializable {
	/// Serialize the struct and append it the end of the stream.
    fn serialize(&self, s: &mut Stream);

    /// Hint about the size of the serialized struct.
    fn serialized_size(&self) -> usize where Self: Sized {
        // fallback, if not overwritten
        serialize(self).len()
    }

    /// Hint about the size of the serialized struct with flags
    fn serialized_size_with_flags(&self, flags: u32) -> usize where Self: Sized {
        serialize_with_flags(self, flags).len()
    }
}

/// Stream used for serialization of Bitcoin Structures
#[derive(Default)]
pub struct Stream {
    buffer: Vec<u8>,
    flags: u32
}

impl Stream {
    pub fn new() -> Self {
        Stream { buffer: Vec::new(), flags: 0 }
    }

    pub fn with_flags(flags: u32) -> Self {
        Stream { buffer: Vec::new(), flags: flags }
    }

    pub fn include_transaction_witness(&self) -> bool {
        (self.flags & SERIALIZE_TRANSACTION_WITNESS) != 0
    }

    pub fn append<T>(&mut self, t: &T) -> &mut Self where T: Serializable{
        t.serialize(self);
        self
    }

    pub fn append_slice(&mut self, bytes: &[u8]) -> &mut Self{
    	self.buffer.write(bytes).unwrap();
        self
    }

    pub fn append_list<T, K>(&mut self, t: &[K]) -> &mut Self where T: Serializable, K: Borrow<T> {
        CompactInteger::from(t.len()).serialize(self);
        for i in t {
            i.borrow().serialize(self);
        }

        self
    }

    pub fn out(self) -> Bytes {
        self.buffer.into()
    }
}

impl Write for Stream {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.buffer.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), io::Error> {
        self.buffer.flush()
    }
}
