mod macros;
use ethers;
use ethers_literal;
use serde;


fn expand_macro() {
    inner_declarative_macro!(<
        USDT, 0xdAC17F958D2ee523a2206206994597C13D831ec7_H160,
        BUSD, 0xdAC17F958D2ee523a2206206994597C13D831ec7_H160,
        USDTBUSD, 0xdAC17F958D2ee523a2206206994597C13D831ec7_H160>;);
}