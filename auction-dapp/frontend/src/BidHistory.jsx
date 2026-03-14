import { useNavigate } from "react-router-dom";
import "./BidHistory.css";

export default function BidHistory({ bidHistory, winner }) {
  const navigate = useNavigate();

  return (
    <div className="history-container">
      <div className="history-header">
        <button className="back-btn" onClick={() => navigate(-1)}>
          ← Back to Auction
        </button>
        <h1>Bid History</h1>
      </div>

      {/* Bid History Table */}
      <div className="history-card">
        <h2>All Bids</h2>
        {bidHistory.length === 0 ? (
          <p className="empty-msg">No bids placed yet.</p>
        ) : (
          <table className="history-table">
            <thead>
              <tr>
                <th>#</th>
                <th>Bidder</th>
                <th>Amount (XLM)</th>
                <th>Time</th>
              </tr>
            </thead>
            <tbody>
              {bidHistory.map((bid, i) => (
                <tr key={i} className={i === 0 ? "top-bid" : ""}>
                  <td>{bidHistory.length - i}</td>
                  <td className="address">{bid.bidder}</td>
                  <td className="amount">{bid.amount} XLM</td>
                  <td>{bid.timestamp}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      {/* Auction Winner Section */}
      <div className="winner-card">
        <h2>🏆 Auction Winner</h2>
        {winner.address ? (
          <div className="winner-info">
            <div className="winner-trophy">🏆</div>
            <div className="winner-details">
              <p className="winner-label">Winner</p>
              <p className="winner-address">{winner.address}</p>
              <p className="winner-label">Winning Bid</p>
              <p className="winner-amount">{winner.amount} XLM</p>
            </div>
          </div>
        ) : (
          <p className="empty-msg">
            No winner yet — auction is still running!
          </p>
        )}
      </div>
    </div>
  );
}