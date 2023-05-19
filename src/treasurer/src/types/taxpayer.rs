use super::*;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct Taxpayer {
    pub pub_addr: String,
    pub exec_addr: String,
}
