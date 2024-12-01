use std::io::{self, Read};

/// Represents a BitcoinZ transaction.
#[derive(Debug)]
pub struct Transaction {
    pub version: u32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u32,
}

impl Transaction {
    /// Deserializes a transaction from a byte stream.
    pub fn from_stream<R: Read>(reader: &mut R) -> Result<Self, String> {
        let version = read_u32(reader)?;
        let input_count = read_varint(reader)?;
        let inputs = (0..input_count)
            .map(|_| TxInput::from_stream(reader))
            .collect::<Result<Vec<_>, _>>()?;
        let output_count = read_varint(reader)?;
        let outputs = (0..output_count)
            .map(|_| TxOutput::from_stream(reader))
            .collect::<Result<Vec<_>, _>>()?;
        let lock_time = read_u32(reader)?;

        Ok(Transaction {
            version,
            inputs,
            outputs,
            lock_time,
        })
    }
}

/// Represents a transaction input.
#[derive(Debug)]
pub struct TxInput {
    pub prev_tx: [u8; 32],
    pub prev_index: u32,
    pub script_sig: Vec<u8>,
    pub sequence: u32,
}

impl TxInput {
    /// Deserializes a transaction input from a byte stream.
    pub fn from_stream<R: Read>(reader: &mut R) -> Result<Self, String> {
        let mut prev_tx = [0; 32];
        reader.read_exact(&mut prev_tx).map_err(|e| e.to_string())?;
        let prev_index = read_u32(reader)?;
        let script_len = read_varint(reader)?;
        let mut script_sig = vec![0; script_len as usize];
        reader.read_exact(&mut script_sig).map_err(|e| e.to_string())?;
        let sequence = read_u32(reader)?;

        Ok(TxInput {
            prev_tx,
            prev_index,
            script_sig,
            sequence,
        })
    }
}

/// Represents a transaction output.
#[derive(Debug)]
pub struct TxOutput {
    pub value: u64,
    pub script_pubkey: Vec<u8>,
}

impl TxOutput {
    /// Deserializes a transaction output from a byte stream.
    pub fn from_stream<R: Read>(reader: &mut R) -> Result<Self, String> {
        let value = read_u64(reader)?;
        let script_len = read_varint(reader)?;
        let mut script_pubkey = vec![0; script_len as usize];
        reader.read_exact(&mut script_pubkey).map_err(|e| e.to_string())?;

        Ok(TxOutput {
            value,
            script_pubkey,
        })
    }
}
