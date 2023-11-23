// pub mod abigen;
pub mod consts;
// pub mod error;
// pub mod init;
// pub mod io;
pub mod lp_types;
pub mod macros;
pub mod txns;
pub mod wallet;

use anyhow;
use ethers::{self, types::H160};
use ethers_rs_types::error::{EthersRSUtilsError, RED_ERROR_STRING};
use ethkai_init::io::{self, utils::parse_file_delim_eq};
use serde::{self, Deserialize, Serialize};

use enum_dispatch;
use ethers_literal::{self, hash, num};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub use ethers_rs_abigen_types;
pub use ethers_rs_proc_macro;
pub use ethers_rs_types;

pub const CONFIGURATIONS_FILE_PATH: &str = "configs.txt";

lazy_static! {
    pub static ref CONFIGURATIONS: HashMap<String, String> =
        io::utils::parse_file_delim_eq(CONFIGURATIONS_FILE_PATH).unwrap_or_else(|_| panic!(
            "{}Unable to parse the {} file",
            *RED_ERROR_STRING, CONFIGURATIONS_FILE_PATH
        ));
}
