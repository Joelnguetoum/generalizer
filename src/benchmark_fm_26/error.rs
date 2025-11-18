
#[allow(dead_code)]
#[derive(Clone,Debug)]
pub enum BenchmarkError{
    FolderAccessError(String),
    InvalidFolderStructure,
    InvalidModelSyntax(String),
    HsfFileError(String),
    HifFileError(String),
    CompositionError(String),
}