# Co-Bot

> Ergonomic asynchronous human-AI collaboration

> Industrial-grade AI engineering


(logo: human and robot high-fiving each other)

Co-bot allows you to do sophisticated multi-agent coding workflows using the LLM
of your choice, on your own hardware. You interact with the LLM through the UI
of your code forge, similar to how you code review contributions from humans.

The asynchronous workflow gives you time to think and interact when its
convenient for you. This reduces multi-tasking pressure when operating multiple
agent instances. AI gets time to think, test hypotheses, and run tests without
interrupting human flow.

Human review of AI results is a built-in part of the process.

All essential web interaction happen through non-AI codepaths. The AI can run
locally, reducing the risk of prompt injection and information exfiltration.

## How to use it

The co-bot workflow demonstrates how the human user and co-bot work together to
implement a ticket.

### Prepare the codebase

1. human gets the codebase ready for coding on their machine
   - install all the development dependencies
   - run the tests to make sure they pass
2. human adds the co-bot config file to the codebase

### Start the agent

3. human starts the cobot CLI
   - in a terminal, container, or VM: `cobot run <ticket>` (ticket id or URL)
4. co-bot creates a new Git worktree for the ticket
5. co-bot creates a Git branch for the ticket

### Planning

6. co-bot creates the implementation plan
   - by running the planner agent
   - the planner agent creates the implementation plan
     - based on the planning instructions for this repo and the ticket text
   - co-bot reviews the implementation plan internally
7. co-bot creates a draft PR with the implementation plan
   - `.co-bot/plan.md`
   - this file will be removed when the ticket is done
8. human reviews the implementation plan via the forge UI
   - human adds comments and modifications through the forge UI
     - like a normal code review
   - co-bot implements all suggestions and pings the human for another review
     round
   - planning is done when the human approves the plan

### Implementation

9. co-bot implements the planned code changes
   - each action item as a separate query
   - red/green TDD
   - documentation agent adds/updates documentation
   - commit after each activity

### Automated review

10. co-bot performs an automated review of the changes just made
    - review-agent reviews the changes and makes comments
    - coding-agent implements the feedback
    - review ends is finished when the review-agent has no more relevant
      feedback

### Human review

11. co-bot pings the human in the PR to review the code
12. human reviews the changes
    - adds comments, modifications, and instructions what to change via the
      forge UI (like a normal code review)
    - runs another "improve this yourself" command
      - this triggers another automated review round
    - coding-agent implements all suggestions
    - review-agent performs an automated review to verify that the changes just
      implemented address the given feedback
    - co-bot pings the human for another review round
    - review is done when the human merges the PR

### Finalize

13. co-bot adds a PR comment with statistics
    - time and money spent

14. co-bot cleans up the Git workspace
    - removes the local Git branch
    - removes the worktree
    - exits

## How it works

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
human-account = "kevgo"                 # forge user account of the human user
bot-account = "co-bot"                  # forge user account of bot
bot-token-env-var = "GITHUB_TOKEN"  # name of the env var from which to read the access token for the forge

[coding]
llm-cli = "wibey -p '{prompt}'"  # placeholder for the prompt
timeout = "1h"                   # abort and reach out to the human if the coding takes longer than 1h
after-code = "make test"         # code snippet to run after

[phases.plan]
model = "opus"
query-file = "query.md"      # file that contains the plan query, could also use `query` to make the query inline

[phases.code]
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
