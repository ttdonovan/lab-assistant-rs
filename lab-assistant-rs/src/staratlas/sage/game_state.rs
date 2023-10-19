use super::*;

use crate::GameState;

pub fn get_game_state_account<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    game_state_pubkey: &Pubkey,
) -> anyhow::Result<Option<GameState>> {
    let program = client.program(SAGE_PROGRAM_ID)?;

    if let Ok(game_state) = program.account::<GameState>(*game_state_pubkey) {
        Ok(Some(game_state))
    } else {
        Ok(None)
    }
}

// pub fn get_game_state_account<C: Deref<Target = impl Signer> + Clone>(
//     client: &Client<C>,
//     game_id: &Pubkey,
//     update_id: u64,
// ) -> anyhow::Result<Option<(Pubkey, GameState)>> {
//     let program = client.program(SAGE_PROGRAM_ID)?;

//     // static findAddress(
//     //     program: SageIDLProgram,
//     //     gameId: PublicKey,
//     //     updateId: BN,
//     //   ): [PublicKey, number] {
//     //     return PublicKey.findProgramAddressSync(
//     //       [
//     //         Buffer.from('GameState'),
//     //         gameId.toBuffer(),
//     //         updateId.toArrayLike(Buffer, 'le', 8),
//     //       ],
//     //       program.programId,
//     //     );
//     //   }

//     let update_id_bytes = update_id.to_le_bytes();
//     dbg!(&update_id_bytes);

//     let (pubkey, _) = Pubkey::find_program_address(
//         &[b"GameState", &game_id.to_bytes(), &update_id_bytes],
//         &SAGE_PROGRAM_ID,
//     );
//     dbg!(&pubkey);

//     // let accounts = program.accounts::<GameState>(vec![])?;
//     // dbg!(&accounts);

//     Ok(None)
// }
