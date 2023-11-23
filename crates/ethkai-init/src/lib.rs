pub mod https;
pub mod ws;
pub mod io;

use std::sync::Arc;
use async_once_cell::OnceCell;
use ethers::providers::{Http, HttpRateLimitRetryPolicy, Middleware, Provider, Ws};
use ethers_rs_types::{error::{EthersRSUtilsError, RED_ERROR_STRING}};

pub static ARC_WS_PROVIDER: OnceCell<Arc<Provider<Ws>>> = OnceCell::new();
pub static ARC_HTTP_PROVIDER: OnceCell<Arc<Provider<Http>>> = OnceCell::new();