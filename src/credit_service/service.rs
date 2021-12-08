
use std::collections::{HashMap};
use std::fs::File;
use std::io::{Write};
use std::sync::Arc;

use ethers::prelude::*;

use crate::bindings::data_compressor::DataCompressor;
use crate::config::Config;
use crate::credit_service::credit_manager::CreditManager;
use crate::errors::AirdropError;


pub struct CreditService<M: Middleware, S: Signer> {
    credit_managers: Vec<CreditManager<M, S>>,
    dc: DataCompressor<SignerMiddleware<M, S>>,
    client: Arc<SignerMiddleware<M, S>>,
    last_block: U64,
}

impl<M: Middleware, S: Signer> CreditService<M, S> {
    pub async fn new(
        config: &Config,
        data_compressor: H160,
        client: Arc<SignerMiddleware<M, S>>,
    ) -> Self {
        let dc = DataCompressor::new(data_compressor, client.clone());
        let credit_managers: Vec<CreditManager<M, S>> = Vec::new();

        CreditService {
            credit_managers,
            dc,
            client,

            last_block: config.last_block,
        }
    }

    pub async fn launch(&mut self) -> Result<(), AirdropError> {
        let cm_list = self
            .dc
            .get_credit_managers_list(self.dc.address())
            .call()
            .await
            .unwrap();

        for cm in cm_list {
            let credit_manager = CreditManager::new(self.client.clone(), &cm).await;
            self.credit_managers.push(credit_manager);
        }

        println!(
            "Getting tester events info from {} to {}",
            0, &self.last_block
        );

        let mut testers: HashMap<Address, u64> = HashMap::new();
        // Updates info
        for cm in self.credit_managers.iter_mut() {
            cm.update(&0u64.into(), &self.last_block).await?;

            for tester in cm.tester.iter() {
                let address = tester.0;
                if testers.contains_key(&address) {
                    *testers.get_mut(&address).unwrap() = testers[&address] + tester.1;
                } else {
                    testers.insert(*address, *tester.1);
                }
            }
        }

        let mut f = File::create("testers.csv").expect("cant create testers.csv");
        for t in testers.iter() {
            write!(f, "{:?}, {}\n", &t.0, &t.1).expect("cant write to testers.csv")
        }

        f.sync_all().expect("cant write to testers.csv");

        println!("\nTotal gearbox users were found: {}", testers.len());
        println!("All data was saved to testers.csv");

        Ok(())
    }
}
