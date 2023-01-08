use super::{Execute, ExecuteWith};
use crate::models::config::Config;
use clap_verbosity_flag::Verbosity;

const DEFAULT_CONFIG: &str = "nedots/nedots.yml";

#[derive(Debug, clap::Parser)]
#[command(author, about, version, arg_required_else_help(true))]
pub struct RootCmd {
    /// Custom config file.
    #[arg(short, long, default_value = DEFAULT_CONFIG)]
    pub(crate) config: String,

    #[command(flatten)]
    pub verbose: Verbosity,

    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum Command {
    Backup(super::backup::BackupCmd),
    Clean(super::clean::CleanCmd),
    Completions(super::completions::CompletionsCmd),
    Init(super::init::InitCmd),
    Install(super::install::InstallCmd),
    Sync(super::sync::SyncCmd),
}

impl super::Initialize for RootCmd {
    fn init(&self, _: &Self) -> anyhow::Result<Config> {
        Ok(Config::default())
    }
}

impl super::RunWith<Config> for RootCmd {
    fn run_with(&self, _: &Config) -> anyhow::Result<()> {
        if let Some(cmd) = &self.cmd {
            match cmd {
                Command::Backup(backup_cmd) => backup_cmd.exec_with(self),
                Command::Clean(clean_cmd) => clean_cmd.exec_with(self),
                Command::Completions(completions_cmd) => completions_cmd.exec(),
                Command::Init(init_cmd) => init_cmd.exec(),
                Command::Install(install_cmd) => install_cmd.exec_with(self),
                Command::Sync(sync_cmd) => sync_cmd.exec_with(self),
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
