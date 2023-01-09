pub(crate) mod cmd;
pub(crate) mod errors;
pub(crate) mod models;
pub(crate) mod ops;
pub(crate) mod utils;

pub use cmd::nedots::RootCmd;

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

pub trait RunWith<T> {
    fn run_with(&self, with: &T) -> anyhow::Result<()>;
}

pub trait Execute: clap::Args + Run {
    fn exec(&self) -> anyhow::Result<()> {
        self.run()
    }
}

/// Implementors will 'Execute' - they will run some code with the intent of
/// notifying the user during or after runtime.
pub trait ExecuteWith<T, V>: clap::Args + RunWith<V> {
    /// Execute with `T`, so `T` is passed to `exec` and is required for valid
    /// runtime.
    ///
    /// * `with`: `&T`, some data required for execution.
    fn exec_with(&self, with: &T) -> anyhow::Result<()>;
}

/// Marker trait.
pub trait BasicCommand {}
