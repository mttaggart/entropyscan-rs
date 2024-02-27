use std::path::PathBuf;

///
/// Holds info about a given target file 
///
#[derive(Debug, Clone)]
pub struct FileEntropy {
    pub path: PathBuf,
    pub entropy: f64
}

