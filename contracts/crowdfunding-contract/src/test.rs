#![cfg(test)]

use crate::{CrowdfundingContract, CrowdfundingContractClient};
use soroban_sdk::{Env, log, Address, String};

#[test]
fn test_crowdfunding_contract() {
    let env = Env::default();
    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let token_credentials = String::from_str(&env, "CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA");
    let recipient_credentials = String::from_str(&env, "CDBJLX5EKCYSEYUKLQ32K37LFSWBLYGDHPFOLEHZAARFC55L5ON24UWN");

    let token_address = Address::from_string(&token_credentials);
    let recipient_address = Address::from_string(&recipient_credentials);

    client.initialize(&recipient_address, &token_address);

    log!(&env, "Recipient Address:", client.recipient());
    log!(&env, "Token Address:", client.token());

    assert_eq!(client.recipient(), recipient_address);
    assert_eq!(client.token(), token_address);

    
}


