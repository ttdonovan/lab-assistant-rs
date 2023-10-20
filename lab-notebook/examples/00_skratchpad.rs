use anchor_client::{
    anchor_lang::prelude::Pubkey, solana_sdk::signer::null_signer::NullSigner, Client, Cluster,
};

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

    // Player Pubkey goes here ("player-pubkey-here")...
    let user_pubkey = Pubkey::from_str("player-pubkey-here").expect("invalid user pubkey");
    let game = labs::init_sage_labs_game(&client, &user_pubkey)?;
    // dbg!(&game);

    // dbg!(&game.cargo_stats_definition_accounts);
    // dbg!(&game.cargo_type_accounts);

    for (fleet_pubkey, fleet_account) in game.user_fleets.iter() {
        let fleet_label = str::from_utf8(&fleet_account.fleet_label)?;

        dbg!(format!("Fleet: {}", fleet_label));
        dbg!(&fleet_pubkey);
        // dbg!(&fleet_account);

        // FIXME: how to get "remaining data" to check fleet state? sorta works but do not understand bytes structur in `get_fleet_state`
        let (fleet_pubkey2, _) = labs::staratlas::sage::fleet_find_address(
            &game.game_id,
            &game.user_profile_pubkey,
            &fleet_account.fleet_label,
        );
        assert_eq!(fleet_pubkey, &fleet_pubkey2, "fleet pubkey mismatch");

        let (fleet_account2, fleet_state) =
            labs::staratlas::sage::get_fleet_state(&client, &fleet_pubkey2)?;

        // FleetState appears to be correct...
        dbg!(&fleet_state);

        let fleet_label2 = str::from_utf8(&fleet_account2.fleet_label);
        dbg!(&fleet_label2);

        // FIXME: these values do not align almost list &feet_account2.fleet_label is off by 8-bytes
        //  thread 'main' panicked at lab-notebook\examples\00_skratchpad.rs:56:9:
        //  assertion `left == right` failed: fleet account mismatch (fleet_label)
        //  left: [65, 85, 84, 79, 45, 83, 68, 85, 45, 35, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        //  right: [32, 104, 71, 151, 50, 88, 206, 199, 65, 85, 84, 79, 45, 83, 68, 85, 45, 35, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        // assert_eq!(
        //     &fleet_account.fleet_label, &fleet_account2.fleet_label,
        //     "fleet account mismatch (fleet_label)"
        // );

        // process didn't exit successfully beasue of `aasert_eq!` above this hasn't been executed/checked
        // assert_eq!(
        //     &fleet_account.owner_profile, &fleet_account2.owner_profile,
        //     "fleet account mismatch (owner_profile)"
        // );

        // break;
    }

    // let (pubkey, _) = labs::staratlas::sage::starbase_find_address(&game.game_id, (40, 30));
    // dbg!(&pubkey);

    // if let Some(starbase_account) =
    //     labs::staratlas::sage::get_starbase_from_coords(&client, &game.game_id, 40, 30)?
    // {
    //     dbg!(&starbase_account.faction);
    //     dbg!(&starbase_account.sector);
    //     let starbase_name = str::from_utf8(&starbase_account.name)?;
    //     dbg!(starbase_name);
    // }

    Ok(())
}
