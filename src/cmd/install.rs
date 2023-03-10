use crate::{models::config::Config, utils::paths::ResolvePath};

#[derive(Debug, clap::Args)]
pub struct InstallCmd {
    /// Only gather this source. Any unique portion of a path in `sources` is
    /// valid. E.g. given a list of [ "/home/user/.bashrc", "/home/user/.zshrc" ],
    /// ".bashrc" or ".zshrc" may be used as a key.
    key: Option<String>,
}

impl super::ValidateConfig for InstallCmd {
    /// `InstallCmd` only validates core directories defined in `Config` - no
    /// need to validate `sources`, etc.
    ///
    /// * `config`: mut Config
    fn validate(&self, mut config: Config) -> anyhow::Result<Config> {
        config = config.resolve_dirs();
        log::debug!("Resolved {:#?}", config);
        Ok(config)
    }
}

impl super::RunWith<Config> for InstallCmd {
    fn run_with(&self, config: &Config) -> anyhow::Result<()> {
        const SUCCESS_MSG: &str = "👍 Installed";

        if let Some(key) = &self.key {
            if let Some(val) = config.get_sources_as_hashmap().get(key.as_str()) {
                let dst = &val.prepend_home();
                let src = crate::utils::join_paths(&config.dots_dir, dst);

                crate::ops::copy(&src, dst)?;
                log::info!(
                    "{} `{}`",
                    SUCCESS_MSG,
                    console::style(dst.display()).green().bold(),
                );
            } else {
                log::error!("❌ `{}` not found", key);
            }
        } else {
            for source in &config.sources {
                let dst = &source.prepend_home();
                let src = crate::utils::join_paths(&config.dots_dir, dst);

                crate::ops::copy(&src, dst)?;
                log::info!(
                    "{} `{}`",
                    SUCCESS_MSG,
                    console::style(dst.display()).green().bold(),
                );
            }

            for repo in &config.git_repos {
                repo.clone()?;
                log::info!(
                    "{} `{}`",
                    SUCCESS_MSG,
                    console::style(repo.path.display()).green().bold(),
                );
            }
        }

        Ok(())
    }
}
