use std::collections::{HashMap};
use std::vec::Vec;
use async_recursion::async_recursion;
use ethers::prelude::*;
use crate::bindings::credit_manager::CreditManager as CM;
use crate::bindings:: CreditManagerEvents;
use crate::credit_service::pool::PoolService;
use crate::errors::AirdropError;


pub struct CreditManager<M: Middleware, S: Signer> {
    pub tester: HashMap<Address, u64>,
    contract: CM<SignerMiddleware<M, S>>,
    pool_service: PoolService<SignerMiddleware<M, S>>,
}

impl<M: Middleware, S: Signer> CreditManager<M, S> {
    pub async fn new(
        client: std::sync::Arc<SignerMiddleware<M, S>>,
        payload: &(
            ethers_core::types::Address,
            bool,
            ethers_core::types::Address,
            bool,
            bool,
            ethers_core::types::U256,
            ethers_core::types::U256,
            ethers_core::types::U256,
            ethers_core::types::U256,
            ethers_core::types::U256,
            Vec<ethers_core::types::Address>,
            Vec<(ethers_core::types::Address, ethers_core::types::Address)>,
        ),
    ) -> Self {
        let contract = CM::new(payload.0, client.clone());
        let pool_service_address = contract.pool_service().call().await.unwrap();
        let pool_service = PoolService::new(pool_service_address, client.clone());

        CreditManager {
            tester: HashMap::new(),
            contract,
            pool_service,
        }
    }

    pub async fn update(&mut self, from_block: &U64, to_block: &U64) -> Result<(), AirdropError> {
        self.pool_service.get_events(from_block, to_block).await;
        for tester in self.pool_service.tester.iter() {
            self.tester.insert(*tester.0, *tester.1);
        }

        self.update_accounts(from_block, to_block).await;

        Ok(())
    }

    fn increase_tester(&mut self, address: &Address) {
        if self.tester.contains_key(address) {
            *self.tester.get_mut(&address).unwrap() = self.tester[address] + 1;
        } else {
            self.tester.insert(*address, 1);
        }
    }

    #[async_recursion]
    async fn load_events(
        &mut self,
        from_block: &U64,
        to_block: &U64,
    ) -> Vec<(CreditManagerEvents, LogMeta)> {
        let events = self
            .contract
            .events()
            .from_block(from_block)
            .to_block(to_block)
            .query_with_meta()
            .await;

        match events {
            Ok(result) => result,
            Err(_) => {
                let mid_block = (from_block + to_block) / 2u64;
                if mid_block == *from_block || mid_block == *to_block {
                    panic!("range is already narrow");
                }

                let mut left_part = self.load_events(from_block, &mid_block).await;
                let mut right_part = self.load_events(&(mid_block + 1u64), to_block).await;
                left_part.append(&mut right_part);
                left_part
            }
        }
    }

    async fn update_accounts(&mut self, from_block: &U64, to_block: &U64) {
        let events = self.load_events(from_block, to_block).await;

        for event in events {
            match &event.0 {
                CreditManagerEvents::OpenCreditAccountFilter(data) => {
                    println!("[{}]: OPEN: {:?}", &event.1.block_number, data);
                    self.increase_tester(&data.on_behalf_of);
                }
                CreditManagerEvents::CloseCreditAccountFilter(data) => {
                    println!("[{}]: CLOSE: {:?} ", &event.1.block_number, data);
                    self.increase_tester(&data.owner);
                }
                CreditManagerEvents::RepayCreditAccountFilter(data) => {
                    println!("[{}]: REPAY: {:?} ", &event.1.block_number, data);
                    self.increase_tester(&data.owner);
                }
                CreditManagerEvents::LiquidateCreditAccountFilter(data) => {
                    println!("[{}]: LIQUIDATE: {:?} ", &event.1.block_number, data);
                    // Noting to pay, cause it's not tester activity
                }
                CreditManagerEvents::IncreaseBorrowedAmountFilter(data) => {
                    println!(
                        "[{}]: INCREASE BORROWING: {:?}",
                        &event.1.block_number, data
                    );
                    self.increase_tester(&data.borrower);
                }
                CreditManagerEvents::AddCollateralFilter(data) => {
                    println!("[{}]: ADD COLLATERAL:  {:?} ", &event.1.block_number, data);

                    self.increase_tester(&data.on_behalf_of);
                }
                CreditManagerEvents::TransferAccountFilter(data) => {
                    println!("[{}]: TRANSFER, {:?}", &event.1.block_number, data);
                    self.increase_tester(&data.new_owner);
                }

                CreditManagerEvents::ExecuteOrderFilter(data) => {
                    println!("[{}]: EXECUTE, {:?} ", &event.1.block_number, data);
                    self.increase_tester(&data.borrower);
                }

                _ => {}
            }
        }
    }
}
