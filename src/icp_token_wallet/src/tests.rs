use super::*;
use ic_cdk::export::Principal;

// Helper to create test principal
fn test_principal(id: u8) -> Principal {
    Principal::from_slice(&[id])
}

#[test]
fn test_init() {
    // Mock caller as the owner
    let owner = test_principal(1);
    ic_cdk::export::ic_cdk_timers::set_caller(owner.clone());

    init();

    let balance = balance_of(owner);
    assert_eq!(balance, 1000000);
}

#[test]
fn test_transfer() {
    // Setup
    let owner = test_principal(1);
    let recipient = test_principal(2);
    ic_cdk::export::ic_cdk_timers::set_caller(owner.clone());

    init();

    // Transfer tokens
    let result = transfer(recipient, 100000);
    assert!(result.is_ok());

    // Check balances
    assert_eq!(balance_of(owner), 900000);
    assert_eq!(balance_of(recipient), 100000);
}

#[test]
fn test_insufficient_balance() {
    let owner = test_principal(1);
    let recipient = test_principal(2);
    ic_cdk::export::ic_cdk_timers::set_caller(owner.clone());

    init();

    // Try to transfer more than available
    let result = transfer(recipient, 2000000);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Insufficient balance");
}

#[test]
fn test_mint() {
    let owner = test_principal(1);
    let user = test_principal(2);
    ic_cdk::export::ic_cdk_timers::set_caller(owner.clone());

    init();

    // Mint tokens as owner
    let result = mint(user, 500000);
    assert!(result.is_ok());
    assert_eq!(balance_of(user), 500000);

    // Try minting as non-owner
    ic_cdk::export::ic_cdk_timers::set_caller(user.clone());
    let result = mint(user, 500000);
    assert!(result.is_err());
}

#[test]
fn test_burn() {
    let owner = test_principal(1);
    ic_cdk::export::ic_cdk_timers::set_caller(owner.clone());

    init();

    let result = burn(500000);
    assert!(result.is_ok());
    assert_eq!(balance_of(owner), 500000);
    assert_eq!(total_supply(), 500000);
}
