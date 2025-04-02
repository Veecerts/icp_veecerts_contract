use std::collections::HashMap;

use super::models::{Asset, Folder};

thread_local! {
    pub static FOLDERS: std::cell::RefCell<HashMap<String, Folder>> = std::cell::RefCell::new(HashMap::new());
    pub static ASSETS: std::cell::RefCell<HashMap<String, Asset>> = std::cell::RefCell::new(HashMap::new());
}
