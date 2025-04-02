use ic_cdk::{caller, update};

use crate::common::utils::uuid::generate_unique_id;

use super::models::{Client, ClientPackageSubscription, Profile, SubscriptionPackage};
use super::stores::{CLIENT_SUBSCRIPTIONS, CLIENTS, SUBSCRIPTION_PACKAGES, USERS};

/// Authenticate the caller and create an empty profile if they don’t have one
#[update]
fn register() -> String {
    let user_principal = caller();
    let current_time = ic_cdk::api::time().to_string();

    USERS.with(|users| {
        let mut users = users.borrow_mut();

        if users.contains_key(&user_principal) {
            return format!("User already registered with principal: {}", user_principal);
        }

        let new_profile = Profile {
            principal: user_principal,
            email: None,
            first_name: None,
            last_name: None,
            image_hash: None,
            date_added: current_time.clone(),
            last_updated: current_time,
        };

        users.insert(user_principal, new_profile);
        format!(
            "User registered successfully with principal: {}",
            user_principal
        )
    })
}

/// Update profile details (only the caller can update their own profile)
#[update]
fn update_profile(
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    image_hash: Option<String>,
) -> String {
    let user_principal = caller();
    let current_time = ic_cdk::api::time().to_string();

    USERS.with(|users| {
        let mut users = users.borrow_mut();

        if let Some(profile) = users.get_mut(&user_principal) {
            profile.email = email.or(profile.email.clone());
            profile.first_name = first_name.or(profile.first_name.clone());
            profile.last_name = last_name.or(profile.last_name.clone());
            profile.image_hash = image_hash.or(profile.image_hash.clone());
            profile.last_updated = current_time;

            return "Profile updated successfully".to_string();
        }

        "User not found".to_string()
    })
}

/// Creates or updates a subscription package
#[update]
fn create_update_subscription_package(
    uuid: Option<String>,
    name: String,
    price: f64,
    storage_capacity_mb: u64,
    monthly_requests: u64,
    max_allowed_sessions: u64,
) -> String {
    let uuid = uuid.unwrap_or_else(generate_unique_id);
    let current_time = ic_cdk::api::time();

    let package = SubscriptionPackage {
        uuid: uuid.clone(),
        name,
        price,
        storage_capacity_mb,
        monthly_requests,
        max_allowed_sessions,
        last_updated: current_time,
    };

    SUBSCRIPTION_PACKAGES.with(|packages| {
        packages.borrow_mut().insert(uuid.clone(), package);
    });

    format!("Subscription package {} created/updated.", uuid)
}

/// Allows a user to subscribe to a package
#[update]
fn create_update_client_package_subscription(subscription_package_uuid: String) -> String {
    let user_principal = caller();
    let current_time = ic_cdk::api::time();
    let expires_at = current_time + (30 * 24 * 60 * 60 * 1_000_000_000); // 30 days in nanoseconds

    // Check if package exists
    let package_price = SUBSCRIPTION_PACKAGES.with(|packages| {
        packages
            .borrow()
            .get(&subscription_package_uuid)
            .map(|p| p.price)
    });

    if package_price.is_none() {
        return "Subscription package not found.".to_string();
    }

    let price = package_price.unwrap();

    // Insert or update the client
    CLIENTS.with(|clients| {
        let mut clients = clients.borrow_mut();
        let client = clients.entry(user_principal).or_insert(Client {
            principal: user_principal,
            uuid: generate_unique_id(),
            active_subscription_uuid: None,
        });

        // Store subscription
        let client_subscription = ClientPackageSubscription {
            client_uuid: client.uuid.clone(),
            subscription_package_uuid: subscription_package_uuid.clone(),
            amount: price,
            expires_at,
        };

        CLIENT_SUBSCRIPTIONS.with(|subs| {
            subs.borrow_mut()
                .insert(client.uuid.clone(), client_subscription);
        });

        client.active_subscription_uuid = Some(subscription_package_uuid);
    });

    format!("Subscription successful for principal: {}", user_principal)
}
