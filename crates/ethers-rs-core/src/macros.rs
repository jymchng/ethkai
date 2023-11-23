
#[macro_export]
// $addr should be `tt` instead of `expr`
macro_rules! impl_into_address_for_tokens {
    ($($ticker:ident, $addr:tt);*$(;)*) => {
        $(
            #[derive(::std::fmt::Debug, ::serde::Deserialize, ::serde::Serialize, ::std::default::Default)]
            pub struct $ticker;
            impl $ticker {
                // `::ethers::types::H160` instead of `crate::ethers::types::H160`
                // because Rust knows `::ethers` means imported crate
                pub const ADDRESS: ::ethers::types::H160 = ::ethers_literal::hash!(
                    $addr
                );
                pub const ADDRESS_BYTES: [u8; 20] = ::ethers_literal::hash!(
                    $addr
                ).0;
            }
            impl ::std::convert::Into<::ethers::types::H160> for $ticker {
                fn into(self) -> ::ethers::types::H160 {
                    <$ticker>::ADDRESS
                }
            }
        )*

        crate::as_item! {
            pub enum TokensEnum {
                $(
                    $ticker($ticker),
                )*
            }
        }
    };
}

#[macro_export]
macro_rules! impl_into_address_for_lpool {
    ($(<$token0:ty, $token1:ty, $id:ident>: $addr:tt);*$(;)*) => {
        $(
            // need to specify `crate` here else Rust thinks `::consts::...` is imported from somewhere
            impl crate::types::LPool<$token0, $token1> {
                pub const ADDRESS: ::ethers::types::H160 = ::ethers_literal::hash!(
                    $addr
                );

                // pub async fn get_reserves<M: ethers::providers::Middleware>(
                //     uniswap_v2_pool: crate::abigen::contracts::uniswap::UniswapV2Pool<M>,
                // ) -> ::std::result::Result<(u128, u128, u32), ::ethers::contract::ContractError<M>> {
                //     let (reserve0, reserve1, blocktimestamplastest): (u128, u128, u32) = uniswap_v2_pool.get_reserves().call().await?;
                //     ::std::result::Result::Ok((reserve0, reserve1, blocktimestamplastest))
                // }
            }

            #[::async_trait::async_trait]
            impl crate::types::LPoolTrait for crate::types::LPool<$token0, $token1> {
                const ADDRESS: ::ethers::types::H160 = Self::ADDRESS;
                const UNISWAP_V2_POOL: ::once_cell::sync::OnceCell<crate::abigen::contracts::uniswap::UniswapV2Pool<Provider<Ws>>> = ::once_cell::sync::OnceCell::new();
            }

        )*

        crate::as_item! {
            // #[::enum_dispatch::enum_dispatch]
            pub enum LiquidityPools {
                $(
                    $id(crate::types::LPool<$token0, $token1>),
                )*
            }

            // impl LiquidityPools {
            //     pub fn get_reserves(&self) -> Self {
            //         match (token0_addr, token1_addr) {
            //             $(
            //                 ($token0::ADDRESS.to_string().as_str())
            //             )*
            //         }
            //     }

            // }
        }
    }
}

#[macro_export]
macro_rules! as_item {
    ($i:item) => {
        $i
    };
}
