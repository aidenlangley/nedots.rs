use crate::models::config::Config;
use std::path::Path;

#[derive(Debug, clap::Args)]
pub struct BackupCmd;

impl super::RunWith<Config> for BackupCmd {
    /// Backup `sources` to `backup_dir/{timestamp}`.
    fn run_with(&self, config: &Config) -> anyhow::Result<()> {
        let dst = config
            .backup_dir
            .join(crate::utils::get_timestamp().as_ref());
        backup(&config.sources, &dst)?;

        log::info!(
            "💽 {} {}",
            console::style("All backed up!").bold(),
            console::style(dst.display()).blue()
        );
        Ok(())
    }
}

/// Make directory, `backup_dir/{timestamp}` and loop through `sources`. Copy
/// each to `to`.
pub fn backup<T>(sources: &[T], to: &T) -> anyhow::Result<()>
where
    T: AsRef<Path>,
{
    let to = to.as_ref();
    log::trace!("Backing up to `{}`", to.display().to_string());

    crate::utils::make_all_dirs(to)?;
    for source in sources {
        let source = source.as_ref();
        let dst = crate::utils::join_paths(to, source);
        crate::copy(source, &dst)?;
    }

    Ok(())
}
