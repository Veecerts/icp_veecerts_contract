use candid::*;
use ic_cdk::id;

#[derive(CandidType, Deserialize)]
pub struct Account {
    pub ownder: String,
    pub subaccount: Option<[u8; 32]>,
}

#[ic_cdk::query]
pub fn get_canister_account() -> Account {
    Account {
        ownder: id().to_string(),
        subaccount: None,
    }
}
