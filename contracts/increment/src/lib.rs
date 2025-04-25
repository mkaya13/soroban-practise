#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);

        count += 1;
        env.storage().instance().set(&COUNTER, &count);
        env.storage().instance().extend_ttl(50, 100);

        log!(&env, "COUNTER: {}", count);

        count
    }

    pub fn decrement(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);

        if count == 0 {
            panic!("The COUNTER is already 0!");
        }

        count -= 1;
        env.storage().instance().set(&COUNTER, &count);
        env.storage().instance().extend_ttl(50, 100);

        log!(&env, "COUNTER: {}", count);

        count
    }

    pub fn double(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);

        count *= 2;
        env.storage().instance().set(&COUNTER, &count);
        env.storage().instance().extend_ttl(50, 100);

        log!(&env, "COUNTER: {}", count);

        count
    }

    pub fn halve(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);

        if count == 0 {
            panic!("The COUNTER cannot be halved because it is 0!");
        }

        count /= 2;
        env.storage().instance().set(&COUNTER, &count);
        env.storage().instance().extend_ttl(50, 100);

        log!(&env, "COUNTER: {}", count);

        count
    }

    pub fn get_current_value(env: Env) -> u32 {

        let count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        log!(&env, "Current Value of Count: {}", count);

        count
    }
}

mod test;
