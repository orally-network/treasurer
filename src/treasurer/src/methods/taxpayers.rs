use anyhow::Context;

use super::*;

#[update]
pub async fn register_taxpayer(msg: String, sig: String) -> Result<String, String> {
    _register_taxpayer(msg, sig)
        .await
        .map_err(|e| format!("failed to register taxpayer: {e}"))
}

async fn _register_taxpayer(msg: String, sig: String) -> Result<String> {
    let pub_addr = rec_eth_addr(&msg, &sig).await?;
    let candid_pub_addr = hex::encode(pub_addr.as_bytes());

    let is_found = STATE.with(|state| {
        state
            .borrow()
            .taxpayers
            .iter()
            .any(|taxpayer| taxpayer.pub_addr == candid_pub_addr)
    });
    if is_found {
        return Err(anyhow!("Taxpayer already exists"))
    }

    let derivation_path = vec![pub_addr.as_bytes().to_vec()];
    let key_name = STATE.with(|state| state.borrow().key_name.clone());
    let exec_addr = get_eth_addr(None, Some(derivation_path), key_name)
        .await
        .map_err(|e| anyhow!("failed to get exec_addr: {e}"))?;

    let taxpayer = Taxpayer {
        pub_addr: candid_pub_addr,
        exec_addr: hex::encode(exec_addr.as_bytes()),
    };

    STATE.with(|state| state.borrow_mut().taxpayers.push(taxpayer));

    Ok(hex::encode(exec_addr.as_bytes()))
}

#[query]
pub async fn get_taxpayer(pub_addr: String) -> Result<String, String> {
    _get_taxpayer(pub_addr)
        .await
        .map_err(|e| format!("failed to get taxpayer: {e}"))
}

async fn _get_taxpayer(pub_addr: String) -> Result<String> {
    let taxpayer = STATE.with(|state| {
        state
            .borrow()
            .taxpayers
            .iter()
            .find(|taxpayer| taxpayer.pub_addr == pub_addr)
            .map(|v| v.clone())
            .context("taxpayer does not exist")
            
    })?;

    Ok(taxpayer.exec_addr)
}


#[update]
pub async fn deposit(req: DepositRequest) -> Result<(), String> {
    _deposit(req)
        .await
        .map_err(|e| format!("failed to deposit: {e}"))
}

async fn _deposit(req: DepositRequest) -> Result<()> {
    match req.deposit_type {
        DepositType::Erc20 => send_erc20(req.amount, req.taxpayer).await
    }
}