use crate::{
    models::{config, git_repo::GitRepo},
    utils::{paths::MakeDirs, spinner::Spinner},
};
use directories::BaseDirs;
use std::path::Path;

#[derive(Debug, clap::Args)]
pub struct InitCmd {
    /// Migrating files from this user
    #[arg(short, long)]
    from_user: Option<String>,

    /// Remote git repository to clone, `root` arg determines the destination
    remote: String,
}

impl super::Run for InitCmd {
    fn run(&self) -> anyhow::Result<()> {
        let root_dir = BaseDirs::new()
            .expect("No BaseDirs")
            .data_local_dir()
            .join("nedots");

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
        let path = Path::new(&root_dir).join(config::DEFAULT_BACKUP_DIR);
        path.make_all_dirs()?;

        // Create `$XDG_CONFIG_HOME/nedots` & create a sample config file..
        init_config()?;

        log::info!("✅ {}", console::style("Initialized!").bold());
        Ok(())
    }
}

fn migrate_user(from_user: &str, root_dir: &Path) -> anyhow::Result<()> {
    log::trace!("Migrating from `{}`", from_user);

    let path = root_dir.join(config::DEFAULT_DOTS_DIR);
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

fn init_config() -> anyhow::Result<()> {
    let config_dir = BaseDirs::new()
        .expect("No BaseDirs")
        .config_dir()
        .join("nedots");

    let config_file = config_dir.join("nedots.yml");
    if !config_file.exists() {
        // If nedots.yml isn't yet present, we'll create an example file.
        log::trace!("Creating sample `{}`...", config_file.display());

        let yaml = serde_yaml::to_string(&config::get_sample())?;
        config_dir.make_all_dirs()?;
        std::fs::write(&config_file, yaml)?;

        log::info!(
            "🗒️ Sample config can be found @ {}",
            console::style(config_file.display()).bold()
        );
    }

    Ok(())
}
