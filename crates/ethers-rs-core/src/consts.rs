// use crate::{abigen::contracts::uniswap::UniswapV2Pool, impl_into_address_for_lpool};
use anyhow::{anyhow, Ok, Result};
use ethers::{
    prelude::k256::U256,
    types::{Address, H160},
};
use std::marker::PhantomData;

pub const UNISWAPV2_FACTORY_ADDRESS: Address =
    crate::ethers_literal::hash!(0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f_H160);
