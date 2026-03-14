#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String};

// Keys used to store data in the contract
#[contracttype]
pub enum DataKey {
    HighestBid,
    HighestBidder,
    AuctionEndTime,
    AuctionStarted,
    ItemName,
}

#[contract]
pub struct AuctionContract;

#[contractimpl]
impl AuctionContract {

    // Call this once to start the auction
    // item_name = name of the item being auctioned
    // duration_secs = how long the auction runs (in seconds)
    pub fn initialize(env: Env, item_name: String, duration_secs: u64) {
        // Stop if auction already started
        if env.storage().instance().has(&DataKey::AuctionStarted) {
            panic!("Auction already initialized");
        }

        // Calculate end time = current time + duration
        let end_time = env.ledger().timestamp() + duration_secs;

        // Save everything to contract storage
        env.storage().instance().set(&DataKey::ItemName,       &item_name);
        env.storage().instance().set(&DataKey::HighestBid,     &0u64);
        env.storage().instance().set(&DataKey::AuctionEndTime, &end_time);
        env.storage().instance().set(&DataKey::AuctionStarted, &true);

        // Emit an event so the frontend knows auction started
        env.events().publish((symbol_short!("init"),), (item_name, end_time));
    }

    // Call this to place a bid
    // bidder = wallet address of the person bidding
    // amount = bid amount in stroops (1 XLM = 10,000,000 stroops)
    pub fn place_bid(env: Env, bidder: Address, amount: u64) {
        // Make sure the bidder signed this transaction
        bidder.require_auth();

        // Check auction exists
        if !env.storage().instance().has(&DataKey::AuctionStarted) {
            panic!("Auction not initialized");
        }

        // Check auction has not ended yet
        let end_time: u64 = env.storage().instance().get(&DataKey::AuctionEndTime).unwrap();
        if env.ledger().timestamp() >= end_time {
            panic!("Auction has ended");
        }

        // Check new bid is higher than current highest
        let current_bid: u64 = env.storage().instance().get(&DataKey::HighestBid).unwrap();
        if amount <= current_bid {
            panic!("Bid must be higher than current highest bid");
        }

        // Save new highest bid and bidder
        env.storage().instance().set(&DataKey::HighestBid,     &amount);
        env.storage().instance().set(&DataKey::HighestBidder,  &bidder);

        // Emit event so frontend can update in real time
        env.events().publish((symbol_short!("new_bid"),), (bidder, amount));
    }

    // Returns the current highest bid amount
    pub fn get_highest_bid(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::HighestBid).unwrap_or(0)
    }

    // Returns the address of the highest bidder
    pub fn get_highest_bidder(env: Env) -> Option<Address> {
        env.storage().instance().get(&DataKey::HighestBidder)
    }

    // Returns when the auction ends (Unix timestamp)
    pub fn get_end_time(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::AuctionEndTime).unwrap_or(0)
    }

    // Returns the name of the auction item
    pub fn get_item_name(env: Env) -> String {
        env.storage().instance().get(&DataKey::ItemName).unwrap()
    }

    // Returns true if auction is still running, false if ended
    pub fn is_active(env: Env) -> bool {
        if !env.storage().instance().has(&DataKey::AuctionStarted) {
            return false;
        }
        let end_time: u64 = env.storage().instance().get(&DataKey::AuctionEndTime).unwrap();
        env.ledger().timestamp() < end_time
    }
}