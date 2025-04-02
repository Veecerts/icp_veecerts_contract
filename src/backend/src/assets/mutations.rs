use crate::{common::utils::uuid::generate_unique_id, users::stores::CLIENTS};
use ic_cdk::api::time;
use ic_cdk::{caller, update};

use super::{
    models::{Asset, Folder},
    stores::{ASSETS, FOLDERS},
};

#[update]
fn create_update_folder(input: Folder) -> Result<Folder, String> {
    let user_principal = caller();

    CLIENTS.with(|clients| {
        let clients = clients.borrow();
        if !clients.contains_key(&user_principal) {
            Err("You are not authorized to perform this action".to_string())
        } else {
            FOLDERS.with(|folders| {
                let mut folders = folders.borrow_mut();

                if folders.contains_key(&input.uuid) {
                    // Update existing folder
                    let folder = folders.get_mut(&input.uuid).unwrap();
                    folder.name = input.name;
                    folder.description = input.description;
                    folder.last_updated = time().to_string();
                    Ok(folder.clone())
                } else {
                    // Create new folder
                    let new_uuid = generate_unique_id();
                    let new_folder = Folder {
                        uuid: new_uuid.clone(),
                        name: input.name,
                        owner_id: user_principal,
                        description: input.description,
                        client_id: user_principal.to_string(),
                        date_added: time().to_string(),
                        last_updated: time().to_string(),
                    };
                    folders.insert(new_uuid, new_folder.clone());
                    Ok(new_folder)
                }
            })
        }
    })
}

/// Create or Update an Asset
#[update]
fn create_update_asset(input: Asset) -> Result<Asset, String> {
    let user_principal = caller();

    CLIENTS.with(|clients| {
        let clients = clients.borrow();
        if !clients.contains_key(&user_principal) {
            Err("You are not authorized to perform this action".to_string())
        } else {
            FOLDERS.with(|folders| {
                let folders = folders.borrow();
                if !folders.contains_key(&input.folder_uuid) {
                    Err("Folder not found".to_string())
                } else {
                    ASSETS.with(|assets| {
                        let mut assets = assets.borrow_mut();
                        if assets.contains_key(&input.uuid) {
                            // Update existing asset
                            let asset = assets.get_mut(&input.uuid).unwrap();
                            asset.name = input.name;
                            asset.description = input.description;
                            asset.ipfs_hash = input.ipfs_hash;
                            asset.size_mb = input.size_mb;
                            asset.last_updated = time().to_string();
                            Ok(asset.clone())
                        } else {
                            // Create new asset
                            let new_uuid = generate_unique_id();
                            let new_asset = Asset {
                                uuid: new_uuid.clone(),
                                name: input.name,
                                owner_id: user_principal,
                                description: input.description,
                                folder_uuid: input.folder_uuid,
                                ipfs_hash: input.ipfs_hash,
                                size_mb: input.size_mb,
                                date_added: time().to_string(),
                                last_updated: time().to_string(),
                            };
                            assets.insert(new_uuid, new_asset.clone());
                            Ok(new_asset)
                        }
                    })
                }
            })
        }
    })
}
