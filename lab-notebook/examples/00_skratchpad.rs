use anchor_client::{
    anchor_lang::{prelude::Pubkey, AnchorDeserialize},
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
use std::str::{self, FromStr};

use lab_assistant as labs;

const RPC_URL: &str = "https://solana-api.syndica.io/access-token/WPoEqWQ2auQQY1zHRNGJyRBkvfOLqw58FqYucdYtmy8q9Z84MBWwqtfVf8jKhcFh/rpc";

fn main() -> anyhow::Result<()> {
    let wallet = Pubkey::new_unique();
    let payer = NullSigner::new(&wallet);
    let client = Client::new(
        Cluster::Custom(RPC_URL.to_string(), RPC_URL.to_string()),
        &payer,
    );

    let user_pubkey = Pubkey::from_str("player-pubkey-here")?;
    let game = labs::init_sage_labs_game(&client, &user_pubkey)?;
    // dbg!(&game);

    // dbg!(&game.cargo_stats_definition_accounts);
    // dbg!(&game.cargo_type_accounts);

    // for (fleet_pubkey, fleet_account) in game.user_fleets.iter() {
    //     let fleet_label = str::from_utf8(&fleet_account.fleet_label)?;

    //     dbg!(format!("Fleet: {}", fleet_label));
    //     dbg!(&fleet_pubkey);
    //     dbg!(&fleet_account);

    //     // TODO: how to get "remaining data" to check fleet state?

    //     break;
    // }

    let (pubkey, _) = labs::staratlas::sage::starbase_find_address(&game.game_id, (40, 30));
    dbg!(&pubkey);

    if let Some(starbase_account) =
        labs::staratlas::sage::get_starbase_from_coords(&client, &game.game_id, 40, 30)?
    {
        dbg!(&starbase_account.faction);
        dbg!(&starbase_account.sector);
        let starbase_name = str::from_utf8(&starbase_account.name)?;
        dbg!(starbase_name);
    }

    Ok(())
}
