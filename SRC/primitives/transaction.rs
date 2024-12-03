use crate::serialize::{Serializable, Deserializable, SerializationError};
use crate::hash::{double_sha256, hash_single};
use crate::script::Script;
use crate::serialize::CompactSize;
use std::io::{Read, Write};

/// Represents a transaction input
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxInput {
    pub prev_out: OutPoint,
    pub script_sig: Script,
    pub sequence: u32,
}

impl Serializable for TxInput {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), SerializationError> {
        self.prev_out.serialize(writer)?;
        self.script_sig.serialize(writer)?;
        writer.write_all(&self.sequence.to_le_bytes())?;
        Ok(())
    }
}

impl Deserializable for TxInput {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self, SerializationError> {
        let prev_out = OutPoint::deserialize(reader)?;
        let script_sig = Script::deserialize(reader)?;
        let mut sequence = [0u8; 4];
        reader.read_exact(&mut sequence)?;
        Ok(TxInput {
            prev_out,
            script_sig,
            sequence: u32::from_le_bytes(sequence),
        })
    }
}

/// Represents a transaction output
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxOutput {
    pub value: u64,
    pub script_pubkey: Script,
}

impl Serializable for TxOutput {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), SerializationError> {
        writer.write_all(&self.value.to_le_bytes())?;
        self.script_pubkey.serialize(writer)?;
        Ok(())
    }
}

impl Deserializable for TxOutput {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self, SerializationError> {
        let mut value = [0u8; 8];
        reader.read_exact(&mut value)?;
        let script_pubkey = Script::deserialize(reader)?;
        Ok(TxOutput {
            value: u64::from_le_bytes(value),
            script_pubkey,
        })
    }
}

/// Represents a unique identifier for a previous transaction output
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutPoint {
    pub txid: [u8; 32],
    pub index: u32,
}

impl Serializable for OutPoint {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), SerializationError> {
        writer.write_all(&self.txid)?;
        writer.write_all(&self.index.to_le_bytes())?;
        Ok(())
    }
}

impl Deserializable for OutPoint {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self, SerializationError> {
        let mut txid = [0u8; 32];
        let mut index = [0u8; 4];
        reader.read_exact(&mut txid)?;
        reader.read_exact(&mut index)?;
        Ok(OutPoint {
            txid,
            index: u32::from_le_bytes(index),
        })
    }
}

/// Represents a BitcoinZ transaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub version: i32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u32,
}

impl Transaction {
    /// Computes the transaction hash (double SHA-256 of the serialized transaction)
    pub fn hash(&self) -> [u8; 32] {
        let mut buffer = Vec::new();
        self.serialize(&mut buffer).expect("Transaction serialization failed");
        double_sha256(&buffer)
    }

    /// Dummy transaction for testing purposes
    pub fn new_dummy() -> Self {
        Transaction {
            version: 1,
            inputs: vec![TxInput {
                prev_out: OutPoint {
                    txid: [0; 32],
                    index: 0,
                },
                script_sig: Script::new(vec![0x6a]), // OP_RETURN
                sequence: 0xFFFFFFFF,
            }],
            outputs: vec![TxOutput {
                value: 5000000000, // 50 BTCZ
                script_pubkey: Script::new(vec![0x76, 0xa9, 0x14]), // OP_DUP OP_HASH160 <pubkeyhash> OP_EQUALVERIFY OP_CHECKSIG
            }],
            lock_time: 0,
        }
    }
}

impl Serializable for Transaction {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), SerializationError> {
        writer.write_all(&self.version.to_le_bytes())?;
        CompactSize(self.inputs.len() as u64).serialize(writer)?;
        for input in &self.inputs {
            input.serialize(writer)?;
        }
        CompactSize(self.outputs.len() as u64).serialize(writer)?;
        for output in &self.outputs {
            output.serialize(writer)?;
        }
        writer.write_all(&self.lock_time.to_le_bytes())?;
        Ok(())
    }
}

impl Deserializable for Transaction {
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self, SerializationError> {
        let mut version = [0u8; 4];
        reader.read_exact(&mut version)?;
        let num_inputs = CompactSize::deserialize(reader)?.0 as usize;
        let mut inputs = Vec::with_capacity(num_inputs);
        for _ in 0..num_inputs {
            inputs.push(TxInput::deserialize(reader)?);
        }
        let num_outputs = CompactSize::deserialize(reader)?.0 as usize;
        let mut outputs = Vec::with_capacity(num_outputs);
        for _ in 0..num_outputs {
            outputs.push(TxOutput::deserialize(reader)?);
        }
        let mut lock_time = [0u8; 4];
        reader.read_exact(&mut lock_time)?;

        Ok(Transaction {
            version: i32::from_le_bytes(version),
            inputs,
            outputs,
            lock_time: u32::from_le_bytes(lock_time),
        })
    }
}
