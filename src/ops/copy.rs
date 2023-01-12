use crate::utils::paths::{get_metadata, make_all_dirs};
use std::path::Path;

pub fn copy(from: &Path, to: &Path) -> anyhow::Result<()> {
    log::trace!("Copying `{}` -> `{}`", from.display(), to.display());

    // There are a couple of ways to check if a given path can be considered a
    // directory - via `Metadata` is preferred, as it catches errors, but
    // `Path::is_dir()` is fine for our purposes too, so no biggy if we don't
    // get any `Metadata`.
    let mut src_is_dir = from.is_dir();
    if let Ok(src_metadata) = get_metadata(from) {
        src_is_dir = src_metadata.is_dir();
    }

    if src_is_dir {
        // When given a directory as `src`, we've been asked to copy the
        // contents of a directory into the `dst` path - we want to create
        // the same directory structure as defined in `src`, so we call `gather`
        // once again, this time with the `src` directory name appended to
        // `dst`.
        for entry in from.read_dir()? {
            let path = entry?.path();
            copy(&path, &to.join(path.file_name().unwrap()))?;
        }
    } else {
        // Now that we are positive we're not handling any directories, it's
        // safe to assume any parent of our file is going to be a directory.
        if let Some(parent) = to.parent() {
            if !parent.exists() {
                make_all_dirs(parent)?;
            }
        }

        if let Err(err) = std::fs::copy(from, to) {
            log::warn!("Couldn't copy {} ({})", from.display(), err);
        }
    }

    Ok(())
}
