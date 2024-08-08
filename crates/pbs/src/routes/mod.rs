mod get_header;
mod proposer;
mod register_validator;
mod router;
mod status;
mod submit_block;

use get_header::handle_get_header;
use proposer::handle_check_proposers_slot;
use register_validator::handle_register_validator;
pub use router::create_app_router;
use status::handle_get_status;
use submit_block::handle_submit_block;
