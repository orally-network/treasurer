use super::{*, treasure::Treasure, config::InitConfig, taxpayer::Taxpayer};

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct State {
    pub controllers: Vec<Principal>,
    pub treasure: Treasure,
    pub siwe_signer_canister: Principal,
    pub taxpayers: Vec<Taxpayer>,
    pub key_name: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            controllers: Default::default(),
            treasure: Default::default(),
            siwe_signer_canister: Principal::anonymous(),
            taxpayers: Default::default(),
            key_name: Default::default(),
        }
    }
}

impl State {
    pub fn init(&mut self, cfg: InitConfig) {
        self.treasure = Treasure {
            token_addr: cfg.token_addr,
            chain_rpc: cfg.chain_rpc,
            chain_id: cfg.chain_id,
            treasurer: cfg.treasurer,
        };
        self.siwe_signer_canister = cfg.siwe_signer_canister;
        self.key_name = cfg.key_name;
    }
}
