use crate::keys::KeyPair;

pub struct Transaction {
    pub inputs: Vec<String>,
    pub outputs: Vec<(String, u64)>,
    pub signature: String,
}

impl Transaction {
    pub fn new(
        inputs: Vec<String>,
        to_address: String,
        amount: u64,
        fee: u64,
        key_pair: &KeyPair,
    ) -> Self {
        let mut outputs = vec![(to_address, amount)];
        let change = inputs.iter().map(|_| fee).sum::<u64>() - amount - fee;
        if change > 0 {
            outputs.push((key_pair.get_address(), change));
        }

        let tx_data = format!("{:?}{:?}", inputs, outputs);
        let signature = key_pair.sign(&tx_data);

        Transaction {
            inputs,
            outputs,
            signature,
        }
    }
}
