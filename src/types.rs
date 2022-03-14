use crate::file_collector::FileExtension;
pub type ProjError = String;
pub type ProjResult<T> = Result<T, ProjError>;
pub type Files = Vec<FileExtension>;
