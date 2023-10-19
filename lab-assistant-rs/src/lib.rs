use anchor_client::{
    solana_client::{
        rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
        rpc_filter::{Memcmp, RpcFilterType},
    },
    solana_sdk::{account::Account, commitment_config::CommitmentConfig, signature::Signer},
    Client,
};
use anchor_lang::prelude::*;
// use borsh::BorshDeserialize;
// use solana_account_decoder::UiAccountEncoding;

use std::ops::Deref;

declare_id!("SAGEqqFewepDHH6hMDcmWy7yjHPpyKLDnRXKb3Ki8e6");

// [start] - https://solscan.io/account/SAGEqqFewepDHH6hMDcmWy7yjHPpyKLDnRXKb3Ki8e6#epZhSDVBrjgL72hW5ED6xsufx4qi5zQZLiVUtT6q4Ri
// Accounts
#[account]
#[derive(Debug)]
pub struct Fleet {
    version: u8,
    game_id: Pubkey,
    owner_profile: Pubkey,
    fleet_ships: Pubkey,
    sub_profile: OptionalNonSystemPubkey,
    sub_profile_invalidator: Pubkey,
    pub fleet_label: [u8; 32],
    ship_counts: ShipCounts,
    warp_cooldown_expires_at: i64,
    scan_cooldown_expires_at: i64,
    stats: ShipStats,
    cargo_hold: Pubkey,
    fuel_tank: Pubkey,
    ammo_bank: Pubkey,
    update_id: u64,
    bump: u8,
}

#[account]
#[derive(Debug)]
pub struct Game {
    version: u8,
    pub update_id: u64,
    profile: Pubkey,
    game_state: Pubkey,
}

#[account]
#[derive(Debug)]
pub struct GameState {
    version: u8,
    update_id: u64,
    game_id: Pubkey,
}

// Types
#[derive(Debug, Clone, AnchorSerialize, borsh::BorshDeserialize)]
pub struct CargoStats {
    cargo_capacity: u32,
    fuel_capacity: u32,
    ammo_capacity: u32,
    ammo_consumption_rate: u32,
    food_consumption_rate: u32,
    mining_rate: u32,
    upgrade_rate: u32,
}

#[derive(Debug, Clone, AnchorSerialize, borsh::BorshDeserialize)]
pub struct OptionalNonSystemPubkey {
    key: Pubkey,
}

#[derive(Debug, Clone, AnchorSerialize, borsh::BorshDeserialize)]
pub struct MiscStats {
    crew: u64,
    respawn_time: u16,
    scan_cool_down: u16,
    scan_repair_kit_amount: u32,
}

#[derive(Debug, Clone, AnchorSerialize, borsh::BorshDeserialize)]
pub struct MovementStats {
    subwarp_speed: u32,
    warp_speed: u32,
    max_warp_distance: u16,
    warp_cool_down: u16,
    subwarp_fuel_consumption_rate: u32,
    warp_fuel_consumption_rate: u32,
    planet_exit_fuel_amount: u32,
}

#[derive(Debug, Clone, AnchorSerialize, borsh::BorshDeserialize)]
pub struct ShipCounts {
    total: u32,
    updated: u32,
    xx_small: u16,
    x_small: u16,
    small: u16,
    medium: u16,
    large: u16,
    capital: u16,
    commander: u16,
    titan: u16,
}

#[derive(Debug, Clone, AnchorSerialize, borsh::BorshDeserialize)]
pub struct ShipStats {
    movement_stats: MovementStats,
    cargo_stats: CargoStats,
    misc_stats: MiscStats,
}
// [end]

mod staratlas;
pub use crate::staratlas::player_profile::PROFILE_PROGRAM_ID;
pub use crate::staratlas::profile_faction::PROFILE_FACTION_PROGRAM_ID;
pub use crate::staratlas::sage::SAGE_PROGRAM_ID;

#[derive(Debug)]
pub struct SagePlayerProfileGameState {
    pub game_id: Pubkey,
    pub game_account: Game,
    pub game_state_account: GameState,
    pub user_profile_pubkey: Pubkey,
    pub user_profile_account: Account,
    pub profile_faction_pubkey: Pubkey,
    pub profile_faction_account: Account,
    pub user_fleets: Vec<(Pubkey, Fleet)>,
}

pub fn init_sage_labs_game<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    user_pubkey: &Pubkey,
) -> anyhow::Result<SagePlayerProfileGameState> {
    // Game (Account)
    let game_accounts = staratlas::sage::get_game_accounts(client)?.unwrap_or(vec![]);
    assert_eq!(game_accounts.len(), 1, "must be only one Game");

    let (game_id, game_account) = &game_accounts.first().expect("cannot find Game account");
    // dbg!(&game_account);

    // Game State (Account)
    let game_state_account =
        staratlas::sage::get_game_state_account(client, &game_account.game_state)?
            .expect("cannot find GameState account");
    //dbg!(&game_state_account);

    // User Profiles (Account)
    let user_profiles = staratlas::player_profile::get_user_profile_accounts(client, user_pubkey)?
        .unwrap_or(vec![]);

    // TODO: verify that it's expected to only have "1" user profile account
    assert_eq!(user_profiles.len(), 1, "must be only one 'Profile' Account");
    // dbg!(&user_profiles);

    let (user_profile_pubkey, user_profile_account) =
        &user_profiles.first().expect("cannot find Account");
    dbg!(&user_profile_account);

    // User Profile Factions (Account)
    let profile_factions =
        staratlas::profile_faction::get_profile_faction_accounts(client, user_profile_pubkey)?
            .unwrap_or(vec![]);

    // TODO: verify that it's expected to only have "1" profile faction account
    assert_eq!(
        profile_factions.len(),
        1,
        "must be only one 'ProfileFactionAccount' Account"
    );

    let (profile_faction_pubkey, profile_faction_account) =
        &profile_factions.first().expect("cannot find Account");
    // dbg!(&profile_faction_account);

    // User Fleet (Accounts)
    let user_fleets =
        staratlas::sage::get_user_fleet_accounts(client, user_profile_pubkey)?.unwrap_or(vec![]);

    let state = SagePlayerProfileGameState {
        game_id: *game_id,
        game_account: game_account.to_owned(),
        game_state_account: game_state_account.to_owned(),
        user_profile_pubkey: *user_profile_pubkey,
        user_profile_account: user_profile_account.to_owned(),
        profile_faction_pubkey: *profile_faction_pubkey,
        profile_faction_account: profile_faction_account.to_owned(),
        user_fleets,
    };

    Ok(state)
}
