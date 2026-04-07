use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to initialize a databse")]
    InitDbErr,
    #[error("Failed to prepare statement")]
    PrepErr
}

impl From<rusqlite::Error> for AppError {
    
    fn from(_: rusqlite::Error) -> Self {
        AppError::InitDbErr
    }
}