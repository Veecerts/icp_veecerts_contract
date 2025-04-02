use super::models::{Client, ClientPackageSubscription, Profile, SubscriptionPackage};
use candid::Principal;
use ic_cdk::storage;
use std::{cell::RefCell, collections::HashMap};

thread_local! {
    pub static USERS: RefCell<HashMap<Principal, Profile>> = RefCell::new(HashMap::new());
    pub static CLIENTS: RefCell<HashMap<Principal, Client>> = RefCell::new(HashMap::new());
    pub static SUBSCRIPTION_PACKAGES: RefCell<HashMap<String, SubscriptionPackage>> = RefCell::new(HashMap::new());
    pub static CLIENT_SUBSCRIPTIONS: RefCell<HashMap<String, ClientPackageSubscription>> = RefCell::new(HashMap::new());
}

/// Initialize empty state
#[ic_cdk::init]
fn init() {
    USERS.with(|users| *users.borrow_mut() = HashMap::new());
    CLIENTS.with(|clients| *clients.borrow_mut() = HashMap::new());
    SUBSCRIPTION_PACKAGES.with(|packages| *packages.borrow_mut() = HashMap::new());
    CLIENT_SUBSCRIPTIONS.with(|subscriptions| *subscriptions.borrow_mut() = HashMap::new());
}

/// Save state before upgrade
#[ic_cdk::pre_upgrade]
fn save_state() {
    let users = USERS.with(|users| users.borrow().clone());
    let clients = CLIENTS.with(|clients| clients.borrow().clone());
    let subscription_packages = SUBSCRIPTION_PACKAGES.with(|packages| packages.borrow().clone());
    let client_subscriptions =
        CLIENT_SUBSCRIPTIONS.with(|subscriptions| subscriptions.borrow().clone());

    storage::stable_save((users, clients, subscription_packages, client_subscriptions))
        .expect("Failed to save state");
}

type Type = (
    HashMap<Principal, Profile>,
    HashMap<Principal, Client>,
    HashMap<String, SubscriptionPackage>,
    HashMap<String, ClientPackageSubscription>,
);

/// Restore state after upgrade
#[ic_cdk::post_upgrade]
fn restore_state() {
    let (users, clients, subscription_packages, client_subscriptions): Type =
        storage::stable_restore().expect("Failed to restore state");

    USERS.with(|state| *state.borrow_mut() = users);
    CLIENTS.with(|state| *state.borrow_mut() = clients);
    SUBSCRIPTION_PACKAGES.with(|state| *state.borrow_mut() = subscription_packages);
    CLIENT_SUBSCRIPTIONS.with(|state| *state.borrow_mut() = client_subscriptions);
}
