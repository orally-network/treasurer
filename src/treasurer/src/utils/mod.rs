pub mod ecdsa;
pub mod erc20;

use std::str::FromStr;

use anyhow::{Context, Result, anyhow};
use ic_cdk::{
    export::candid::Nat, api::time,
};

use ic_web3::types::{H160, U256, U64, H256};
use ic_web3::transports::ICHttp;
use ic_web3::Web3;
use ic_web3::ic::{get_eth_addr, KeyInfo};
use ic_web3::{contract::{Contract, Options}, types::Address};

use crate::STATE;

const TIMEOUT: u64 = 60 * 60;

async fn wait_until_confimation(tx_hash: &H256, w3: &Web3<ICHttp>) -> Result<()> {
    let start = time();
    let mut current_time = start;

    while (start - current_time) < TIMEOUT {
        let tx_receipt = w3
            .eth()
            .transaction_receipt(*tx_hash)
            .await
            .context("failed to get tx receipt")?;

        if let Some(tx_receipt) = tx_receipt {
            if let Some(status) = tx_receipt.status {
                if status.as_u64() == 0 {
                    return Err(anyhow!("tx failed"));
                }

                return Ok(());
            }
        }

        current_time = time();
    }

    ic_cdk::println!("timeout: {TIMEOUT}, data: {}", start-current_time);

    Err(anyhow!("tx timeout"))
}