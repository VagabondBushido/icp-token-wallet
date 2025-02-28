# Secure ICRC-2 Token Wallet on ICP Blockchain

A Rust-based token wallet for the Internet Computer Protocol (ICP) blockchain that supports sending and receiving ICRC-2 tokens with built-in security features.

## Overview

This project implements a secure token wallet on the Internet Computer blockchain using Rust. The wallet supports the ICRC-2 token standard and provides functionality for:

- Sending tokens to other addresses
- Receiving tokens from other users
- Displaying token balances
- Minting new tokens (owner only)
- Burning tokens to reduce supply
- Listing all accounts with balances

## Features

### Smart Contract Features

- **Token Management**: Implementation of the ICRC-2 token standard
- **Transfers**: Secure token transfers between principals
- **Balance Tracking**: Real-time balance updates and queries
- **Owner Controls**: Privileged operations for the contract owner
- **Security Checks**: Validation for all token operations

### Security Features

- Principal-based authentication
- Balance verification before transfers
- Owner-only access for sensitive operations
- Protection against invalid operations
- Secure state management through Rust's ownership model

## Setup Instructions

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [DFINITY SDK (dfx)](https://internetcomputer.org/docs/current/developer-docs/setup/install/)
- [Node.js and npm](https://nodejs.org/) (for frontend development if needed)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/icp-token-wallet.git
   cd icp-token-wallet
   ```

2. Start a local Internet Computer replica:
   ```bash
   dfx start --clean --background
   ```

3. Deploy the canister:
   ```bash
   dfx deploy
   ```

## Usage

### Checking Token Balance

```bash
# Check your own balance
dfx canister call icp_token_wallet balance_of "(principal \"$(dfx identity get-principal)\")"

# Check balance of another address
dfx canister call icp_token_wallet balance_of "(principal \"RECIPIENT_PRINCIPAL_ID\")"
```

### Transferring Tokens

```bash
# Transfer 50000 tokens to another principal
dfx canister call icp_token_wallet transfer "(principal \"RECIPIENT_PRINCIPAL_ID\", 50000)"
```

### Minting Tokens (Owner Only)

```bash
# Mint 25000 tokens to a principal
dfx canister call icp_token_wallet mint "(principal \"RECIPIENT_PRINCIPAL_ID\", 25000)"
```

### Burning Tokens

```bash
# Burn 10000 tokens from your account
dfx canister call icp_token_wallet burn "(10000)"
```

### Getting Total Supply

```bash
dfx canister call icp_token_wallet total_supply
```

### Listing All Accounts

```bash
dfx canister call icp_token_wallet list_accounts
```

## Implementation Details

### State Management

The token wallet uses a thread-local RefCell to manage state:

```rust
thread_local! {
    static STATE: RefCell<TokenWallet> = RefCell::new(TokenWallet::default());
}
```

This approach allows for mutable access to state while maintaining the security guarantees of the Internet Computer.

### Token Wallet Structure

```rust
struct TokenWallet {
    // Maps account to balance
    balances: HashMap<Principal, u64>,
    // Owner of the contract
    owner: Principal,
    // Total supply of tokens
    total_supply: u64,
}
```

### Authentication

User authentication is based on the principal ID from the Internet Computer:

```rust
let caller = ic_cdk::caller();
```

### Security Measures

Token operations include appropriate validations:

```rust
// Example: Transfer operation with balance check
if sender_balance < amount {
    return Err("Insufficient balance".to_string());
}

// Example: Owner-only operation check
if caller != state.owner {
    return Err("Only the owner can mint tokens".to_string());
}
```

## Testing

### Manual Testing

You can test the wallet functionality using the dfx command line:

1. Create a test user:
   ```bash
   dfx identity new test_user
   dfx identity use test_user
   TEST_PRINCIPAL=$(dfx identity get-principal)
   dfx identity use default
   ```

2. Transfer tokens to the test user:
   ```bash
   dfx canister call icp_token_wallet transfer "(principal \"$TEST_PRINCIPAL\", 50000)"
   ```

3. Check the test user's balance:
   ```bash
   dfx canister call icp_token_wallet balance_of "(principal \"$TEST_PRINCIPAL\")"
   ```

4. Try security restrictions (this should fail):
   ```bash
   dfx identity use test_user
   dfx canister call icp_token_wallet mint "(principal \"$TEST_PRINCIPAL\", 100000)"
   dfx identity use default
   ```

### Unit Tests

The codebase includes unit tests that verify the core functionality:

```rust
// Located in src/icp_token_wallet/src/tests.rs
// Tests balance checking, transfers, minting, and security features
```

To run the tests:
```bash
cd src/icp_token_wallet
cargo test
```

## Code Structure

```
icp_token_wallet/
├── src/
│   └── icp_token_wallet/
│       ├── src/
│       │   ├── lib.rs          # Main canister code
│       │   └── tests.rs        # Unit tests
│       ├── icp_token_wallet.did # Candid interface
│       └── Cargo.toml          # Rust dependencies
└── dfx.json                    # Project configuration
```

## ICRC-2 Standard Implementation

The ICRC-2 token standard is implemented with the following core functions:

1. **transfer**: Allows users to send tokens to other principals
2. **balance_of**: Queries the balance of a specific principal
3. **total_supply**: Returns the total token supply
4. **mint**: Creates new tokens (restricted to owner)
5. **burn**: Destroys tokens, reducing the total supply

## Security Considerations

- Only the owner can mint new tokens
- Users can only burn tokens from their own accounts
- Balance verification prevents spending tokens you don't have
- Transfer validation ensures proper token movement
- Principal-based authentication ensures operation integrity

## Future Improvements

- Add approval/allowance functionality for more complete ICRC-2 support
- Implement transaction history tracking
- Create a web frontend for easier interaction
- Add more advanced security features like multi-signature support
- Implement token metadata and symbol information
