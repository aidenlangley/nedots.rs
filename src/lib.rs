// Private `cmd`, commands are the consumers of any API.
mod cmd;

pub(crate) mod errors;
pub(crate) mod models;
pub(crate) mod ops;
pub(crate) mod utils;

pub use cmd::{nedots::RootCmd, Execute};
