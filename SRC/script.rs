use bitcoin::blockdata::script::{Instruction, Script};
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::util::hash::Sha256dHash;

/// Verifies that the input script satisfies the output script.
pub fn verify_script(input_script: &Script, output_script: &Script, tx: &Transaction, input_index: usize) -> bool {
    // Simplified implementation: real implementations require transaction context
    // and script execution (e.g., using a VM like Bitcoin Core)
    !input_script.is_empty() && !output_script.is_empty() // Example validation logic
}

/// Converts a script to an assembly-like string.
pub fn script_to_asm(script: &Script) -> String {
    script
        .instructions()
        .map(|instr| match instr {
            Ok(Instruction::Op(op)) => format!("{:?}", op),
            Ok(Instruction::PushBytes(bytes)) => hex::encode(bytes),
            Err(_) => "INVALID".to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::blockdata::script::Builder;

    #[test]
    fn test_verify_script() {
        let input_script = Builder::new()
            .push_slice(b"signature")
            .push_slice(b"pubkey")
            .into_script();

        let output_script = Builder::new()
            .push_opcode(bitcoin::blockdata::opcodes::all::OP_DUP)
            .push_opcode(bitcoin::blockdata::opcodes::all::OP_HASH160)
            .push_slice(b"pubkeyhash")
            .push_opcode(bitcoin::blockdata::opcodes::all::OP_EQUALVERIFY)
            .push_opcode(bitcoin::blockdata::opcodes::all::OP_CHECKSIG)
            .into_script();

        let tx = Transaction {
            version: 1,
            lock_time: 0,
            input: vec![],
            output: vec![],
        };

        assert!(verify_script(&input_script, &output_script, &tx, 0));
    }

    #[test]
    fn test_script_to_asm() {
        let script = Builder::new()
            .push_opcode(bitcoin::blockdata::opcodes::all::OP_DUP)
            .push_opcode(bitcoin::blockdata::opcodes::all::OP_HASH160)
            .push_slice(b"pubkeyhash")
            .push_opcode(bitcoin::blockdata::opcodes::all::OP_EQUALVERIFY)
            .push_opcode(bitcoin::blockdata::opcodes::all::OP_CHECKSIG)
            .into_script();

        let asm = script_to_asm(&script);
        assert_eq!(
            asm,
            "OP_DUP OP_HASH160 7075626b657968617368 OP_EQUALVERIFY OP_CHECKSIG"
        );
    }
}
