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
use solana_program::pubkey;

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

pub const PROFILE_FACTION_PROGRAM_ID: Pubkey =
    pubkey!("pFACSRuobDmvfMKq1bAzwj27t6d2GJhSCHb1VcfnRmq");
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
