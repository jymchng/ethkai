// use crate::abigen::contracts::uniswap::{
//     SwapExactTokensForETHSupportingFeeOnTransferTokensCall, UniswapV2, UniswapV2Calls,
// };
// use crate::error::INITIALIZATION_ERROR_STRING;
// use anyhow::{anyhow, Result};
// use ethers::{
//     abi::AbiDecode,
//     types::{
//         transaction::response::{Transaction, TransactionReceipt},
//         Address, Eip1559TransactionRequest, U256,
//     },
// };

// pub fn input_data(txn: Transaction) -> Result<()> {
//     let input_data = txn.input;
//     let decoded_input_data = SwapExactTokensForETHSupportingFeeOnTransferTokensCall::decode(
//         &input_data,
//     )
//     .map_err(|_err| {
//         anyhow!(
//             "{}Unable to decode the `input_data`",
//             *INITIALIZATION_ERROR_STRING
//         )
//     })?;
//     let amount_in: U256 = decoded_input_data.amount_in;
//     let amount_out_min: U256 = decoded_input_data.amount_out_min;
//     let path: Vec<Address> = decoded_input_data.path;
//     let to: Address = decoded_input_data.to;
//     let deadline: U256 = decoded_input_data.deadline;
//     anyhow::Ok(())
// }
