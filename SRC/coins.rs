use std::collections::HashMap;

/// Represents a transaction output.
#[derive(Debug, Clone, PartialEq)]
pub struct Coin {
    pub value: u64,         // Value in satoshis
    pub script_pubkey: Vec<u8>, // Output script
    pub height: u32,        // Block height at which the output was created
    pub spent: bool,        // Whether the coin has been spent
}

/// Represents a transaction input point.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutPoint {
    pub txid: String, // Transaction ID
    pub index: u32,   // Output index
}

/// UTXO database.
pub struct UTXOSet {
    map_coins: HashMap<OutPoint, Coin>,
}

impl UTXOSet {
    /// Creates a new, empty UTXO set.
    pub fn new() -> Self {
        UTXOSet {
            map_coins: HashMap::new(),
        }
    }

    /// Checks if a coin exists and is unspent.
    pub fn have_coin(&self, outpoint: &OutPoint) -> bool {
        self.map_coins
            .get(outpoint)
            .map_or(false, |coin| !coin.spent)
    }

    /// Adds a coin to the UTXO set.
    pub fn add_coin(&mut self, outpoint: OutPoint, coin: Coin) {
        self.map_coins.insert(outpoint, coin);
    }

    /// Spends a coin by marking it as spent.
    pub fn spend_coin(&mut self, outpoint: &OutPoint) -> bool {
        if let Some(coin) = self.map_coins.get_mut(outpoint) {
            if !coin.spent {
                coin.spent = true;
                return true;
            }
        }
        false
    }

    /// Fetches a coin from the UTXO set.
    pub fn access_coin(&self, outpoint: &OutPoint) -> Option<&Coin> {
        self.map_coins.get(outpoint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utxo_set_operations() {
        let mut utxo_set = UTXOSet::new();

        let outpoint = OutPoint {
            txid: "abcd1234".to_string(),
            index: 0,
        };

        let coin = Coin {
            value: 5000,
            script_pubkey: vec![0x76, 0xa9, 0x14], // Example P2PKH script
            height: 100,
            spent: false,
        };

        // Add coin
        utxo_set.add_coin(outpoint.clone(), coin.clone());
        assert!(utxo_set.have_coin(&outpoint));

        // Spend coin
        assert!(utxo_set.spend_coin(&outpoint));
        assert!(!utxo_set.have_coin(&outpoint));

        // Access coin
        let fetched_coin = utxo_set.access_coin(&outpoint).unwrap();
        assert_eq!(fetched_coin.value, 5000);
        assert!(fetched_coin.spent);
    }
}
