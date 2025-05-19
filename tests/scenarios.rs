#![cfg(test)]
use soroban_sdk::{testutils::Env as _, Address};

#[test]
fn test_freeze_blocks_transfer() {
    let e = soroban_sdk::testutils::Env::default();
    let admin = Address::random(&e);
    let user = Address::random(&e);

   
}

#[test]
fn test_unfreeze_allows_transfer() {
    
}
