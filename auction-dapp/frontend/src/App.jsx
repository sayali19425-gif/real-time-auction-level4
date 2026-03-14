import { useState, useEffect, useRef } from "react";
import { BrowserRouter, Routes, Route, useNavigate } from "react-router-dom";
import "./App.css";
import BidHistory from "./BidHistory";

const TOKEN_CONTRACT_ADDRESS = "CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX_TOKEN";
const AUCTION_CONTRACT_ADDRESS = "CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX_AUCTION";
const AUCTION_ITEM = "Rare Digital Artwork #001";
const AUCTION_DURATION = 10 * 60;

function useCountdown(endTimestamp) {
  const [timeLeft, setTimeLeft] = useState(0);
  useEffect(() => {
    const calc = () => {
      const diff = Math.max(0, endTimestamp - Math.floor(Date.now() / 1000));
      setTimeLeft(diff);
    };
    calc();
    const id = setInterval(calc, 1000);
    return () => clearInterval(id);
  }, [endTimestamp]);
  const m = String(Math.floor(timeLeft / 60)).padStart(2, "0");
  const s = String(timeLeft % 60).padStart(2, "0");
  return { display: `${m}:${s}`, expired: timeLeft === 0 };
}

const MOCK_BALANCES = { default: 5000 };
function getTokenBalance(wallet) {
  return MOCK_BALANCES[wallet] ?? 5000;
}

function AuctionPage({ bidHistory, setBidHistory, winner, setWinner }) {
  const navigate = useNavigate();
  const [wallet, setWallet] = useState("");
  const [walletInput, setWalletInput] = useState("");
  const [connected, setConnected] = useState(false);
  const [highestBid, setHighestBid] = useState(0);
  const [highestBidder, setHighestBidder] = useState("");
  const [bidder1Wallet, setBidder1Wallet] = useState("");
  const [bidder1Amount, setBidder1Amount] = useState("");
  const [bidder2Wallet, setBidder2Wallet] = useState("");
  const [bidder2Amount, setBidder2Amount] = useState("");
  const [toast, setToast] = useState(null);
  const toastTimer = useRef(null);
  const endTime = useRef(Math.floor(Date.now() / 1000) + AUCTION_DURATION);
  const { display: timeDisplay, expired } = useCountdown(endTime.current);

  function showToast(msg) {
    setToast(msg);
    clearTimeout(toastTimer.current);
    toastTimer.current = setTimeout(() => setToast(null), 3500);
  }

  function connectWallet() {
    if (!walletInput.trim() || walletInput.length < 10) {
      showToast("Enter a valid Stellar public key");
      return;
    }
    setWallet(walletInput.trim());
    setConnected(true);
    showToast("Wallet connected successfully!");
  }

  function placeBid(bidderWallet, amountStr, bidderLabel) {
    if (expired) { showToast("Auction has ended"); return; }
    const amount = parseFloat(amountStr);
    if (!bidderWallet.trim()) { showToast(`Enter wallet for ${bidderLabel}`); return; }
    if (!amountStr || isNaN(amount) || amount <= 0) { showToast("Enter a valid bid amount"); return; }
    if (amount <= highestBid) { showToast(`Bid must exceed current highest: ${highestBid} XLM`); return; }
    if (getTokenBalance(bidderWallet) < amount) { showToast("Insufficient token balance"); return; }

    const newBid = {
      bidder: bidderWallet,
      amount,
      timestamp: new Date().toLocaleTimeString(),
    };

    setHighestBid(amount);
    setHighestBidder(bidderWallet);
    setBidHistory((prev) => [newBid, ...prev]);
    setWinner({ address: bidderWallet, amount });
    showToast(`${bidderLabel} placed highest bid: ${amount} XLM`);
  }

  if (!connected) {
    return (
      <div className="app-container" style={{ display: "flex", justifyContent: "center", alignItems: "center", minHeight: "100vh" }}>
        <div className="bid-panel" style={{ maxWidth: 420, width: "100%" }}>
          <h2 style={{ marginBottom: "1.5rem", textAlign: "center", fontSize: "1.3rem" }}>
            Connect Stellar Wallet
          </h2>
          <p style={{ color: "#a0aec0", fontSize: "0.85rem", marginBottom: "1rem" }}>
            Token: <code style={{ fontSize: "0.75rem", color: "#667eea" }}>{TOKEN_CONTRACT_ADDRESS}</code>
          </p>
          <input
            placeholder="Enter your Stellar public key (G...)"
            value={walletInput}
            onChange={(e) => setWalletInput(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && connectWallet()}
          />
          <button onClick={connectWallet}>Connect Wallet</button>
        </div>
        {toast && <div className="toast">{toast}</div>}
      </div>
    );
  }

  return (
    <div className="app-container">
      <header className="app-header">
        <h1>Real-Time Auction Bid</h1>
        <div className="wallet-section">
          <span className="token-balance">
            Balance: {getTokenBalance(wallet).toLocaleString()} BID
          </span>
          <span className="wallet-address">
            {wallet.slice(0, 8)}...{wallet.slice(-6)}
          </span>
        </div>
      </header>

      <div className="auction-info">
        <div className="info-card">
          <div className="label">Item</div>
          <div className="value" style={{ fontSize: "0.9rem" }}>{AUCTION_ITEM}</div>
        </div>
        <div className="info-card">
          <div className="label">Highest Bid</div>
          <div className="value">{highestBid > 0 ? `${highestBid} XLM` : "No bids yet"}</div>
        </div>
        <div className="info-card">
          <div className="label">Time Left</div>
          <div className="timer">{expired ? "ENDED" : timeDisplay}</div>
        </div>
        <div className="info-card">
          <div className="label">Top Bidder</div>
          <div className="value" style={{ fontSize: "0.75rem", wordBreak: "break-all" }}>
            {highestBidder ? `${highestBidder.slice(0, 10)}...` : "—"}
          </div>
        </div>
      </div>

      <div className="bid-panels">
        <div className="bid-panel">
          <h3>Bidder 1</h3>
          <div className="token-balance">Balance: {getTokenBalance(bidder1Wallet || "default").toLocaleString()} BID</div>
          <input placeholder="Bidder 1 wallet address" value={bidder1Wallet} onChange={(e) => setBidder1Wallet(e.target.value)} />
          <input placeholder="Bid amount in XLM" type="number" value={bidder1Amount} onChange={(e) => setBidder1Amount(e.target.value)} />
          <button onClick={() => placeBid(bidder1Wallet, bidder1Amount, "Bidder 1")}>Place Bid</button>
        </div>

        <div className="bid-panel">
          <h3>Bidder 2</h3>
          <div className="token-balance">Balance: {getTokenBalance(bidder2Wallet || "default").toLocaleString()} BID</div>
          <input placeholder="Bidder 2 wallet address" value={bidder2Wallet} onChange={(e) => setBidder2Wallet(e.target.value)} />
          <input placeholder="Bid amount in XLM" type="number" value={bidder2Amount} onChange={(e) => setBidder2Amount(e.target.value)} />
          <button onClick={() => placeBid(bidder2Wallet, bidder2Amount, "Bidder 2")}>Place Bid</button>
        </div>
      </div>

      {/* View Bid History Button */}
      <div style={{ textAlign: "center", marginBottom: "1.5rem" }}>
        <button
          onClick={() => navigate("/history")}
          style={{
            background: "rgba(102,126,234,0.2)",
            color: "#a5b4fc",
            border: "1px solid rgba(102,126,234,0.4)",
            borderRadius: "10px",
            padding: "0.75rem 2rem",
            fontSize: "1rem",
            cursor: "pointer",
            transition: "background 0.2s",
          }}
        >
          View Bid History & Winner →
        </button>
      </div>

      <div style={{ fontSize: "0.72rem", color: "#4a5568", textAlign: "center", padding: "0.5rem 0" }}>
        Token: {TOKEN_CONTRACT_ADDRESS} | Auction: {AUCTION_CONTRACT_ADDRESS}
      </div>

      {toast && <div className="toast">{toast}</div>}
    </div>
  );
}

export default function App() {
  const [bidHistory, setBidHistory] = useState([]);
  const [winner, setWinner] = useState({ address: "", amount: 0 });

  return (
    <BrowserRouter>
      <Routes>
        <Route
          path="/"
          element={
            <AuctionPage
              bidHistory={bidHistory}
              setBidHistory={setBidHistory}
              winner={winner}
              setWinner={setWinner}
            />
          }
        />
        <Route
          path="/history"
          element={
            <BidHistory
              bidHistory={bidHistory}
              winner={winner}
            />
          }
        />
      </Routes>
    </BrowserRouter>
  );
}