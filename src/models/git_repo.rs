use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GitRepo {
    pub(crate) remote: String,
    pub(crate) path: PathBuf,
}

impl GitRepo {
    pub(crate) fn new(remote: &str, path: &Path) -> Self {
        Self {
            remote: remote.to_string(),
            path: path.to_path_buf(),
        }
    }

    fn run_cmd(&self, args: &[&str]) -> anyhow::Result<()> {
        crate::utils::run_cmd("git", args)
    }

    pub(crate) fn clone(&self) -> anyhow::Result<()> {
        self.run_cmd(&["clone", &self.remote, &self.path.display().to_string()])
    }

    pub(crate) fn add(&self, pattern: &str) -> anyhow::Result<()> {
        self.run_cmd(&["-C", &self.path.display().to_string(), "add", pattern])
    }

    pub(crate) fn commit(&self, msg: &str) -> anyhow::Result<()> {
        let res = self.run_cmd(&["-C", &self.path.display().to_string(), "commit", "-m", msg]);
        if res.is_err() {
            log::trace!("Expected `git commit` to error and it did, moving on...");
        }

        Ok(())
    }

    pub(crate) fn push(&self) -> anyhow::Result<()> {
        self.run_cmd(&["-C", &self.path.display().to_string(), "push"])
    }

    pub(crate) fn init_submodules(&self) -> anyhow::Result<()> {
        self.run_cmd(&[
            "-C",
            &self.path.display().to_string(),
            "submodule",
            "update",
            "--init",
            "--recursive",
        ])
    }

    pub(crate) fn pull(&self) -> anyhow::Result<()> {
        self.run_cmd(&["-C", &self.path.display().to_string(), "pull"])
    }
}
