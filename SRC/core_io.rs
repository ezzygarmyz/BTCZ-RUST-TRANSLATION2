use bitcoin::blockdata::transaction::Transaction;
use bitcoin::consensus::encode::{deserialize, serialize};
use bitcoin::util::address::Address;
use bitcoin::Script;

/// Encodes a transaction into a hex string.
pub fn encode_hex_tx(tx: &Transaction) -> String {
    hex::encode(serialize(tx))
}

/// Decodes a transaction from a hex string.
pub fn decode_hex_tx(hex_tx: &str) -> Option<Transaction> {
    hex::decode(hex_tx)
        .ok()
        .and_then(|bytes| deserialize(&bytes).ok())
}

/// Converts a script to an assembly-like string representation.
pub fn script_to_asm(script: &Script) -> String {
    script
        .instructions()
        .map(|instr| match instr {
            Ok(bitcoin::blockdata::script::Instruction::Op(op)) => format!("{:?}", op),
            Ok(bitcoin::blockdata::script::Instruction::PushBytes(bytes)) => {
                hex::encode(bytes)
            }
            Err(_) => "INVALID".to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::blockdata::transaction::{Transaction, TxIn, TxOut};
    use bitcoin::blockdata::script::Script;
    use bitcoin::OutPoint;

    #[test]
    fn test_encode_decode_hex_tx() {
        // Create a dummy transaction
        let tx = Transaction {
            version: 1,
            lock_time: 0,
            input: vec![TxIn {
                previous_output: OutPoint {
                    txid: Default::default(),
                    vout: 0,
                },
                script_sig: Script::new(),
                sequence: 0xFFFFFFFF,
                witness: vec![],
            }],
            output: vec![TxOut {
                value: 1000,
                script_pubkey: Script::new(),
            }],
        };

        let hex_tx = encode_hex_tx(&tx);
        let decoded_tx = decode_hex_tx(&hex_tx).unwrap();

        assert_eq!(tx, decoded_tx);
    }

    #[test]
    fn test_script_to_asm() {
        let script = Script::from(vec![0x76, 0xa9, 0x14, 0x1f, 0x2e, 0x3d, 0x4c, 0x5b, 0x6a, 0x7f, 0x8e, 0x9d, 0xac, 0xbb, 0xca, 0xd9, 0xe8, 0xf7, 0x06, 0x15, 0x24, 0x33, 0x88, 0xac]);
        let asm = script_to_asm(&script);

        assert_eq!(asm, "OP_DUP OP_HASH160 1f2e3d4c5b6a7f8e9dacbbcaded8f706152433 OP_EQUALVERIFY OP_CHECKSIG");
    }
}
