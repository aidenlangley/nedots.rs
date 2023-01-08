use crate::{
    models::{config::Config, git_repo::GitRepo},
    utils::spinner::Spinner,
};

#[derive(Debug, clap::Parser)]
/// Collect files & directories & sync with remote.
pub(crate) struct SyncCmd {
    #[arg(short, long)]
    /// Gather dots before syncing.
    gather: bool,

    #[arg(short, long)]
    /// Don't push to remote, useful for testing.
    nopush: bool,
}

impl super::ValidateConfig for SyncCmd {}

impl super::RunWith for SyncCmd {
    fn run_with(&self, config: &Config) -> anyhow::Result<()> {
        fn git_add_commit_push(repo: &GitRepo, push: bool) -> anyhow::Result<()> {
            let spinner = Spinner::start();

            spinner.set_msg(&format!(
                " Adding latest changes... {}",
                console::style(repo.path.display()).blue()
            ));
            repo.add(".")?;

            spinner.set_msg(&format!(
                " Commiting latest changes... {}",
                console::style(repo.path.display()).blue()
            ));
            repo.commit(&format!("Latest {}", chrono::offset::Local::now()))?;

            spinner.set_msg(&format!(
                " Pulling latest changes... {}",
                console::style(&repo.remote).blue()
            ));
            repo.pull()?;

            if push {
                spinner.set_msg(&format!(
                    " Pushing to remote... {}",
                    console::style(&repo.remote).blue()
                ));
                repo.push()?;
            }

            spinner.finish();
            Ok(())
        }

        if self.gather {
            let spinner = Spinner::start();
            spinner.set_msg(" Gathering source files & directories...");

            for source in &config.sources {
                let dst = crate::utils::join_paths(&config.dots_dir, source);
                crate::ops::copy(source, &dst)?;
            }
            spinner.finish();

            for repo in &config.git_repos {
                git_add_commit_push(repo, !self.nopush)?;
            }
        }

        git_add_commit_push(&GitRepo::new(&config.remote, &config.root), !self.nopush)?;

        log::info!("✅ {}", console::style("Synced!").bold());
        Ok(())
    }
}
