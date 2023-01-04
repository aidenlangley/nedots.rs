pub(crate) mod backup;
pub(crate) mod clean;
pub(crate) mod init;
pub(crate) mod install;
pub(crate) mod nedots;
pub(crate) mod sync;

use crate::models::config::{self, Config};
use std::path::Path;

pub trait Initialize<T = Config> {
    fn init(&self, root_args: &super::RootCmd) -> anyhow::Result<T>;
}

impl<T: ValidateConfig> Initialize<Config> for T {
    fn init(&self, root_args: &super::RootCmd) -> anyhow::Result<Config> {
        let mut config: Config;

        let xdg_config_home: Option<&'static str> = option_env!("XDG_CONFIG_HOME");
        if let Some(path) = xdg_config_home {
            config = config::read(&Path::new(path).join(&root_args.config))?;
        } else {
            let path = Path::new(env!("HOME")).join(".config");
            config = config::read(&path.join(&root_args.config))?;
        }

        let xdg_data_home: Option<&'static str> = option_env!("XDG_DATA_HOME");
        if let Some(path) = xdg_data_home {
            config.root = Path::new(path).join("nedots");
        } else {
            config.root = Path::new(env!("HOME")).join(".local/share/nedots");
        }

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
