use super::*;

pub fn get_user_profile_accounts<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    user_pubkey: &Pubkey,
) -> anyhow::Result<Option<Vec<(Pubkey, Account)>>> {
    let program = client.program(PROFILE_PROGRAM_ID)?;

    let config = RpcProgramAccountsConfig {
        filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new_base58_encoded(
            30,
            &user_pubkey.to_bytes(),
        ))]),
        account_config: RpcAccountInfoConfig {
            commitment: Some(CommitmentConfig::confirmed()),
            ..Default::default()
        },
        with_context: Some(false),
    };

    let user_profile_accounts = program
        .rpc()
        .get_program_accounts_with_config(&program.id(), config)?;

    if user_profile_accounts.is_empty() {
        Ok(None)
    } else {
        Ok(Some(user_profile_accounts))
    }
}
