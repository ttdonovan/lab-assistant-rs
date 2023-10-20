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

use std::fmt;
use std::ops::Deref;
use std::str;

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
// #[safe_zero_copy_account]
#[account] // #[account(zero_copy)]
#[derive(Debug)] // borsh::BorshDeserialize, borsh::BorshSerialize
pub struct Fleet {
    /// The data version of this account.
    pub version: u8,
    /// The game id this belongs to.
    pub game_id: Pubkey,
    /// The owner's profile.
    pub owner_profile: Pubkey,
    /// Fleet Ships Key
    pub fleet_ships: Pubkey,
    /// The fleet's sub-authority.
    /// If [`Some`] will have the exclusive ability to interact with this fleet.
    pub sub_profile: OptionalNonSystemPubkey,
    /// The authority for revoking a sun-authority.
    pub sub_profile_invalidator: Pubkey,
    /// The label or name of the fleet.
    pub fleet_label: [u8; 32],
    /// The number of ships in the fleet.
    pub ship_counts: ShipCounts,
    /// The time at which the warp cooldown expires
    pub warp_cooldown_expires_at: i64,
    /// The time at which the scan cooldown expires
    pub scan_cooldown_expires_at: i64,
    /// The fleet's stats.
    pub stats: ShipStats, // FIXME: should this code be using `ShipStatsUnpacked` and not `ShipStats`?.
    /// The Cargo pod representing the fleet's cargo hold
    pub cargo_hold: Pubkey,
    /// The Cargo pod representing the fleet's fuel tank
    pub fuel_tank: Pubkey,
    /// The Cargo pod representing the fleet's ammo bank
    pub ammo_bank: Pubkey,
    /// The update id for the `Fleet`
    pub update_id: u64,
    /// The fleet's bump.
    pub bump: u8,
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
#[zero_copy]
#[derive(Debug, borsh::BorshDeserialize, borsh::BorshSerialize)]
pub struct OptionalNonSystemPubkey {
    key: Pubkey,
}

#[derive(Debug, Clone, borsh::BorshDeserialize)]
pub struct StarbaseLoadingBay {
    key: Pubkey,
    last_update: i64,
}

#[derive(Debug, Clone, borsh::BorshDeserialize)]
pub struct Idle {
    sector: [i64; 2],
}

#[derive(Debug, Clone, borsh::BorshDeserialize)]
pub struct MineAsteroid {
    asteroid: Pubkey,
    resource: Pubkey,
    start: i64,
    end: i64,
    last_update: i64,
}

#[derive(Debug, Clone, borsh::BorshDeserialize)]
pub struct MoveWarp {
    from_sector: [i64; 2],
    to_sector: [i64; 2],
    warp_start: i64,
    warp_finish: i64,
}

#[derive(Debug, Clone, borsh::BorshDeserialize)]
pub struct MoveSubwarp {
    from_sector: [i64; 2],
    to_sector: [i64; 2],
    current_sector: [i64; 2],
    depature_time: i64,
    arrival_time: i64,
    fuel_expenditure: u64,
    last_update: u64,
}

#[derive(Debug, Clone, borsh::BorshDeserialize)]
pub struct Respawn {
    sector: [i64; 2],
    start: i64,
}

// #[repr(C, packed)]
// #[safe_zero_copy]
#[zero_copy]
#[derive(Debug, borsh::BorshDeserialize, borsh::BorshSerialize)]
pub struct ShipCounts {
    /// The total number of ships in the fleet.
    pub total: u32,
    /// Used when updating a fleet.
    /// Value is 0 when fleet update is in progress
    pub updated: u32,
    /// The number of xx small ships in the fleet.
    pub xx_small: u16,
    /// The number of x small ships in the fleet.
    pub x_small: u16,
    /// The number of small ships in the fleet.
    pub small: u16,
    /// The number of medium ships in the fleet.
    pub medium: u16,
    /// The number of large ships in the fleet.
    pub large: u16,
    /// The number of capital ships in the fleet.
    pub capital: u16,
    /// The number of commander ships in the fleet.
    pub commander: u16,
    /// The number of titan ships in the fleet.
    pub titan: u16,
}

/// A ship's stats
// #[safe_zero_copy]
// #[zero_copy]
#[derive(Debug, Copy, Clone, borsh::BorshDeserialize, borsh::BorshSerialize)] // StrongTypedStruct, Unpackable
#[repr(C)] // FIXME(?): cannot do `#[repr(C, packed)]` and `std::mem::size_of::<crate::ShipStats>() = 72`
pub struct ShipStats {
    // FIXME: should this code be using `ShipStatsUnpacked` and not `ShipStats`?.
    /// Movement stats for the ship
    // #[strong_sub_struct]
    // #[packed_sub_struct]
    pub movement_stats: MovementStats,
    /// Cargo stats for the ship
    // #[strong_sub_struct]
    // #[packed_sub_struct]
    pub cargo_stats: CargoStats,
    /// Miscellaneous stats for the ship
    // #[strong_sub_struct]
    // #[packed_sub_struct]
    pub misc_stats: MiscStats,
}

/// A ship's movement stats
// #[safe_zero_copy]
#[zero_copy]
#[derive(Debug, borsh::BorshDeserialize, borsh::BorshSerialize)] // StrongTypedStruct, Unpackable
pub struct MovementStats {
    /// the amount of distance that the ship can cover in one second while sub-warping
    // #[fixed_point(1_000_000, DivUnit<AU, Second>)]
    pub subwarp_speed: u32,
    /// the amount of distance that the ship can cover in one second while warping
    // #[fixed_point(1_000_000, DivUnit<AU, Second>)]
    pub warp_speed: u32,
    /// the max distance that the ship can warp
    // #[fixed_point(100, AU)]
    pub max_warp_distance: u16,
    /// the time it takes the ship to be able to warp again after a warp
    // #[fixed_point(1, Second)]
    pub warp_cool_down: u16,
    /// the amount of fuel consumed by the ship when sub-warp moving
    // #[fixed_point(100, DivUnit<Fuel, AU>)]
    pub subwarp_fuel_consumption_rate: u32,
    /// the amount of fuel consumed by the ship when warp moving
    // #[fixed_point(100, DivUnit<Fuel, AU>)]
    pub warp_fuel_consumption_rate: u32,
    /// the amount of fuel required to exit a planet
    // #[fixed_point(1, Fuel)]
    pub planet_exit_fuel_amount: u32,
}

/// A ship's cargo stats
// #[safe_zero_copy]
#[zero_copy]
#[derive(Debug, borsh::BorshDeserialize, borsh::BorshSerialize)] // StrongTypedStruct, Unpackable
pub struct CargoStats {
    /// the capacity of the ship's cargo hold
    pub cargo_capacity: u32,
    /// the capacity of the ship's fuel tank
    // #[fixed_point(1, Fuel)]
    pub fuel_capacity: u32,
    /// the capacity of the ship's ammo bank
    // #[fixed_point(1, Ammo)]
    pub ammo_capacity: u32,
    /// the amount of ammo consumed per second by the ship when doing non-combat activities e.g. mining
    // #[fixed_point(10_000, DivUnit<Ammo, Second>)]
    pub ammo_consumption_rate: u32,
    /// the amount of food consumed per second by the ship when doing non-combat activities e.g. mining
    // #[fixed_point(10_000, DivUnit<Food, Second>)]
    pub food_consumption_rate: u32,
    /// the amount of resources that can be mined by a ship per second
    // #[fixed_point(10_000, DivUnit<Unitless, Second>)]
    pub mining_rate: u32,
    /// the amount of upgrade material that is consumed by a ship per second while upgrading a Starbase
    // #[fixed_point(100, DivUnit<Unitless, Second>)]
    pub upgrade_rate: u32,
}

/// A ship's miscellaneous stats
// #[safe_zero_copy]
#[zero_copy]
#[derive(Debug, borsh::BorshDeserialize, borsh::BorshSerialize)] // StrongTypedStruct, Unpackable
pub struct MiscStats {
    // FIXME: should this code be using `MiscStatsUnpacked` (16) and not `MiscStats` (12)?.
    /// Number of crew in the ship
    // #[fixed_point(1, Crew)]
    pub crew: u64,
    /// the time it takes the ship to respawn
    // #[fixed_point(1, Second)]
    pub respawn_time: u16,
    /// the time it takes the ship to be able to scan again after scanning
    // #[fixed_point(1, Second)]
    pub scan_cool_down: u16,
    /// the amount of food required to do a scan
    // #[fixed_point(1, Toolkit)]
    pub scan_repair_kit_amount: u32,
}
// [end - sage]

pub mod staratlas;
pub use crate::staratlas::cargo::CARGO_PROGRAM_ID;
pub use crate::staratlas::player_profile::PROFILE_PROGRAM_ID;
pub use crate::staratlas::profile_faction::PROFILE_FACTION_PROGRAM_ID;
pub use crate::staratlas::sage::{FleetState, SAGE_PROGRAM_ID};

pub struct LabAssistant<'a, C: 'a> {
    pub client: &'a Client<C>,
    pub game: SagePlayerProfileGameState,
    pub user_pubkey: Pubkey,
    pub user_fleets: Vec<UserFleet>,
}

impl<'a, C: Deref<Target = impl Signer> + Clone> LabAssistant<'a, C> {
    pub fn load_game(client: &'a Client<C>, user_pubkey: &Pubkey) -> anyhow::Result<Self> {
        let game = init_sage_labs_game(client, user_pubkey)?;

        let mut user_fleets = vec![];
        for (pubkey, fleet) in game.user_fleet_accounts.iter() {
            let (_fleet, fleet_state) = staratlas::sage::get_fleet_state(client, pubkey)?;
            // dbg!(&fleet_state);

            let fleet_label = str::from_utf8(&fleet.fleet_label)?;
            let fleet_label_trimmed = fleet_label.trim_end_matches(char::from(0));

            user_fleets.push(UserFleet {
                pubkey: pubkey.to_owned(),
                fleet: fleet.to_owned(),
                fleet_label: fleet_label_trimmed.into(),
                fleet_state: fleet_state.to_owned(),
            });
        }

        Ok(LabAssistant {
            client,
            game,
            user_pubkey: *user_pubkey,
            user_fleets,
        })
    }
}

impl<'a, C: 'a> fmt::Debug for LabAssistant<'a, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LabAssistant")
            .field("user_pubkey", &self.user_pubkey)
            .field("user_fleets", &self.user_fleets)
            .finish()
    }
}

pub struct UserFleet {
    pubkey: Pubkey,
    fleet: Fleet,
    fleet_label: String,
    fleet_state: FleetState,
}

impl fmt::Debug for UserFleet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserFleet")
            .field("pubkey", &self.pubkey)
            .field("fleet_label", &self.fleet_label)
            .field("fleet_state", &self.fleet_state)
            .finish()
    }
}

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
    pub user_fleet_accounts: Vec<(Pubkey, Fleet)>,
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
    let user_fleet_accounts =
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
        user_fleet_accounts,
    };

    Ok(state)
}
