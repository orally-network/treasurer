pub mod state;
pub mod config;
pub mod treasure;
pub mod taxpayer;

use ic_cdk::export::{
    Principal,
    {candid::{CandidType, Nat}, serde::{Deserialize, Serialize}}
};