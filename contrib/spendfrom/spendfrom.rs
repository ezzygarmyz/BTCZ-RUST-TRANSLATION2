use bitcoin::{Address, Amount, Transaction, TxOut, Network};
use bitcoin::consensus::encode::serialize;
use bitcoin::util::psbt::PartiallySignedTransaction;
use jsonrpc::Client;
use serde_json::json;
use std::collections::HashMap;
use std::str::FromStr;
use std::error::Error;

/// Represents the SpendFrom module.
pub struct SpendFrom {
    rpc_client: Client,
    fee_per_kb: Amount,
}

impl SpendFrom {
    /// Creates a new instance of SpendFrom.
    pub fn new(rpc_url: &str, rpc_user: &str, rpc_password: &str, fee_per_kb: Amount) -> Self {
        let rpc_client = Client::new(rpc_url, Some((rpc_user.to_string(), rpc_password.to_string())));
        SpendFrom { rpc_client, fee_per_kb }
    }

    /// Sends coins from specific addresses.
    pub fn send_from(
        &self,
        from_addresses: Vec<&str>,
        to_address: &str,
        amount: Amount,
        change_address: Option<&str>,
    ) -> Result<String, Box<dyn Error>> {
        // Validate and parse addresses
        let from_addrs: Vec<Address> = from_addresses
            .iter()
            .map(|addr| Address::from_str(addr))
            .collect::<Result<_, _>>()?;
        let to_addr = Address::from_str(to_address)?;
        let change_addr = change_address.map(|addr| Address::from_str(addr)).transpose()?;

        // List unspent transactions
        let unspent: Vec<UnspentOutput> = self.rpc_client.call("listunspent", &[])?;
        let mut selected_utxos = Vec::new();
        let mut total_amount = Amount::from_sat(0);

        // Select UTXOs from the specified addresses
        for utxo in unspent {
            if from_addrs.contains(&utxo.address) {
                selected_utxos.push(utxo.clone());
                total_amount += utxo.amount;
                if total_amount >= amount {
                    break;
                }
            }
        }

        if total_amount < amount {
            return Err("Insufficient funds".into());
        }

        // Create raw transaction
        let mut outputs = HashMap::new();
        outputs.insert(to_addr.to_string(), amount.to_btc());

        // Calculate change
        let change_amount = total_amount - amount - self.estimate_fee(&selected_utxos)?;
        if change_amount > Amount::from_sat(0) {
            let change_addr = change_addr.unwrap_or_else(|| from_addrs[0].clone());
            outputs.insert(change_addr.to_string(), change_amount.to_btc());
        }

        let raw_tx: String = self.rpc_client.call("createrawtransaction", &[json!(selected_utxos), json!(outputs)])?;
        let signed_tx: SignedTransaction = self.rpc_client.call("signrawtransactionwithwallet", &[json!(raw_tx)])?;

        if !signed_tx.complete {
            return Err("Transaction signing failed".into());
        }

        let txid: String = self.rpc_client.call("sendrawtransaction", &[json!(signed_tx.hex)])?;
        Ok(txid)
    }

    /// Estimates the transaction fee based on selected UTXOs.
    fn estimate_fee(&self, utxos: &[UnspentOutput]) -> Result<Amount, Box<dyn Error>> {
        let tx_size = self.calculate_tx_size(utxos);
        Ok(self.fee_per_kb * (tx_size as f64 / 1000.0))
    }

    /// Calculates the size of the transaction.
    fn calculate_tx_size(&self, utxos: &[UnspentOutput]) -> usize {
        // Rough estimate: 180 bytes per input, 34 bytes per output, 10 bytes for overhead
        10 + utxos.len() * 180 + 2 * 34
    }
}

/// Represents an unspent transaction output.
#[derive(Clone, Debug, Deserialize)]
struct UnspentOutput {
    txid: String,
    vout: u32,
    address: Address,
    script_pub_key
