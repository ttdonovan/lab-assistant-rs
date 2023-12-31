use anchor_lang::AnchorDeserialize;

use super::*;

use crate::{Fleet, Idle, MineAsteroid, MoveSubwarp, MoveWarp, Respawn, StarbaseLoadingBay};

const AMMO_MINT: Pubkey = pubkey!("ammoK8AkX2wnebQb35cDAZtTkvsXQbi82cGeTnUvvfK");
const FOOD_MINT: Pubkey = pubkey!("foodQJAztMzX1DKpLaiounNe2BDMds5RNuPC6jsNrDG");
const FUEL_MINT: Pubkey = pubkey!("fueL3hBZjLLLJHiFH9cqZoozTG3XQZ53diwFPwbzNim");
const TOOL_MINT: Pubkey = pubkey!("tooLsNYLiVqzg8o4m3L2Uetbn62mvMWRqkog6PQeYKL");
const SDU_MINT: Pubkey = pubkey!("SDUsgfSZaDhhZ76U3ZgvtFiXsfnHbf2VrzYxjBZ5YbM");

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

pub fn fleet_find_address(
    game_id: &Pubkey,
    player_profile: &Pubkey,
    fleet_label: &[u8; 32],
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"Fleet",
            &game_id.to_bytes(),
            &player_profile.to_bytes(),
            fleet_label,
        ],
        &SAGE_PROGRAM_ID,
    )
}

pub fn fleet_repair_kit_token_address(fleet: &Fleet) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            &fleet.cargo_hold.to_bytes(),
            &TOKEN_PROGRAM_ID.to_bytes(),
            &TOOL_MINT.to_bytes(),
        ],
        &ASSOCIATED_TOKEN_PROGRAM_ID,
    )
}

pub fn fleet_sdu_token_address(fleet: &Fleet) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            &fleet.cargo_hold.to_bytes(),
            &TOKEN_PROGRAM_ID.to_bytes(),
            &SDU_MINT.to_bytes(),
        ],
        &ASSOCIATED_TOKEN_PROGRAM_ID,
    )
}

pub fn fleet_fuel_token_address(fleet: &Fleet) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            &fleet.cargo_hold.to_bytes(),
            &TOKEN_PROGRAM_ID.to_bytes(),
            &FUEL_MINT.to_bytes(),
        ],
        &ASSOCIATED_TOKEN_PROGRAM_ID,
    )
}

#[derive(Debug, Clone)]
pub enum FleetState {
    StarbaseLoadingBay(StarbaseLoadingBay),
    Idle(Idle),
    MineAsteroid(MineAsteroid),
    MoveWarp(MoveWarp),
    MoveSubwarp(MoveSubwarp),
    Respawn(Respawn),
}

pub fn get_fleet_state<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    fleet: &Pubkey,
) -> anyhow::Result<(Fleet, FleetState)> {
    let program = client.program(SAGE_PROGRAM_ID)?;

    // get account data from solana network result is Vec<u8>
    let account_data = program.rpc().get_account_data(fleet)?;
    // dbg!(&account_data.len()); // &account_data.len() = 503

    // create a mut slice of account data
    let mut data_slice = account_data.as_slice();
    // dbg!(&data_slice.len()); // &data_slice.len() = 503

    // FIXME: this is not the correct fleet account when deserialized...
    //   but oddly the remaing data is correct... in most casts of testing "Idle" fleet state
    //
    // Values for MIN_DATA_SIZE copied from typscript code:
    //
    // const MOVEMENT_STATS_MIN_DATA_SIZE: usize = 28; // FIXME(?): why 28 vs 24?
    // const CARGO_STATS_MIN_DATA_SIZE: usize = 28;
    // const MISC_STATS_MIN_DATA_SIZE: usize = 12; // FIXME(?): found in typescript comment `u64 + (2 * u16) = 8 + (2 * 2) + 4 = 16``
    // const SHIP_STATS_MIN_DATA_SIZE: usize =
    //     MOVEMENT_STATS_MIN_DATA_SIZE + CARGO_STATS_MIN_DATA_SIZE + MISC_STATS_MIN_DATA_SIZE;
    // const FLEET_MIN_DATA_SIZE: usize = 269 + SHIP_STATS_MIN_DATA_SIZE;

    // dbg!(std::mem::size_of::<crate::MovementStats>()); // std::mem::size_of::<crate::MovementStats>() = 24
    // dbg!(std::mem::size_of::<crate::CargoStats>()); // std::mem::size_of::<crate::CargoStats>() = 28
    // dbg!(std::mem::size_of::<crate::MiscStats>()); // std::mem::size_of::<crate::MiscStats>() = 16 (idl MiscStats and MiscStatsUnpacked)
    // dbg!(std::mem::size_of::<crate::ShipStats>()); // std::mem::size_of::<crate::ShipStats>() = 72 (idl ShipStats and ShipStatsUnpacked)
    // dbg!(std::mem::size_of::<crate::Fleet>()); // std::mem::size_of::<crate::Fleet>() = 416

    let fleet_account = Fleet::deserialize(&mut data_slice)?;

    // return the fleet label from the account for debugging
    let fleet_label = std::str::from_utf8(&fleet_account.fleet_label);
    // dbg!(&fleet_label); // Uta8Error

    // FIXME: no idea what these 8-bytes represent here...
    let unkown_slice = data_slice.get(..8).unwrap();
    // dbg!(&unkown_slice); // mostly something like this... [0, 0, 0, 0, 0, 0, 0, 255] (sometimes 254 and 253, depending on fleet &Pubkey)
    // dbg!(&unkown_slice.len()); // &unkown_slice.len() = 8

    // believe this is the "remaining data" to determine a fleet state
    let remaining_data = data_slice.get(8..).unwrap();
    // dbg!(&remaining_data);
    // dbg!(&remaining_data.len()); // &remaining_data.len() = 89

    let discrimiator = remaining_data[0];
    let mut remaining_data = remaining_data.get(1..).unwrap();

    // dbg!(&discrimiator); // mostly 1 for Idle fleet state
    // dbg!(&remaining_data.len()); // &remaining_data.len() = 88

    let fleet_state = match discrimiator {
        0 => {
            let starbase_loading_bay = StarbaseLoadingBay::deserialize(&mut remaining_data)?;
            FleetState::StarbaseLoadingBay(starbase_loading_bay)
        }
        1 => {
            let idle = Idle::deserialize(&mut remaining_data)?;
            FleetState::Idle(idle)
        }
        2 => {
            let mine_astriod = MineAsteroid::deserialize(&mut remaining_data)?;
            FleetState::MineAsteroid(mine_astriod)
        }
        3 => {
            let move_ware = MoveWarp::deserialize(&mut remaining_data)?;
            FleetState::MoveWarp(move_ware)
        }
        4 => {
            let move_subwarp = MoveSubwarp::deserialize(&mut remaining_data)?;
            FleetState::MoveSubwarp(move_subwarp)
        }
        5 => {
            let respawn = Respawn::deserialize(&mut remaining_data)?;
            FleetState::Respawn(respawn)
        }
        _ => {
            panic!("Unknown fleet state");
        }
    };

    Ok((fleet_account, fleet_state))
}

//  /**
//    * Generic asteroid mining handler
//    * @param program - SAGE program
//    * @param cargoProgram - cargo program
//    * @param profileFaction - the faction of the profile that owns the fleet that is mining
//    * @param fleet - the fleet
//    * @param starbase - the Starbase
//    * @param mineItem - the mine item
//    * @param resource - the resource
//    * @param planet - the planet
//    * @param cargoHold - the fleet cargo hold cargo pod
//    * @param ammoBank - the fleet ammo bank cargo pod
//    * @param foodCargoType - the food cargo type
//    * @param ammoCargoType - the ammo cargo type
//    * @param resourceCargoType - the cargo type for the resource being mined
//    * @param cargoStatsDefinition - the cargo stats definition
//    * @param gameState - the game state. No longer used, maintained for backwards compatibility
//    * @param gameId - the SAGE game id
//    * @param foodTokenFrom - the source token account for food
//    * @param ammoTokenFrom - the source token account for ammo
//    * @param resourceTokenFrom - the source token account for the resource
//    * @param resourceTokenTo - the destination token account for the resource
//    * @param foodMint - the food token mint
//    * @param ammoMint - the ammo token mint
//    * @returns InstructionReturn
// */
// static asteroidMiningHandler(
//     program: SageIDLProgram,
//     cargoProgram: CargoIDLProgram,
//     profileFaction: PublicKey,
//     fleet: PublicKey,
//     starbase: PublicKey,
//     mineItem: PublicKey,
//     resource: PublicKey,
//     planet: PublicKey,
//     cargoHold: PublicKey,
//     ammoBank: PublicKey,
//     foodCargoType: PublicKey,
//     ammoCargoType: PublicKey,
//     resourceCargoType: PublicKey,
//     cargoStatsDefinition: PublicKey,
//     gameState: PublicKey,
//     gameId: PublicKey,
//     foodTokenFrom: PublicKey,
//     ammoTokenFrom: PublicKey,
//     resourceTokenFrom: PublicKey,
//     resourceTokenTo: PublicKey,
//     foodMint: PublicKey,
//     ammoMint: PublicKey,
//   ): InstructionReturn {
//     return this.fleetStateHandler(program, fleet, [
//       { pubkey: profileFaction, isSigner: false, isWritable: false },
//       { pubkey: cargoHold, isSigner: false, isWritable: true },
//       { pubkey: ammoBank, isSigner: false, isWritable: true },
//       { pubkey: mineItem, isSigner: false, isWritable: false },
//       { pubkey: resource, isSigner: false, isWritable: true },
//       { pubkey: planet, isSigner: false, isWritable: true },
//       { pubkey: starbase, isSigner: false, isWritable: false },
//       { pubkey: foodTokenFrom, isSigner: false, isWritable: true },
//       { pubkey: ammoTokenFrom, isSigner: false, isWritable: true },
//       { pubkey: resourceTokenFrom, isSigner: false, isWritable: true },
//       { pubkey: resourceTokenTo, isSigner: false, isWritable: true },
//       { pubkey: foodMint, isSigner: false, isWritable: true },
//       { pubkey: ammoMint, isSigner: false, isWritable: true },
//       { pubkey: foodCargoType, isSigner: false, isWritable: false },
//       { pubkey: ammoCargoType, isSigner: false, isWritable: false },
//       { pubkey: resourceCargoType, isSigner: false, isWritable: false },
//       {
//         pubkey: cargoStatsDefinition,
//         isSigner: false,
//         isWritable: false,
//       },
//       { pubkey: gameId, isSigner: false, isWritable: false },
//       {
//         pubkey: cargoProgram.programId,
//         isSigner: false,
//         isWritable: false,
//       },
//       { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
//     ]);
//   }

// /**
//    * Stop mining an asteroid
//    * @param program - SAGE program
//    * @param cargoProgram - cargo program
//    * @param key - the key authorized to run this instruction
//    * @param playerProfile - the profile with the required permissions for the instruction
//    * @param profileFaction - the profile's faction
//    * @param fleet - the fleet
//    * @param resource - the resource
//    * @param planet - the planet
//    * @param fuelTank - the fleet's fuel tank cargo pod
//    * @param fuelCargoType - the fuel cargo type
//    * @param cargoStatsDefinition - the cargo stats definition
//    * @param gameState - the game state
//    * @param gameId - the SAGE game id
//    * @param fuelTokenFrom - the source token account for fuel
//    * @param fuelMint - the fuel token mint
//    * @param input - the instruction input params
//    * @returns InstructionReturn
//    */
//   static stopMiningAsteroid(
//     program: SageIDLProgram,
//     cargoProgram: CargoIDLProgram,
//     key: AsyncSigner,
//     playerProfile: PublicKey,
//     profileFaction: PublicKey,
//     fleet: PublicKey,
//     resource: PublicKey,
//     planet: PublicKey,
//     fuelTank: PublicKey,
//     fuelCargoType: PublicKey,
//     cargoStatsDefinition: PublicKey,
//     gameState: PublicKey,
//     gameId: PublicKey,
//     fuelTokenFrom: PublicKey,
//     fuelMint: PublicKey,
//     input: StopMiningAsteroidInput,
//   ): InstructionReturn {
//     return async () => [
//       {
//         instruction: await program.methods
//           .stopMiningAsteroid(input)
//           .accountsStrict({
//             gameAccountsFleetAndOwner: {
//               gameFleetAndOwner: {
//                 fleetAndOwner: {
//                   fleet,
//                   owningProfile: playerProfile,
//                   owningProfileFaction: profileFaction,
//                   key: key.publicKey(),
//                 },
//                 gameId,
//               },
//               gameState,
//             },
//             resource,
//             planet,
//             fuelTank,
//             cargoType: fuelCargoType,
//             cargoStatsDefinition,
//             tokenFrom: fuelTokenFrom,
//             tokenMint: fuelMint,
//             cargoProgram: cargoProgram.programId,
//             tokenProgram: TOKEN_PROGRAM_ID,
//           })
//           .instruction(),
//         signers: [key],
//       },
//     ];
//   }}