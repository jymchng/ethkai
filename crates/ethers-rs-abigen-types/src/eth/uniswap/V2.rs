use ethers::{prelude::abigen, types::Address};
use std::marker::PhantomData;

pub struct UniswapV2_Consts;

impl UniswapV2_Consts {
    pub const FACTORY_ADDRESS: Address =
        crate::ethers_literal::hash!(0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f_H160);
}

abigen!(
    UniswapV2Router,
    "src/eth/uniswap/abis/uniswapV2Router.abi",
    event_derives(serde::Deserialize, serde::Serialize)
);


abigen!(
    UniswapV2Pool,
    "src/eth/uniswap/abis/uniswapV2Pool.abi",
    event_derives(serde::Deserialize, serde::Serialize)
);

abigen!(
    UniswapV2Factory,
    "src/eth/uniswap/abis/uniswapV2Factory.abi",
    event_derives(serde::Deserialize, serde::Serialize)
);
