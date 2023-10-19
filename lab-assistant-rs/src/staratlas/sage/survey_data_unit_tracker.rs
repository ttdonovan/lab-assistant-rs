use super::*;

use crate::SurveyDataUnitTracker;

pub fn get_sdu_tracker_accounts<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
) -> anyhow::Result<Option<Vec<(Pubkey, SurveyDataUnitTracker)>>> {
    let program = client.program(SAGE_PROGRAM_ID)?;

    let sdu_tracker_accounts = program.accounts::<SurveyDataUnitTracker>(vec![])?;

    if sdu_tracker_accounts.is_empty() {
        Ok(None)
    } else {
        Ok(Some(sdu_tracker_accounts))
    }
}
