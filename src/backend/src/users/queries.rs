use candid::Principal;
use ic_cdk::{caller, query};

use crate::{Client, SubscriptionPackage};

use super::models::Profile;
use super::stores::{CLIENT_SUBSCRIPTIONS, CLIENTS, SUBSCRIPTION_PACKAGES, USERS};

#[query]
fn get_profile() -> Option<Profile> {
    let user_principal = caller();
    USERS.with(|users| users.borrow().get(&user_principal).cloned())
}

#[query]
fn get_profile_by_principal(principal: Principal) -> Option<Profile> {
    USERS.with(|users| users.borrow().get(&principal).cloned())
}

/// Check if a user has an active subscription
#[query]
fn check_subscription_status() -> String {
    let user_principal = caller();

    CLIENTS.with(|clients| {
        let clients = clients.borrow();
        if let Some(client) = clients.get(&user_principal) {
            if let Some(sub_uuid) = &client.active_subscription_uuid {
                CLIENT_SUBSCRIPTIONS.with(|subs| {
                    let subs = subs.borrow();
                    if let Some(subscription) = subs.get(sub_uuid) {
                        let current_time = ic_cdk::api::time();
                        if subscription.expires_at > current_time {
                            format!(
                                "Subscription is active. Expires at: {}",
                                subscription.expires_at
                            )
                        } else {
                            "Subscription has expired.".to_string()
                        }
                    } else {
                        "Subscription not found".to_string()
                    }
                });
            }
        }
        "No active subscription found.".to_string()
    })
}

#[query]
fn subscription_packages() -> Vec<SubscriptionPackage> {
    SUBSCRIPTION_PACKAGES.with(|packages| packages.borrow().values().cloned().collect())
}

/// Query to get the client associated with the caller
#[query]
fn get_client() -> Option<Client> {
    let user_principal = caller();

    CLIENTS.with(|clients| {
        let clients = clients.borrow();
        clients.get(&user_principal).cloned()
    })
}
