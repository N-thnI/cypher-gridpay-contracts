#![cfg(test)]

use soroban_sdk::{token::StellarAssetClient, testutils::Address as _, Address, Env};

/// Registers a Stellar asset and mints `1_000_000_000` of it to `holder`
/// (normally the refund contract), so refund payouts can actually settle.
pub(crate) fn funded_token(env: &Env, holder: &Address) -> Address {
    let issuer = Address::generate(env);
    let token = env.register_stellar_asset_contract_v2(issuer).address();
    StellarAssetClient::new(env, &token)
        .mock_all_auths()
        .mint(holder, &1_000_000_000);
    token
}
