# Payment Engine

a rust program that processes varying types of transactions for clients

## 👋 Getting Started

- build: `cargo build`
- run the unit tests: `cargo test`
- run the program: `cargo run -- transactions.csv > accounts.csv`

## Assumptions

all assumptions from the specification pdf were adopted along with the following...

### Transaction Ids

Assuming that the first transaction which appears for a client is their actual first transaction and the rest will appear in the right order with respect to the client

If this assumption is inaccurate, then that means an incoming transaction could have a lower id than the last transaction processed for the client. In that case, the account state needs to be recompiled starting from that new earlier transaction. Otherwise, if tx_id is higher than the last transaction, the program can continue without issue.

### Chargebacks & Resolutions

I assume that both `chargeback` and `resolve` are only meant for `deposit` transactions... It is unclear to me how chargebacks/resolutions could revert `withdraw` transactions...

for instance,

- both `chargeback` and `resolve` move funds from `available` -> `held`; this makes sense if we were undoing a `deposit` transaction, but to undo a `withdraw` transaction, I wouldn't expect funds to be moved from the available account at all
- when a `chargeback` is complete, the funds are removed from `held` and not moved back into `available`; again, this makes sense if we were undoing a `deposit` transaction, but this doesn't make sense if the `tx_id` is pointing to a `withdraw` transaction to be reversed

## Improvements

As mentioned in the challenge specifications, in a real environment, the transactions are coming from thousands of concurrent TCP streams. In that case, there are two major objectives to be completed:

1. a database must be implemented for storing transaction history and account states
   - remove the `InMemoryDatabase` implementation and replace it with a real database
   - relevant crates: rocksdb, prisma-client, sled, redb
   - databases should have their own internal locking to avoid data race conditions
   - schema would be very similar to the structs found in the services folder
2. make the program asynchronous, multi-threaded, and receive buffered data
   - one or more threads for receiving transactions and writing those to the database
   - one or more threads that listen for new transactions and update account states accordingly
   - relevant functions should be made asynchronous (crates: tokio)
