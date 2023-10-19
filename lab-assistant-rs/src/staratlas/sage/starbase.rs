use super::*;

use crate::Starbase;

pub fn get_starbase_from_coords<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    game_id: &Pubkey,
    x: i64,
    y: i64,
) -> anyhow::Result<Option<Starbase>> {
    let program = client.program(SAGE_PROGRAM_ID)?;
    let (pubkey, _) = starbase_find_address(game_id, (x, y));

    if let Ok(account) = program.account::<Starbase>(pubkey) {
        Ok(Some(account))
    } else {
        Ok(None)
    }
}

pub fn starbase_find_address(game_id: &Pubkey, sector_coordinates: (i64, i64)) -> (Pubkey, u8) {
    let x_bytes = sector_coordinates.0.to_le_bytes();
    let y_bytes = sector_coordinates.1.to_le_bytes();

    Pubkey::find_program_address(
        &[b"Starbase", &game_id.to_bytes(), &x_bytes, &y_bytes],
        &SAGE_PROGRAM_ID,
    )
}
