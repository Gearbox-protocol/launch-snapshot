use async_recursion::async_recursion;
use ethers::abi::Address;
use ethers::contract::LogMeta;
use ethers::prelude::{Middleware,  U64};
use std::collections::HashMap;

use crate::bindings::pool_service::PoolService as PoolContract;
use crate::bindings::PoolServiceEvents;

pub struct PoolService<M: Middleware> {
    contract: PoolContract<M>,
    pub tester: HashMap<Address, u64>,
}

impl<M: Middleware> PoolService<M> {
    pub fn new(address: Address, client: std::sync::Arc<M>) -> Self {
        let contract = PoolContract::new(address, client.clone());
        PoolService {
            contract,
            tester: HashMap::new(),
        }
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
    ) -> Vec<(PoolServiceEvents, LogMeta)> {
        let events = self
            .contract
            .events()
            .from_block(from_block)
            .to_block(to_block)
            .query_with_meta()
            .await;

        match events {
            Ok(result) => result,
            Err(err) => {
                println!("Query err: {:?}", err);

                let mid_block = (from_block + to_block) / 2u64;
                if mid_block == *from_block || mid_block == *to_block {
                    panic!("range is too narrow");
                }

                let mut left_part = self.load_events(from_block, &mid_block).await;

                let mut right_part = self.load_events(&(mid_block + 1u64), to_block).await;
                left_part.append(&mut right_part);
                left_part
            }
        }
    }

    pub async fn get_events(&mut self, from_block: &U64, to_block: &U64) {
        let events = self.load_events(from_block, to_block).await;

        for event in events {
            match &event.0 {
                PoolServiceEvents::AddLiquidityFilter(data) => {
                    println!("[{}]: AddLiquidity: {:?}", &event.1.block_number, data);
                    self.increase_tester(&data.on_behalf_of);
                }
                PoolServiceEvents::RemoveLiquidityFilter(data) => {
                    println!("[{}]: RemoveLiquidity: {:?} ", &event.1.block_number, data);
                    self.increase_tester(&data.sender);
                }

                _ => {}
            }
        }
    }
}
