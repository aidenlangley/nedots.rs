pub mod paths;
pub use paths::{join_paths, make_all_dirs, prepend_home, resolve_path};

pub(crate) mod misc;
pub(crate) use misc::{get_timestamp, run_cmd};

pub(crate) mod spinner;
