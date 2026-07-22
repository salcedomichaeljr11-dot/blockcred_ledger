# BlockCred Ledger

Tamper-proof blockchain student logbook built on Stellar.

## Problem

Universities rely on spreadsheets and centralized databases that can delay transcript requests and make academic record verification difficult.

## Solution

Professors submit academic records through a Soroban smart contract, creating immutable student logbook entries that students and registrars can instantly verify.

## Timeline

Week 1
- Smart contract

Week 2
- Web frontend

Week 3
- Wallet integration

Week 4
- Demo & deployment

## Stellar Features

- Soroban Smart Contracts
- Custom Assets
- Trustlines

## Vision

Build a trusted academic credential network that universities, scholarship providers, and employers can use to verify student achievements without paper documents.

## Prerequisites

- Rust
- Soroban CLI
- Stellar Testnet Account

## Build

```bash
soroban contract build
```

## Test

```bash
cargo test
```

## Deploy

```bash
soroban contract deploy \
--wasm target/wasm32-unknown-unknown/release/blockcred_ledger.wasm \
--network testnet
```

## Sample Invocation

```bash
soroban contract invoke \
--id CONTRACT_ID \
--network testnet \
-- add_record \
--professor GABC... \
--student GXYZ... \
--course "Blockchain" \
--grade 95 \
--attendance 100 \
--internship_hours 120
```

## License

MIT

✅ Transaction submitted successfully!
🔗 https://stellar.expert/explorer/testnet/tx/d976386fa5e99a091131067146a15d2b388ec55bbedbaae61aeae41f4d18ac6a
🔗 https://lab.stellar.org/r/testnet/contract/CDKDY74U46VPQWDPAZNRIK7TAYABXZYDTO5MD3SGKAAULLAZD77G4IDQ
✅ Deployed!
CDKDY74U46VPQWDPAZNRIK7TAYABXZYDTO5MD3SGKAAULLAZD77G4IDQ