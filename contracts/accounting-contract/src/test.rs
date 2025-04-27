#![cfg(test)]
use crate::{AccountingContract, AccountingContractClient, UserProfile};
use soroban_sdk::{Env, testutils::Address as _, Address, symbol_short, log, Vec};

#[test]
fn test_accounting_contract() {
    let env = Env::default();
    env.mock_all_auths(); // For the authorization of the user.
    let contract_id = env.register(AccountingContract, ());
    let client = AccountingContractClient::new(&env, &contract_id);

    let mut all_users_in_test = Vec::<UserProfile>::new(&env);

    // Define input data
    let username_1 = symbol_short!("Enver");
    let age_1 = 65;
    let user_address_1 = Address::generate(&env);

    let username_2 = symbol_short!("Mert");
    let age_2 = 29;
    let user_address_2 = Address::generate(&env);

    all_users_in_test.push_back(UserProfile {
        username: username_1.clone(),
        age: age_1,
        address: user_address_1.clone(),
    });

    all_users_in_test.push_back(UserProfile {
        username: username_2.clone(),
        age: age_2,
        address: user_address_2.clone(),
    });

    client.set_user_profile(&username_1, &age_1, &user_address_1);
    client.set_user_profile(&username_2, &age_2, &user_address_2);

    let latest_user = client.return_latest_user().unwrap();
    let all_users = client.return_all_users().unwrap();

    log!(&env, "Starting UserProfile Struct Test");

    assert_eq!(latest_user.username, username_2);
    assert_eq!(latest_user.age, age_2);
    assert_eq!(latest_user.address, user_address_2);

    log!(&env, "All the users are here:", all_users);
    log!(&env, "First User is here:", all_users.get(0));

    assert_eq!(all_users, all_users_in_test);

    log!(&env, "Test 1", all_users);
    log!(&env, "Test 2", all_users_in_test);

    // Loop through all users
    for user in all_users.iter() {
        log!(&env, "Username: ", user.username);
        log!(&env, "Age: ", user.age);
        log!(&env, "Address: ", user.address);
    }   

}


