#[derive(thiserror::Error, Debug)]
pub enum CustomError {
    #[error("An error from serde_json, may cause by invalid json format. {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("An error from std::io")]
    IoError(#[from] std::io::Error),
}
