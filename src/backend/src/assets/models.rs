use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Folder {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub client_id: String,
    pub owner_id: Principal,
    pub date_added: String,
    pub last_updated: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Asset {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub folder_uuid: String,
    pub ipfs_hash: String,
    pub size_mb: f64,
    pub owner_id: Principal,
    pub date_added: String,
    pub last_updated: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Paginated<T> {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
    pub opts: Option<T>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct AssetFilter {
    pub name: Option<String>,
    pub description: Option<String>,
    pub min_size_mb: Option<f64>,
    pub max_size_mb: Option<f64>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct AssetQueryOptions {
    pub filter: Option<AssetFilter>,
    pub ordering: Option<AssetOrdering>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct AssetOrdering {
    pub date_added: Option<bool>,
    pub last_updated: Option<bool>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct FolderQueryOptions {
    pub filter: Option<FolderFilter>,
    pub ordering: Option<FolderOrdering>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct FolderFilter {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct FolderOrdering {
    pub date_added: Option<bool>,
    pub last_updated: Option<bool>,
}
