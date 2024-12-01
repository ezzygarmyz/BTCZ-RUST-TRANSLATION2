use bitcoin::{OutPoint, Amount};
use std::collections::HashSet;

/// Represents UTXO selection controls for transactions.
pub struct CoinControl {
    pub allow_other_inputs: bool,
    pub minimum_amount: Amount,
    pub maximum_amount: Amount,
    selected_outputs: HashSet<OutPoint>,
}

impl CoinControl {
    /// Creates a new CoinControl instance with default values.
    pub fn new() -> Self {
        CoinControl {
            allow_other_inputs: false,
            minimum_amount: Amount::from_sat(0),
            maximum_amount: Amount::MAX,
            selected_outputs: HashSet::new(),
        }
    }

    /// Checks if a UTXO is selected.
    pub fn is_selected(&self, output: &OutPoint) -> bool {
        self.selected_outputs.contains(output)
    }

    /// Selects a UTXO for use in a transaction.
    pub fn select(&mut self, output: OutPoint) {
        self.selected_outputs.insert(output);
    }

    /// Unselects a UTXO.
    pub fn unselect(&mut self, output: &OutPoint) {
        self.selected_outputs.remove(output);
    }

    /// Clears all selected UTXOs.
    pub fn clear(&mut self) {
        self.selected_outputs.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::Txid;

    #[test]
    fn test_coin_control() {
        let mut coin_control = CoinControl::new();

        let output1 = OutPoint {
            txid: Txid::default(),
            vout: 0,
        };
        let output2 = OutPoint {
            txid: Txid::default(),
            vout: 1,
        };

        // Test selection
        coin_control.select(output1.clone());
        assert!(coin_control.is_selected(&output1));
        assert!(!coin_control.is_selected(&output2));

        // Test unselection
        coin_control.unselect(&output1);
        assert!(!coin_control.is_selected(&output1));

        // Test clear
        coin_control.select(output1.clone());
        coin_control.select(output2.clone());
        coin_control.clear();
        assert!(!coin_control.is_selected(&output1));
        assert!(!coin_control.is_selected(&output2));
    }
}
