#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]

use proc_macro2::{self, Ident, TokenStream as TokenStream2};
use proc_macro::TokenStream;
use serde::Deserialize;
use std::convert::From;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fmt::Write};
use syn::parse::{Parse, ParseStream};
use syn::{braced, parse_macro_input, token, Field, Token, Type, LitStr, Lit};
use quote::quote;
use std::collections::HashSet;
use std::cell::RefCell;
use ethers_rs_types::{wallet::{chain_id}};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Deserialize, Clone)]
struct ChainIDRecord {
    // Hex,ChainId (Decimal),Network
    // 0x1,1,Ethereum Main Network (Mainnet)
    #[serde(rename(deserialize = "Hex"))]
    hex_str: String,
    #[serde(rename(deserialize = "ChainId (Decimal)"))]
    chain_id: u64,
    #[serde(rename(deserialize = "Network"))]
    network_name: String,
}

struct TokenStreamParsedStruct {
    new_ty: Ident,
    file_path: LitStr,
}

impl Parse for TokenStreamParsedStruct {
    // impl_chain_ids_wallets!(UniswapV2AMM, "chain_ids.csv");
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let uniswapv2amm: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let file_path: LitStr = input.parse()?;
        Ok(Self {
            new_ty: uniswapv2amm,
            file_path: file_path,
        })
    }
}

fn parse_file_path(path: impl AsRef<str>) -> Result<PathBuf> {
    let mut file_path = PathBuf::from_str(path.as_ref())?;
    if file_path.is_relative() {
        // set root at manifest dir, if the path exists
        if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
            dbg!(&manifest_dir);
            let new = PathBuf::from(manifest_dir).join(&file_path);
            dbg!(&new);
            if new.exists() {
                file_path = new;
            } else {
                panic!("Cannot find the file_path = {}", file_path.display())
            }
        }
    };
    Ok(file_path)
}

pub fn transform_stream_hash(input: TokenStream) -> TokenStream {
    let TokenStreamParsedStruct {
        new_ty,
        file_path,
    } = parse_macro_input!(input as TokenStreamParsedStruct);
    eprintln!("{new_ty:?}, {:?}", file_path.value());
    let file_path = match parse_file_path(&file_path.value()) {
        Ok(file_path) => file_path,
        Err(err) => panic!("Unable to parse the `file_path` {} due to {}", &file_path.value(), err),
    };
    let csv_records = match parse_csv_for_chain_id_records(&file_path) {
        Ok(records) => {records},
        Err(err) => panic!("Unable to get the `csv_records` {}", err)
    };
    dbg!(&csv_records);
    TokenStream::new().into()
}


fn parse_csv_for_chain_id_records(csv_path: impl AsRef<Path>) -> Result<Vec<ChainIDRecord>> {
    let mut csv_reader = csv::Reader::from_path(csv_path)?;
    eprintln!("{:?}", &csv_reader);
    let mut result_vector = vec![];

    for record in csv_reader.deserialize() {
        let record_: ChainIDRecord = record?;
        eprintln!("{:?}", &record_);
        result_vector.push(record_);
    }
    Ok(result_vector)
}