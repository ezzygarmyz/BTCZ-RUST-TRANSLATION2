use crate::transaction::{Transaction, TxInput, TxOutput};
use crate::utxo::{Utxo, UtxoSet};
use crate::crypto::keys::PrivateKey;
use crate::util::fee::{FeeCalculator};
use crate::script::Script;
use thiserror::Error;

/// Errors that can occur during transaction building
#[derive(Debug, Error)]
pub enum TransactionBuilderError {
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Invalid UTXO set")]
    InvalidUtxoSet,
    #[error("Transaction construction failed")]
    ConstructionFailed,
}

/// A builder for constructing BitcoinZ transactions
pub struct TransactionBuilder {
    inputs: Vec<Utxo>,
    outputs: Vec<TxOutput>,
    change_address: Option<Script>,
    fee_calculator: FeeCalculator,
}

impl TransactionBuilder {
    /// Creates a new TransactionBuilder with the specified fee calculator
    pub fn new(fee_calculator: FeeCalculator) -> Self {
        TransactionBuilder {
            inputs: Vec::new(),
            outputs: Vec::new(),
            change_address: None,
            fee_calculator,
        }
    }

    /// Adds a UTXO as an input for the transaction
    pub fn add_input(&mut self, utxo: Utxo) -> &mut Self {
        self.inputs.push(utxo);
        self
    }

    /// Adds an output to the transaction
    pub fn add_output(&mut self, recipient: Script, amount: u64) -> &mut Self {
        self.outputs.push(TxOutput { script_pubkey: recipient, value: amount });
        self
    }

    /// Sets the change address for the transaction
    pub fn set_change_address(&mut self, address: Script) -> &mut Self {
        self.change_address = Some(address);
        self
    }

    /// Builds the transaction
    pub fn build(&self) -> Result<Transaction, TransactionBuilderError> {
        // Validate inputs and outputs
        if self.inputs.is_empty() || self.outputs.is_empty() {
            return Err(TransactionBuilderError::ConstructionFailed);
        }

        let total_input: u64 = self.inputs.iter().map(|input| input.value).sum();
        let total_output: u64 = self.outputs.iter().map(|output| output.value).sum();
        let fee = self.fee_calculator.calculate_fee(self.inputs.len(), self.outputs.len());

        if total_input < total_output + fee {
            return Err(TransactionBuilderError::InsufficientFunds);
        }

        let mut tx_inputs = self.inputs.iter().map(|utxo| {
            TxInput {
                prev_out: utxo.outpoint.clone(),
                script_sig: Script::new(vec![]),
                sequence: 0xFFFFFFFF,
            }
        }).collect::<Vec<_>>();

        let mut tx_outputs = self.outputs.clone();

        // Add change output if there's leftover value
        if let Some(change_address) = &self.change_address {
            let change_amount = total_input - total_output - fee;
            if change_amount > 0 {
                tx_outputs.push(TxOutput {
                    script_pubkey: change_address.clone(),
                    value: change_amount,
                });
            }
        }

        Ok(Transaction::new(1, tx_inputs, tx_outputs, 0))
    }
}
