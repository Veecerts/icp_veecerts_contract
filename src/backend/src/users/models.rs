use candid::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, CandidType)]
pub struct Profile {
    pub principal: Principal,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub image_hash: Option<String>,
    pub date_added: String,
    pub last_updated: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct SubscriptionPackage {
    pub uuid: String,
    pub name: String,
    pub price: f64,
    pub storage_capacity_mb: u64,
    pub monthly_requests: u64,
    pub max_allowed_sessions: u64,
    pub last_updated: u64,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Client {
    pub principal: Principal,
    pub uuid: String,
    pub active_subscription_uuid: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct ClientPackageSubscription {
    pub client_uuid: String,
    pub subscription_package_uuid: String,
    pub amount: f64,
    pub expires_at: u64,
}
