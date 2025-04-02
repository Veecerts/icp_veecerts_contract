use candid::{CandidType, Principal};
use ic_cdk::{call, caller, id};
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct AccountBalanceArgs {
    pub account: String,
}

#[derive(CandidType, Deserialize)]
pub struct Tokens {
    pub e8s: u64,
}

#[ic_cdk::query]
pub async fn check_canister_balance() -> Result<u64, String> {
    let canister_account = id().to_string();
    let args = AccountBalanceArgs {
        account: canister_account,
    };

    let result: Result<(Tokens,), _> = call(
        Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
        "account_balance",
        (args,),
    )
    .await;

    match result {
        Ok((tokens,)) => Ok(tokens.e8s),
        Err(err) => Err(format!("Balance check failed: {:?}", err)),
    }
}

#[ic_cdk::query]
pub async fn check_balance(account_ownder: Principal) -> Result<u64, String> {
    let args = AccountBalanceArgs {
        account: account_ownder.to_string(),
    };

    let ledger_canister = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();

    let result: Result<(Tokens,), _> = call(ledger_canister, "account_balance", (args,)).await;

    match result {
        Ok((tokens,)) => Ok(tokens.e8s),
        Err(err) => Err(format!("Balance check failed: {:?}", err)),
    }
}

#[ic_cdk::query]
pub async fn my_balance() -> Result<u64, String> {
    check_balance(caller()).await
}
