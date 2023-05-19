use super::*;

const TOKEN_ABI: &[u8] = include_bytes!("../../assets/ERC20ABI.json");

pub async fn send_erc20(value: Nat, from: String) -> Result<()> {
    let from = H160::from_str(&from)
    .context("invalid taxpayer format")?;

    let derivation_path = vec![from.as_bytes().to_vec()];
    let (key_name, treasure) = STATE.with(|state| {
        let state = state.borrow();

        (state.key_name.clone(), state.treasure.clone())
    });

    let receiver = H160::from_str(&treasure.treasurer)?;

    let exec_addr = get_eth_addr(None, Some(derivation_path.clone()), key_name.clone())
        .await
        .map_err(|e| anyhow!("failed to get exec_addr: {e}"))?;

    let w3 = Web3::new(
        ICHttp::new(&treasure.chain_rpc, None, None)
            .context("failed to connect to a node")?,
    );

    let contract_addr = Address::from_str(&treasure.token_addr)
        .context("invalid a contract address")?;
    let contract = Contract::from_json(w3.eth(), contract_addr, TOKEN_ABI)?;

    let tx_count = w3.eth().transaction_count(exec_addr, None).await?;
    let gas_price = w3.eth().gas_price().await?;
    let options = Options::with(|op| { 
        op.nonce = Some(tx_count);
        op.gas_price = Some(gas_price);
        op.transaction_type = Some(U64::from(2))
    });
    let value = U256::from_str(&value.to_string())?;
    let sender = hex::encode(exec_addr.as_bytes());
    let key_info = KeyInfo {derivation_path, key_name};
    let chain_id = treasure.chain_id.0.to_u64_digits().last().expect("chain id should be u64").clone();

    let tx_hash = contract
        .signed_call("transfer", (receiver, value,), options, sender, key_info, chain_id)
        .await?;

    wait_until_confimation(&tx_hash, &w3).await?;

    Ok(())
}
