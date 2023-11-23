use ethers_rs_proc_macro::impl_token_lp;

fn test_structsgen() {
    impl_token_lp!(UniswapV2AMM, "data.csv");
}