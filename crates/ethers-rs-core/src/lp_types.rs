use crate::impl_into_address_for_lpool;
use anyhow::{anyhow, Ok, Result};
use async_once_cell::OnceCell;
use async_trait::async_trait;
use ethers::{
    providers::{Provider, Ws},
    types::{Address, Bytes, H160, U256},
};
use ethers_literal::hash;
use ethers_rs_proc_macro::impl_token_lp;
use ethers_rs_types::error::{EthersRSUtilsError, RED_ERROR_STRING};
use once_cell;
use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::Arc,
};

impl_token_lp!(UniswapV2AMM, "impl_tokens.csv");

// impl LPoolTrait for LPool<Token::USDT, Token::WETH> {
//     const ADDRESS:H160 = Self::ADDRESS;
//     const UNISWAP_V2_POOL: OnceCell<UniswapV2Pool<Provider<Ws>>> = OnceCell::new();
// }

// impl LPoolTrait for LPool<Token::WETH, Token::PEPE2> {
//     const ADDRESS:H160 = Self::ADDRESS;
//     const UNISWAP_V2_POOL: once_cell::sync::OnceCell<UniswapV2Pool<Provider<Ws>>> = OnceCell::new();
// }

// impl<Token0, Token1> LiquidityPool<Token0, Token1>
// where
//     Token0: Into<H160>,
//     Token1: Into<H160>,
// {
//     pub async fn get_reserves<M: ethers::providers::Middleware>(
//         uniswap_v2_pool: UniswapV2Pool<M>,
//     ) -> Result<(u128, u128, u32)> {
//         let (reserve0, reserve1, blocktimestamplastest) =
//             uniswap_v2_pool.get_reserves().call().await.map_err(|err| {
//                 anyhow!(
//                     "{}Unable to call `get_reserves` from `{uniswap_v2_pool:?}`, {err}",
//                     *RED_ERROR_STRING
//                 )
//             })?;
//         Ok((reserve0, reserve1, blocktimestamplastest))
//     }
// }

// pub struct Reserve<Token0, Token1>
// where
//     Token0: Into<H160>,
//     Token1: Into<H160>,
// {
//     token0_reserve: u128,
//     token1_reserve: u128,
//     blocktimestamp: u32,
//     price: u128,
//     _token0: PhantomData<Token0>,
//     _token1: PhantomData<Token1>,
// }
