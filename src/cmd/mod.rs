pub(crate) mod backup;
pub(crate) mod clean;
pub(crate) mod completions;
pub(crate) mod init;
pub(crate) mod install;
pub(crate) mod nedots;
pub(crate) mod sync;

use crate::models::config::{self, Config};
use directories::BaseDirs;

pub trait Initialize<T = Config> {
    fn init(&self, root_args: &super::RootCmd) -> anyhow::Result<T>;
}

impl<T: ValidateConfig> Initialize<Config> for T {
    fn init(&self, root_args: &super::RootCmd) -> anyhow::Result<Config> {
        let base_dirs = BaseDirs::new().expect("No BaseDirs");
        let mut config = config::read(&base_dirs.config_dir().join(&root_args.config))?;
        config.root = base_dirs.data_local_dir().join("nedots");

        log::debug!("Raw config `{:#?}`", config);
        self.validate(config)
    }
}

pub trait ValidateConfig {
    fn validate(&self, mut config: Config) -> anyhow::Result<Config> {
        config = config.resolve_paths();
        log::debug!("Resolved {:#?}", config);
        Ok(config)
    }
}

pub trait Run {
    fn run(&self, _config: &Config) -> anyhow::Result<()> {
        Ok(())
    }
}

pub trait Execute: clap::Args + Run {
    fn exec(&self, _: &super::RootCmd) -> anyhow::Result<()> {
        self.run(&Config::default())
    }
}

impl<T: clap::Args + Run + Initialize<Config>> Execute for T {
    fn exec(&self, root_args: &super::RootCmd) -> anyhow::Result<()> {
        self.run(&self.init(root_args)?)
    }
}
