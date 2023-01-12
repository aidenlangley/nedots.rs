use crate::errors::Error;
use directories::BaseDirs;
use std::{
    borrow::Cow,
    fs::Metadata,
    path::{Path, PathBuf},
    time::SystemTime,
};

pub fn resolve_path<T>(path: &T) -> anyhow::Result<Cow<'static, Path>>
where
    T: AsRef<Path> + ?Sized,
{
    let path = path.as_ref();
    log::trace!("Resolving `{}`...", path.display());

    let mut pb = path.to_path_buf();
    if !path.exists() {
        pb = prepend_home(path).to_path_buf();
    }

    match pb.canonicalize() {
        Ok(resolved) => {
            log::trace!("Resolved `{}`", resolved.display());
            log::trace!("--");
            Ok(resolved.into())
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

pub fn prepend_home<T>(path: &T) -> Cow<'static, Path>
where
    T: AsRef<Path> + ?Sized,
{
    let pp = BaseDirs::new().expect("No BaseDirs").home_dir().join(path);
    log::trace!("+$HOME `{}` -> `{}`", path.as_ref().display(), pp.display());

    pp.into()
}

pub fn make_all_dirs<T>(path: &T) -> anyhow::Result<()>
where
    T: AsRef<Path> + ?Sized,
{
    let path = path.as_ref();
    if let Err(err) = std::fs::create_dir_all(path) {
        let err = Error::MakeDir {
            path: path.display().to_string(),
            err,
        };
        Err(err.into())
    } else {
        log::debug!("{}{}", console::style("++").green(), path.display());
        Ok(())
    }
}

pub fn _remove_all_dirs<T>(path: &T) -> anyhow::Result<()>
where
    T: AsRef<Path> + ?Sized,
{
    let path = path.as_ref();
    if let Err(err) = std::fs::remove_dir_all(path) {
        let err = Error::_RemoveDir {
            path: path.display().to_string(),
            err,
        };
        Err(err.into())
    } else {
        log::debug!("{}{}", console::style("--").red(), path.display());
        Ok(())
    }
}

pub fn get_metadata<T>(path: &T) -> anyhow::Result<Metadata>
where
    T: AsRef<Path> + ?Sized,
{
    let path = path.as_ref();
    if let Ok(metadata) = path.metadata() {
        Ok(metadata)
    } else {
        let err = Error::Metadata(path.display().to_string());
        Err(err.into())
    }
}

pub fn _get_modified<T>(path: &T) -> anyhow::Result<SystemTime>
where
    T: AsRef<Path> + ?Sized,
{
    let path = path.as_ref();
    if let Ok(modified) = get_metadata(path)?.modified() {
        Ok(modified)
    } else {
        let err = Error::_ModifiedTime(path.display().to_string());
        Err(err.into())
    }
}

/// Splits left & right paths by '/' (Linux only) then combines them into
/// an absolute path
///
/// * `left`: left hand side
/// * `right`: right hand side
pub fn join_paths<T>(left: T, right: T) -> Cow<'static, Path>
where
    T: AsRef<Path>,
{
    let (left, right) = (left.as_ref(), right.as_ref());
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

    Path::new("/").join(path).into()
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write, path::Path};

    #[test]
    fn resolve_missing_file_path_fails() {
        let path = Path::new("resolve_path_test.txt");
        assert!(super::resolve_path(path).is_err())
    }

    #[test]
    fn resolve_existing_file() {
        let mut file = File::create("resolves.txt").expect("failed to create resolves.txt");
        file.write_all(b"I _will_ resolve!")
            .expect("failed to write to resolves.txt");

        let path = Path::new("resolves.txt");
        assert!(super::resolve_path(path).is_ok());

        std::fs::remove_file("resolves.txt").expect("failed to remove resolves.txt");
        assert!(!path.exists());
    }

    #[test]
    fn make_remove_all_dirs() {
        let mut path = Path::new("nested/dir/structure/");
        assert!(super::make_all_dirs(path).is_ok());

        path = Path::new("nested/");
        assert!(super::_remove_all_dirs(path).is_ok());

        // Check it's been deleted.
        assert!(!path.exists())
    }

    #[test]
    fn get_metadata_modified() {
        let mut file = File::create("metadata.txt").expect("failed to create metadata.txt");

        let path = Path::new("metadata.txt");
        assert!(super::get_metadata(path).is_ok());

        file.write_all(b"Retrieving metadata")
            .expect("failed to write to metadata.txt");
        assert!(super::_get_modified(path).is_ok());

        std::fs::remove_file("metadata.txt").expect("failed to remove metadata.txt");
        assert!(!path.exists());
    }
}
