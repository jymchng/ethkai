use anyhow::{anyhow, Ok, Result};
use dbg;
use ethers::{
    providers::{Middleware, Provider, Ws},
    types::{Filter, H160, U256},
};
use ethers_literal::{hash, num};
use ethers_rs_core::lp_types::{LiquidityPool, UniswapV2AMM, USDT, ZOOMER};
use ethers_rs_types::error::RED_ERROR_STRING;
use ethers_rs_types::init::{self, types::WhichAPIProvider};
use rayon::vec;
use std::str::FromStr;
use tokio::{self, spawn};

#[tokio::main]
async fn main() -> Result<()> {
    // AlchemyHttpProvider is the only one working.
    // Infura error: Error: (code: -32601, message: The method eth_newPendingTransactionFilter does not exist/is not available, data: None)
    // Quicknode: Error: Deserialization Error: EOF while parsing a value at line 1 column 0. Response:
    // let ALCHEMY_WS_PROVIDER = init::ws::init_provider(&WhichAPIProvider::ALCHEMY)
    //     .await
    //     .expect(&format!(
    //         "{}Cannot instantiate `ALCHEMY_WS_PROVIDER`",
    //         *ethers_rs_types::error::INITIALIZATION_ERROR_STRING,
    //     ));

    // USDT, 0xdAC17F958D2ee523a2206206994597C13D831ec7_H160;
    // NEON, 0x6ee9742d17b527e682248dca85952e4fe190061d_H160;
    // WETH, 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2_H160;
    // ZOOMER, 0x0d505c03d30e65f6e9b4ef88855a47a89e4b7676_H160;
    // PEPE2, 0xfb66321d7c674995dfcc2cb67a30bc978dc862ad_H160;
    // <Token::USDT, Token::WETH, USDTWETHV2>: 0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852_H160;
    // <Token::ZOOMER, Token::WETH, ZOOMERWETHV2>: 0xc2c701110de2b9503e98619d9c9e3017877b0f72_H160;
    // <Token::WETH, Token::PEPE2, WETHPEPE2V2>: 0x076a3e1500f3110d8f4445d396a3d7ca6d0ca269_H160;
    let zoomer_hex_string: &str = "0x0d505c03d30e65f6e9b4ef88855a47a89e4b7676";
    let weth_hex_string: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
    let pepe2_hex_string: &str = "0xfb66321d7c674995dfcc2cb67a30bc978dc862ad";
    let usdt_hex_string: &str = "0xdAC17F958D2ee523a2206206994597C13D831ec7";

    let wethpepe2_lpool = UniswapV2AMM::from_addresses(
        H160::from_str(weth_hex_string)?,
        H160::from_str(pepe2_hex_string)?,
    );

    let usdtweth_lpool = UniswapV2AMM::from_addresses(
        H160::from_str(usdt_hex_string)?,
        H160::from_str(weth_hex_string)?,
    );

    let (reserve0, reserve1, blocktimestamplastest) = usdtweth_lpool.get_reserves(&WhichAPIProvider::INFURA).await.unwrap();
    println!("reserve0={reserve0}, reserve1={reserve1}, blockTimestampLast={blocktimestamplastest}!");

    let mut handles = vec![];

    // let wethpepe2_lpool = LPool::<WETH, PEPE2>::default();
    // let usdtweth_lpool = LPool::<USDT, WETH>::default();

    let thread_0 =
        spawn(async move { usdtweth_lpool.get_reserves(&WhichAPIProvider::INFURA).await });
    handles.push(thread_0);

    let thread_1 = spawn(async move {
        wethpepe2_lpool
            .get_reserves(&WhichAPIProvider::INFURA)
            .await
    });
    handles.push(thread_1);

    for handle in handles.into_iter() {
        match handle.await {
            std::result::Result::Ok(inner_result) => {
                if let std::result::Result::Ok((reserve0, reserve1, blocktimestamplastest)) =
                    inner_result
                {
                    println!("reserve0={reserve0}, reserve1={reserve1}, blockTimestampLast={blocktimestamplastest}!");
                };
            }
            Err(err) => {
                println!("{}Failed to get reserves, err={err}", *RED_ERROR_STRING);
            }
        };
    }
    println!("Ended!");
    Ok(())
}
