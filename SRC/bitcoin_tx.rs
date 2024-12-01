use bitcoin::blockdata::transaction::{Transaction, TxIn, TxOut};
use bitcoin::consensus::encode::{deserialize, serialize};
use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::util::psbt::PartiallySignedTransaction;
use bitcoin::util::key::PrivateKey;
use bitcoin::Script;
use clap::{App, Arg};
use std::str::FromStr;

/// Creates a raw Bitcoin transaction.
fn create_raw_transaction(inputs: Vec<(String, u32)>, outputs: Vec<(String, u64)>) -> String {
    let txins: Vec<TxIn> = inputs
        .into_iter()
        .map(|(txid, vout)| TxIn {
            previous_output: bitcoin::OutPoint::new(txid.parse().unwrap(), vout),
            script_sig: Script::new(),
            sequence: 0xFFFFFFFF,
            witness: vec![],
        })
        .collect();

    let txouts: Vec<TxOut> = outputs
        .into_iter()
        .map(|(address, amount)| TxOut {
            value: amount,
            script_pubkey: Address::from_str(&address).unwrap().script_pubkey(),
        })
        .collect();

    let tx = Transaction {
        version: 2,
        lock_time: 0,
        input: txins,
        output: txouts,
    };

    hex::encode(serialize(&tx))
}

/// Signs a raw Bitcoin transaction with provided private keys.
fn sign_raw_transaction(raw_tx: &str, keys: Vec<&str>) -> String {
    let tx: Transaction = deserialize(&hex::decode(raw_tx).unwrap()).unwrap();
    let mut psbt = PartiallySignedTransaction::from_unsigned_tx(tx).unwrap();

    for key in keys {
        let privkey = PrivateKey::from_str(key).unwrap();
        psbt.sign(&privkey.key, Network::Bitcoin).unwrap();
    }

    hex::encode(serialize(&psbt.extract_tx()))
}

fn main() {
    let matches = App::new("bitcoin-tx")
        .version("0.1.0")
        .author("BitcoinZ Developers")
        .about("Command Line Interface for BitcoinZ Transaction Management")
        .subcommand(
            App::new("createrawtransaction")
                .about("Creates a raw Bitcoin transaction")
                .arg(Arg::new("inputs").about("Transaction inputs").required(true))
                .arg(Arg::new("outputs").about("Transaction outputs").required(true)),
        )
        .subcommand(
            App::new("signrawtransaction")
                .about("Signs a raw Bitcoin transaction")
                .arg(Arg::new("rawtx").about("Raw transaction in hex").required(true))
                .arg(
                    Arg::new("keys")
                        .about("Private keys for signing")
                        .multiple_occurrences(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("createrawtransaction") {
        let inputs: Vec<(String, u32)> = serde_json::from_str(matches.value_of("inputs").unwrap())
            .expect("Invalid input format");
        let outputs: Vec<(String, u64)> = serde_json::from_str(matches.value_of("outputs").unwrap())
            .expect("Invalid output format");
        let raw_tx = create_raw_transaction(inputs, outputs);
        println!("{}", raw_tx);
    } else if let Some(matches) = matches.subcommand_matches("signrawtransaction") {
        let raw_tx = matches.value_of("rawtx").unwrap();
        let keys: Vec<&str> = matches
            .values_of("keys")
            .unwrap_or_default()
            .collect();
        let signed_tx = sign_raw_transaction(raw_tx, keys);
        println!("{}", signed_tx);
    } else {
        eprintln!("Error: Unknown command");
    }
}
