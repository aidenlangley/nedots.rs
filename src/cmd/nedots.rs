use super::{Execute, Run};
use crate::models::config::Config;
use clap_verbosity_flag::Verbosity;
use std::path::PathBuf;

const DEFAULT_ROOT: &str = ".nedots";
const DEFAULT_CFG_FILE: &str = ".nedots.yml";

#[derive(Debug, clap::Parser)]
#[command(author, about, version, arg_required_else_help(true))]
pub struct RootCmd {
    #[arg(short, long, default_value = DEFAULT_ROOT)]
    /// Root directory.
    pub(crate) root: String,

    /// Custom config file.
    #[arg(short, long = "cfg", default_value = DEFAULT_CFG_FILE)]
    pub(crate) cfg_file: String,

    #[arg(short = 'C', long = "cfgpath")]
    /// Path to config file, if provided then root & cfg_file are discarded.
    pub(crate) cfg_path: Option<PathBuf>,

    #[arg(short, long)]
    /// Where to store config files/(ne)dots.
    pub(crate) dots: Option<PathBuf>,

    #[arg(short, long)]
    /// Where to store backups of config files/(ne)dots.
    pub(crate) backups: Option<PathBuf>,

    #[command(flatten)]
    pub verbose: Verbosity,

    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum Command {
    Backup(super::backup::BackupCmd),
    Clean(super::clean::CleanCmd),
    Init(super::init::InitCmd),
    Install(super::install::InstallCmd),
    Sync(super::sync::SyncCmd),
}

impl super::Run for RootCmd {
    fn run(&self, _: &Config) -> anyhow::Result<()> {
        if let Some(cmd) = &self.cmd {
            match cmd {
                Command::Backup(backup_cmd) => backup_cmd.exec(self),
                Command::Clean(clean_cmd) => clean_cmd.exec(self),
                Command::Init(init_cmd) => init_cmd.exec(self),
                Command::Install(install_cmd) => install_cmd.exec(self),
                Command::Sync(sync_cmd) => sync_cmd.exec(self),
            }
        } else {
            Ok(())
        }
    }
}

impl super::Execute for RootCmd {
    fn exec(&self, _: &RootCmd) -> anyhow::Result<()> {
        self.run(&Config::default())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn verify() {
        <super::RootCmd as clap::CommandFactory>::command().debug_assert()
    }
}
