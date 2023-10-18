use anchor_client::{
    anchor_lang::prelude::Pubkey,
    solana_client::{
        rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
        rpc_filter::{Memcmp, RpcFilterType},
    },
    solana_sdk::{
        account::Account, commitment_config::CommitmentConfig, signature::Signer,
        signer::null_signer::NullSigner,
    },
    Client, Cluster,
};
use bs58::encode;

use std::ops::Deref;
use std::str::FromStr;

use lab_assistant::{get_user_profile_accounts, SAGE_PROGRAM_ID};

fn get_starbase_from_coords<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    x: i64,
    y: i64,
) -> anyhow::Result<Option<Account>> {
    let program = client.program(SAGE_PROGRAM_ID)?;

    let x_bytes = x.to_le_bytes();
    let x58 = encode(&x_bytes).into_string();
    let y_bytes = y.to_le_bytes();
    let y58 = encode(&y_bytes).into_string();

    dbg!(&x_bytes);
    dbg!(&x58);
    dbg!(&y_bytes);
    dbg!(&y58);

    let config = RpcProgramAccountsConfig {
        filters: Some(vec![
            RpcFilterType::Memcmp(Memcmp::new_base58_encoded(41, &x58.into_bytes())),
            RpcFilterType::Memcmp(Memcmp::new_base58_encoded(49, &y58.into_bytes())),
        ]),
        account_config: RpcAccountInfoConfig {
            commitment: Some(CommitmentConfig::confirmed()),
            ..Default::default()
        },
        with_context: Some(false),
    };

    let starbases = program
        .rpc()
        .get_program_accounts_with_config(&program.id(), config)?;

    dbg!(&starbases);

    Ok(None)
}

// let starbase_data = &starbases[0].account.data;
// dbg!(&starbase_data);
// let starbase = Account::try_from_slice_unchecked(starbase_data)?;

// Ok(starbase)

const RPC_URL: &str = "https://solana-api.syndica.io/access-token/WPoEqWQ2auQQY1zHRNGJyRBkvfOLqw58FqYucdYtmy8q9Z84MBWwqtfVf8jKhcFh/rpc";

fn main() -> anyhow::Result<()> {
    let wallet = Pubkey::new_unique();
    let payer = NullSigner::new(&wallet);
    let client = Client::new(
        Cluster::Custom(RPC_URL.to_string(), RPC_URL.to_string()),
        &payer,
    );

    let user_pubkey = Pubkey::from_str("player-pubkey-here")?;
    let user_profiles = get_user_profile_accounts(&client, &user_pubkey)?;
    dbg!(&user_profiles);

    let starbase = get_starbase_from_coords(&client, 40, 30)?;
    dbg!(&starbase);

    Ok(())
}
