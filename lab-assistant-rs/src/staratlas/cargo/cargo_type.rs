use super::*;

use crate::CargoType;

pub fn get_cargo_type_accounts<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    cargo_stats_def_seq_id: u16,
) -> anyhow::Result<Option<Vec<(Pubkey, CargoType)>>> {
    let program = client.program(CARGO_PROGRAM_ID)?;

    let seq_id_bytes = cargo_stats_def_seq_id.to_be_bytes();
    let accounts = program.accounts::<CargoType>(vec![RpcFilterType::Memcmp(
        Memcmp::new_base58_encoded(75, &seq_id_bytes),
    )])?;

    if accounts.is_empty() {
        Ok(None)
    } else {
        Ok(Some(accounts))
    }
}
