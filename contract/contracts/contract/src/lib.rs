#![no_std]

use soroban_sdk::{
    contract, contractimpl, symbol_short, Address, Env, Map, Vec,
};

#[contract]
pub struct RoyaltySplitter;

#[contractimpl]
impl RoyaltySplitter {
    // Initialize recipients and their percentage shares
    pub fn init(env: Env, recipients: Vec<Address>, shares: Vec<u32>) {
        if recipients.len() != shares.len() {
            panic!("Mismatched input");
        }

        // Calculate total shares manually (since .sum() isn't supported)
        let mut total: u32 = 0;
        for i in 0..shares.len() {
            total += shares.get(i).unwrap();
        }

        if total != 100 {
            panic!("Shares must sum to 100");
        }

        let mut map: Map<Address, u32> = Map::new(&env);

        for i in 0..recipients.len() {
            let addr = recipients.get(i).unwrap();
            let share = shares.get(i).unwrap();
            map.set(addr, share);
        }

        env.storage().instance().set(&symbol_short!("SHARES"), &map);
    }

    // Split incoming amount based on shares
    pub fn split(env: Env, amount: i128) -> Map<Address, i128> {
        let shares: Map<Address, u32> = env
            .storage()
            .instance()
            .get(&symbol_short!("SHARES"))
            .unwrap();

        let mut payouts: Map<Address, i128> = Map::new(&env);

        // Soroban Map iteration requires keys()
        let keys: Vec<Address> = shares.keys();

        for i in 0..keys.len() {
            let addr = keys.get(i).unwrap();
            let share = shares.get(addr.clone()).unwrap();

            let payout = amount * (share as i128) / 100;
            payouts.set(addr, payout);
        }

        payouts
    }

    // Get stored shares
    pub fn get_shares(env: Env) -> Map<Address, u32> {
        env.storage()
            .instance()
            .get(&symbol_short!("SHARES"))
            .unwrap()
    }
}