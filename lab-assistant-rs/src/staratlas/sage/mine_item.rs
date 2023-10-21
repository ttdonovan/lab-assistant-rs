use super::*;

use crate::MineItem;

pub fn mine_item_account<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    mine_item_pubkey: &Pubkey,
) -> anyhow::Result<Option<MineItem>> {
    let program = client.program(SAGE_PROGRAM_ID)?;

    if let Some(account) = program.account::<MineItem>(*mine_item_pubkey).ok() {
        Ok(Some(account))
    } else {
        Ok(None)
    }
}

// static findAddress(
//     program: SageIDLProgram,
//     gameId: PublicKey,
//     mint: PublicKey,
//   ): [PublicKey, number] {
//     return PublicKey.findProgramAddressSync(
//       [Buffer.from('MineItem'), gameId.toBuffer(), mint.toBuffer()],
//       program.programId,
//     );
//   }

pub fn mine_item_find_address(game_id: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"MineItem", &game_id.to_bytes(), &mint.to_bytes()],
        &SAGE_PROGRAM_ID,
    )
}
