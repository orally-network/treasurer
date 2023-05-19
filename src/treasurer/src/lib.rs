pub mod types;
pub mod utils;
pub mod methods;
pub mod migrations;

use std::cell::RefCell;
use types::{state::State, config::InitConfig};

use ic_cdk::{
    api::management_canister::http_request::{HttpResponse, TransformArgs},
    init, query
};

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();
}

#[init]
pub fn init(cfg: InitConfig) {
    STATE.with(|state| state.borrow_mut().init(cfg))
}

#[query]
fn transform(response: TransformArgs) -> HttpResponse {
    response.response
}