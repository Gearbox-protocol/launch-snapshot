//
extern crate hex;
use std::sync::Arc;
use anyhow::Result;
use ethers::prelude::*;

use crate::bindings::address_provider::AddressProvider;
use crate::config::Config;
use crate::credit_service::service::CreditService;

mod bindings;
mod config;
mod credit_service;

mod errors;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Gearbox tester airdrop helper!");

    let config = Config::default();
    println!("Address provider: {:?} ", &config.address_provider);

    dbg!(&config);

    let provider = Provider::<Http>::try_from(config.eth_provider_rpc.clone())?;

    // create a wallet and connect it to the provider
    let wallet = config.private_key.parse::<LocalWallet>()?;
    let kovan: u64 = 42;
    let w2 = wallet.with_chain_id(kovan);
    let client: ethers::prelude::SignerMiddleware<
        ethers::prelude::Provider<ethers::prelude::Http>,
        ethers::prelude::Wallet<ethers_core::k256::ecdsa::SigningKey>,
    > = SignerMiddleware::new(provider.clone(), w2);

    let client = Arc::new(client);

    let address_provider = AddressProvider::new(config.address_provider, client.clone());

    let data_compressor_addr = address_provider.get_data_compressor().call().await.unwrap();

    let mut credit_service =
        CreditService::new(&config, data_compressor_addr, client.clone()).await;

    credit_service.launch().await.expect("Can get events for airdrop");

    Ok(())
}
