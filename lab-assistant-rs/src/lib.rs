use anchor_client::{
    anchor_lang::prelude::Pubkey,
    solana_client::{
        rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
        rpc_filter::{Memcmp, RpcFilterType},
    },
    solana_sdk::{account::Account, commitment_config::CommitmentConfig, signature::Signer},
    Client,
};
use solana_program::pubkey;

use std::ops::Deref;

pub const PROFILE_PROGRAM_ID: Pubkey = pubkey!("pprofELXjL5Kck7Jn5hCpwAL82DpTkSYBENzahVtbc9");
pub const SAGE_PROGRAM_ID: Pubkey = pubkey!("SAGEqqFewepDHH6hMDcmWy7yjHPpyKLDnRXKb3Ki8e6");

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
