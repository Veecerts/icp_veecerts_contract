use std::mem;
use std::result::Result as StdResult;
use std::{cell::RefCell, collections::HashMap};

use candid::*;
use ic_cdk::{api, init, post_upgrade, pre_upgrade, query, storage, update};
use serde::Deserialize;

type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug, Clone, Deserialize, CandidType)]
struct NFT {
    id: u64,
    owner: Principal,
    metadata: String,
    collection_id: u64,
}

#[derive(Debug, Clone, Deserialize, CandidType)]
struct NFTCollection {
    id: u64,
    name: String,
    symbol: String,
    owner: Principal,
    description: String,
    logo: Option<String>,
    tokens: HashMap<u64, NFT>,
}

#[derive(Debug, Clone, Deserialize, CandidType)]
struct NFTCollectionOutput {
    id: u64,
    name: String,
    symbol: String,
    owner: Principal,
    description: String,
    logo: Option<String>,
}

#[derive(Debug, Default, Deserialize, CandidType)]
struct CanisterState {
    collections: HashMap<u64, NFTCollection>,
    tx_id: u128,
}

impl CanisterState {
    pub fn tx_id(&mut self) -> u128 {
        let tx_id = self.tx_id;
        self.tx_id += 1;
        tx_id
    }
}

#[derive(Debug, CandidType)]
pub enum NFTError {
    Unauthorized,
    TokenNotFound,
    CollectionNotFound,
    InvalidTokenID,
}

fn parse_token_id(token_id: String) -> Result<(u64, u64), NFTError> {
    if let Some((token_id_str, collection_id_str)) = token_id.split_once("x") {
        let token_id = match token_id_str.parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(NFTError::InvalidTokenID),
        };
        let collection_id = match collection_id_str.parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(NFTError::InvalidTokenID),
        };
        Ok((collection_id, token_id))
    } else {
        Err(NFTError::InvalidTokenID)
    }
}

#[init]
fn init() {
    STATE.with(|state| {
        *state.borrow_mut() = CanisterState {
            collections: HashMap::new(),
            tx_id: 1,
        }
    })
}

#[pre_upgrade]
fn save_state() {
    let state = STATE.with(|state| mem::take(&mut *state.borrow_mut()));
    storage::stable_save((state,)).expect("Failed to save state");
}

#[post_upgrade]
fn restore_state() {
    let (CanisterState { collections, tx_id },) =
        storage::stable_restore().expect("Failed to restore state");
    let state = CanisterState { collections, tx_id };
    STATE.with(|state0| *state0.borrow_mut() = state);
}

#[update]
fn create_nft(
    name: String,
    symbol: String,
    description: String,
    logo: Option<String>,
) -> Result<u128, NFTError> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let collection_id = state.collections.len() as u64 + 1;
        let caller = api::caller();

        let new_collection = NFTCollection {
            id: collection_id,
            name,
            symbol,
            owner: caller,
            logo,
            description,
            tokens: HashMap::new(),
        };

        state.collections.insert(collection_id, new_collection);

        Ok(state.tx_id())
    })
}

#[update]
fn mint_nft(collection_id: u64, metadata: String) -> Result<u128, NFTError> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let caller = api::caller();

        match state.collections.get_mut(&collection_id) {
            Some(collection) => {
                if collection.owner != caller {
                    return Err(NFTError::Unauthorized);
                }
                let token_id = collection.tokens.len() as u64 + 1;
                let new_nft = NFT {
                    id: token_id,
                    owner: caller,
                    collection_id: collection.id,
                    metadata,
                };
                collection.tokens.insert(token_id, new_nft);
                Ok(state.tx_id())
            }
            None => Err(NFTError::CollectionNotFound),
        }
    })
}

/// Burns an NFT identified by `token_id`.
///
/// # Parameters
/// - `token_id` (`String`): A unique identifier in the format `{collection_id}x{token_id}`.
/// - `caller` (`Principal`): The principal initiating the burn request. Must be the token owner.
///
/// # Returns
/// - `Ok(u128)`: A unique transaction ID if successful.
/// - `Err(NFTError)`: If the token or collection is not found, or the caller is unauthorized.
///
/// # Errors
/// - `NFTError::InvalidTokenID`: The `token_id` is improperly formatted.
/// - `NFTError::TokenNotFound`: The token does not exist.
/// - `NFTError::Unauthorized`: Caller is not the token owner.
#[update]
fn burn_nft(token_id: String) -> Result<u128, NFTError> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let caller = api::caller();
        let (token_id, collection_id) = parse_token_id(token_id)?;

        match state.collections.get_mut(&collection_id) {
            Some(collection) => match collection.tokens.get(&token_id) {
                Some(token) => {
                    if token.owner != caller {
                        return Err(NFTError::Unauthorized);
                    }
                    collection.tokens.remove(&token_id);
                    Ok(state.tx_id())
                }
                None => Err(NFTError::TokenNotFound),
            },
            None => Err(NFTError::CollectionNotFound),
        }
    })
}

/// Transfer an NFT identified by a `token_id`
///
/// # Parameters
/// - `token_id` (`String`): A unique identifier for the token to be burned.
///     The format of `token_id` is `{collection_id}x{token_id}`, where:
///     - `{collection_id}`: The ID of the NFTCollection to which the token belongs.
///     - `{token_id}`: The unique ID of the token within that collection
/// - `from` (`Principal`): The current owner and sender of this NFT
/// - `to` (`Principal`): The intended reciever of this NFT
/// ## Example:
/// ```
/// let token_id = "1x42".to_string(); // Collection 1, Token 42
/// let sender = Principal::from_text("aaaaa-aa").unwrap();
/// let recipient = Principal::from_text("bbbbb-bb").unwrap();
///
/// match transfer_nft(token_id, sender, recipient) {
///     Ok(tx_id) => println!("NFT transferred successfully, tx_id: {}", tx_id),
///     Err(e) => println!("Failed to transfer NFT: {:?}", e),
/// }
/// ```
#[update]
fn transfer_nft(token_id: String, from: Principal, to: Principal) -> Result<u128, NFTError> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let caller = api::caller();
        let (token_id, collection_id) = parse_token_id(token_id)?;

        match state.collections.get_mut(&collection_id) {
            Some(collection) => match collection.tokens.get_mut(&token_id) {
                Some(token) => {
                    if token.owner != from || token.owner != caller {
                        return Err(NFTError::Unauthorized);
                    }
                    token.owner = to;
                    Ok(state.tx_id())
                }
                None => Err(NFTError::TokenNotFound),
            },
            None => Err(NFTError::CollectionNotFound),
        }
    })
}

#[query]
fn get_nft_metadata(token_id: String) -> Result<Option<NFT>, NFTError> {
    STATE.with(|state| {
        let state = state.borrow();
        let (token_id, collection_id) = parse_token_id(token_id)?;

        match state.collections.get(&collection_id) {
            Some(collection) => match collection.tokens.get(&token_id) {
                Some(token) => Ok(Some(token.clone())),
                None => Ok(None),
            },
            None => Ok(None),
        }
    })
}

#[query]
fn collection_metadata(collection_id: u64) -> Option<NFTCollectionOutput> {
    STATE.with(|state| {
        let state = state.borrow();
        match state.collections.get(&collection_id) {
            Some(collection) => Some(NFTCollectionOutput {
                id: collection.id,
                description: collection.description.clone(),
                symbol: collection.symbol.clone(),
                name: collection.name.clone(),
                owner: collection.owner.clone(),
                logo: collection.logo.clone(),
            }),
            None => None,
        }
    })
}

#[query]
fn symbol(collection_id: u64) -> Option<String> {
    STATE.with(|state| {
        let state = state.borrow();
        match state.collections.get(&collection_id) {
            Some(collection) => Some(collection.symbol.clone()),
            None => None,
        }
    })
}

#[query]
fn name(collection_id: u64) -> Option<String> {
    STATE.with(|state| {
        let state = state.borrow();
        match state.collections.get(&collection_id) {
            Some(collection) => Some(collection.name.clone()),
            None => None,
        }
    })
}

#[query]
fn description(collection_id: u64) -> Option<String> {
    STATE.with(|state| {
        let state = state.borrow();
        match state.collections.get(&collection_id) {
            Some(collection) => Some(collection.description.clone()),
            None => None,
        }
    })
}

#[query]
fn logo(collection_id: u64) -> Option<String> {
    STATE.with(|state| {
        let state = state.borrow();
        match state.collections.get(&collection_id) {
            Some(collection) => collection.logo.clone(),
            None => None,
        }
    })
}

thread_local! {
    static STATE: RefCell<CanisterState> = RefCell::new(CanisterState { collections: HashMap::new(), tx_id: 1 });
}

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::fs::write;
        use std::path::PathBuf;

        let current_file = file!();
        let dir = PathBuf::from(current_file);
        let dir = dir.parent().unwrap().parent().unwrap();
        let dir_name = dir.to_str().unwrap().split_once("/").unwrap().1;
        let file_path = PathBuf::from(format!("{}.did", dir_name));
        write(file_path, export_candid()).expect("Write failed.");
    }
}
