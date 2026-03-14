#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, String,
};

#[contracttype]
pub enum DataKey {
    Balance(Address),
    TotalSupply,
    Admin,
    Name,
    Symbol,
    Decimals,
}

#[contract]
pub struct AuctionTokenContract;

#[contractimpl]
impl AuctionTokenContract {
    pub fn initialize(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        decimals: u32,
        initial_supply: i128,
    ) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::Decimals, &decimals);
        env.storage().instance().set(&DataKey::TotalSupply, &initial_supply);
        env.storage().instance().set(&DataKey::Balance(admin.clone()), &initial_supply);
        env.events().publish((symbol_short!("init"),), (admin, initial_supply));
    }

    pub fn name(env: Env) -> String {
        env.storage().instance().get(&DataKey::Name).unwrap()
    }

    pub fn symbol(env: Env) -> String {
        env.storage().instance().get(&DataKey::Symbol).unwrap()
    }

    pub fn decimals(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Decimals).unwrap()
    }

    pub fn balance(env: Env, owner: Address) -> i128 {
        env.storage().instance().get(&DataKey::Balance(owner)).unwrap_or(0)
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0)
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        assert!(amount > 0, "Amount must be positive");
        let from_balance = Self::balance(env.clone(), from.clone());
        assert!(from_balance >= amount, "Insufficient balance");
        let to_balance = Self::balance(env.clone(), to.clone());
        env.storage().instance().set(&DataKey::Balance(from.clone()), &(from_balance - amount));
        env.storage().instance().set(&DataKey::Balance(to.clone()), &(to_balance + amount));
        env.events().publish((symbol_short!("transfer"),), (from, to, amount));
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        assert!(amount > 0, "Amount must be positive");
        let to_balance = Self::balance(env.clone(), to.clone());
        let total: i128 = Self::total_supply(env.clone());
        env.storage().instance().set(&DataKey::Balance(to.clone()), &(to_balance + amount));
        env.storage().instance().set(&DataKey::TotalSupply, &(total + amount));
        env.events().publish((symbol_short!("mint"),), (to, amount));
    }

    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();
        assert!(amount > 0, "Amount must be positive");
        let from_balance = Self::balance(env.clone(), from.clone());
        assert!(from_balance >= amount, "Insufficient balance");
        let total: i128 = Self::total_supply(env.clone());
        env.storage().instance().set(&DataKey::Balance(from.clone()), &(from_balance - amount));
        env.storage().instance().set(&DataKey::TotalSupply, &(total - amount));
        env.events().publish((symbol_short!("burn"),), (from, amount));
    }
}