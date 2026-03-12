# Co-Bot

> Ergonomic asynchronous human-AI collaboration

- gives humans time to think
- gives AI time to think and test hypotheses
- the most direct path to high-quality AI results
- the tool to reach for when creating high-quality results that require human
  guidance and review

Icon: human and robot high-fiving each other

## Functionality

### Kickoff

- I run the CLI app on a separate desktop on my computer
- I tell it to implement a ticket (Jira, GitHub)
  - it prints all applicable tickets, I select one
- it creates a new Git worktree for the ticket

### Planning

- the planner agent analyzes the problem and creates an implementation plan
  - based on the planning instructions for this repo
  - based on the ticket text
- it opens a draft PR for this ticket containing an MD file with the
  implementation plan (`.co-bot/ticket.md`)
- implementation plan review loop
  - human adds comments, modifications, and instructions what to change to the
    file via the forge UI (like a normal code review)
  - AI implements all suggestions
  - review loop is done when the human approves the plan, otherwise do another
    iteration

### Implementation

- coding agent implements the plan
  - one action item at a time
  - each action item as a new query
    - plan as context
    - summary of existing changes as context
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
  VM/Docker, or on a cloud VM
- it runs completely headless, I interact with it through my code forge web UI
- `ai-team init` creates the config and prompt files
- receives updates from the forge by polling the forge API

Config file:

```toml
[tracker]
type = "Jira"
url = "https://jira.walmart.com"
human = "k0g0kip"
token_env_var = "JIRA_TOKEN"  # name of the env var from which to read the access token for the tracker

[forge]
type = "github"
url = "gec01.walmart.com"
human = "kevgo"                 # forge user account of the human user
bot = "co-bot"                  # forge user account of bot
token_env_var = "GITHUB_TOKEN"  # name of the env var from which to read the access token for the forge

[coding]
llm-cli = "wibey -p '{prompt}'"  # placeholder for the prompt
timeout = "1h"                   # abort and reach out to the human if the coding takes longer than 1h

[agents.planner]
model = "opus"

[agents.coder]
model = "sonnet"

[agents.reviewer]
model = "opus"

[agents.documenter]
model = "sonnet"

[workflows.bug-fix]
phases = [ "build", "test", "review" ]

[workflows.feature]
phases = [ "plan", "build", "test", "review" ]
```

Prompt files in the `.co-bot/phases` folder:

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
