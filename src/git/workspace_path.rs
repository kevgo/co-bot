use crate::domain::IssueId;
use crate::errors::Result;
use camino::Utf8PathBuf;

pub fn workspace_path(
    template: &str,
    issue_id: &IssueId,
    issue_title: &str,
) -> Result<Utf8PathBuf> {
    let path = template
        .replace("{{ticket.id}}", &issue_id.to_string())
        .replace("{{ticket.title}}", &escape(issue_title));
    Ok(Utf8PathBuf::from(path))
}

fn escape(text: &str) -> String {
    text.to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect()
}

#[cfg(test)]
mod tests {

    mod escape {
        #[test]
        fn text() {
            let give = "Hello World!";
            let have = super::super::escape(give);
            assert_eq!(have, "hello-world");
        }
    }

    mod workspace_path {
        use crate::domain::IssueId;

        #[test]
        fn workspace_path() {
            let path = super::super::workspace_path(
                "{{ticket.id}}-{{ticket.title}}",
                &IssueId::from("123"),
                "Test Ticket",
            )
            .unwrap();
            assert_eq!(path, "123-test-ticket");
        }
    }
}
