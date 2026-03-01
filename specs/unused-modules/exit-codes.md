# User Story: Exit Codes

## Story

**As a** developer using CI/CD,
**I want** Grove to return meaningful exit codes,
**So that** my CI pipeline can automatically fail when unused code is detected.

## Command

```
grove unused
```

(Exit codes are automatic based on results)

To suppress non-zero exit:

```
grove unused --no-exit-code
```

## Exit Code Definitions

| Code | Meaning |
|------|---------|
| `0` | Successful run, **no issues found** |
| `1` | Successful run, **issues found** (unused files, exports, or deps) |
| `2` | **Error** — configuration error, file system error, or invalid arguments |

## Expected Behavior

1. After analysis completes, if ANY issues are reported → exit code `1`
2. If no issues are found → exit code `0`
3. If the tool fails before completing analysis → exit code `2`
4. `--no-exit-code` flag forces exit code `0` even when issues are found (useful for advisory mode)

## CI Integration Examples

```yaml
# GitHub Actions - fail on unused code
- name: Check unused code
  run: grove unused

# GitHub Actions - advisory only (don't fail)
- name: Check unused code (advisory)
  run: grove unused --no-exit-code
```

```bash
# Shell script
grove unused
if [ $? -eq 1 ]; then
  echo "Found unused code!"
fi
```

## Acceptance Criteria

- [ ] Exit code `0` when no issues found
- [ ] Exit code `1` when any issues found
- [ ] Exit code `2` when tool errors (bad config, file access error, etc.)
- [ ] `--no-exit-code` forces exit `0` regardless of results
- [ ] Exit code is correct even when output is suppressed or redirected
- [ ] Error messages go to stderr, results go to stdout

## How to Test

1. Run `grove unused` on a clean project → `echo $?` returns `0`
2. Run `grove unused` on a project with unused files → `echo $?` returns `1`
3. Run `grove unused --config nonexistent.json` → `echo $?` returns `2`
4. Run `grove unused --no-exit-code` on a project with issues → `echo $?` returns `0`

## Edge Cases

- Warnings only (no errors) → exit code `1` (warnings are still issues)
- `--no-exit-code` with a tool error → still exits `2` (tool errors are not suppressible)
