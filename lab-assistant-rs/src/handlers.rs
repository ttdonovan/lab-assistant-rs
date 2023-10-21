use anchor_lang::prelude::Pubkey;
use solana_program::pubkey;

use crate::{commands::Mine, staratlas, Client, Fleet, FleetState, Game, Resource, Signer};

use std::ops::Deref;

const HYDROGEN_MINT: Pubkey = pubkey!("HYDR4EPHJcDPcaLYUcNCtrXUdt1PnaN4MvE655pevBYp");

fn get_resource_token_by_name(name: &str) -> Pubkey {
    match name {
        "Hydrogen" => HYDROGEN_MINT,
        _ => unimplemented!(),
    }
}

pub fn handle_mining<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    fleet: &Pubkey,
    fleet_state: &FleetState,
    fleet_info: &Fleet,
    game_info: &Game,
) -> anyhow::Result<Mine> {
    dbg!("Mining Handler");

    let mine = match fleet_state {
        FleetState::MineAsteroid(mine_asteroid) => {
            // dbg!(mine_asteroid.start);
            // dbg!(mine_asteroid.last_update);
            // dbg!(mine_asteroid.resource);

            let resource_info = staratlas::sage::resource_account(client, &mine_asteroid.resource)?
                .expect("Resource not found");
            // dbg!(&resource_info.location);
            // dbg!(&resource_info.location_type);
            // dbg!(&resource_info.system_richness);
            // dbg!(&resource_info.mine_item);

            let mine_item_info =
                staratlas::sage::mine_item_account(client, &resource_info.mine_item)?
                    .expect("MineItem not found");
            // dbg!(mine_item_info.resource_hardness);

            let mine_item_name =
                std::str::from_utf8(&mine_item_info.name)?.trim_end_matches(char::from(0));
            // dbg!(mine_item_name);

            let resource_token = get_resource_token_by_name(mine_item_name);

            // TODO: before execute stop mining need to check if it's done...
            mine_cmd_stop_mining(
                fleet,
                fleet_info,
                &mine_asteroid.resource,
                &resource_info,
                &resource_token,
                game_info,
            )
        }
        _ => {
            dbg!("Not Mining");
            Mine::NoOp
        }
    };

    Ok(mine)
}

fn mine_cmd_stop_mining(
    fleet: &Pubkey,
    fleet_info: &Fleet,
    resource: &Pubkey,
    resource_info: &Resource,
    resource_token: &Pubkey,
    game_info: &Game,
) -> Mine {
    dbg!("Cmd Stop Mining");

    let planet: &Pubkey = &resource_info.location;

    // TODO: sage::mine_item::mine_item_planet_resource_token(mine_item_pubkey: &Pubkey, resource_token_pubkey: &Pubkey)?
    let (planet_resource_token, _) = Pubkey::find_program_address(
        &[
            &resource_info.mine_item.to_bytes(),
            &staratlas::TOKEN_PROGRAM_ID.to_bytes(),
            &resource_token.to_bytes(),
        ],
        &staratlas::ASSOCIATED_TOKEN_PROGRAM_ID,
    );

    // TODO: sage::fleet::fleet_cargo_hold_resource_token(fleet_cargo_hold_pubkey: &Pubkey, resource_token_pubkey: &Pubkey)?
    let (fleet_resource_token, _) = Pubkey::find_program_address(
        &[
            &fleet_info.cargo_hold.to_bytes(),
            &staratlas::TOKEN_PROGRAM_ID.to_bytes(),
            &resource.to_bytes(),
        ],
        &staratlas::ASSOCIATED_TOKEN_PROGRAM_ID,
    );

    // TODO: sage::fleet::fleet_cargo_hold_(game_)food_token(fleet_cargo_hold_pubkey: &Pubkey, game_mints_food_pubkey: &Pubkey)?
    let (fleet_food_token, _) = Pubkey::find_program_address(
        &[
            &fleet_info.cargo_hold.to_bytes(),
            &staratlas::TOKEN_PROGRAM_ID.to_bytes(),
            &game_info.mints.food.to_bytes(),
        ],
        &staratlas::ASSOCIATED_TOKEN_PROGRAM_ID,
    );

    // TODO: sage::fleet::fleet_ammo_bank_(game_)ammo_token(fleet_ammo_bank_pubkey: &Pubkey, game_mints_ammo_pubkey: &Pubkey)?
    let (fleet_ammo_token, _) = Pubkey::find_program_address(
        &[
            &fleet_info.ammo_bank.to_bytes(),
            &staratlas::TOKEN_PROGRAM_ID.to_bytes(),
            &game_info.mints.ammo.to_bytes(),
        ],
        &staratlas::ASSOCIATED_TOKEN_PROGRAM_ID,
    );

    Mine::StopMining {
        fleet: (
            *fleet,
            [fleet_resource_token, fleet_food_token, fleet_ammo_token],
        ),
        planet: (*planet, [planet_resource_token]),
    }
}
