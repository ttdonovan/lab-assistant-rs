use super::*;

use crate::Resource;

pub fn resource_account<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    resource_pubkey: &Pubkey,
) -> anyhow::Result<Option<Resource>> {
    let program = client.program(SAGE_PROGRAM_ID)?;

    if let Some(account) = program.account::<Resource>(*resource_pubkey).ok() {
        Ok(Some(account))
    } else {
        Ok(None)
    }
}

pub fn resource_find_address(mine_item: &Pubkey, location: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"Resource", &mine_item.to_bytes(), &location.to_bytes()],
        &SAGE_PROGRAM_ID,
    )
}
