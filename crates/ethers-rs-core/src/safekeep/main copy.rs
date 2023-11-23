use anyhow::{anyhow, Result};
use dbg;
use ethers_rs_core::{
    abigen::contracts::uniswap::UniswapV2Pool,
    consts::{self, LPool, Token},
    error::RED_ERROR_STRING,
    init::{
        self,
        https::{ALCHEMY_HTTP_PROVIDER, INFURA_HTTP_PROVIDER, QUICK_NODE_HTTP_PROVIDER},
        types::WhichAPIProvider,
    },
};
use ethers::{providers::Middleware, types::U256};
use futures::{executor::block_on, StreamExt};
use tracing::instrument;
use tracing::{debug, error, info, span, warn, Level};
use ethers_literal::{num, hash};

#[tokio::main]
async fn main() -> Result<()> {
    // AlchemyHttpProvider is the only one working.
    // Infura error: Error: (code: -32601, message: The method eth_newPendingTransactionFilter does not exist/is not available, data: None)
    // Quicknode: Error: Deserialization Error: EOF while parsing a value at line 1 column 0. Response:
    let ALCHEMY_WS_PROVIDER = init::ws::init_provider(&WhichAPIProvider::ALCHEMY)
        .await
        .expect(&format!(
            "{}Cannot instantiate `ALCHEMY_WS_PROVIDER`",
            *ethers_rs_core::error::INITIALIZATION_ERROR_STRING,
        ));

    // while let Some(txn) = ALCHEMY_WS_PROVIDER
    //     .watch_pending_transactions()
    //     .await?
    //     .transactions_unordered(20usize)
    //     .next()
    //     .await
    // {
    //     match txn {
    //         Ok(txn) => println!("{:?}", txn),
    //         Err(_err) => {}
    //     }
    // }

    let uniswapv2_pool = UniswapV2Pool::new(
        // LOL need to dereference it, which makes sense since `Lazy` must implement Deref to give `T` of `Lazy<T>`
        // Previous error encountered: the trait `From<ethers::ethers_contract::Lazy<H160>>` is not implemented for `H160`
        LPool::<USDT, WETH, V2>::ADDRESS,
        ALCHEMY_WS_PROVIDER.into(),
    );

    println!(
        "Contract address: {:?}; USDT Token Address: {:?}; WETH Token Address: {:?}",
        LPool::<USDT, WETH, V2>::ADDRESS,
        USDT::ADDRESS,
        WETH::ADDRESS,
    );
    // let (reserve0, reserve1, blockTimestampLast): (u128, u128, u32) =
    //     uniswapv2_pool.get_reserves().call().await.map_err(|err| {
    //         anyhow!(
    //             "{}Unable to call `get_reserves` from `{uniswapv2_pool:?}`, {err}",
    //             *RED_ERROR_STRING
    //         )
    //     })?;
    let (reserve0, reserve1, blockTimestampLast): (u128, u128, u32) = 
        LPool::<USDT, WETH, V2>::get_reserves(uniswapv2_pool).await?;

    println!(
        "reserve0={} reserve1={} blockTimestampLast={}",
        reserve0, reserve1, blockTimestampLast
    );
    Ok(())
}


// 08 July 2023

#[tokio::main]
async fn main() -> Result<()> {
    // AlchemyHttpProvider is the only one working.
    // Infura error: Error: (code: -32601, message: The method eth_newPendingTransactionFilter does not exist/is not available, data: None)
    // Quicknode: Error: Deserialization Error: EOF while parsing a value at line 1 column 0. Response:
    let ALCHEMY_WS_PROVIDER = init::ws::init_provider(&WhichAPIProvider::ALCHEMY)
        .await
        .expect(&format!(
            "{}Cannot instantiate `ALCHEMY_WS_PROVIDER`",
            *ethers_rs_core::error::INITIALIZATION_ERROR_STRING,
        ));

    

    // let uniswapv2_factory = UniswapV2Factory::new(
    //     UniswapV2_Consts::FACTORY_ADDRESS,
    //     ALCHEMY_WS_PROVIDER.into(),
    // );

    // let uniswapv2_pool = UniswapV2Pool::new(
    //     // LOL need to dereference it, which makes sense since `Lazy` must implement Deref to give `T` of `Lazy<T>`
    //     // Previous error encountered: the trait `From<ethers::ethers_contract::Lazy<H160>>` is not implemented for `H160`
    //     LPool::<USDT, WETH>::ADDRESS,
    //     ALCHEMY_WS_PROVIDER.into(),
    // );

    let mut handles = vec![];

    let thread_0 = spawn(async {
        <LPool::<USDT, WETH> as LPoolTrait>::get_reserves(&WhichAPIProvider::INFURA).await
    });
    handles.push(thread_0);

    let thread_1 = spawn(async {
        <LPool::<WETH, PEPE2> as LPoolTrait>::get_reserves(&WhichAPIProvider::INFURA).await
    });
    handles.push(thread_1);
    for handle in handles.into_iter() {
        match handle.await {
            std::result::Result::Ok(inner_result) => {
            if let std::result::Result::Ok((reserve0, reserve1, blocktimestamplastest)) = inner_result {
                println!("reserve0={reserve0}, reserve1={reserve1}, blockTimestampLast={blocktimestamplastest}!");
            };
        },
            Err(err) => {
                eprintln!("{}Failed to get reserves, err={err}", *RED_ERROR_STRING);
            }
        };
    }

    // println!(
    //     "Contract address: {:?}; USDT Token Address: {:?}; WETH Token Address: {:?}",
    //     LPool::<USDT, WETH>::ADDRESS,
    //     USDT::ADDRESS,
    //     WETH::ADDRESS,
    // );
    // let (reserve0, reserve1, blockTimestampLast): (u128, u128, u32) =
    //     uniswapv2_pool.get_reserves().call().await.map_err(|err| {
    //         anyhow!(
    //             "{}Unable to call `get_reserves` from `{uniswapv2_pool:?}`, {err}",
    //             *RED_ERROR_STRING
    //         )
    //     })?;
    // let (reserve0, reserve1, blockTimestampLast): (u128, u128, u32) = 
    //     LPool::<USDT, WETH>::get_reserves(uniswapv2_pool).await?;
    
    Ok(())
}


// 09 July 2023
#[tokio::main]
async fn main() -> Result<()> {
    // AlchemyHttpProvider is the only one working.
    // Infura error: Error: (code: -32601, message: The method eth_newPendingTransactionFilter does not exist/is not available, data: None)
    // Quicknode: Error: Deserialization Error: EOF while parsing a value at line 1 column 0. Response:
    let ALCHEMY_WS_PROVIDER = init::ws::init_provider(&WhichAPIProvider::ALCHEMY)
        .await
        .expect(&format!(
            "{}Cannot instantiate `ALCHEMY_WS_PROVIDER`",
            *ethers_rs_core::error::INITIALIZATION_ERROR_STRING,
        ));

    let zoomer_hex_string: &str = "0x0d505c03d30e65f6e9b4ef88855a47a89e4b7676";

    let mut handles = vec![];

    let wethpepe2_lpool = LPool::<WETH, PEPE2>::default();
    let usdtweth_lpool = LPool::<USDT, WETH>::default();

    let thread_0 = spawn(async move {
        usdtweth_lpool.get_reserves(&WhichAPIProvider::INFURA).await
    });
    handles.push(thread_0);

    let thread_1 = spawn(async move {
        wethpepe2_lpool.get_reserves(&WhichAPIProvider::INFURA).await
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
                eprintln!("{}Failed to get reserves, err={err}", *RED_ERROR_STRING);
            }
        };
    }

    Ok(())
}


pub struct H160_Wrap(H160);

impl PartialEq for H160_Wrap {
    fn eq(&self, other: &Self) -> bool {
        let myaddr = self.0;
        let otheraddr = other.0;

        myaddr.to_fixed_bytes() == otheraddr.to_fixed_bytes()
    }
}

impl DerefMut for H160_Wrap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for H160_Wrap {
    type Target = H160;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}