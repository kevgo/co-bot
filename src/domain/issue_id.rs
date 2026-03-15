use std::fmt::Display;

// an issue id
#[derive(Debug, Eq, PartialEq)]
pub struct IssueId(String);

impl Display for IssueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for IssueId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for IssueId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&IssueId> for i32 {
    fn from(value: &IssueId) -> Self {
        value.0.parse().expect("issue id must be a number")
    }
}
