use hex::FromHex;
use hash::H256;
use ser::{deserialize};
use merkle_root::merkle_root;
use {BlockHeader, Transaction};
use super::RepresentH256;

#[derive(Debug, PartialEq, Clone, Serializable, Deserializable)]
pub struct Block {
    pub block_header: BlockHeader,
    pub transactions: Vec<Transaction>
}

impl From<&'static str> for Block {
    fn from(s: &'static str) -> Self {
        deserialize(&s.from_hex().unwrap() as &[u8]).unwrap()
    }
}

impl RepresentH256 for Block {
    fn h256(&self) -> H256 {
    	self.hash()
    }
}

impl Block {
	pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
	    Block { block_header: header, transactions}
	}

	/// Returns the block's merkle root
	pub fn merkle_root(&self) -> H256 {
		let hashes = self.transactions.iter().map(Transaction::hash).collect::<Vec<H256>>();
		merkle_root(&hashes)
	}

	/// Returns the block's witness merkle root
	pub fn witness_merkle_root(&self) -> H256 {
	    let hashes = match self.transactions.split_first() {
	        None => vec![],
	        Some((_, rest)) => {
	        	let mut hashes = vec![H256::from(0)];
	        	hashes.extend(rest.iter().map(Transaction::witness_hash));
	        	hashes
	        },
	    };
	    merkle_root(&hashes)
	}

	pub fn transactions(&self) -> &[Transaction] {
	    &self.transactions
	}

	pub fn header(&self) -> &BlockHeader {
	    &self.block_header
	}

	pub fn hash(&self) -> H256 {
	    self.block_header.hash()
	}
}

