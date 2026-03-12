# AI team

> Human and AI develop software as a team.

## Functionality

### Kickoff

- I run the CLI app on a separate desktop on my computer
- I tell it to implement a ticket (Jira, GitHub)
  - it prints all applicable tickets, I select one
- it creates a new Git worktree for the ticket

### Planning

- the planner agent creates an implementation plan
  - based on the planning instructions for the repo
  - based on reads the ticket
- it opens a draft PR for this ticket containing an MD file with the
  implementation plan
- implementation plan review loop
  - human adds comments, modifications, and instructions what to change to the
    file
  - it implements the instruction
  - when the human is happy, they approve the plan

### Implementation

- coding agent implements the plan
  - tests first (red/green)
- documentation agent adds/updates documentation

### Automated review

- review agent reviews the changes and makes comments
- coding agents implements the feedback
- review ends when the review agent has no more comments

### Human review

- human gets pinged in a comment
- human reviews the changes
  - human adds comments, modifications, and instructions what to change to the
    file
  - it implements the instructions
  - it performs another automated review loop
  - loop ends when the human merges the PR

### Finalize

- it comments with statistics on the PR
  - time and money spent
- it removes the local Git branch and worktree

## How it works

- CLI app written in Rust
- I run the CLI app (one or many instances) in a separate desktops on my
  computer (to use my capable computer for AI development), in a local
  VM/Docker, or in the cloud
- it runs completely headless, I interact with it through my code forge web UI

Config file:

```toml
[tracker]
type = "Jira"
url = "https://jira.walmart.com"
token_env_var = "JIRA_TOKEN"  # name of the env var from which to read the access token for the tracker

[forge]
type = "GitHub"
url = "gec01.walmart.com"
token_env_var = "GITHUB_TOKEN"  # name of the env var from which to read the access token for the forge

[LLM]
cli = "wibey -p '{prompt}'"  # placeholder for the prompt

[agents]
planner.model = "opus"
coder.model = "sonnet"
reviewer.model = "opus"
documenter.model = "sonnet"
```

Prompt files in the `.ai-team` folder:

- `planner.md`

  ```md
  You are a planner...
  ```

- `coder.md`

  ```md
  You are an expert Python software developer...
  ```

- `reviewer.md`

  ```
  You are an export code reviewer...
  ```

- `documenter.md`

  ```
  You are an expert technical documenter...
  ```
