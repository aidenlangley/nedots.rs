use crate::{
    models::{config::Config, git_repo::GitRepo},
    utils::{paths::MakeDirs, spinner::Spinner},
    RootCmd,
};
use std::path::{Path, PathBuf};

#[derive(Debug, clap::Args)]
/// Initialize `nedots`
pub(crate) struct InitCmd {
    /// Migrating files from this user.
    #[arg(short, long)]
    from_user: Option<String>,

    /// Remote git repository to clone, `root` arg determines the destination.
    remote: String,
}

impl super::Initialize for InitCmd {
    fn init(&self, _: &RootCmd) -> anyhow::Result<Config> {
        let root_dir: PathBuf;
        let xdg_data_home: Option<&'static str> = option_env!("XDG_DATA_HOME");
        if let Some(path) = xdg_data_home {
            root_dir = Path::new(path).join("nedots");
        } else {
            root_dir = Path::new(env!("HOME")).join(".local/share/nedots");
        }

        if !root_dir.exists() {
            log::trace!("Initializing {} @ {}...", &self.remote, &root_dir.display());
            let spinner = Spinner::start();
            let repo = GitRepo::new(&self.remote, &root_dir);

            spinner.set_msg(&format!(
                " Initializing {} @ {}...",
                console::style(&self.remote).blue(),
                console::style(&root_dir.display()).blue(),
            ));
            repo.clone()?;
            repo.init_submodules()?;

            spinner.finish();
        } else {
            log::debug!("{} @ {} exists", self.remote, &root_dir.display());
        }

        if let Some(user) = &self.from_user {
            migrate_user(user, &root_dir)?;
        }

        // Make backup directory
        let path = Path::new(&root_dir).join(crate::models::config::DEFAULT_BACKUP_DIR);
        path.make_all_dirs()?;

        let config_dir: PathBuf;
        let xdg_config_home: Option<&'static str> = option_env!("XDG_CONFIG_HOME");
        if let Some(path) = xdg_config_home {
            config_dir = Path::new(path).join("nedots");
        } else {
            config_dir = Path::new(env!("HOME")).join(".config/nedots");
        }

        let config_file = config_dir.join("nedots.yml");
        if !config_file.exists() {
            // If .nedots.yml isn't yet present, we'll create an example file.
            log::trace!("Creating sample `{}`...", config_file.display());

            let yaml = serde_yaml::to_string(&crate::models::config::get_sample())?;
            config_dir.make_all_dirs()?;
            std::fs::write(&config_file, yaml)?;

            log::info!(
                "🗒️ Sample config can be found @ {}",
                console::style(config_file.display()).bold()
            );
        }

        log::info!("✅ {}", console::style("Initialized!").bold());
        Ok(Config::default())
    }
}

impl super::Run for InitCmd {}

fn migrate_user(from_user: &str, root_dir: &Path) -> anyhow::Result<()> {
    log::trace!("Migrating from `{}`", from_user);

    let path = root_dir.join(crate::models::config::DEFAULT_DOTS_DIR);
    let from_path = path.join(format!("home/{}", from_user));
    let to_path = crate::utils::join_paths(&path, Path::new(env!("HOME")));

    log::trace!(
        "Renaming `{}` -> `{}`",
        from_path.display(),
        to_path.display()
    );
    std::fs::rename(from_path, to_path)?;

    Ok(())
}
