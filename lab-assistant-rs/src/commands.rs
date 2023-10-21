use anchor_lang::prelude::Pubkey;

pub enum Commands {
    Mine(Mine),
    Scan(Scan),
    Transport(Transport),
}

#[derive(Debug)]
pub enum Mine {
    NoOp,
    StopMining {
        fleet: (Pubkey, [Pubkey; 3]), // (fleet, [fleet_resource_token, fleet_food_token, fleet_ammo_token])
        planet: (Pubkey, [Pubkey; 1]), // (planet, [planet_resource_token])
    },
}

#[derive(Debug)]
pub enum Scan {}

#[derive(Debug)]
pub enum Transport {}
