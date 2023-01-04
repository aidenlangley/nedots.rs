use super::git_repo::GitRepo;
use crate::utils::paths::ResolvePath;
use anyhow::{Context, Error};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub root: PathBuf,
    pub dots_dir: PathBuf,
    pub backup_dir: PathBuf,
    pub remote: String,
    pub sources: Vec<PathBuf>,
    pub git_repos: Vec<GitRepo>,

    #[serde(skip)]
    pub _systemd_services: Vec<String>,
}

pub(crate) const DEFAULT_DOTS_DIR: &str = "dots";
pub(crate) const DEFAULT_BACKUP_DIR: &str = "backups";

pub(crate) fn read(path: &Path) -> anyhow::Result<Config> {
    let path = path.resolve_path()?;

    log::trace!("Reading `{}`...", path.display());
    let file = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read `{}`", path.display()))?;

    log::trace!("Deserializing...");
    let config: Config = serde_yaml::from_str(&file)
        .with_context(|| format!("Failed to deserialize `{:#?}`", &file))?;

    Ok(config)
}

pub(crate) fn get_sample() -> Config {
    Config {
        root: ".nedots".into(),
        dots_dir: "dots".into(),
        backup_dir: "backups".into(),
        remote: "git@git.sr.ht:~nedia/nedots.rs".to_string(),
        sources: vec![
            ".config/bspwm".into(),
            ".profile".into(),
            "/etc/hostname".into(),
            "Wallpapers".into(),
        ],
        git_repos: vec![GitRepo {
            id: "nvim".to_string(),
            remote: "git@git.sr.ht:~nedia/config.nvim".to_string(),
            path: ".config/nvim".into(),
        }],
        _systemd_services: vec![],
    }
}

impl Config {
    pub(crate) fn resolve_paths(mut self) -> Config {
        self = self.resolve_dirs();
        self = self.resolve_sources();
        self
    }

    pub(crate) fn resolve_dirs(mut self) -> Config {
        fn log_error(err: Error) {
            log::error!("❌ {}", err);
        }

        match self.root.resolve_path() {
            Ok(path) => self.root = path,
            Err(err) => log_error(err),
        }

        self.dots_dir = self.root.join(self.dots_dir);
        match self.dots_dir.resolve_path() {
            Ok(path) => self.dots_dir = path,
            Err(err) => log_error(err),
        }

        self.backup_dir = self.root.join(self.backup_dir);
        match self.backup_dir.resolve_path() {
            Ok(path) => self.backup_dir = path,
            Err(err) => log_error(err),
        }

        self
    }

    pub(crate) fn resolve_sources(mut self) -> Config {
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
