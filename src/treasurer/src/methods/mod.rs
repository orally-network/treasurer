pub mod state;
pub mod taxpayers;

use anyhow::{Result, anyhow};

use ic_web3::ic::get_eth_addr;
use ic_cdk::{
    api::management_canister::main::{canister_status, CanisterIdRecord},
    export::{Principal, {candid::{CandidType, Nat}, serde::{Deserialize, Serialize}}}, update, query
};

use crate::{STATE, utils::{ecdsa::rec_eth_addr, erc20::send_erc20}, types::taxpayer::Taxpayer};

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum DepositType {
    Erc20,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct DepositRequest {
    amount: Nat,
    taxpayer: String,
    deposit_type: DepositType,
}