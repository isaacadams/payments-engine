# Payment Engine

a rust program that processes varying types of transactions for clients

## 👋 Getting Started

- build: `cargo build`
- run the unit tests: `cargo test`
- run the program: `cargo run -- transactions.csv`

## Assumptions

### Transaction Ids

Assuming that the first transaction which appears for a client is their actual first transaction.

If an incoming transaction has a lower id than the last transaction processed for the client, then state needs to be recompiled from that point otherwise, if tx_id is higher than the last transaction, state can be adjusted without recompiling
