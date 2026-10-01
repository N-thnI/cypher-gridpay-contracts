#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};
use test_utils::{create_mock_token, generate_parties, setup_test_env};

use crate::{EscrowContract, EscrowContractClient};

fn create_contract(env: &Env) -> EscrowContractClient {
    let contract_id = env.register_contract(None, EscrowContract);
    EscrowContractClient::new(env, &contract_id)
}

#[test]
fn test_setup_test_env_provides_funded_parties() {
    let test_env = setup_test_env(1_000);
    let parties = &test_env.parties;

    assert_eq!(test_env.token.client.balance(&parties.payer), 1_000);
    assert_eq!(test_env.token.client.balance(&parties.payee), 1_000);
    assert_eq!(test_env.token.client.balance(&parties.arbiter), 1_000);
}

#[test]
fn test_create_mock_token_registers_asset() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let token = create_mock_token(&env, &admin);

    token.admin_client.mint(&admin, &500);
    assert_eq!(token.client.balance(&admin), 500);
}

#[test]
fn test_generate_parties_are_distinct() {
    let env = Env::default();
    let parties = generate_parties(&env);

    assert_ne!(parties.admin, parties.payer);
    assert_ne!(parties.payer, parties.payee);
    assert_ne!(parties.payee, parties.arbiter);
}

#[test]
fn test_escrow_flow_uses_shared_fixtures() {
    let test_env = setup_test_env(10_000);
    let env = &test_env.env;
    let parties = &test_env.parties;
    let token = &test_env.token;

    let client = create_contract(env);
    let escrow_id = client.create_escrow(
        &parties.payer,
        &parties.payee,
        &parties.arbiter,
        &token.address,
        &5_000,
    );

    client.fund_escrow(&escrow_id, &parties.payer);
    assert_eq!(token.client.balance(&parties.payer), 5_000);

    client.release_escrow(&escrow_id, &parties.payer);
    assert_eq!(token.client.balance(&parties.payee), 15_000);
}
