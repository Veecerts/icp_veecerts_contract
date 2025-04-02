use candid::{Principal, export_service};

use crate::transactions::accounts::Account;
use assets::models::*;
use users::models::*;

pub mod assets;
pub mod common;
pub mod transactions;
pub mod users;

#[ic_cdk::query(name = "__get_candid_interface_tmp_hack")]
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
