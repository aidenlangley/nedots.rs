//! `nedots` is a CLI application. I've attempted to build in some abstraction,
//! but not too much - this library offers a few traits that chain together an
//! `init`, `exec` and `run` function.
//!
//! A `RootCmd` is a struct that defines flags & args for the base command, in
//! my case, `nedots`. This is found in `src/cmd/nedots.rs` and derives
//! `clap::Parser`.
//!
//! `RootCmd` implements a basic `Initialize` - no command is aware of another,
//! so you're unable to `init` in `RootCmd` in order to provide data for
//! subcommands, so `RootCmd` does next to nothing in `init`.
//!
//! It also implements `RunWith<T>`. `Run` is the only trait that really needs
//! to be implemented. This is where the command logic belongs.
//!
//! `Run` is intended for commands that don't require any other data at runtime.
//!
//! `RunWith` can be implemented with a generic argument, so implementors of
//! `RunWith` want to be provided some data at runtime.
//!
//! `Execute` & `Execute` need not be implemented - they will call `run` or
//! `run_with`. `run_with` accepts `init` as an argument.
//!
//! `Initialize` takes two generic arguments, `T` is the return value (wrapped
//! in a `Result`) and `V` is the command arguments (in other words, `RootCmd`
//! or a subcommand struct that derives `clap::Parser` or `clap::Args`.) Here
//! you would load a config file or do some prep-work, and provide `T` to
//! `exec_with` which is then forwarded onto `run_with`.

pub(crate) mod cmd;
pub(crate) mod errors;
pub(crate) mod models;
pub(crate) mod utils;

pub use cmd::nedots::RootCmd;

use crate::utils::paths::{get_metadata, make_all_dirs};
use std::path::Path;

/// Implementors will take steps to `Initialize` before runtime. They return `T`
/// and `V` is passed to `init` and is required for valid `Initialization`.
pub trait Initialize<T, V> {
    /// Use args in order to initialize and return `T`.
    fn init(&self, args: &V) -> anyhow::Result<T>;
}

/// Implementors will `Run`. Shares similarities with `Execute`, but differs
/// because `exec` is typically called to `run` implementor so that the logical
/// scope of `Run` is constrained to its bare necessities.
pub trait Run {
    fn run(&self) -> anyhow::Result<()>;
}

/// Can pass `T` to `run_with` - a config struct or some other data may be
/// passed here.
pub trait RunWith<T> {
    fn run_with(&self, with: &T) -> anyhow::Result<()>;
}

/// Must implement `clap::Args` + `Run`.
///
/// This trait has a blanket implementation for all applicable structs, so no
/// need to implement unless explicity required.
pub trait Execute: clap::Args + Run {
    fn exec(&self) -> anyhow::Result<()> {
        self.run()
    }
}

/// Implementors will 'Execute' - they will run some code with the intent of
/// notifying the user during or after runtime.
///
/// A blanket implementation exists for all applicable structs, so no need to
/// worry about this unless you know that you need to.
pub trait ExecuteWith<T, V>: clap::Args + RunWith<V> {
    /// Execute with `T`, so `T` is passed to `exec` and is required for valid
    /// runtime.
    fn exec_with(&self, with: &T) -> anyhow::Result<()>;
}

/// Recursively copies a file `from` -> `to`. Creates intermediary directories,
/// and reports failure, but does not stop operation.
pub fn copy<T>(from: &T, to: &T) -> anyhow::Result<()>
where
    T: AsRef<Path> + ?Sized,
{
    let (from, to) = (from.as_ref(), to.as_ref());

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
        log::trace!("Copying `{}` -> `{}`", from.display(), to.display());

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

        log::trace!("--");
    }

    Ok(())
}
