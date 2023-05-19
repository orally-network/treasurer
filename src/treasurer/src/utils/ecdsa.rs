use super::*;

pub async fn rec_eth_addr(msg: &str, sig: &str) -> Result<H160> {
    let siwe_canister = STATE.with(|state| state.borrow().siwe_signer_canister.clone());

    let msg = msg.to_string();
    let sig = sig.to_string();

    let (signer,): (String,) = ic_cdk::call(siwe_canister, "get_signer", (msg, sig))
        .await
        .map_err(|(code, msg)| anyhow!("{:?}: {}", code, msg))?;

    H160::from_str(&signer).context("failed to parse signer address")
}