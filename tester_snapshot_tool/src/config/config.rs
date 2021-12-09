extern crate dotenv;
use crate::U64;
use ethers::prelude::Address;
use std::fmt::Debug;
use std::{env};

#[derive(Debug)]
pub struct Config {
    pub private_key: String,
    pub eth_provider_rpc: String,
    pub address_provider: Address,
    pub last_block: U64,
}

impl Default for Config {
    fn default() -> Self {
        dotenv::from_filename(".env").ok();
        dotenv::from_filename(".env.local").ok();
        let address_provider =
            str_to_address(env::var("ADDRESS_PROVIDER").expect("No ADDRESS_PROVIDER"));
        let eth_provider_rpc = env::var("ETH_PROVIDER").expect("No ETH_PROVIDER");
        let private_key = env::var("PRIVATE_KEY").expect("No PRIVATE_KEY");
        let last_block = env::var("LAST_BLOCK")
            .expect("NO LAST BLOCK")
            .parse::<u64>()
            .expect("INCORRECT LAST BLOCK")
            .into();

        Config {
            address_provider,
            private_key,
            eth_provider_rpc,
            last_block,
        }
    }
}

pub fn str_to_address(address: String) -> Address {
    let addr = hex::decode(address.as_str().strip_prefix("0x").unwrap())
        .expect(format!("Decoding of {} address failed", address).as_str());
    Address::from_slice(addr.as_slice())
}
