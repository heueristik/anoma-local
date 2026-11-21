use std::path::PathBuf;

/// A directory with the keys of the wrapper circuit for one inner verifying key, as `single_party_setup` writes it.
///
/// The keys stay on disk: the proving key alone has about 1.5 GB.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Keys {
    directory: PathBuf,
}

impl Keys {
    /// Refers to the keys in `directory`. A function that uses the keys reports a missing file.
    pub fn new(directory: impl Into<PathBuf>) -> Self {
        Self {
            directory: directory.into(),
        }
    }

    pub(crate) fn encoded_directory(&self) -> &[u8] {
        self.directory.as_os_str().as_encoded_bytes()
    }
}
