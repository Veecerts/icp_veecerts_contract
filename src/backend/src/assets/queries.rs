use ic_cdk::query;

use super::{
    models::{Asset, AssetQueryOptions, Folder, FolderQueryOptions, Paginated},
    stores::{ASSETS, FOLDERS},
};

#[query]
pub fn client_folder_assets(
    user_id: String,
    folder_id: String,
    opts: Option<Paginated<AssetQueryOptions>>,
) -> Vec<Asset> {
    let assets = ASSETS.with(|assets| {
        assets
            .borrow()
            .values()
            .filter(|asset| asset.owner_id.to_string() == user_id && asset.folder_uuid == folder_id)
            .cloned()
            .collect::<Vec<_>>()
    });
    apply_asset_filters(assets, opts)
}

#[query]
pub fn client_folder(user_id: String, id: String) -> Option<Folder> {
    FOLDERS.with(|folders| {
        folders
            .borrow()
            .values()
            .find(|folder| folder.owner_id.to_string() == user_id && folder.uuid == id)
            .cloned()
    })
}

#[query]
pub fn client_folders(user_id: String, opts: Option<Paginated<FolderQueryOptions>>) -> Vec<Folder> {
    let folders = FOLDERS.with(|folders| {
        folders
            .borrow()
            .values()
            .filter(|folder| folder.owner_id.to_string() == user_id)
            .cloned()
            .collect::<Vec<_>>()
    });
    apply_folder_filters(folders, opts)
}

#[query]
pub fn client_assets(user_id: String, opts: Option<Paginated<AssetQueryOptions>>) -> Vec<Asset> {
    let assets = ASSETS.with(|assets| {
        assets
            .borrow()
            .values()
            .filter(|asset| asset.owner_id.to_string() == user_id)
            .cloned()
            .collect::<Vec<_>>()
    });
    apply_asset_filters(assets, opts)
}

fn apply_asset_filters(
    mut assets: Vec<Asset>,
    opts: Option<Paginated<AssetQueryOptions>>,
) -> Vec<Asset> {
    if let Some(opts) = opts {
        if let Some(filter) = opts.opts.clone().and_then(|o| o.filter) {
            if let Some(name) = filter.name {
                assets.retain(|asset| asset.name.contains(&name));
            }
            if let Some(description) = filter.description {
                assets.retain(|asset| asset.description.contains(&description));
            }
            if let Some(min_size) = filter.min_size_mb {
                assets.retain(|asset| asset.size_mb > min_size);
            }
            if let Some(max_size) = filter.max_size_mb {
                assets.retain(|asset| asset.size_mb < max_size);
            }
        }
        if let Some(ordering) = opts.opts.and_then(|o| o.ordering) {
            if let Some(date_added) = ordering.date_added {
                if date_added {
                    assets.sort_by_key(|asset| asset.date_added.clone());
                } else {
                    assets.sort_by_key(|asset| std::cmp::Reverse(asset.date_added.clone()));
                }
            }
            if let Some(last_updated) = ordering.last_updated {
                if last_updated {
                    assets.sort_by_key(|asset| asset.last_updated.clone());
                } else {
                    assets.sort_by_key(|asset| std::cmp::Reverse(asset.last_updated.clone()));
                }
            }
        }
    }
    assets
}

fn apply_folder_filters(
    mut folders: Vec<Folder>,
    opts: Option<Paginated<FolderQueryOptions>>,
) -> Vec<Folder> {
    if let Some(opts) = opts {
        if let Some(filter) = opts.opts.clone().and_then(|o| o.filter) {
            if let Some(name) = filter.name {
                folders.retain(|folder| folder.name.contains(&name));
            }
            if let Some(description) = filter.description {
                folders.retain(|folder| folder.description.contains(&description));
            }
        }
        if let Some(ordering) = opts.opts.and_then(|o| o.ordering) {
            if let Some(date_added) = ordering.date_added {
                if date_added {
                    folders.sort_by_key(|folder| folder.date_added.clone());
                } else {
                    folders.sort_by_key(|folder| std::cmp::Reverse(folder.date_added.clone()));
                }
            }
            if let Some(last_updated) = ordering.last_updated {
                if last_updated {
                    folders.sort_by_key(|folder| folder.last_updated.clone());
                } else {
                    folders.sort_by_key(|folder| std::cmp::Reverse(folder.last_updated.clone()));
                }
            }
        }
    }
    folders
}
