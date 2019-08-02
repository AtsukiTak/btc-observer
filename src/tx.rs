use crate::rpc::rpc;
use bitcoin::util::address::Address;
use bitcoin_hashes::sha256d::Hash;
use bitcoincore_rpc::{
    json::{GetRawTransactionResult, GetRawTransactionResultVin},
    RpcApi,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Txid(pub Hash);

impl Txid {
    pub fn query_raw_tx(&self) -> GetRawTransactionResult {
        rpc(|client| client.get_raw_transaction_verbose(&self.0, None))
    }

    pub fn query_receivers(&self) -> Vec<Address> {
        self.query_raw_tx()
            .vout
            .into_iter()
            .flat_map(|vout| vout.script_pub_key.addresses)
            .flat_map(|addrs| addrs.into_iter())
            .collect()
    }

    pub fn query_senders(&self) -> Vec<Address> {
        self.query_raw_tx()
            .vin
            .iter()
            .flat_map(query_senders_from_vin)
            .collect()
    }
}

fn query_senders_from_vin(vin: &GetRawTransactionResultVin) -> Vec<Address> {
    match (&vin.txid, &vin.vout) {
        (Some(ref txid), Some(ref vout_n)) => {
            let input_tx = rpc(|client| client.get_raw_transaction_verbose(txid, None));
            let input_vout = input_tx
                .vout
                .into_iter()
                .find(|vout| vout.n == *vout_n)
                .unwrap();
            input_vout.script_pub_key.addresses.unwrap_or(Vec::new())
        }
        _ => Vec::new(),
    }
}
