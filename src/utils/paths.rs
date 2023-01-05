use crate::errors::Error;
use directories::BaseDirs;
use std::path::{Path, PathBuf};

pub trait ResolvePath {
    fn resolve_path(&self) -> anyhow::Result<PathBuf>;
    fn prepend_home(&self) -> PathBuf;
}

impl ResolvePath for Path {
    /// See ResolvePath for PathBuf. Here, we convert Path to PathBuf, and run
    /// the equivalent function.
    fn resolve_path(&self) -> anyhow::Result<PathBuf> {
        self.to_path_buf().resolve_path()
    }

    /// See ResolvePath for PathBuf. Here, we convert Path to PathBuf, and run
    /// the equivalent function.
    fn prepend_home(&self) -> PathBuf {
        self.to_path_buf().prepend_home()
    }
}

impl ResolvePath for PathBuf {
    /// Given PathBuf is automatically coerced to a Path as it's passed to this
    /// fn by reference.
    ///
    /// We check if the path exists, and if it doesn't, we
    /// prepend $HOME.
    ///
    /// Finally, we canonicalize the path since we don't want to
    /// use a relative path. Failure to canonicalize will result in an error
    /// being thrown and handled.
    fn resolve_path(&self) -> anyhow::Result<PathBuf> {
        log::trace!("Resolving `{}`...", self.display());

        let mut path = self.clone();
        if !path.exists() {
            path = self.prepend_home();
        }

        match path.canonicalize() {
            Ok(path) => {
                log::trace!("Resolved `{}`", path.display());
                Ok(path)
            }
            Err(err) => {
                let err = Error::ResolvePath {
                    path: path.display().to_string(),
                    err,
                };
                Err(err.into())
            }
        }
    }

    /// Sometimes we just want to prepend home, which is half of the resolution
    /// process.
    fn prepend_home(&self) -> PathBuf {
        let path = BaseDirs::new().expect("No BaseDirs").home_dir().join(self);
        log::trace!("+$HOME `{}` -> `{}`", self.display(), path.display());

        path
    }
}

pub trait MakeDirs {
    fn make_all_dirs(&self) -> anyhow::Result<()>;
}

impl MakeDirs for Path {
    fn make_all_dirs(&self) -> anyhow::Result<()> {
        if let Err(err) = std::fs::create_dir_all(self) {
            let err = Error::MakeDir {
                path: self.display().to_string(),
                err,
            };
            Err(err.into())
        } else {
            log::debug!("{}{}", console::style("++").green(), self.display());
            Ok(())
        }
    }
}

impl MakeDirs for PathBuf {
    fn make_all_dirs(&self) -> anyhow::Result<()> {
        self.as_path().make_all_dirs()
    }
}

pub trait RemoveDirs {
    fn remove_all_dirs(&self) -> anyhow::Result<()>;
}

impl RemoveDirs for Path {
    fn remove_all_dirs(&self) -> anyhow::Result<()> {
        if let Err(err) = std::fs::remove_dir_all(self) {
            let err = Error::RemoveDir {
                path: self.display().to_string(),
                err,
            };
            Err(err.into())
        } else {
            log::debug!("{}{}", console::style("--").red(), self.display());
            Ok(())
        }
    }
}

impl RemoveDirs for PathBuf {
    fn remove_all_dirs(&self) -> anyhow::Result<()> {
        self.as_path().remove_all_dirs()
    }
}

pub trait Metadata {
    fn get_metadata(&self) -> anyhow::Result<std::fs::Metadata>;
    fn get_modified(&self) -> anyhow::Result<std::time::SystemTime>;
}

impl Metadata for Path {
    fn get_metadata(&self) -> anyhow::Result<std::fs::Metadata> {
        if let Ok(metadata) = self.metadata() {
            Ok(metadata)
        } else {
            let err = Error::Metadata(self.display().to_string());
            Err(err.into())
        }
    }

    fn get_modified(&self) -> anyhow::Result<std::time::SystemTime> {
        if let Ok(modified) = self.get_metadata()?.modified() {
            Ok(modified)
        } else {
            let err = Error::ModifiedTime(self.display().to_string());
            Err(err.into())
        }
    }
}

impl Metadata for PathBuf {
    fn get_metadata(&self) -> anyhow::Result<std::fs::Metadata> {
        self.as_path().get_metadata()
    }

    fn get_modified(&self) -> anyhow::Result<std::time::SystemTime> {
        self.as_path().get_modified()
    }
}

/// Splits left & right paths by '/' (Linux only) then combines them into
/// an absolute path.
///
/// * `left`: PathBuf, left hand side.
/// * `right`: PathBuf, right hand side.
pub fn join_paths(left: &Path, right: &Path) -> PathBuf {
    log::trace!("Joining `{}` -> `{}`", left.display(), right.display());

    let path: PathBuf = [
        left.display()
            .to_string()
            .split(std::path::MAIN_SEPARATOR)
            .collect(),
        right
            .display()
            .to_string()
            .split(std::path::MAIN_SEPARATOR)
            .collect::<PathBuf>(),
    ]
    .into_iter()
    .collect();

    Path::new("/").join(path)
}

#[cfg(test)]
mod tests {
    use super::{MakeDirs, Metadata, RemoveDirs, ResolvePath};
    use std::{fs::File, io::Write, path::Path};

    #[test]
    fn resolve_missing_file_path_fails() {
        let path = Path::new("resolve_path_test.txt");
        assert!(path.resolve_path().is_err())
    }

    #[test]
    fn resolve_existing_file() {
        let mut file = File::create("resolves.txt").expect("failed to create resolves.txt");
        file.write_all(b"I _will_ resolve!")
            .expect("failed to write to resolves.txt");

        let path = Path::new("resolves.txt");
        assert!(path.resolve_path().is_ok());

        std::fs::remove_file("resolves.txt").expect("failed to remove resolves.txt");
        assert!(!path.exists());
    }

    #[test]
    fn make_remove_all_dirs() {
        let mut path = Path::new("nested/dir/structure/");
        assert!(path.make_all_dirs().is_ok());

        path = Path::new("nested/");
        assert!(path.remove_all_dirs().is_ok());

        // Check it's been deleted.
        assert!(!path.exists())
    }

    #[test]
    fn get_metadata_modified() {
        let mut file = File::create("metadata.txt").expect("failed to create metadata.txt");

        let path = Path::new("metadata.txt");
        assert!(path.get_metadata().is_ok());

        file.write_all(b"Retrieving metadata")
            .expect("failed to write to metadata.txt");
        assert!(path.get_modified().is_ok());

        std::fs::remove_file("metadata.txt").expect("failed to remove metadata.txt");
        assert!(!path.exists());
    }
}
