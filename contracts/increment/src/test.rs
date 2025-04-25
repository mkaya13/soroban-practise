#![cfg(test)]
use crate::{IncrementContract, IncrementContractClient};
use soroban_sdk::{Env, log};

#[test]
fn test_1() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);
    

    log!(&env, "Starting increment test");
    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
    log!(&env, "Starting decrementing test");
    assert_eq!(client.decrement(), 2);

    log!(&env, "Starting doubling test");
    assert_eq!(client.double(), 4);

    log!(&env, "Starting halving test");
    assert_eq!(client.halve(), 2);

    log!(&env, "Current value of the COUNTER:", client.get_current_value())

}


#[test]
#[should_panic(expected = "The COUNTER is already 0!")]
fn test_2() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);

    // Set counter to 0 directly (or do no increment)
    client.decrement(); // This should panic
}

#[test]
#[should_panic(expected = "The COUNTER cannot be halved because it is 0!")]
fn test_halve() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);

    // Set counter to 0 directly (or do no increment)
    client.halve(); // This should panic
}