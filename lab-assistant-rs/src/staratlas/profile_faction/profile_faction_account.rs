use super::*;

pub fn get_profile_faction_accounts<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    user_profile_pubkey: &Pubkey,
) -> anyhow::Result<Option<Vec<(Pubkey, Account)>>> {
    let program = client.program(PROFILE_FACTION_PROGRAM_ID)?;

    let config = RpcProgramAccountsConfig {
        filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new_base58_encoded(
            9,
            &user_profile_pubkey.to_bytes(),
        ))]),
        account_config: RpcAccountInfoConfig {
            commitment: Some(CommitmentConfig::confirmed()),
            ..Default::default()
        },
        with_context: Some(false),
    };

    let user_profile_faction_accounts = program
        .rpc()
        .get_program_accounts_with_config(&program.id(), config)?;

    if user_profile_faction_accounts.is_empty() {
        Ok(None)
    } else {
        Ok(Some(user_profile_faction_accounts))
    }
}
