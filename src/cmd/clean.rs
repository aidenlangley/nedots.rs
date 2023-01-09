//! Command to clean up temporary files, `dots` or `backups`.
//!
//! Useful in the event that we need a clean slate. This command can serve to
//! perform any messy or complicated clean up operations.

use crate::{models::config::Config, utils::paths::MakeDirs};
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
fn confirm_clean(prompt: &str, path: &Path, assumeyes: bool) -> anyhow::Result<()> {
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
fn confirm(
    prompt: &str,
    func: impl Fn(&Path) -> anyhow::Result<()>,
    path: &Path,
) -> anyhow::Result<()> {
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
fn clean(dir: &Path) -> anyhow::Result<()> {
    trash::delete(dir)?;
    dir.make_all_dirs()?;

    log::info!(
        "🗑️ {} {}",
        console::style("Cleaned").bold(),
        console::style(dir.display()).bold().red()
    );
    Ok(())
}
