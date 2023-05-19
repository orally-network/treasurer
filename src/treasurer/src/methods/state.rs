use super::*;

#[update]
pub async fn init_controllers() -> Vec<Principal> {
    let canister_id_record = CanisterIdRecord {
        canister_id: ic_cdk::id(),
    };

    let (canister_status,) = canister_status(canister_id_record)
        .await
        .expect("should execute in the IC environment");

    STATE.with(|state| {
        state.borrow_mut().controllers = canister_status.settings.controllers;
    });

    STATE.with(|state| state.borrow().controllers.clone())
}