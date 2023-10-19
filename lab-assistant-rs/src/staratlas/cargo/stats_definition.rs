use super::*;

use crate::CargoStatsDefinition;

pub fn get_cargo_stats_definition_accounts<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
) -> anyhow::Result<Option<Vec<(Pubkey, CargoStatsDefinition)>>> {
    let program = client.program(CARGO_PROGRAM_ID)?;

    let accounts = program.accounts::<CargoStatsDefinition>(vec![])?;

    if accounts.is_empty() {
        Ok(None)
    } else {
        Ok(Some(accounts))
    }
}
