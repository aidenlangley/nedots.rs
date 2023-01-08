pub(crate) mod backup;
pub(crate) mod clean;
pub(crate) mod completions;
pub(crate) mod init;
pub(crate) mod install;
pub(crate) mod nedots;
pub(crate) mod sync;

use crate::models::config::{self, Config};
use directories::BaseDirs;

pub trait Initialize<T = Config, V = super::RootCmd> {
    fn init(&self, args: &V) -> anyhow::Result<T>;
}

pub trait ValidateConfig {
    fn validate(&self, mut config: Config) -> anyhow::Result<Config> {
        config = config.resolve_paths();
        log::debug!("Resolved {:#?}", config);
        Ok(config)
    }
}

pub trait Run {
    fn run(&self) -> anyhow::Result<()>;
}

pub trait RunWith<T = Config> {
    fn run_with(&self, with: &T) -> anyhow::Result<()>;
}

pub trait Execute: clap::Args + Run {
    fn exec(&self) -> anyhow::Result<()> {
        self.run()
    }
}

pub trait ExecuteWith<T = super::RootCmd>: clap::Args + RunWith {
    fn exec_with(&self, with: &T) -> anyhow::Result<()>;
}

impl<T: ValidateConfig> Initialize<Config> for T {
    fn init(&self, root_args: &super::RootCmd) -> anyhow::Result<Config> {
        let base_dirs = BaseDirs::new().expect("No BaseDirs");
        let mut config = config::read(&base_dirs.config_dir().join(&root_args.config))?;
        config.root = base_dirs.data_local_dir().join("nedots");

        log::debug!("Raw {:#?}", config);
        self.validate(config)
    }
}

impl<T: clap::Args + Run> Execute for T {
    fn exec(&self) -> anyhow::Result<()> {
        self.run()
    }
}

impl<T: clap::Args + RunWith + Initialize> ExecuteWith for T {
    fn exec_with(&self, root_args: &super::RootCmd) -> anyhow::Result<()> {
        self.run_with(&self.init(root_args)?)
    }
}
