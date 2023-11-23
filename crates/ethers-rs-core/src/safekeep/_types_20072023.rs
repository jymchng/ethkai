use crate::{
    abigen::contracts::uniswap::{UniswapV2Pool, UniswapV2_Consts},
    error::RED_ERROR_STRING,
    impl_into_address_for_lpool,
    init::{self, types::WhichAPIProvider, ARC_WS_PROVIDER},
};
use anyhow::{anyhow, Ok, Result};
use async_once_cell::OnceCell;
use async_trait::async_trait;
use ethers::{
    providers::{Provider, Ws},
    types::{Address, Bytes, H160, U256},
};
use ethers_literal::hash;
use std::{marker::PhantomData, sync::Arc, ops::{Deref, DerefMut}};
use once_cell;


// DO NOT DELETE THESE - IMPORTANT REFERENCE

impl LiquidityPools {
    pub fn from_addresses(token0_addr: H160, token1_addr: H160) -> Self {
        match (token0_addr.to_fixed_bytes(), token1_addr.to_fixed_bytes()) {
            (Token::USDT::ADDRESS_BYTES, Token::WETH::ADDRESS_BYTES) => {
                Self::USDTWETHV2(LPool::<Token::USDT, Token::WETH>::default())
            },
            (Token::ZOOMER::ADDRESS_BYTES, Token::WETH::ADDRESS_BYTES) => {
                Self::ZOOMERWETHV2(LPool::<Token::ZOOMER, Token::WETH>::default())
            },
            (Token::WETH::ADDRESS_BYTES, Token::PEPE2::ADDRESS_BYTES) => {
                Self::WETHPEPE2V2(LPool::<Token::WETH, Token::PEPE2>::default())
            },
            _ => {unreachable!()},
        }
    }

    pub async fn get_reserves(&self, which_provider: &WhichAPIProvider) -> Result<(u128, u128, u32)> {
        match self {
            Self::USDTWETHV2(val) => {
                val.get_reserves(which_provider).await
            },
            Self::ZOOMERWETHV2(val) => {
                val.get_reserves(which_provider).await
            },
            Self::WETHPEPE2V2(val) => {
                val.get_reserves(which_provider).await
            },
            _ => {unreachable!()},
        }
    }

    pub async fn swap(&self,
        amount_0_out: U256,
        amount_1_out: U256,
        to: H160,
        data: Bytes,
        which_provider: &WhichAPIProvider,)
        -> Result<()> {
        match self {
            Self::USDTWETHV2(val) => {
                val.swap(amount_0_out, amount_1_out, to, data, which_provider).await
            },
            Self::ZOOMERWETHV2(val) => {
                val.swap(amount_0_out, amount_1_out, to, data, which_provider).await
            },
            Self::WETHPEPE2V2(val) => {
                val.swap(amount_0_out, amount_1_out, to, data, which_provider).await
            },
            _ => {unreachable!()},
        }
    }
}


pub mod Token {
    use crate::impl_into_address_for_tokens;

    impl_into_address_for_tokens!(
        USDT, 0xdAC17F958D2ee523a2206206994597C13D831ec7_H160;
        NEON, 0x6ee9742d17b527e682248dca85952e4fe190061d_H160;
        WETH, 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2_H160;
        ZOOMER, 0x0d505c03d30e65f6e9b4ef88855a47a89e4b7676_H160;
        PEPE2, 0xfb66321d7c674995dfcc2cb67a30bc978dc862ad_H160;
    );
}

#[derive(Default)]
pub struct LiquidityPool<Token0, Token1>
where
    Token0: Into<H160>,
    Token1: Into<H160>,
{
    _token0: PhantomData<Token0>,
    _token1: PhantomData<Token1>,
}

pub type LPool<Token0, Token1> = LiquidityPool<Token0, Token1>;

impl_into_address_for_lpool! {
    <Token::USDT, Token::WETH, USDTWETHV2>: 0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852_H160;
    <Token::ZOOMER, Token::WETH, ZOOMERWETHV2>: 0xc2c701110de2b9503e98619d9c9e3017877b0f72_H160;
    <Token::WETH, Token::PEPE2, WETHPEPE2V2>: 0x076a3e1500f3110d8f4445d396a3d7ca6d0ca269_H160;
}

#[async_trait]
pub trait LPoolTrait {
    const ADDRESS: H160;
    const UNISWAP_V2_POOL: once_cell::sync::OnceCell<UniswapV2Pool<Provider<Ws>>>;

    async fn get_ws_provider(which_provider: &WhichAPIProvider) -> Result<&Arc<Provider<Ws>>> {
        ARC_WS_PROVIDER.get_or_try_init(
            async {
                let provider = init::ws::init_provider(which_provider).await?;
                let provider = Arc::new(provider);
                Ok(Arc::clone(&provider))
            }
        ).await
    }

    async fn get_reserves(&self, which_provider: &WhichAPIProvider) -> Result<(u128, u128, u32)> {
        let provider = Self::get_ws_provider(which_provider).await?;
        let uniswap_v2_pool_once_cell = Self::UNISWAP_V2_POOL;
        let uniswap_v2_pool = uniswap_v2_pool_once_cell
            .get_or_init(|| UniswapV2Pool::<Provider<Ws>>::new(Self::ADDRESS, Arc::clone(provider)));
        let (reserve0, reserve1, blocktimestamplastest) =
            uniswap_v2_pool.get_reserves().call().await.map_err(|err| {
                anyhow!(
                    "{}Unable to call `get_reserves` from `{uniswap_v2_pool:?}`, {err}",
                    *RED_ERROR_STRING
                )
            })?;
        Ok((reserve0, reserve1, blocktimestamplastest))
    }

    async fn swap(
        &self,
        amount_0_out: U256,
        amount_1_out: U256,
        to: H160,
        data: Bytes,
        which_provider: &WhichAPIProvider,
    ) -> Result<(())> {
        let provider = Self::get_ws_provider(which_provider).await?;
        let uniswap_v2_pool_once_cell = Self::UNISWAP_V2_POOL;
        let uniswap_v2_pool = uniswap_v2_pool_once_cell
            .get_or_init(|| UniswapV2Pool::<Provider<Ws>>::new(Self::ADDRESS, Arc::clone(provider)));
        uniswap_v2_pool
            .swap(amount_0_out, amount_1_out, to, data)
            .call()
            .await
            .map_err(|err| {
                anyhow!(
                    "{}Unable to call `swap` from `{uniswap_v2_pool:?}`, {err}",
                    *RED_ERROR_STRING
                )
            })?;
        Ok(())
    }
}

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

