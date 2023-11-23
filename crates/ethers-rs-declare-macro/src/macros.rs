// $addr should be `tt` instead of `expr`
#[macro_export]
macro_rules! inner_declarative_macro {
    ($(<$token0:ident, $token0addr:tt, $token1:ident, $token1addr:tt, $id:ident, $contract:tt>;)*$(;)*) => {
        $(
            #[derive(::std::fmt::Debug, ::serde::Deserialize, ::serde::Serialize)]
            pub struct $token0;
            impl $token0 {
                // `::ethers::types::H160` instead of `crate::ethers::types::H160`
                // because Rust knows `::ethers` means imported crate
                pub const ADDRESS: ::ethers::types::H160 = ::ethers_literal::hash!(
                    $token0addr
                );
            }
            impl ::std::convert::Into<::ethers::types::H160> for $token0 {
                fn into(self) -> ::ethers::types::H160 {
                    <$token0>::ADDRESS
                }
            }

            #[derive(::std::fmt::Debug, ::serde::Deserialize, ::serde::Serialize)]
            pub struct $token1;
            impl $token1 {
                // `::ethers::types::H160` instead of `crate::ethers::types::H160`
                // because Rust knows `::ethers` means imported crate
                pub const ADDRESS: ::ethers::types::H160 = ::ethers_literal::hash!(
                    $token1addr
                );
            }
            impl ::std::convert::Into<::ethers::types::H160> for $token1 {
                fn into(self) -> ::ethers::types::H160 {
                    <$token1>::ADDRESS
                }
            }

            impl LPool<$token0, $token1> {
                pub const ADDRESS: ::ethers::types::H160 = ::ethers_literal::hash!(
                    $contract
                );
            }

            pub struct LPool<$token0, $token1>
            where
                $token0: ::std::convert::Into<::ethers::types::H160>,
                $token1: ::std::convert::Into<::ethers::types::H160>,
            {
                _token0: ::std::marker::PhantomData<$token0>,
                _token1: ::std::marker::PhantomData<$token1>,
            }

        )*

        crate::as_item! {
            enum LiquidityPools {
                $(
                    $id(LPool<$token0, $token1>),
                )*
            }
        }

    };
}

#[macro_export]
macro_rules! as_item {
    ($i:item) => {
        $i
    };
}
