//! Command to clean up temporary files, `dots` or `backups`.
//!
//! Useful in the event that we need a clean slate. This command can serve to
//! perform any messy or complicated clean up operations.

use crate::models::config::Config;
use std::path::Path;

#[derive(Debug, clap::Args)]
pub struct CleanCmd {
    /// Clean up `dots`
    #[arg(short, long)]
    dots: bool,

    /// Clean up `backups`
    #[arg(short, long)]
    backups: bool,

    /// Won't prompt you to confirm the operation when cleaning
    #[arg(short = 'y', long)]
    assumeyes: bool,
}

impl super::RunWith<Config> for CleanCmd {
    /// Remove dots & backup directories.
    ///
    /// * `config`: &Config
    fn run_with(&self, config: &Config) -> anyhow::Result<()> {
        if self.dots {
            confirm_clean(
                &format!(
                    " ~ {}. Continue?",
                    console::style("Cleaning dots").yellow().bold()
                ),
                &config.dots_dir,
                self.assumeyes,
            )?
        }

        if self.backups {
            confirm_clean(
                &format!(
                    " ~ {}. Continue?",
                    console::style("Cleaning backups").yellow().bold()
                ),
                &config.backup_dir,
                self.assumeyes,
            )?
        }

        Ok(())
    }
}

/// Helper `fn` that sits inbetween `clean` and `confirm`.
///
/// * `prompt`: &str, msg to display to the user.
/// * `path`: &Path, path of directory to remove.
/// * `assumeyes`: bool, if given as true, we assume user will say yes to prompt.
fn confirm_clean<T>(prompt: &str, path: &T, assumeyes: bool) -> anyhow::Result<()>
where
    T: AsRef<Path>,
{
    match assumeyes {
        true => clean(path),
        false => confirm(prompt, clean, path),
    }
}

/// Let the user confirm their choice by presenting a `dialoger::Confirm`.
///
/// * `prompt`: `&str`, msg to display to the user.
/// * `func`: `fn`, function to run if user gives an affirmative response.
/// * `path`: `&Path`, path to run function on.
fn confirm<T>(prompt: &str, func: impl Fn(T) -> anyhow::Result<()>, path: T) -> anyhow::Result<()>
where
    T: AsRef<Path>,
{
    if dialoguer::Confirm::new()
        .with_prompt(prompt)
        .interact()
        .is_ok()
    {
        return func(path);
    }

    Ok(())
}

/// Remove this directory.
///
/// * `dir`: `&Path` path to remove.
fn clean<T>(dir: &T) -> anyhow::Result<()>
where
    T: AsRef<Path>,
{
    trash::delete(dir)?;
    crate::utils::make_all_dirs(dir)?;

    log::info!(
        "🗑️ {} {}",
        console::style("Cleaned").bold(),
        console::style(dir.as_ref().display()).bold().red()
    );
    Ok(())
}
