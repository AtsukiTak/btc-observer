use bitcoin_hashes::sha256d::Hash;
use bitcoincore_rpc::RpcApi;
use std::time::Duration;

use crate::rpc::rpc;
use crate::tx::Txid;

const MEMPOOL_OBSERVE_INTERVAL: Duration = Duration::from_secs(1);

pub struct MempoolTxidIter {
    last_mempool: Vec<Hash>,
    next_txid_buf: Vec<Hash>,
}

impl Iterator for MempoolTxidIter {
    type Item = Txid;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_txid_buf.is_empty() {
            self.load_new_txids_until_success();
        }
        // この時点では、必ずnext_txid_bufが空ではないはず
        Some(Txid(self.next_txid_buf.pop().unwrap()))
    }
}

impl MempoolTxidIter {
    pub fn new() -> Self {
        MempoolTxidIter {
            last_mempool: Vec::new(),
            next_txid_buf: Vec::new(),
        }
    }

    fn load_new_txids_once(&mut self) {
        let latest_mempool = rpc(|client| client.get_raw_mempool());
        let new_txids = diff_mempool(latest_mempool.as_slice(), self.last_mempool.as_slice());
        self.last_mempool = latest_mempool;
        self.next_txid_buf = new_txids;
    }

    fn load_new_txids_until_success(&mut self) {
        loop {
            self.load_new_txids_once();
            if !self.next_txid_buf.is_empty() {
                break;
            }
            std::thread::sleep(MEMPOOL_OBSERVE_INTERVAL);
        }
    }
}

fn diff_mempool(latest_mempool: &[Hash], last_mempool: &[Hash]) -> Vec<Hash> {
    latest_mempool
        .iter()
        .filter(|txid| !last_mempool.contains(txid))
        .copied()
        .collect()
}
