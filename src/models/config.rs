use super::git_repo::GitRepo;
use crate::utils::resolve_path;
use anyhow::Context;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub(crate) const DEFAULT_DOTS_DIR: &str = "dots";
pub(crate) const DEFAULT_BACKUP_DIR: &str = "backups";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(skip, default)]
    pub(crate) root: PathBuf,

    #[serde(skip, default)]
    pub(crate) dots_dir: PathBuf,

    #[serde(skip, default)]
    pub(crate) backup_dir: PathBuf,

    pub(crate) remote: String,
    pub(crate) sources: Vec<PathBuf>,
    pub(crate) git_repos: Vec<GitRepo>,
}

impl Config {
    pub(crate) fn resolve_paths(mut self) -> Config {
        self = self.resolve_sources();
        self
    }

    pub(crate) fn resolve_sources(mut self) -> Config {
        self.sources = self
            .sources
            .into_iter()
            .map(|s| match resolve_path(&s) {
                Ok(path) => path.into(),
                Err(err) => {
                    log::error!("❌ {}", err);
                    Path::new("").to_path_buf()
                }
            })
            .filter(|s| s.ne(Path::new("")))
            .collect();

        self.git_repos = self
            .git_repos
            .into_iter()
            .map(|mut gr| match resolve_path(&gr.path) {
                Ok(path) => {
                    gr.path = path.into();
                    gr
                }
                Err(err) => {
                    log::error!("❌ {}", err);
                    gr.path = Path::new("").into();
                    gr
                }
            })
            .filter(|gr| gr.path.ne(Path::new("")))
            .collect();

        self
    }

    pub(crate) fn get_sources_as_hashmap(&self) -> HashMap<&str, PathBuf> {
        let mut all_parts: Vec<&str> = Vec::new();
        for pb in &self.sources {
            let _: Vec<&str> = pb
                .to_str()
                .unwrap_or("")
                .split('/')
                .map(|part| {
                    all_parts.push(part);
                    part
                })
                .collect();
        }

        all_parts.sort();
        all_parts.dedup_by(|a, b| a.eq(&b));

        let mut hash_map: HashMap<&str, PathBuf> = HashMap::new();
        for part in &all_parts {
            if !part.is_empty() {
                hash_map.insert(
                    part,
                    self.sources
                        .iter()
                        .find(|s| s.to_str().unwrap_or("").contains(part))
                        .unwrap()
                        .to_path_buf(),
                );
            }
        }

        log::debug!("Keyed sources `{:#?}`", &hash_map);
        hash_map
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root: PathBuf::default(),
            dots_dir: PathBuf::default(),
            backup_dir: PathBuf::default(),
            remote: "git@git.sr.ht:~nedia/nedots".into(),
            sources: vec![".config/nedots".into()],
            git_repos: vec![GitRepo::new(
                "git@git.sr.ht:~nedia/config.nvim",
                ".config/nvim",
            )],
        }
    }
}

pub(crate) fn read<T>(path: T) -> anyhow::Result<Config>
where
    T: AsRef<Path>,
{
    let path = path.as_ref();

    log::trace!("Reading `{}`...", path.display());
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read `{}`", path.display()))?;

    log::trace!("Deserializing...");
    let config: Config = serde_yaml::from_str(&raw)
        .with_context(|| format!("Failed to deserialize `{:#?}`", &raw))?;

    Ok(config)
}
