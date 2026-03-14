#!/bin/bash

echo "=== STEP 1: Build AuctionToken ==="
cd contract/auction_token
cargo build --target wasm32-unknown-unknown --release

echo "=== STEP 2: Build AuctionBid v2 ==="
cd ../auction_bid_v2
cargo build --target wasm32-unknown-unknown --release

echo "=== STEP 3: Deploy frontend to Vercel ==="
cd ../../frontend
vercel --prod

echo "=== Done! Paste contract addresses in README.md ==="
```

---

## Final structure should look like this ✅
```
auction-dapp/
├── .github/
│   └── workflows/
│       └── ci.yml        ← NEW
├── contract/
│   ├── auction_token/
│   ├── auction_bid_v2/
│   └── src/
├── frontend/
├── deploy.sh             ← NEW
├── README.md
└── .gitignore