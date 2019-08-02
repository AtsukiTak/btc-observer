mod observer;
mod rpc;
mod tx;
mod txid_stream;

use bitcoin::util::address::Address;
use clap::{App, Arg};
use std::str::FromStr;
use txid_stream::MempoolTxidIter;

fn main() {
    let Args {
        receiver_addrs,
        sender_addrs,
    } = get_args();

    let observer = observer::Observer {
        receiver_addrs,
        sender_addrs,
    };

    MempoolTxidIter::new()
        .filter_map(move |txid| observer.check_tx(&txid))
        .for_each(|observed| println!("{:?}", observed));
}

struct Args {
    receiver_addrs: Vec<Address>,
    sender_addrs: Vec<Address>,
}

fn get_args() -> Args {
    let matches = App::new("Bitcoin observer")
        .arg(
            Arg::with_name("receiver_addr")
                .short("r")
                .multiple(true)
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("sender_addr")
                .short("s")
                .multiple(true)
                .takes_value(true),
        )
        .get_matches();
    let receivers = matches
        .values_of("receiver_addr")
        .unwrap()
        .map(|s| Address::from_str(s).unwrap())
        .collect();
    let senders = matches
        .values_of_lossy("sender_addr")
        .unwrap_or(Vec::new())
        .into_iter()
        .map(|s| Address::from_str(s.as_str()).unwrap())
        .collect();
    Args {
        receiver_addrs: receivers,
        sender_addrs: senders,
    }
}
