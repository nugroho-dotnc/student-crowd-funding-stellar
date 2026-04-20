# Student Grant Crowdfunding

A Soroban-based smart contract built for students trying to raise funds for their projects. Simple, direct, and completely decentralized on the Stellar network.

### What is this?
Instead of using centralized platforms to gather donations, this project lets students spin up their own native fundraising campaign. Donors can directly pitch in using Freighter or any Stellar-compatible wallet.

The catch? The funds are locked. The contract ensures that the creator can only withdraw the money once they actually hit their funding target. This mechanism naturally keeps the creator accountable and protects the donors.

### Under the Hood
- **Standalone Instances**: Every grant is deployed as its own contract. This avoids messy state mappings and isolates the security per project.
- **Strict Goal Checking**: We track the exact `raised_amount` against the `target_amount` using stroops (1 XLM = 10,000,000 stroops).
- **Bulletproof Withdrawals**: Uses the Checks-Effects-Interactions (CEI) pattern. We flip the withdrawal state before touching the token transfer to prevent reentrancy attacks.
- **Built-in Auth**: Relies heavily on Soroban's `require_auth()` to make sure donors explicitly sign off on transactions before any XLM leaves their wallet.

### Network Setup

If you want to poke around or integrate the frontend, here's the current Testnet contract:

**Contract ID**: `[paste-your-contract-id-here]`

### Dashboard Preview
*(A quick look at the UI state for the campaign)*

![Preview](./screenshot.png)
