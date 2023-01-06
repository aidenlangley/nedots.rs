use crate::{models::config::Config, utils::paths::MakeDirs};
use std::path::{Path, PathBuf};

#[derive(Debug, clap::Args)]
/// Backup local configuration files/(ne)dots
pub(crate) struct BackupCmd;

// Use default `Config` validation.
impl super::ValidateConfig for BackupCmd {}

impl super::RunWith<Config> for BackupCmd {
    /// Backup `sources` to `backup_dir/{timestamp}`.
    ///
    /// * `config`: &Config
    fn run_with(&self, config: &Config) -> anyhow::Result<()> {
        let dst = &config.backup_dir.join(crate::utils::get_timestamp());
        backup(&config.sources, dst)?;

        log::info!(
            "💽 {} {}",
            console::style("All backed up!").bold(),
            console::style(dst.display()).blue()
        );
        Ok(())
    }
}

/// Make directory, `backup_dir/{timestamp}` and loop through `sources`. Copy
/// each to `dst`.
///
/// * `sources`: &[PathBuf]
/// * `dst`: &Path
pub(crate) fn backup(sources: &[PathBuf], dst: &Path) -> anyhow::Result<()> {
    log::trace!("Backing up to `{}`", dst.display().to_string());
    dst.make_all_dirs()?;

    for source in sources {
        let dst = crate::utils::join_paths(dst, source);
        crate::ops::copy(source, &dst)?;
    }

    Ok(())
}
