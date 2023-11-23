

fn main() {
    impl_token_lp!(UniswapV2AMM, "data.csv");
    println!("{:?}", USDT::ADDRESS);
    println!("{:?}", WETH::ADDRESS);
    println!("{:?}", ZOOMER::ADDRESS);
}

// 10 July 2023

fn main() {
    #[derive(Default)]
    pub struct A;
    #[derive(Default)]
    pub struct B;
    #[derive(Default)]
    pub struct C;
    #[derive(Default)]
    pub struct D;

    #[derive(Default)]
    pub struct Pair<T, U> {
        t: T,
        u: U,
    }

    #[async_trait::async_trait]
    #[enum_dispatch::enum_dispatch(Pairs, Default)] // `Pairs` is the enum
    pub trait Print {
        // const WHAT_NUM: u32;
        fn print(&self) -> &u32 {
            &69
        }
    }

    #[async_trait::async_trait]
    impl Print for Pair<A, B> {
        // const WHAT_NUM: u32 = Self::A_NUM;
    }

    #[async_trait::async_trait]
    impl Print for Pair<C, D> {
        // const WHAT_NUM: u32 = Self::A_NUM;
    }

    #[async_trait::async_trait]
    impl Print for Pair<A, D> {
        // const WHAT_NUM: u32 = Self::A_NUM;
    }

    #[async_trait::async_trait]
    impl Print for Pair<C, B> {
        // const WHAT_NUM: u32 = Self::A_NUM;
    }

    #[enum_dispatch::enum_dispatch]
    pub enum Pairs {
        AB(Pair<A, B>),
        CD(Pair<C, D>),
        AD(Pair<A, D>),
        CB(Pair<C, B>),
    }

    impl Pairs {
        fn from_str(s: &str) -> Self {
            match s {
                "AB" => Self::AB(Pair::<A, B>::default()),
                "CD" => Self::CD(Pair::<C, D>::default()),
                "AD" => Self::AD(Pair::<A, D>::default()),
                "CB" => Self::CB(Pair::<C, B>::default()),
                _ => {
                    unreachable!()
                }
            }
        }
    }

    let A_NUM = "AD";
    let an_enum_variant = Pairs::from_str(A_NUM);
    println!("{}", an_enum_variant.print());
}
