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

// [start - cargo] - https://solscan.io/account/Cargo8a1e6NkGyrjy4BQEW4ASGKs9KSyDyUrXMfpJoiH#anchorProgramIDL
// Accounts
#[account]
#[derive(Debug)]
pub struct CargoStatsDefinition {
    version: u8,
    authority: Pubkey,
    default_cargo_type: Pubkey,
    stats_count: u16,
    seq_id: u16,
}

#[account]
#[derive(Debug)]
pub struct CargoType {
    version: u8,
    stats_definition: Pubkey,
    mint: Pubkey,
    bump: u8,
    stats_count: u16,
    seq_id: u16,
}
// [end - cargo]

// [start - sage] - https://solscan.io/account/SAGEqqFewepDHH6hMDcmWy7yjHPpyKLDnRXKb3Ki8e6#epZhSDVBrjgL72hW5ED6xsufx4qi5zQZLiVUtT6q4Ri
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

#[account]
#[derive(Debug)]
pub struct Starbase {
    version: u8,
    game_id: Pubkey,
    pub sector: [i64; 2],
    crafting_facility: Pubkey,
    pub name: [u8; 64],
    sub_coordinates: [i64; 2],
    pub faction: u8,
    bump: u8,
    seq_id: u16,
    state: u8,
    level: u8,
    hp: u64,
    sp: u64,
    sector_ring_available: u8,
    upgrade_state: i64,
    built_destroyed_timestamp: i64,
    num_upgrading_fleets: u64,
    total_upgrade_rate: u64,
    received_upgrade_materials: u64,
    required_upgrade_materials: u64,
    last_updated_rate_timestamp: i64,
}

#[account]
#[derive(Debug)]
pub struct SurveyDataUnitTracker {
    version: u8,
    game_id: Pubkey,
    mint: Pubkey,
    signer: Pubkey,
    signer_bump: u8,
    survy_data_units_by_seconds: [u32; 60],
    limit: u32,
    scan_cooldown: u16,
    probability: u16,
    max: u16,
    num_sectors: u16,
    last_update: i64,
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
// [end - sage]

pub mod staratlas;
pub use crate::staratlas::cargo::CARGO_PROGRAM_ID;
pub use crate::staratlas::player_profile::PROFILE_PROGRAM_ID;
pub use crate::staratlas::profile_faction::PROFILE_FACTION_PROGRAM_ID;
pub use crate::staratlas::sage::SAGE_PROGRAM_ID;

#[derive(Debug)]
pub struct SagePlayerProfileGameState {
    pub game_id: Pubkey,
    pub game_account: Game,
    pub game_state_account: GameState,
    pub sdu_tracker_accounts: Vec<(Pubkey, SurveyDataUnitTracker)>, // TODO: one or many?
    pub cargo_stats_definition_accounts: Vec<(Pubkey, CargoStatsDefinition)>, // TODO: one or many?
    pub cargo_type_accounts: Vec<(Pubkey, CargoType)>,
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

    // SDU Tracker (Accounts)
    let sdu_tracker_accounts = staratlas::sage::get_sdu_tracker_accounts(client)?.unwrap_or(vec![]);

    // TODO: verify that SDU tracker account(s) is one or many
    assert!(
        !sdu_tracker_accounts.is_empty(),
        "must be at least one SDU Tracker account"
    );

    let cargo_stats_definition_accounts =
        staratlas::cargo::get_cargo_stats_definition_accounts(client)?.unwrap_or(vec![]);

    // TODO: verify that CargoStatsDefinition account(s) is one or many
    assert!(
        !cargo_stats_definition_accounts.is_empty(),
        "must be at least one CargoStatsDefinition account"
    );

    // Cargo Type (Accounts)
    let (_pubkey, cargo_stats_defintion_account) = &cargo_stats_definition_accounts
        .first()
        .expect("cannot find Account");
    let cargo_type_accounts =
        staratlas::cargo::get_cargo_type_accounts(client, cargo_stats_defintion_account.seq_id)?
            .unwrap_or(vec![]);
    // dbg!(&cargo_type_accounts);

    // User Profiles (Account)
    let user_profiles = staratlas::player_profile::get_user_profile_accounts(client, user_pubkey)?
        .unwrap_or(vec![]);

    // TODO: verify that it's expected to only have "1" user profile account
    assert_eq!(user_profiles.len(), 1, "must be only one 'Profile' Account");
    // dbg!(&user_profiles);

    let (user_profile_pubkey, user_profile_account) =
        &user_profiles.first().expect("cannot find Account");
    // dbg!(&user_profile_account);

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
        sdu_tracker_accounts,
        cargo_stats_definition_accounts,
        cargo_type_accounts,
        user_profile_pubkey: *user_profile_pubkey,
        user_profile_account: user_profile_account.to_owned(),
        profile_faction_pubkey: *profile_faction_pubkey,
        profile_faction_account: profile_faction_account.to_owned(),
        user_fleets,
    };

    Ok(state)
}
