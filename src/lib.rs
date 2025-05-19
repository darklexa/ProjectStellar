#![no_std]
extern crate alloc;

use soroban_sdk::{contractimpl, Address, Env, Map};

pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    /// Mint `amount` tokens into `to`.
    pub fn mint(env: Env, to: Address, amount: i128) {
        let mut m: Map<Address, i128> = Map::new(&env);
        let cur = m.get(to.clone()).unwrap_or(0);
        m.set(to.clone(), cur + amount);
    }

    /// Transfer `amount` from `from` → `to`, rejecting if frozen or insufficient.
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        // 1) freeze‐check
        let frozen: Map<Address, bool> = Map::new(&env);
        if frozen.get(from.clone()).unwrap_or(false) {
            panic!("account frozen");
        }
        // 2) subtract
        let mut mb: Map<Address, i128> = Map::new(&env);
        let fb = mb.get(from.clone()).unwrap_or(0);
        if fb < amount {
            panic!("insufficient funds");
        }
        mb.set(from.clone(), fb - amount);
        // 3) credit
        let mut tb: Map<Address, i128> = Map::new(&env);
        let tb_cur = tb.get(to.clone()).unwrap_or(0);
        tb.set(to.clone(), tb_cur + amount);
    }

    /// Freeze `who`’s account.
    pub fn freeze_account(env: Env, who: Address) {
        let mut m: Map<Address, bool> = Map::new(&env);
        m.set(who, true);
    }

    /// Unfreeze `who`’s account.
    pub fn unfreeze_account(env: Env, who: Address) {
        let mut m: Map<Address, bool> = Map::new(&env);
        m.set(who, false);
    }

    /// Read `who`’s balance.
    pub fn balance(env: Env, who: Address) -> i128 {
        Map::<Address, i128>::new(&env).get(who).unwrap_or(0)
    }
}
