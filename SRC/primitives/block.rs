use crate::transaction::Transaction;
use crate::serialize::{Serializable, Deserializable, SerializationError};
use crate::hash::{double_sha256, hash_merkle_root};
use crate::serialize::CompactSize;
use std::io::{Read, Write};

/// Represents a BitcoinZ block header
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockHeader {
    pub version: i32,
    pub prev_block_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u32,
    pub bits: u32,
    pub nonce: u32,
}

impl BlockHeader {
    /// Computes the block header hash (double SHA-256)
    pub fn hash(&self) -> [u8; 32] {
        let mut buffer = Vec::new();
        self.serialize(&mut buffer).expect("BlockHeader serialization failed");
        double_sha256(&buffer)
    }
}

impl Serializable for BlockHeader {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), SerializationError> {
        writer.write_all(&self.version.to_le_bytes())?;
        writer.write_all(&self.prev_block_hash)?;
        writer.write_all(&self.merkle_root)?;
        writer.write_all(&self.timestamp.to_le_bytes())?;
        writer.write_all(&self.bits.to_le_bytes())?;
        writer.write_all(&self.nonce.to_le_bytes())?;
        Ok(())
    }
}

impl Deserializable for BlockHeader {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self, SerializationError> {
        let mut version = [0u8; 4];
        let mut prev_block_hash = [0u8; 32];
        let mut merkle_root = [0u8; 32];
        let mut timestamp = [0u8; 4];
        let mut bits = [0u8; 4];
        let mut nonce = [0u8; 4];

        reader.read_exact(&mut version)?;
        reader.read_exact(&mut prev_block_hash)?;
        reader.read_exact(&mut merkle_root)?;
        reader.read_exact(&mut timestamp)?;
        reader.read_exact(&mut bits)?;
        reader.read_exact(&mut nonce)?;

        Ok(BlockHeader {
            version: i32::from_le_bytes(version),
            prev_block_hash,
            merkle_root,
            timestamp: u32::from_le_bytes(timestamp),
            bits: u32::from_le_bytes(bits),
            nonce: u32::from_le_bytes(nonce),
        })
    }
}

/// Represents a BitcoinZ block
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Computes the block hash
    pub fn hash(&self) -> [u8; 32] {
        self.header.hash()
    }

    /// Computes the Merkle root of the block's transactions
    pub fn merkle_root(&self) -> [u8; 32] {
        hash_merkle_root(&self.transactions)
    }
}

impl Serializable for Block {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), SerializationError> {
        self.header.serialize(writer)?;
        CompactSize(self.transactions.len() as u64).serialize(writer)?;
        for tx in &self.transactions {
            tx.serialize(writer)?;
        }
        Ok(())
    }
}

impl Deserializable for Block {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self, SerializationError> {
        let header = BlockHeader::deserialize(reader)?;
        let num_txs = CompactSize::deserialize(reader)?.0 as usize;
        let mut transactions = Vec::with_capacity(num_txs);

        for _ in 0..num_txs {
            transactions.push(Transaction::deserialize(reader)?);
        }

        Ok(Block { header, transactions })
    }
}
