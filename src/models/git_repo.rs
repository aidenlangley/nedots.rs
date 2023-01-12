use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GitRepo {
    pub remote: String,
    pub path: PathBuf,
}

impl GitRepo {
    pub fn new<T>(remote: &str, path: T) -> Self
    where
        T: AsRef<Path>,
    {
        Self {
            remote: remote.into(),
            path: path.as_ref().into(),
        }
    }

    fn run_cmd(&self, args: &[&str]) -> anyhow::Result<()> {
        crate::utils::run_cmd("git", args)
    }

    pub fn clone(&self) -> anyhow::Result<()> {
        self.run_cmd(&["clone", &self.remote, &self.path.display().to_string()])
    }

    pub fn add(&self, pattern: &str) -> anyhow::Result<()> {
        self.run_cmd(&["-C", &self.path.display().to_string(), "add", pattern])
    }

    pub fn commit(&self, msg: &str) -> anyhow::Result<()> {
        let res = self.run_cmd(&["-C", &self.path.display().to_string(), "commit", "-m", msg]);
        if res.is_err() {
            log::trace!("Expected `git commit` to error and it did, moving on...");
        }

        Ok(())
    }

    pub fn push(&self) -> anyhow::Result<()> {
        self.run_cmd(&["-C", &self.path.display().to_string(), "push"])
    }

    pub fn init_submodules(&self) -> anyhow::Result<()> {
        self.run_cmd(&[
            "-C",
            &self.path.display().to_string(),
            "submodule",
            "update",
            "--init",
            "--recursive",
        ])
    }

    pub fn pull(&self) -> anyhow::Result<()> {
        self.run_cmd(&["-C", &self.path.display().to_string(), "pull"])
    }
}
