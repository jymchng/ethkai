use anyhow;
use async_once_cell;
use async_trait;
use enum_dispatch;
use ethers;
use ethers_rs_abigen_types;
use ethers_rs_core;
use ethers_rs_proc_macro::{impl_token_lp, impl_chain_ids_wallets};
use once_cell;
use std::marker::PhantomData;

fn main() {
    impl_token_lp!(UniswapV2AMM, "impl_tokens.csv");
    impl_chain_ids_wallets!(Wallets, "chain_ids.csv")
}
