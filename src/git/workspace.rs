use camino::{Utf8Path, Utf8PathBuf};
use std::fmt::Display;

/// a Git workspace
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Workspace(Utf8PathBuf);

impl Display for Workspace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl AsRef<Utf8Path> for Workspace {
    fn as_ref(&self) -> &Utf8Path {
        &self.0
    }
}

impl From<&str> for Workspace {
    fn from(path: &str) -> Self {
        Self(Utf8PathBuf::from(path.to_string()))
    }
}

impl From<String> for Workspace {
    fn from(path: String) -> Self {
        Self(Utf8PathBuf::from(path))
    }
}
