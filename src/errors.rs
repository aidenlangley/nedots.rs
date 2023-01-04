#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("`{0}` failed! Review the output & try again")]
    Command(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Failed to make dir @ {path} ({err})")]
    MakeDir { path: String, err: std::io::Error },

    #[error("No metadata ({0})")]
    Metadata(String),

    #[error("No modified time ({0})")]
    ModifiedTime(String),

    #[error("Failed to remove dir @ {path} ({err})")]
    RemoveDir { path: String, err: std::io::Error },

    #[error("Failed to resolve {path} ({err})")]
    ResolvePath { path: String, err: std::io::Error },
}
