use std::io::{self, Read};
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct CoinbaseTransaction {
    pub version: u32,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub lock_time: u32,
}

#[derive(Debug)]
pub struct TransactionInput {
    pub previous_output: [u8; 32], // TXID
    pub script_sig: Vec<u8>,
    pub sequence: u32,
}

#[derive(Debug)]
pub struct TransactionOutput {
    pub value: u64,
    pub script_pubkey: Vec<u8>,
}

impl CoinbaseTransaction {
    pub fn from_bytes(data: &[u8]) -> io::Result<Self> {
        let mut cursor = io::Cursor::new(data);

        let version = cursor.read_u32::<LittleEndian>()?;
        let input_count = cursor.read_u8()? as usize;

        let mut inputs = Vec::with_capacity(input_count);
        for _ in 0..input_count {
            let mut previous_output = [0u8; 32];
            cursor.read_exact(&mut previous_output)?;
            let script_sig_len = cursor.read_u8()? as usize;
            let mut script_sig = vec![0; script_sig_len];
            cursor.read_exact(&mut script_sig)?;
            let sequence = cursor.read_u32::<LittleEndian>()?;

            inputs.push(TransactionInput {
                previous_output,
                script_sig,
                sequence,
            });
        }

        let output_count = cursor.read_u8()? as usize;
        let mut outputs = Vec::with_capacity(output_count);
        for _ in 0..output_count {
            let value = cursor.read_u64::<LittleEndian>()?;
            let script_pubkey_len = cursor.read_u8()? as usize;
            let mut script_pubkey = vec![0; script_pubkey_len];
            cursor.read_exact(&mut script_pubkey)?;

            outputs.push(TransactionOutput {
                value,
                script_pubkey,
            });
        }

        let lock_time = cursor.read_u32::<LittleEndian>()?;

        Ok(CoinbaseTransaction {
            version,
            inputs,
            outputs,
            lock_time,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coinbase_transaction() {
        let raw_data = include_bytes!("../fuzzing/DeserializeTx/input/coinbase.bin");
        let tx = CoinbaseTransaction::from_bytes(raw_data).unwrap();
        println!("{:?}", tx);
    }
}
