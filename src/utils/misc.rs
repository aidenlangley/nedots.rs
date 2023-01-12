use crate::errors::Error;
use std::{borrow::Cow, process::Command};

/// Wraps `Command` - when command is successful & `stderr` is not empty,
/// it'll forward `stderr` to user. When logging verbosity is `Trace`, `stdout`
/// will also be forwarded to user.
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

/// Returns current timestamp.
pub fn get_timestamp() -> Cow<'static, str> {
    format!("{}", chrono::offset::Local::now().timestamp()).into()
}
