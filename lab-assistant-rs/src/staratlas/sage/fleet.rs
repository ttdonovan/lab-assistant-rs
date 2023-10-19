use super::*;

use crate::Fleet;

pub fn get_user_fleet_accounts<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    user_profile_pubkey: &Pubkey,
) -> anyhow::Result<Option<Vec<(Pubkey, Fleet)>>> {
    let program = client.program(SAGE_PROGRAM_ID)?;

    // // https://docs.rs/solana-account-decoder/latest/solana_account_decoder/
    // let config = RpcProgramAccountsConfig {
    //     filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new_base58_encoded(
    //         41,
    //         &user_profile_pubkey.to_bytes(),
    //     ))]),
    //     account_config: RpcAccountInfoConfig {
    //         encoding: Some(UiAccountEncoding::Base64),
    //         commitment: Some(CommitmentConfig::confirmed()),
    //         ..Default::default()
    //     },
    //     with_context: Some(false),
    // };

    // let user_fleet_accounts = program
    // .rpc()
    // .get_program_accounts_with_config(&program.id(), config)?;

    // if user_fleet_accounts.is_empty() {
    //     Ok(None)
    // } else {
    //     Ok(Some(user_fleet_accounts))
    // }

    let accounts = program.accounts::<Fleet>(vec![RpcFilterType::Memcmp(
        Memcmp::new_base58_encoded(41, &user_profile_pubkey.to_bytes()),
    )])?;

    // TODO: able to get fleet accounts but how to get the "remaining data"?

    if accounts.is_empty() {
        Ok(None)
    } else {
        Ok(Some(accounts))
    }
}
