use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize)]
struct TokenWallet {
    // Maps account to balance
    balances: HashMap<Principal, u64>,
    // Owner of the contract
    owner: Principal,
    // Total supply of tokens
    total_supply: u64,
}

// Implement custom Default instead of using the derive
impl Default for TokenWallet {
    fn default() -> Self {
        TokenWallet {
            balances: HashMap::new(),
            owner: Principal::anonymous(), // Use anonymous as default
            total_supply: 0,
        }
    }
}

thread_local! {
    static STATE: RefCell<TokenWallet> = RefCell::new(TokenWallet::default());
}

#[init]
fn init() {
    let owner = ic_cdk::caller();
    let initial_supply = 1_000_000;

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.owner = owner;
        state.total_supply = initial_supply;
        state.balances.insert(owner, initial_supply);
    });

    ic_cdk::println!("Token wallet initialized with owner: {:?}", owner);
}

#[query]
fn balance_of(account: Principal) -> u64 {
    STATE.with(|state| state.borrow().balances.get(&account).cloned().unwrap_or(0))
}

#[query]
fn total_supply() -> u64 {
    STATE.with(|state| state.borrow().total_supply)
}

#[update]
fn transfer(to: Principal, amount: u64) -> Result<(), String> {
    let caller = ic_cdk::caller();

    if caller == to {
        return Err("Cannot transfer to yourself".to_string());
    }

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let sender_balance = state.balances.get(&caller).cloned().unwrap_or(0);

        if sender_balance < amount {
            return Err("Insufficient balance".to_string());
        }

        // Update sender balance
        state.balances.insert(caller, sender_balance - amount);

        // Update recipient balance
        let recipient_balance = state.balances.get(&to).cloned().unwrap_or(0);
        state.balances.insert(to, recipient_balance + amount);

        ic_cdk::println!(
            "Transfer of {} tokens from {:?} to {:?}",
            amount,
            caller,
            to
        );

        Ok(())
    })
}

// Security function - only owner can mint new tokens
#[update]
fn mint(to: Principal, amount: u64) -> Result<(), String> {
    let caller = ic_cdk::caller();

    STATE.with(|state| {
        let mut state = state.borrow_mut();

        if caller != state.owner {
            return Err("Only the owner can mint tokens".to_string());
        }

        let recipient_balance = state.balances.get(&to).cloned().unwrap_or(0);
        state.balances.insert(to, recipient_balance + amount);
        state.total_supply += amount;

        ic_cdk::println!("Minted {} tokens to {:?}", amount, to);

        Ok(())
    })
}

// Burn tokens (reduce supply)
#[update]
fn burn(amount: u64) -> Result<(), String> {
    let caller = ic_cdk::caller();

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let caller_balance = state.balances.get(&caller).cloned().unwrap_or(0);

        if caller_balance < amount {
            return Err("Insufficient balance to burn".to_string());
        }

        state.balances.insert(caller, caller_balance - amount);
        state.total_supply -= amount;

        ic_cdk::println!("Burned {} tokens from {:?}", amount, caller);

        Ok(())
    })
}

#[query]
fn get_owner() -> Principal {
    STATE.with(|state| state.borrow().owner)
}

// List all accounts with non-zero balances
#[query]
fn list_accounts() -> Vec<(Principal, u64)> {
    STATE.with(|state| {
        let state = state.borrow();
        state
            .balances
            .iter()
            .filter(|(_, &balance)| balance > 0)
            .map(|(principal, balance)| (*principal, *balance))
            .collect()
    })
}
