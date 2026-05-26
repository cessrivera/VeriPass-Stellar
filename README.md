# VeriPass Stellar


Generate temporary blockchain-verified estate guest passes without infrastructure overhead.


## Problem & Solution
Gated residential estate managers in Lagos, Nigeria, suffer from high security vulnerabilities and slow visitor check-ins because manual paper logbooks are easily falsified, prone to data loss, and cause long vehicle queues at the front gate during peak hours.


VeriPass Stellar allows residents to issue time-bound visitor passes as custom Soroban-tracked assets that security guards instantly verify via a lightweight mobile web view, leveraging Stellar’s near-instant finality and low transaction fees to maintain an immutable access log without local server infrastructure.


## Timeline
* **Day 1:** Core Smart Contract design, testing, and deployment setup.
* **Day 2:** Frontend Guard Verification Interface & QR code generation layer.
* **Day 3:** Testing, verification optimizations, and pitching video production.


## Stellar Features Used
* Soroban Smart Contracts (State tracking & permission validations)
* Stellar Custom Assets / Trustlines (Pass ownership representation)


## Vision and Purpose
To democratize digital access control for communities within developing regions, enabling top-tier security standards through minimal asset pipelines and public ledger primitives.


## Prerequisites
* Rust (v1.75+)
* Soroban CLI (v20.0.0+)
* Target `wasm32-unknown-unknown` installed via rustup


## How to Build
```bash
soroban contract build

