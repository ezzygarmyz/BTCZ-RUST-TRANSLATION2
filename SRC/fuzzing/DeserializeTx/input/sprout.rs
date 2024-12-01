use std::io::{self, Read};
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct SproutTransaction {
    pub shielded_inputs: Vec<ShieldedInput>,
    pub shielded_outputs: Vec<ShieldedOutput>,
    pub join_split_data: JoinSplitData,
}

#[derive(Debug)]
pub struct ShieldedInput {
    pub nullifier: [u8; 32],
    pub proof: Vec<u8>,
}

#[derive(Debug)]
pub struct ShieldedOutput {
    pub cm: [u8; 32],
    pub ephemeral_key: [u8; 32],
    pub enc_ciphertext: Vec<u8>,
}

#[derive(Debug)]
pub struct JoinSplitData {
    pub vpub_old: u64,
    pub vpub_new: u64,
    pub anchor: [u8; 32],
    pub zkproof: Vec<u8>,
    pub ciphertexts: Vec<Vec<u8>>,
}

impl SproutTransaction {
    pub fn from_bytes(data: &[u8]) -> io::Result<Self> {
        let mut cursor = io::Cursor::new(data);

        // Parse shielded inputs
        let input_count = cursor.read_u8()? as usize;
        let mut shielded_inputs = Vec::with_capacity(input_count);
        for _ in 0..input_count {
            let mut nullifier = [0u8; 32];
            cursor.read_exact(&mut nullifier)?;
            let proof_len = cursor.read_u16::<LittleEndian>()? as usize;
            let mut proof = vec![0; proof_len];
            cursor.read_exact(&mut proof)?;

            shielded_inputs.push(ShieldedInput { nullifier, proof });
        }

        // Parse shielded outputs
        let output_count = cursor.read_u8()? as usize;
        let mut shielded_outputs = Vec::with_capacity(output_count);
        for _ in 0..output_count {
            let mut cm = [0u8; 32];
            let mut ephemeral_key = [0u8; 32];
            cursor.read_exact(&mut cm)?;
            cursor.read_exact(&mut ephemeral_key)?;
            let enc_len = cursor.read_u16::<LittleEndian>()? as usize;
            let mut enc_ciphertext = vec![0; enc_len];
            cursor.read_exact(&mut enc_ciphertext)?;

            shielded_outputs.push(ShieldedOutput {
                cm,
                ephemeral_key,
                enc_ciphertext,
            });
        }

        // Parse join-split data
        let vpub_old = cursor.read_u64::<LittleEndian>()?;
        let vpub_new = cursor.read_u64::<LittleEndian>()?;
        let mut anchor = [0u8; 32];
        cursor.read_exact(&mut anchor)?;
        let zkproof_len = cursor.read_u16::<LittleEndian>()? as usize;
        let mut zkproof = vec![0; zkproof_len];
        cursor.read_exact(&mut zkproof)?;

        let ciphertext_count = cursor.read_u8()? as usize;
        let mut ciphertexts = Vec::with_capacity(ciphertext_count);
        for _ in 0..ciphertext_count {
            let ct_len = cursor.read_u16::<LittleEndian>()? as usize;
            let mut ciphertext = vec![0; ct_len];
            cursor.read_exact(&mut ciphertext)?;
            ciphertexts.push(ciphertext);
        }

        let join_split_data = JoinSplitData {
            vpub_old,
            vpub_new,
            anchor,
            zkproof,
            ciphertexts,
        };

        Ok(SproutTransaction {
            shielded_inputs,
            shielded_outputs,
            join_split_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprout_transaction() {
        let raw_data = include_bytes!("../fuzzing/DeserializeTx/input/sprout.bin");
        let tx = SproutTransaction::from_bytes(raw_data).unwrap();
        println!("{:?}", tx);
    }
}
