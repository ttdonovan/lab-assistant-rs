use super::*;

use crate::Game;

pub fn get_game_accounts<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
) -> anyhow::Result<Option<Vec<(Pubkey, Game)>>> {
    let program = client.program(SAGE_PROGRAM_ID)?;

    let game_accounts = program.accounts::<Game>(vec![])?;

    if game_accounts.is_empty() {
        Ok(None)
    } else {
        Ok(Some(game_accounts))
    }
}
