use super::*;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct InitConfig {
    pub token_addr: String,
    pub chain_rpc: String,
    pub siwe_signer_canister: Principal,
    pub key_name: String,
    pub chain_id: Nat,
    pub treasurer: String,
}
