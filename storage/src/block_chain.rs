use hash::H256;
use chain::{IndexedBlock, IndexedBlockHeader};
use {Error, BlockOrigin, Store, SideChainOrigin};

pub trait ForkChain {
    /// Returns fork underlying store.
    fn store(&self) -> &Store;

    /// Flush fork to canon chain.
    /// Should not be used directly from outside of `BlockChain`.
    fn flush(&self) -> Result<(), Error>;
}

pub trait BlockChain {
    /// Inserts new block into blockchain
    fn insert(&self, block: IndexedBlock) -> Result<(), Error>;

    /// Rollback single best block. Return new best block hash
    fn rollback_best(&self) -> Result<H256, Error>;

    /// Canonize block with given hash
    fn canonize(&self, block_hash: &H256) -> Result<(), Error>;

    /// decanonize best block
    fn decanonize(&self) -> Result<H256, Error>;

    /// Checks the block origin
    fn block_origin(&self, header: &IndexedBlockHeader) -> Result<BlockOrigin, Error>;
}

pub trait Forkable {
    /// Fork current blockchain
    /// Lifetime guarantees fork relationship with canon chain.
    fn fork<'a>(&'a self, side_chain: SideChainOrigin) -> Result<Box<ForkChain + 'a>, Error>;

    /// Switch blockchain to given fork
    /// Lifetime guarantees that fork comes from this canon chain
    fn switch_to_fork<'a>(&'a self, fork: Box<ForkChain + 'a>) -> Result<(), Error>;
}
