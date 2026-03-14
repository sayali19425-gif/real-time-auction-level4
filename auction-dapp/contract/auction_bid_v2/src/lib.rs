#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Vec, String,
};

mod token_interface {
    use soroban_sdk::{contractclient, Address, Env};

    #[contractclient(name = "TokenClient")]
    pub trait TokenInterface {
        fn balance(env: Env, owner: Address) -> i128;
        fn transfer(env: Env, from: Address, to: Address, amount: i128);
    }
}

#[contracttype]
#[derive(Clone)]
pub struct Bid {
    pub bidder: Address,
    pub amount: i128,
    pub timestamp: u64,
}

#[contracttype]
pub enum DataKey {
    HighestBid,
    HighestBidder,
    BidHistory,
    AuctionEnd,
    Owner,
    TokenContract,
    AuctionItem,
    Active,
}

#[contract]
pub struct AuctionBidContract;

#[contractimpl]
impl AuctionBidContract {
    pub fn initialize(
        env: Env,
        owner: Address,
        token_contract: Address,
        auction_item: String,
        duration_seconds: u64,
    ) {
        owner.require_auth();
        let end_time = env.ledger().timestamp() + duration_seconds;
        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::TokenContract, &token_contract);
        env.storage().instance().set(&DataKey::AuctionItem, &auction_item);
        env.storage().instance().set(&DataKey::AuctionEnd, &end_time);
        env.storage().instance().set(&DataKey::HighestBid, &0_i128);
        env.storage().instance().set(&DataKey::Active, &true);
        env.storage().instance().set(&DataKey::BidHistory, &Vec::<Bid>::new(&env));
        env.events().publish((symbol_short!("started"),), (auction_item, end_time));
    }

    pub fn place_bid(env: Env, bidder: Address, amount: i128) {
        bidder.require_auth();
        let active: bool = env.storage().instance().get(&DataKey::Active).unwrap_or(false);
        assert!(active, "Auction is not active");
        let end_time: u64 = env.storage().instance().get(&DataKey::AuctionEnd).unwrap();
        assert!(env.ledger().timestamp() < end_time, "Auction has ended");
        let current_highest: i128 = env.storage().instance().get(&DataKey::HighestBid).unwrap_or(0);
        assert!(amount > current_highest, "Bid must exceed current highest bid");

        // Inter-contract call: verify bidder has enough tokens
        let token_address: Address = env.storage().instance().get(&DataKey::TokenContract).unwrap();
        let token_client = token_interface::TokenClient::new(&env, &token_address);
        let bidder_balance = token_client.balance(&bidder);
        assert!(bidder_balance >= amount, "Insufficient token balance");

        env.storage().instance().set(&DataKey::HighestBid, &amount);
        env.storage().instance().set(&DataKey::HighestBidder, &bidder);

        let mut history: Vec<Bid> = env.storage().instance().get(&DataKey::BidHistory).unwrap_or(Vec::new(&env));
        history.push_back(Bid {
            bidder: bidder.clone(),
            amount,
            timestamp: env.ledger().timestamp(),
        });
        env.storage().instance().set(&DataKey::BidHistory, &history);
        env.events().publish((symbol_short!("new_bid"),), (bidder, amount));
    }

    pub fn end_auction(env: Env) {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();
        let active: bool = env.storage().instance().get(&DataKey::Active).unwrap_or(false);
        assert!(active, "Auction already ended");
        env.storage().instance().set(&DataKey::Active, &false);

        let highest_bid: i128 = env.storage().instance().get(&DataKey::HighestBid).unwrap_or(0);
        if highest_bid > 0 {
            let winner: Address = env.storage().instance().get(&DataKey::HighestBidder).unwrap();

            // Inter-contract call: transfer winning bid tokens to owner
            let token_address: Address = env.storage().instance().get(&DataKey::TokenContract).unwrap();
            let token_client = token_interface::TokenClient::new(&env, &token_address);
            token_client.transfer(&winner, &owner, &highest_bid);

            env.events().publish((symbol_short!("won"),), (winner, highest_bid));
        }
        env.events().publish((symbol_short!("ended"),), (highest_bid,));
    }

    pub fn get_highest_bid(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::HighestBid).unwrap_or(0)
    }

    pub fn get_highest_bidder(env: Env) -> Option<Address> {
        env.storage().instance().get(&DataKey::HighestBidder)
    }

    pub fn get_bid_history(env: Env) -> Vec<Bid> {
        env.storage().instance().get(&DataKey::BidHistory).unwrap_or(Vec::new(&env))
    }

    pub fn get_end_time(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::AuctionEnd).unwrap_or(0)
    }

    pub fn is_active(env: Env) -> bool {
        env.storage().instance().get(&DataKey::Active).unwrap_or(false)
    }
}