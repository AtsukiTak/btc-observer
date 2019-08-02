use crate::tx::Txid;
use bitcoin::util::address::Address;

pub struct Observer {
    pub receiver_addrs: Vec<Address>,
    pub sender_addrs: Vec<Address>,
}

#[derive(Debug)]
pub struct Observed {
    pub txid: Txid,
    pub receiver: Address,
    pub sender: Address,
}

impl Observer {
    pub fn check_tx(&self, txid: &Txid) -> Option<Observed> {
        self.check_tx_receiver(txid).and_then(|receiver| {
            self.check_tx_sender(txid).map(|sender| Observed {
                txid: txid.clone(),
                receiver,
                sender,
            })
        })
    }

    fn check_tx_receiver(&self, txid: &Txid) -> Option<Address> {
        txid.query_receivers()
            .into_iter()
            .find(|addr| self.receiver_addrs.contains(addr))
    }

    // 監視対象のsender_addrsが空の場合は、常にチェックに通る
    fn check_tx_sender(&self, txid: &Txid) -> Option<Address> {
        txid.query_senders()
            .into_iter()
            .find(|addr| self.sender_addrs.is_empty() || self.sender_addrs.contains(addr))
    }
}
