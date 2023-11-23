// #![doc = include_str!("../Readme.md")]
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

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Deserialize, Clone)]
struct Record {
    token0: String,
    token0addr: String,
    token1: String,
    token1addr: String,
    contract: String,
}

impl Parse for TokenStreamParsedStruct {
    // impl_token_lp!(UniswapV2AMM, "data.csv");
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

struct TokenStreamParsedStruct {
    new_ty: Ident,
    file_path: LitStr,
}

/// Iterate over a [`TokenStream`] and transform all [`TokenTree`]s.
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
    let csv_records = match parse_csv(&file_path) {
        Ok(records) => {records},
        Err(err) => panic!("Unable to get the `csv_records` {}", err)
    };
    dbg!(&csv_records);
    let mut final_tokenstream = TokenStream2::new();
    let lp_pool_tokenstream = quote!{
        #[derive(::std::default::Default, ::std::clone::Clone)]
        pub struct LiquidityPool<Token0, Token1>
            where
                Token0: ::std::convert::Into<::ethers::types::H160>,
                Token1: ::std::convert::Into<::ethers::types::H160>,
            {
                _token0: ::std::marker::PhantomData<Token0>,
                _token1: ::std::marker::PhantomData<Token1>,
            }
        pub type LPool<Token0, Token1> = LiquidityPool<Token0, Token1>;
    };
    let uniswapv2pool_tokenstream = quote!(
        ::ethers_rs_abigen_types::eth::uniswap::V2::UniswapV2Pool
    );
    let which_provider_tokenstream = quote!{
        ::ethers_rs_types::init::types::WhichAPIProvider
    };
    let red_error_string_tokenstream = quote!{
        ::ethers_rs_types::error::RED_ERROR_STRING
    };
    let ethers_provider_path = quote!{
        ::ethers::providers
    };
    let lpooltrait_tokenstream = quote!{    
        #[::async_trait::async_trait]
        pub trait LPoolTrait {
            const ADDRESS: ethers::types::H160;
            const UNISWAP_V2_POOL: once_cell::sync::OnceCell<#uniswapv2pool_tokenstream<#ethers_provider_path::Provider<#ethers_provider_path::Ws>>>;
            
            async fn get_ws_provider(which_provider: &#which_provider_tokenstream) -> 
            ::anyhow::Result<&std::sync::Arc<#ethers_provider_path::Provider<#ethers_provider_path::Ws>>>
            {
                ::ethkai_init::ARC_WS_PROVIDER.get_or_try_init(
                    async {
                        let provider = ::ethkai_init::ws::init_provider(which_provider).await?;
                        let provider = ::std::sync::Arc::new(provider);
                        Ok(::std::sync::Arc::clone(&provider))
                    }
                ).await
            }

            async fn get_reserves(&self, which_provider: &#which_provider_tokenstream) -> ::anyhow::Result<(u128, u128, u32)> {
                let provider = Self::get_ws_provider(which_provider).await?;
                let uniswap_v2_pool_once_cell = Self::UNISWAP_V2_POOL;
                let uniswap_v2_pool = uniswap_v2_pool_once_cell
                    .get_or_init(|| #uniswapv2pool_tokenstream::new(Self::ADDRESS, ::std::sync::Arc::clone(provider)));
                let (reserve0, reserve1, blocktimestamplastest) =
                    uniswap_v2_pool.get_reserves().call().await.map_err(|err| {
                        ::anyhow::anyhow!(
                            "{}Unable to call `get_reserves` from `{:?}`, {}",
                            *#red_error_string_tokenstream,
                            uniswap_v2_pool,
                            err,
                        )
                    })?;
                Ok((reserve0, reserve1, blocktimestamplastest))
            }

            async fn swap(
                &self,
                amount_0_out: ::ethers::types::U256,
                amount_1_out: ::ethers::types::U256,
                to: ::ethers::types::H160,
                data: ::ethers::types::Bytes,
                which_provider: &#which_provider_tokenstream,
            ) -> anyhow::Result<(())> {
                let provider = Self::get_ws_provider(which_provider).await?;
                let uniswap_v2_pool_once_cell = Self::UNISWAP_V2_POOL;
                let uniswap_v2_pool = uniswap_v2_pool_once_cell
                    .get_or_init(|| #uniswapv2pool_tokenstream::new(Self::ADDRESS, ::std::sync::Arc::clone(provider)));
                uniswap_v2_pool
                    .swap(amount_0_out, amount_1_out, to, data)
                    .call()
                    .await
                    .map_err(|err| {
                        anyhow::anyhow!(
                            "{}Unable to call `swap` from `{:?}`, {}",
                            *#red_error_string_tokenstream,
                            uniswap_v2_pool,
                            err,
                        )
                    })?;
                Ok(())
            }
        }
    };
    let generated_structs_tokenstream: TokenStream2 = generate_structs_tokenstream(&csv_records);
    let generated_enum_token_stream: TokenStream2 = generate_enum_tokenstream(new_ty, &csv_records);
    final_tokenstream.extend(lp_pool_tokenstream);
    final_tokenstream.extend(lpooltrait_tokenstream);
    final_tokenstream.extend(generated_structs_tokenstream);
    final_tokenstream.extend(generated_enum_token_stream);
    final_tokenstream.into()
}

/// From: https://github.com/Mossop/enum_dispatch/blob/master/src/enum_dispatch_variant.rs
/// When expanding shorthand `enum_dispatch` enum syntax, each specified, unnamed type variant must
/// acquire an associated identifier to use for the name of the standard Rust enum variant.
///
/// Note that `proc_macro_attribute`s cannot provide custom syntax parsing. Unless using a
/// function-style procedural macro, each type must already be parseable as a unit enum variant.
/// This rules out, for example, generic types with lifetime or type parameters. For these, an
/// explicitly named variant must be used.
fn into_type(ident: syn::Ident) -> syn::Type {
    syn::Type::Path(syn::TypePath {
        path: syn::Path {
            leading_colon: None,
            segments: syn::punctuated::Punctuated::from_iter(vec![syn::PathSegment {
                arguments: syn::PathArguments::None,
                ident: ident.to_owned()
            }])
        },
        qself: None
    })
}

fn generate_variant_names_for_enum_tokenstream(tokens0: String, tokens1: String) -> syn::Ident {
    let enum_variant_ident = syn::Ident::new(&format!("{tokens0}{tokens1}"), proc_macro2::Span::call_site());
    enum_variant_ident
}

fn generate_enum_tokenstream(newtype: syn::Ident, csv_records: &Vec<Record>) -> TokenStream2 {
    let csv_records = csv_records.clone();
    let tokens0 = csv_records.iter().map(|record| {
        let token0 = record.clone().token0;
        let token0_ty = syn::parse_str::<syn::Type>(&token0).expect(&format!("Unable to convert {} to a `syn::Type`", token0));
        token0_ty
    });
    let tokens1 = csv_records.iter().map(|record| {
        let token1 = record.clone().token1;
        let token1_ty = syn::parse_str::<syn::Type>(&token1).expect(&format!("Unable to convert {} to a `syn::Type`", token1));
        token1_ty
    });
    let enum_variant_idents = csv_records.iter().map(|record| {
        let record = record.clone();
        generate_variant_names_for_enum_tokenstream(record.token0, record.token1)
    });
    let tokens0_cloned = tokens0.clone();
    let tokens1_cloned = tokens1.clone();
    let enum_variant_idents_cloned = enum_variant_idents.clone();
    let mut final_tokenstream = TokenStream2::new();

    let enum_tokenstream = quote!{
        // [enum_dispatch]
        pub enum #newtype {
            #(
                #enum_variant_idents_cloned(LPool<#tokens0_cloned, #tokens1_cloned>),
            )*
        }
    };

    let tokens0_cloned = tokens0.clone();
    let tokens1_cloned = tokens1.clone();
    let enum_variant_idents_cloned = enum_variant_idents.clone();
    
    let enum_impl_tokenstream_one = quote!{
        impl #newtype {
            pub fn from_addresses(token0_addr: ethers::types::H160, token1_addr: ethers::types::H160) -> Self {
                match (token0_addr.to_fixed_bytes(), token1_addr.to_fixed_bytes()) {
                    #((#tokens0_cloned::ADDRESS_BYTES, #tokens1_cloned::ADDRESS_BYTES) => {
                        Self::#enum_variant_idents_cloned(LPool::<#tokens0_cloned, #tokens1_cloned>::default())
                    },)*
                    _ => {unreachable!()},
                }
            }
        }
    };

    let which_provider_tokenstream = quote!{
        ::ethers_rs_types::init::types::WhichAPIProvider
    };
    let enum_variant_idents_cloned = enum_variant_idents.clone();

    let which_provider_tokenstream_cloned = which_provider_tokenstream.clone();

    let enum_impl_tokenstream_two = quote!{
        impl #newtype {
            pub async fn get_reserves(&self, which_provider: &#which_provider_tokenstream_cloned) -> anyhow::Result<(u128, u128, u32)> {
                match self {
                    #(Self::#enum_variant_idents_cloned(val) => {
                        val.get_reserves(which_provider).await
                    },)*
                    _ => {unreachable!()},
                }
            }
        }
    };

    let enum_variant_idents_cloned = enum_variant_idents.clone();

    let which_provider_tokenstream_cloned = which_provider_tokenstream.clone();

    let enum_impl_tokenstream_three = quote!{
        impl #newtype {
            pub async fn swap(&self,
            amount_0_out: ethers::types::U256,
            amount_1_out: ethers::types::U256,
            to: ethers::types::H160,
            data: ethers::types::Bytes,
            which_provider: &#which_provider_tokenstream_cloned)
            -> anyhow::Result<()> {
            match self {
                #(Self::#enum_variant_idents_cloned(val) => {
                    val.swap(amount_0_out, amount_1_out, to, data, which_provider).await
                },)*
                _ => {unreachable!()},}
            }
        }
    };

    final_tokenstream.extend(enum_tokenstream);
    final_tokenstream.extend(enum_impl_tokenstream_one);
    final_tokenstream.extend(enum_impl_tokenstream_two);
    final_tokenstream.extend(enum_impl_tokenstream_three);
    final_tokenstream
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

fn generate_structs_tokenstream(csv_records: &Vec<Record>) -> TokenStream2 {
    let mut hs = HashSet::new();
    let mut tokenstream = proc_macro2::TokenStream::new();
    for csv_record in csv_records {
        dbg!(&csv_record);
        let token0 = syn::parse_str::<Type>(&csv_record.token0).unwrap();
        let token1 = syn::parse_str::<Type>(&csv_record.token1).unwrap();
        let token0addr = match proc_macro2::TokenStream::from_str(&csv_record.token0addr) {
            Ok(addr) => {addr},
            Err(err) => panic!("Cannot form a `TokenStream` from {} due to {}", &csv_record.token0addr, err),
        };
        let token1addr = match proc_macro2::TokenStream::from_str(&csv_record.token1addr) {
            Ok(addr) => {addr},
            Err(err) => panic!("Cannot form a `TokenStream` from {} due to {}", &csv_record.token1addr, err),
        };
        let contract = match proc_macro2::TokenStream::from_str(&csv_record.contract) {
            Ok(addr) => {addr},
            Err(err) => panic!("Cannot form a `TokenStream` from {} due to {}", &csv_record.contract, err),
        };

        let mut zero_stream = if !hs.contains(&&csv_record.token0) {
            dbg!(&hs);
            quote!{
            #[derive(::std::fmt::Debug, ::serde::Deserialize, ::serde::Serialize, ::std::default::Default, ::std::clone::Clone)]
            pub struct #token0;

            impl #token0 {
                pub const ADDRESS: ::ethers::types::H160 = ::ethers_literal::hash!(#token0addr);
                pub const ADDRESS_BYTES: [u8; 20] = ::ethers_literal::hash!(
                    #token0addr
                ).0;
            }
            impl ::std::convert::Into<::ethers::types::H160> for #token0 {
                fn into(self) -> ::ethers::types::H160 {
                    <#token0>::ADDRESS
                }
            }
        }
        } else {
            dbg!("[`zero_stream` did not run]");
            quote!{}
        };
        hs.insert(&csv_record.token0);
        
        let mut first_stream = if !hs.contains(&&csv_record.token1) {
            dbg!(&hs);
            quote!{
                #[derive(::std::fmt::Debug, ::serde::Deserialize, ::serde::Serialize, ::std::default::Default, ::std::clone::Clone)]
                pub struct #token1;
                impl #token1 {
                    pub const ADDRESS: ::ethers::types::H160 = ::ethers_literal::hash!(#token1addr);
                    pub const ADDRESS_BYTES: [u8; 20] = ::ethers_literal::hash!(
                        #token1addr
                    ).0;
                }
                impl ::std::convert::Into<::ethers::types::H160> for #token1 {
                    fn into(self) -> ::ethers::types::H160 {
                        <#token1>::ADDRESS
                    }
                }
            }
            } else {
                dbg!("[`first_stream` did not run]");
                quote!{}
        };
        hs.insert(&csv_record.token1);

        let mut second_stream: TokenStream2 = quote!{
            impl LiquidityPool<#token0, #token1>
            where
                #token0: ::std::convert::Into<::ethers::types::H160>,
                #token1: ::std::convert::Into<::ethers::types::H160>,
            {
                pub const ADDRESS: ::ethers::types::H160 = ::ethers_literal::hash!(#contract);
            }

        }.into();

        let uniswapv2pool_tokenstream = quote!(::ethers_rs_abigen_types::eth::uniswap::V2::UniswapV2Pool);
        let which_provider_tokenstream = quote!{
            ::ethers_rs_types::init::types::WhichAPIProvider
        };
        let red_error_string_tokenstream = quote!{
            ::ethers_rs_types::error::RED_ERROR_STRING
        };
        let ethers_provider_path = quote!{
            ::ethers::providers
        };

        let third_stream: TokenStream2 = quote!{
            #[::async_trait::async_trait]
            impl LPoolTrait for LPool<#token0, #token1> {
                const ADDRESS: ::ethers::types::H160 = Self::ADDRESS;
                const UNISWAP_V2_POOL: ::once_cell::sync::OnceCell<#uniswapv2pool_tokenstream<#ethers_provider_path::Provider<#ethers_provider_path::Ws>>> = ::once_cell::sync::OnceCell::new();
            }
        };
        
        second_stream.extend(third_stream);
        first_stream.extend(second_stream);
        zero_stream.extend(first_stream);
        tokenstream.extend(zero_stream);
    }
    tokenstream
}

fn parse_csv(csv_path: impl AsRef<Path>) -> Result<Vec<Record>> {
    let mut csv_reader = csv::Reader::from_path(csv_path)?;
    eprintln!("{:?}", &csv_reader);
    let mut result_vector = vec![];

    for record in csv_reader.deserialize() {
        let record_: Record = record?;
        eprintln!("{:?}", &record_);
        result_vector.push(record_);
    }
    Ok(result_vector)
}

pub trait IdentExt {
    fn create_from_str(&self, string: &str) -> Self;
    fn as_str(&self) -> String;
}

impl IdentExt for proc_macro2::Ident {
    /// Generates a new identifier using the given string template as the name and
    /// the span from the `self` [Ident]. The template string can contain `{}`
    /// placeholders for the `self` [Ident] name.
    fn create_from_str(&self, name_with_template_placeholder: &str) -> Self {
        let name = str::replace(name_with_template_placeholder, "{}", &self.to_string());
        proc_macro2::Ident::new(&name, self.span())
    }

    fn as_str(&self) -> String { std::string::ToString::to_string(&self) }
}