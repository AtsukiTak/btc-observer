use bitcoincore_rpc::{Auth, Client, Error as RpcError};
use std::time::Duration;
use std::ops::Deref;
use lazy_static::lazy_static;

const RETRY_INTERVAL: Duration = Duration::from_secs(5);

lazy_static! {
    pub static ref RPC_CLIENT: Client = {
        let url = std::env::var("RPC_URL").expect("RPC_URL is not specified");
        let user = std::env::var("RPC_USER").expect("RPC_USER is not specified");
        let pass = std::env::var("RPC_PASS").expect("RPC_PASS is not specified");
        let auth = Auth::UserPass(user, pass);
        Client::new(url, auth).unwrap()
    };
}

pub fn rpc<F, T>(mut f: F) -> T
where
    F: FnMut(&Client) -> Result<T, RpcError>,
{
    loop {
        match f(RPC_CLIENT.deref()) {
            Ok(t) => {
                return t;
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
        std::thread::sleep(RETRY_INTERVAL);
    }
}
