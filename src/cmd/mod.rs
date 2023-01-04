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
        if let Some(cfg_path) = &root_args.cfg_path {
            config = config::read(cfg_path)?;
        } else {
            let path = Path::new(&root_args.root).join(Path::new(&root_args.cfg_file));
            config = config::read(&path)?;
            config.root = Path::new(&root_args.root).to_path_buf();
        }

        if let Some(dots_dir) = &root_args.dots {
            config.dots_dir = Path::new(dots_dir).to_path_buf();
        }

        if let Some(backup_dir) = &root_args.backups {
            config.backup_dir = Path::new(backup_dir).to_path_buf();
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
