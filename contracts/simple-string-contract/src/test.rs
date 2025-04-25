#![cfg(test)]
use crate::{SimpleStringContract, SimpleStringContractClient};
use soroban_sdk::{Env, log, symbol_short, Symbol};

#[test]
fn test_simple_string_contract() {
    let env = Env::default();
    let contract_id = env.register(SimpleStringContract, ());
    let client = SimpleStringContractClient::new(&env, &contract_id);

    log!(&env, "Starting set_user_name test");
    let user_name = symbol_short!("Mert");

    assert_eq!(client.set_user_name(&user_name), symbol_short!("Mert"));

    log!(&env, "Current User Name:", client.get_user_name())
}


