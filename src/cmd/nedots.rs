use super::{Execute, ExecuteWith};
use crate::models::config::Config;
use clap_verbosity_flag::Verbosity;

const DEFAULT_CONFIG: &str = "nedots/nedots.yml";

#[derive(Debug, clap::Parser)]
#[command(author, about, version, arg_required_else_help(true))]
pub struct RootCmd {
    /// Custom config file
    #[arg(short, long, default_value = DEFAULT_CONFIG)]
    pub config: String,

    #[command(flatten)]
    pub verbose: Verbosity,

    #[command(subcommand)]
    cmd: Option<SubCommand>,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum SubCommand {
    /// Backup local configuration files/(ne)dots
    Backup(super::backup::BackupCmd),
    /// Remove temporary files, `dots` & `backups`
    Clean(super::clean::CleanCmd),
    /// Generate shell completions
    Completions(super::completions::CompletionsCmd),
    /// Initialize `nedots`
    Init(super::init::InitCmd),
    /// Install files & directories
    Install(super::install::InstallCmd),
    /// Collect files & directories & sync with remote
    Sync(super::sync::SyncCmd),
}

impl super::Initialize<Config, RootCmd> for RootCmd {
    fn init(&self, _: &Self) -> anyhow::Result<Config> {
        Ok(Config::default())
    }
}

impl super::RunWith<Config> for RootCmd {
    fn run_with(&self, _: &Config) -> anyhow::Result<()> {
        if let Some(cmd) = &self.cmd {
            match cmd {
                SubCommand::Backup(backup_cmd) => backup_cmd.exec_with(self),
                SubCommand::Clean(clean_cmd) => clean_cmd.exec_with(self),
                SubCommand::Completions(completions_cmd) => completions_cmd.exec(),
                SubCommand::Init(init_cmd) => init_cmd.exec(),
                SubCommand::Install(install_cmd) => install_cmd.exec_with(self),
                SubCommand::Sync(sync_cmd) => sync_cmd.exec_with(self),
            }
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn verify() {
        <super::RootCmd as clap::CommandFactory>::command().debug_assert()
    }
}
