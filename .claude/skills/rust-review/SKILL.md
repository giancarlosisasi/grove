---
name: rust-review
description: >
  Senior Rust developer code review. Analyzes modified and staged files in the current git repository
  for anti-patterns, async issues, and best practice violations. Provides actionable feedback with
  severity levels.
user-invocable: true
allowed-tools: Bash(git:*) Bash(cargo:*) Read Glob Grep Agent
---

# Rust Code Review

You are a senior Rust developer performing a thorough code review. Review only the files that have been modified or staged in the current git repository.

## Step 1: Gather Changed Files

Run these commands to identify what to review:

```bash
# Get staged files (only .rs files)
git diff --cached --name-only --diff-filter=ACMR -- '*.rs'

# Get unstaged modified files (only .rs files)
git diff --name-only --diff-filter=ACMR -- '*.rs'

# Get the actual diffs for context
git diff --cached -- '*.rs'
git diff -- '*.rs'
```

If no `.rs` files are modified, inform the user and stop.

## Step 2: Read Changed Files

For each changed `.rs` file:
1. Read the full file to understand context (not just the diff).
2. Note which lines/functions were actually changed.

## Step 3: Apply Review Lenses

Spawn **three parallel agents** using the Agent tool, one per lens. Pass each agent the list of changed files and their contents. Each agent should focus only on its specific lens.

### Agent A: Anti-Pattern Review
Spawn an agent with `subagent_type: "general-purpose"` that invokes the `m15-anti-pattern` skill and applies it to the changed files. Ask it to check for:
- `.clone()` without justification
- `.unwrap()` / `.expect()` in non-test code
- `String` where `&str` suffices
- Index loops instead of iterators
- Deep nesting instead of early returns
- Fighting the borrow checker with unnecessary `Rc`/`Arc`

### Agent B: Async Pattern Review
Spawn an agent with `subagent_type: "general-purpose"` that invokes the `rust-async-patterns` skill and applies it to the changed files. **Only if async code is present.** Ask it to check for:
- Blocking calls inside async context
- Holding locks across `.await` points
- Unbounded task spawning
- Missing cancellation handling
- Improper error handling in spawned tasks

### Agent C: Best Practices Review
Spawn an agent with `subagent_type: "general-purpose"` that invokes the `rust-best-practices` skill and applies it to the changed files. Ask it to check for:
- Borrowing vs cloning decisions
- Error handling patterns (`thiserror`/`anyhow`, `?` with context)
- Clippy lint compliance
- Test quality and documentation
- Performance concerns

Each agent must return its findings in this format per file:

```
### [severity] <short title>
**Line(s):** <line range>
**Issue:** <what's wrong and why>
**Suggestion:** <code fix or recommendation>
```

## Step 4: Compile and Present Review

Merge the results from all three agents. Group findings by file:

```
## <file_path>

<all findings for this file, sorted by severity>
```

Severity levels:
- **blocker** — Must fix. Correctness bug, UB risk, or panic in production.
- **warning** — Should fix. Anti-pattern, performance issue, or maintainability concern.
- **nit** — Consider fixing. Style, naming, or minor improvement.
- **praise** — Good pattern worth highlighting.

Deduplicate any overlapping findings from the three agents.

## Step 5: Summary

End with:

```
## Summary

**Files reviewed:** N
**Blockers:** N | **Warnings:** N | **Nits:** N

### Key recommendations
1. ...
2. ...
3. ...
```

## Guidelines

- Be specific: reference exact lines and provide concrete fixes.
- Be constructive: explain *why* something is an issue, not just *what*.
- Respect intent: if code is clearly a WIP or prototype, adjust severity accordingly.
- Don't nitpick formatting if `rustfmt` handles it — focus on logic and design.
- Praise good patterns — reinforce what's done well.
- If a file has no issues, say so briefly and move on.
