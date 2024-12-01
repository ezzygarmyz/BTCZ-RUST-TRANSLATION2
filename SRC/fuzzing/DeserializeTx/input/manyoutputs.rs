use std::io::{self, Read};
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct ManyOutputsTransaction {
    pub version: u32,
    pub outputs: Vec<TransactionOutput>,
}

#[derive(Debug)]
pub struct TransactionOutput {
    pub value: u64,
    pub script_pubkey: Vec<u8>,
}

impl ManyOutputsTransaction {
    pub fn from_bytes(data: &[u8]) -> io::Result<Self> {
        let mut cursor = io::Cursor::new(data);

        let version = cursor.read_u32::<LittleEndian>()?;
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

        Ok(ManyOutputsTransaction { version, outputs })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manyoutputs_transaction() {
        let raw_data = include_bytes!("../fuzzing/DeserializeTx/input/manyoutputs.bin");
        let tx = ManyOutputsTransaction::from_bytes(raw_data).unwrap();
        println!("{:?}", tx);
    }
}
