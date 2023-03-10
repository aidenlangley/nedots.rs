pub mod paths;
pub use paths::join_paths;
pub mod spinner;

use crate::errors::Error;
use std::process::Command;

pub fn run_cmd(prog: &str, args: &[&str]) -> anyhow::Result<()> {
    log::trace!("`{} {}`...", prog, args.join(" "));

    let output = Command::new(prog).args(args).output()?;
    if !output.status.success() {
        if !output.stderr.is_empty() {
            eprintln!("{}", String::from_utf8(output.stderr)?);
        }

        if log::log_enabled!(log::Level::Trace) && !output.stdout.is_empty() {
            println!("{}", String::from_utf8(output.stdout)?);
        }

        return Err(Error::Command(format!("`{} {}` failed", prog, args.join(" "))).into());
    }

    Ok(())
}

pub fn get_timestamp() -> String {
    format!("{}", chrono::offset::Local::now().timestamp())
}
