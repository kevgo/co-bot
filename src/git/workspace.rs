use camino::Utf8PathBuf;
use std::fmt::Display;

/// a Git workspace
pub struct Workspace(Utf8PathBuf);

impl Display for Workspace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl AsRef<Utf8PathBuf> for Workspace {
    fn as_ref(&self) -> &Utf8PathBuf {
        &self.0
    }
}

impl From<Utf8PathBuf> for Workspace {
    fn from(path: Utf8PathBuf) -> Self {
        Self(path)
    }
}
