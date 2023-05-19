use super::*;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct Treasure {
    pub token_addr: String,
    pub chain_rpc: String,
    pub chain_id: Nat,
    pub treasurer: String,
}