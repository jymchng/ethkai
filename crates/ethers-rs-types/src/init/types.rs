use std::fmt;

pub const INFURA_API_HTTP_URL: &str = "https://mainnet.infura.io/v3/";
pub const MORALIS_API_HTTP_URL: &str = "https://deep-index.moralis.io/api/v2/";
pub const ALCHEMY_API_HTTP_URL: &str = "https://eth-mainnet.g.alchemy.com/v2/";
pub const QUICKNODE_API_HTTP_URL: &str =
    "https://palpable-muddy-putty.discover.quiknode.pro/";

pub const INFURA_API_WS_URL: &str = "wss://mainnet.infura.io/ws/v3/";

pub const ALCHEMY_API_WS_URL: &str = "wss://eth-mainnet.g.alchemy.com/v2/";

// pub struct INFURA;
// pub struct MORALIS;
// pub struct ALCHEMY;
// pub struct QUICKNODE;

// pub trait APIProvider {
//     const HTTP_URL: &str;
//     const WS_URL: &str;
// }

// impl INFURA {
//     pub const HTTP_URL: &str = "https://mainnet.infura.io/v3/";
//     pub const WS_URL: &str = "wss://mainnet.infura.io/ws/v3/";
// }

// impl MORALIS {
//     pub const HTTP_URL: &str = "https://deep-index.moralis.io/api/v2/";
// }


#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum WhichAPIProvider {
    MORALIS,
    ALCHEMY,
    INFURA,
    QUICKNODE,
}

impl fmt::Display for WhichAPIProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MORALIS => write!(f, "Moralis API Provider"),
            Self::ALCHEMY => write!(f, "Alchemy API Provider"),
            Self::INFURA => write!(f, "Infura API Provider"),
            Self::QUICKNODE => write!(f, "Quicknode API Provider"),
            _ => unreachable!(),
        }
    }
}
