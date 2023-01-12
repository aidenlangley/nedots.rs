use super::git_repo::GitRepo;
use crate::utils::paths::ResolvePath;
use anyhow::Context;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(skip, default)]
    pub root: PathBuf,

    #[serde(skip, default)]
    pub dots_dir: PathBuf,

    #[serde(skip, default)]
    pub backup_dir: PathBuf,

    pub remote: String,
    pub sources: Vec<PathBuf>,
    pub git_repos: Vec<GitRepo>,
}

pub const DEFAULT_DOTS_DIR: &str = "dots";
pub const DEFAULT_BACKUP_DIR: &str = "backups";

pub fn read(path: &Path) -> anyhow::Result<Config> {
    let path = path.resolve_path()?;

    log::trace!("Reading `{}`...", path.display());
    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read `{}`", path.display()))?;

    log::trace!("Deserializing...");
    let config: Config = serde_yaml::from_str(&raw)
        .with_context(|| format!("Failed to deserialize `{:#?}`", &raw))?;

    Ok(config)
}

pub fn get_sample() -> Config {
    Config {
        root: PathBuf::default(),
        dots_dir: PathBuf::default(),
        backup_dir: PathBuf::default(),
        remote: "git@git.sr.ht:~nedia/nedots".to_string(),
        sources: vec![".config/nedots".into()],
        git_repos: vec![GitRepo {
            remote: "git@git.sr.ht:~nedia/config.nvim".to_string(),
            path: ".config/nvim".into(),
        }],
    }
}

impl Config {
    pub fn resolve_paths(mut self) -> Config {
        self = self.resolve_dirs();
        self = self.resolve_sources();
        self
    }

    pub fn resolve_dirs(mut self) -> Config {
        fn log_error(err: anyhow::Error) {
            log::error!("❌ {}", err);
        }

        match self.root.resolve_path() {
            Ok(path) => self.root = path,
            Err(err) => log_error(err),
        }

        self.dots_dir = self.root.join(DEFAULT_DOTS_DIR);
        match self.dots_dir.resolve_path() {
            Ok(path) => self.dots_dir = path,
            Err(err) => log_error(err),
        }

        self.backup_dir = self.root.join(DEFAULT_BACKUP_DIR);
        match self.backup_dir.resolve_path() {
            Ok(path) => self.backup_dir = path,
            Err(err) => log_error(err),
        }

        self
    }

    pub fn resolve_sources(mut self) -> Config {
        self.sources = self
            .sources
            .into_iter()
            .map(|s| match s.resolve_path() {
                Ok(path) => path,
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
            .map(|mut gr| match gr.path.resolve_path() {
                Ok(path) => {
                    gr.path = path;
                    gr
                }
                Err(err) => {
                    log::error!("❌ {}", err);
                    gr.path = Path::new("").to_path_buf();
                    gr
                }
            })
            .filter(|gr| gr.path.ne(Path::new("")))
            .collect();

        self
    }

    pub fn get_sources_as_hashmap(&self) -> HashMap<&str, PathBuf> {
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
