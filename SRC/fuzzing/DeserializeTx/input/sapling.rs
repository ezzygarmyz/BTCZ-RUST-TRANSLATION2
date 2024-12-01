use std::io::{self, Read};
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct SaplingTransaction {
    pub shielded_inputs: Vec<ShieldedInput>,
    pub shielded_outputs: Vec<ShieldedOutput>,
}

#[derive(Debug)]
pub struct ShieldedInput {
    pub cv: [u8; 32],
    pub nullifier: [u8; 32],
    pub rk: [u8; 32],
    pub proof: Vec<u8>,
}

#[derive(Debug)]
pub struct ShieldedOutput {
    pub cmu: [u8; 32],
    pub ephemeral_key: [u8; 32],
    pub enc_ciphertext: Vec<u8>,
    pub out_ciphertext: Vec<u8>,
}

impl SaplingTransaction {
    pub fn from_bytes(data: &[u8]) -> io::Result<Self> {
        let mut cursor = io::Cursor::new(data);

        let input_count = cursor.read_u8()? as usize;
        let mut shielded_inputs = Vec::with_capacity(input_count);
        for _ in 0..input_count {
            let mut cv = [0u8; 32];
            let mut nullifier = [0u8; 32];
            let mut rk = [0u8; 32];
            cursor.read_exact(&mut cv)?;
            cursor.read_exact(&mut nullifier)?;
            cursor.read_exact(&mut rk)?;
            let proof_len = cursor.read_u16::<LittleEndian>()? as usize;
            let mut proof = vec![0; proof_len];
            cursor.read_exact(&mut proof)?;

            shielded_inputs.push(ShieldedInput {
                cv,
                nullifier,
                rk,
                proof,
            });
        }

        let output_count = cursor.read_u8()? as usize;
        let mut shielded_outputs = Vec::with_capacity(output_count);
        for _ in 0..output_count {
            let mut cmu = [0u8; 32];
            let mut ephemeral_key = [0u8; 32];
            cursor.read_exact(&mut cmu)?;
            cursor.read_exact(&mut ephemeral_key)?;
            let enc_len = cursor.read_u16::<LittleEndian>()? as usize;
            let mut enc_ciphertext = vec![0; enc_len];
            cursor.read_exact(&mut enc_ciphertext)?;
            let out_len = cursor.read_u16::<LittleEndian>()? as usize;
            let mut out_ciphertext = vec![0; out_len];
            cursor.read_exact(&mut out_ciphertext)?;

            shielded_outputs.push(ShieldedOutput {
                cmu,
                ephemeral_key,
                enc_ciphertext,
                out_ciphertext,
            });
        }

        Ok(SaplingTransaction {
            shielded_inputs,
            shielded_outputs,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sapling_transaction() {
        let raw_data = include_bytes!("../fuzzing/DeserializeTx/input/sapling.bin");
        let tx = SaplingTransaction::from_bytes(raw_data).unwrap();
        println!("{:?}", tx);
    }
}
