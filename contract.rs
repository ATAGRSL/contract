use std::collections::HashMap;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen, AccountId, Balance, PanicOnDefault,
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Token {
    balances: HashMap<AccountId, Balance>,
    is_frozen: HashMap<AccountId, bool>,
}

#[near_bindgen]
impl Token {
    pub fn freeze_account(&mut self, account_id: AccountId) {
        self.is_frozen.insert(account_id.clone(), true);
    }

    pub fn unfreeze_account(&mut self, account_id: AccountId) {
        self.is_frozen.insert(account_id.clone(), false);
    }

    pub fn transfer(&mut self, receiver_id: AccountId, amount: Balance) {
        let sender_id = env::predecessor_account_id();
        assert!(!self.is_frozen.get(&sender_id).unwrap_or(&false), "Sender account is frozen");
        assert!(!self.is_frozen.get(&receiver_id).unwrap_or(&false), "Receiver account is frozen");

        let sender_balance = self.balances.get(&sender_id).unwrap_or(&0);
        assert!(sender_balance >= amount, "Not enough balance");

        // Perform the transfer
        self.balances.insert(sender_id.clone(), sender_balance - amount);
        *self.balances.entry(receiver_id.clone()).or_insert(0) += amount;
    }

    pub fn get_balance(&self, account_id: AccountId) -> Balance {
        *self.balances.get(&account_id).unwrap_or(&0)
    }
}
