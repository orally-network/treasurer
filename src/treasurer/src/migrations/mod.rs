use ic_cdk::{pre_upgrade, post_upgrade, storage};
use crate::{STATE, types::state::State};

#[pre_upgrade]
pub fn pre_upgrade() {
    let state = STATE.with(|state| state.borrow().clone());

    storage::stable_save((state,)).expect("should be able to save");
}

#[post_upgrade]
pub fn post_upgrade() {
    let (state,) : (State,) = storage::stable_restore().expect("should be able to restore");

    STATE.with(|s| s.replace(state));
}
