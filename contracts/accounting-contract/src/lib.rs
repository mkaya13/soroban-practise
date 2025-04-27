#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Symbol, Address, Env, symbol_short, Vec};

const LATEST_USER_KEY: Symbol = symbol_short!("PROFILE");
const ALL_USERS_KEY: Symbol = symbol_short!("ALL");

#[contract]
pub struct AccountingContract;

#[contracttype]  // To do serialization of the struct
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserProfile {
    pub username: Symbol,
    pub age: u32,
    pub address: Address,
}

#[contractimpl]
impl AccountingContract {

    pub fn set_user_profile(env: Env, username: Symbol, age: u32, address: Address ) {

        // Require authentication from the user
        address.require_auth();


        // Try to get existing Vec<UserProfile>, or create an empty one
        let mut profiles = env.storage().instance().get::<Symbol, Vec<UserProfile>>(&ALL_USERS_KEY).unwrap_or(Vec::new(&env));

        // Create the new profile
        let user_profile = UserProfile {username, age, address: address.clone()};

        // Push the new profile into the Vec
        profiles.push_back(user_profile.clone());

        // ✅ Save the updated profiles Vec back to storage
        env.storage().instance().set(&ALL_USERS_KEY, &profiles);
        // Save the latest profile separately
        env.storage().instance().set(&LATEST_USER_KEY, &user_profile);

    }

    pub fn return_latest_user(env: Env) -> Option<UserProfile> {

        let latest_user = env.storage().instance().get(&LATEST_USER_KEY);

        latest_user
    }

    pub fn return_all_users(env: Env) -> Option<Vec<UserProfile>> {
        let all_users = env.storage().instance().get(&ALL_USERS_KEY);

        all_users
    }
    

}

mod test;
