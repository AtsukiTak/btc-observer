use bitcoin::util::address::Address;
use crate::tx::Txid;

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

    fn check_tx_sender(&self, txid: &Txid) -> Option<Address> {
        txid.query_senders()
            .into_iter()
            .find(|addr| self.sender_addrs.contains(addr))
    }
}
