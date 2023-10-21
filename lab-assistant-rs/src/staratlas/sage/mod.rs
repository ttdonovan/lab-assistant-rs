use crate::staratlas::*;

pub const SAGE_PROGRAM_ID: Pubkey = pubkey!("SAGEqqFewepDHH6hMDcmWy7yjHPpyKLDnRXKb3Ki8e6");

mod fleet;
pub use fleet::*;

mod game_state;
pub use game_state::*;

mod game;
pub use game::*;

mod mine_item;
pub use mine_item::*;

mod resource;
pub use resource::*;

mod starbase;
pub use starbase::*;

mod survey_data_unit_tracker;
pub use survey_data_unit_tracker::*;
