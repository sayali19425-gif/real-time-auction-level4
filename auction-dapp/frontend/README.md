# ⚡ Real-Time Auction Bid dApp — Level 4

A production-ready decentralized auction platform built with React (Vite) and two inter-connected Soroban smart contracts on Stellar Testnet.

---

## 🔗 Live Demo

**[https://your-vercel-url.vercel.app](https://your-vercel-url.vercel.app)**

> *(Replace this with your actual Vercel URL after deployment)*

---

## ✅ CI/CD Pipeline

[![CI/CD Pipeline](https://github.com/sayali19425-gif/Real-time-auction-bid/actions/workflows/ci.yml/badge.svg)](https://github.com/sayali19425-gif/Real-time-auction-bid/actions/workflows/ci.yml)

The pipeline runs automatically on every push to `main` and does:
- Installs Node.js dependencies
- Runs `npm run build`
- Builds both Soroban contracts to WASM
- Deploys to Vercel on successful main branch builds

---

## 📱 Mobile Responsive View

The UI is fully responsive across all screen sizes.

| Desktop View | Mobile View |
|---|---|
| Two bid panels side by side | Panels stack vertically |
| Full bid history table | Condensed table |
| Full wallet address shown | Truncated wallet address |

> *(Add your mobile screenshot here after taking it from Chrome DevTools)*

---

## 📋 Contract Addresses (Stellar Testnet)

| Contract | Address |
|----------|---------|
| AuctionToken (BID token) | `REPLACE_WITH_TOKEN_CONTRACT_ADDRESS` |
| AuctionBid v2 | `REPLACE_WITH_AUCTION_CONTRACT_ADDRESS` |

**Inter-Contract Call Transaction Hash:**
`REPLACE_WITH_TRANSACTION_HASH`

🔍 View on Explorer:
[https://stellar.expert/explorer/testnet](https://stellar.expert/explorer/testnet)

### Token Details
| Property | Value |
|----------|-------|
| Name | BidToken |
| Symbol | BID |
| Decimals | 7 |
| Initial Supply | 1,000,000 BID |

---

## 🏗️ How Inter-Contract Calls Work
```
User Places Bid (Frontend)
        │
        ▼
AuctionBid Contract (auction_bid_v2)
        │
        │── calls ──▶ AuctionToken Contract
        │                  ├── balance(bidder)   ← check if bidder has enough tokens
        │                  └── transfer(winner, owner, amount) ← on auction end
        │
        ▼
Bid Accepted or Rejected
```

When a bid is placed, `AuctionBid` calls `AuctionToken` to verify the bidder has enough BID tokens. When the auction ends, tokens are transferred to the owner automatically.

---

## ✨ Features

### Level 3 (Base)
- Wallet connection with Stellar public key
- Real-time auction countdown timer
- Dual bidder panels with bid validation
- Bid comparison logic
- Responsive UI

### Level 4 (New)
- ✅ Custom BID Token Contract (mint, burn, transfer)
- ✅ Inter-contract calls (balance check + token transfer)
- ✅ Separate Bid History page with winner display
- ✅ GitHub Actions CI/CD pipeline
- ✅ Mobile responsive design
- ✅ Deployed on Vercel

---

## 🔧 Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | React, Vite, JavaScript |
| Styling | CSS with responsive media queries |
| Smart Contracts | Rust, Soroban SDK 21 |
| Blockchain | Stellar Testnet |
| CI/CD | GitHub Actions |
| Hosting | Vercel |

---

## 📁 Project Structure
```
Real-time-auction-bid/
│
├── .github/
│   └── workflows/
│       └── ci.yml                    ← CI/CD pipeline
│
├── contract/
│   ├── src/
│   │   └── lib.rs                    ← Original auction contract
│   ├── auction_token/                ← NEW: Custom BID token
│   │   ├── src/lib.rs
│   │   └── Cargo.toml
│   └── auction_bid_v2/               ← NEW: Inter-contract calls
│       ├── src/lib.rs
│       └── Cargo.toml
│
├── frontend/
│   ├── src/
│   │   ├── App.jsx                   ← Main auction page
│   │   ├── App.css                   ← Mobile responsive styles
│   │   ├── BidHistory.jsx            ← NEW: Bid history + winner page
│   │   ├── BidHistory.css            ← NEW: History page styles
│   │   └── main.jsx
│   ├── index.html
│   ├── package.json
│   └── vite.config.js
│
├── deploy.sh                         ← Deployment helper
└── README.md
```

---

## 🛠️ Local Setup
```bash
# 1. Clone the repository
git clone https://github.com/sayali19425-gif/Real-time-auction-bid
cd real-time-auction/auction-dapp

# 2. Install frontend dependencies
cd frontend
npm install

# 3. Start development server
npm run dev

# 4. Open in browser
# http://localhost:5173
```

---

## 🔨 Build Contracts
```bash
# Install Rust WASM target
rustup target add wasm32-unknown-unknown

# Build AuctionToken contract
cd contract/auction_token
cargo build --target wasm32-unknown-unknown --release

# Build AuctionBid v2 contract
cd ../auction_bid_v2
cargo build --target wasm32-unknown-unknown --release
```

---

## 🚀 Deploy Contracts to Stellar Testnet
```bash
# Deploy AuctionToken
stellar contract deploy \
  --wasm contract/auction_token/target/wasm32-unknown-unknown/release/auction_token.wasm \
  --source YOUR_SECRET_KEY \
  --network testnet

# Deploy AuctionBid v2
stellar contract deploy \
  --wasm contract/auction_bid_v2/target/wasm32-unknown-unknown/release/auction_bid_v2.wasm \
  --source YOUR_SECRET_KEY \
  --network testnet
```

---

## 📝 Git Commits (Level 4)
```
feat: add AuctionToken custom BID token Soroban contract
feat: add inter-contract calls in AuctionBid v2
feat: verify bidder token balance before accepting bid
feat: transfer winning bid tokens to owner on auction end
feat: add GitHub Actions CI/CD pipeline
fix: mobile responsive design - stack bid panels on small screens
feat: add separate bid history page with auction winner display
docs: update README with contract addresses and CI badge
```

---

## 👩‍💻 Author

Sayali — Learning decentralized application development on Stellar blockchain.

---

## 📜 License

Open-source for educational purposes.
```

---

## After you get your Vercel URL, replace these 2 lines:
```
**[https://your-vercel-url.vercel.app](https://your-vercel-url.vercel.app)**
```
with your real URL like:
```
**[https://real-time-auction-bid.vercel.app](https://real-time-auction-bid.vercel.app)**