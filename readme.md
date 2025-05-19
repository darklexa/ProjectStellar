# Project Name: **MicroLend**

## About Me

* **Name:** Betül Güler (darklexa)
* **Role:** Product Specialist Technical Support
* **Background:** Passionate about coding.

---

## Project Details

**MicroLend** is a Soroban-based micro-lending token contract on Stellar Testnet. It allows a lending platform (the Admin) to mint loan tokens, distribute loans, collect repayments, and handle defaults by freezing accounts. Borrowers and lenders interact through trustless, on-chain token transfers.

Key features:

* `mint(to, amount)`: Create loan tokens for distribution
* `transfer(from, to, amount)`: Move tokens, blocked if the sender is frozen
* `freeze_account(who)`: Restrict defaulting borrowers from transfers
* `unfreeze_account(who)`: Re-enable transfers after resolution
* `balance(who)`: View outstanding loan balance

---

## Vision

To democratize access to small-scale credit worldwide by automating micro-loans through smart contracts. **MicroLend** envisions a financial ecosystem where underbanked individuals access transparent, low-cost loans without intermediaries, fostering economic growth and community empowerment.

---

## Software Development Plan

1. **Smart Contract Core:** Implement `mint`, `transfer`, and `balance` functions in Rust for Soroban.
2. **Freeze/Unfreeze Logic:** Add `freeze_account` and `unfreeze_account` to manage defaulting borrowers.
3. **Local Testing:** Write unit tests for happy paths and freeze scenarios.
4. **Deployment Script:** Automate build & deploy via GitHub Actions to Stellar Testnet.
5. **Frontend Integration (optional):** Build a simple CLI or web UI to call contract endpoints.
6. **Production Deployment:** Deploy to Mainnet and monitor usage.

---

## Installation & Deployment

```bash
# Clone the repo
git clone https://github.com/darklexa/ProjectStellar.git
cd micro_lend

# Install Soroban CLI (once)
cargo install soroban-cli --locked

# Build and deploy to Testnet
soroban contract build
soroban contract deploy --network testnet target/wasm32v1-none/release/micro_lending.wasm
```

**Contract Address:** `

## Frontend

1. Open the repo in VS Code or any editor.  
2. `cd frontend`  
3. `npm install`  
4. `npm start`  
5. Visit http://localhost:3000 to interact with your contract.


---

## Mascot Logo

![Mascot](logo.png)

