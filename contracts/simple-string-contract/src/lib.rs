#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const USERNAME: Symbol = symbol_short!("USERNAME");

#[contract]
pub struct SimpleStringContract;

#[contractimpl]
impl SimpleStringContract {
    
    pub fn set_user_name(env: Env, value: Symbol) -> Symbol {
        env.storage().instance().set(&USERNAME, &value);
        value
    }

    pub fn get_user_name(env: Env) -> Symbol {
        
        let username = env.storage().instance().get(&USERNAME).unwrap_or_else(|| symbol_short!(""));
        
        log!(&env, "Current User Name: {}", username);

        username
    }
}

mod test;
