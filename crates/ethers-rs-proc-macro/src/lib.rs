use proc_macro::TokenStream;
use ethers;
use ethers_literal;
use async_trait;
use enum_dispatch;
use ethers_rs_abigen_types;
use ethers_rs_types;
use ethkai_init;

mod structsgen;
mod chainsgen;

// #[doc = include_str!("../Readme.md")]
#[proc_macro]
pub fn impl_token_lp(stream: TokenStream) -> TokenStream {
    structsgen::transform_stream_hash(stream).into()
}

#[proc_macro]
pub fn impl_chain_ids_wallets(stream: TokenStream) -> TokenStream {
    chainsgen::transform_stream_hash(stream).into()
}

// fn use_macro() {
//     impl_token_lp!(UniswapV2AMM, "data.csv");
// }
