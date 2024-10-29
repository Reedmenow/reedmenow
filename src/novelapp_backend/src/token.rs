use ic_cdk::export::{candid::{CandidType, Deserialize}};
use ic_cdk_macros::*;
use std::cell::RefCell;

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Token {
    symbol: String,
    name: String,
    total_supply: u64,
    balance: u64,
}

thread_local! {
    static TOKEN: RefCell<Token> = RefCell::new(Token {
        symbol: "NVL".to_string(),
        name: "NVL Token".to_string(),
        total_supply: 1_000_000,
        balance: 0,
    });
}

#[update]
fn transfer(to: String, amount: u64) -> bool {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        if token.balance >= amount {
            token.balance -= amount;
            true
        } else {
            false
        }
    })
}

#[query]
fn balance_of() -> u64 {
    TOKEN.with(|token| token.borrow().balance)
}
