pub mod backup;
pub mod clean;
pub mod completions;
pub mod init;
pub mod install;
pub mod nedots;
pub mod sync;

use std::path::Path;

use crate::{
    models::config::{self, Config},
    Execute, ExecuteWith, Initialize, RootCmd, Run, RunWith,
};

pub trait ValidateConfig {
    fn validate(&self, mut config: Config) -> anyhow::Result<Config> {
        config = config.resolve_paths();
        log::debug!("Resolved {:#?}", config);
        Ok(config)
    }
}

impl<T: ValidateConfig> Initialize<Config, RootCmd> for T {
    fn init(&self, root_args: &RootCmd) -> anyhow::Result<Config> {
        let base_dirs = directories::BaseDirs::new().expect("No BaseDirs");
        let config_path = match Path::new(&root_args.config).canonicalize() {
            Ok(path) => path,
            Err(_) => base_dirs.config_dir().join(&root_args.config),
        };

        let mut config = config::read(config_path)?;
        config.root = base_dirs.data_local_dir().join("nedots");

        log::debug!("Raw {:#?}", config);
        self.validate(config)
    }
}

impl<T> Execute for T
where
    T: clap::Args + Run,
{
    fn exec(&self) -> anyhow::Result<()> {
        self.run()
    }
}

impl<T> ExecuteWith<RootCmd, Config> for T
where
    T: clap::Args + RunWith<Config> + Initialize<Config, RootCmd>,
{
    fn exec_with(&self, root_args: &RootCmd) -> anyhow::Result<()> {
        self.run_with(&self.init(root_args)?)
    }
}

// Use default `Config` validation.
impl ValidateConfig for backup::BackupCmd {}
impl ValidateConfig for clean::CleanCmd {}
impl ValidateConfig for sync::SyncCmd {}
