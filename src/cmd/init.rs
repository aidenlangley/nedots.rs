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
    fn init(&self, root_args: &RootCmd) -> anyhow::Result<Config> {
        let root_path = Path::new(&root_args.root);
        if !root_path.exists() {
            log::trace!("Initializing {} @ {}...", &self.remote, &root_args.root);
            let spinner = Spinner::start();
            let repo = GitRepo::new(&self.remote, Path::new(&root_args.root));

            spinner.set_msg(&format!(
                " Initializing {} @ {}...",
                console::style(&self.remote).blue(),
                console::style(&root_args.root).blue(),
            ));
            repo.clone()?;
            repo.init_submodules()?;

            spinner.finish();
        } else {
            log::debug!("{} @ {} exists", self.remote, &root_args.root);
        }

        if let Some(user) = &self.from_user {
            migrate_user(user, &root_args.root, root_args.dots.as_ref())?;
        }

        // Make backup directory
        let mut path = Path::new(&root_args.root).to_path_buf();
        if let Some(backups_dir) = &root_args.backups {
            path.push(backups_dir);
        } else {
            path.push(crate::models::config::DEFAULT_BACKUP_DIR);
        }

        path.make_all_dirs()?;

        let cfg_file = root_path.join(".nedots.yml");
        if !cfg_file.exists() {
            // If .nedots.yml isn't yet present, we'll create an example file.
            log::trace!("Creating sample `{}`...", cfg_file.display());

            let mut config = crate::models::config::get_sample();
            config.root = root_path.to_path_buf();
            config.remote = self.remote.clone();

            let yaml = serde_yaml::to_string(&crate::models::config::get_sample())?;
            std::fs::write(&cfg_file, yaml)?;

            log::info!(
                "🗒️ Sample config can be found @ {}",
                console::style(cfg_file.display()).bold()
            );
        }

        log::info!("✅ {}", console::style("Initialized!").bold());
        Ok(Config::default())
    }
}

impl super::Run for InitCmd {}

fn migrate_user(from_user: &str, root_dir: &str, dots_dir: Option<&PathBuf>) -> anyhow::Result<()> {
    log::trace!("Migrating from `{}`", from_user);

    let home = env!("HOME").strip_prefix('/').unwrap();
    let mut path = Path::new(&format!("{}/{}", home, root_dir)).to_path_buf();
    if let Some(dots_dir) = dots_dir {
        path.push(dots_dir);
    } else {
        path.push(crate::models::config::DEFAULT_DOTS_DIR);
    }

    let from_path = path.join(format!("home/{}", from_user));
    let to_path = path.join(home);

    let mut from_str = from_path.display().to_string();
    let mut to_str = to_path.display().to_string();

    from_str.insert(0, '/');
    to_str.insert(0, '/');

    log::trace!("Renaming `{}` -> `{}`", from_str, to_str);
    std::fs::rename(from_str, to_str)?;

    Ok(())
}
