use crate::{models::config::Config, utils::paths::MakeDirs};
use std::path::Path;

#[derive(Debug, clap::Args)]
/// Clean up `dots` & `backups`. Example: `nedots clean -db` to clean both.
pub(crate) struct CleanCmd {
    #[arg(short, long)]
    /// Clean up `dots`.
    dots: bool,

    #[arg(short, long)]
    /// Clean up `backups`.
    backups: bool,

    #[arg(short = 'y', long)]
    /// Won't prompt you to confirm the operation when cleaning.
    assumeyes: bool,
}

// Use default `Config` validation.
impl super::ValidateConfig for CleanCmd {}

impl super::Run for CleanCmd {
    fn run(&self, config: &Config) -> anyhow::Result<()> {
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

fn confirm_clean(prompt: &str, path: &Path, assumeyes: bool) -> anyhow::Result<()> {
    match assumeyes {
        true => clean(path),
        false => confirm(prompt, path, clean),
    }
}

fn confirm(
    prompt: &str,
    path: &Path,
    func: impl Fn(&Path) -> anyhow::Result<()>,
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
