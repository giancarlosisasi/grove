# User Story: Circular Detection Exit Codes

## Story

**As a** developer using CI/CD,
**I want** Grove's circular command to return meaningful exit codes,
**So that** my CI pipeline automatically fails when circular dependencies are detected.

## Command

```
grove circular ./src/index.ts
```

Or with custom exit code:

```
grove circular ./src/index.ts --exit-code circular:1
```

## Exit Code Definitions

### Default Behavior

| Code | Meaning |
|------|---------|
| `0` | Successful run (regardless of whether circulars were found) |
| `2` | Error — invalid arguments, file not found, config error |

By default, finding circular dependencies does NOT cause a non-zero exit. This is "advisory mode."

### With `--exit-code`

The `--exit-code` flag lets you specify a custom exit code for specific conditions:

```
grove circular ./src/index.ts --exit-code circular:1
```

| Code | Meaning |
|------|---------|
| `0` | No circular dependencies found |
| `1` | Circular dependencies found (custom, specified by user) |
| `2` | Error |

The format is `CONDITION:CODE` where:
- `CONDITION` is `circular` (the only supported condition for now)
- `CODE` is an integer 0-128

## Why Default is Exit 0

Unlike the `unused` command (which exits 1 on issues by default), the `circular` command defaults to exit 0 even with circulars. This matches dpdm's behavior and reflects that:

1. Many projects have some circular dependencies that are harmless
2. Adding circular detection to CI should be an explicit opt-in decision
3. Teams need time to fix existing circulars before enforcing in CI

When a team is ready to enforce, they add `--exit-code circular:1`.

## Acceptance Criteria

- [ ] Default exit code is `0` even when circulars are found
- [ ] `--exit-code circular:N` exits with code N when circulars are found
- [ ] Exit code `2` for tool errors (always, regardless of `--exit-code`)
- [ ] Code N must be between 0 and 128
- [ ] Invalid `--exit-code` format produces a clear error
- [ ] Works correctly with `--output json` and other flags

## How to Test

1. Create a project with circular dependencies
2. Run `grove circular ./src/index.ts` → `echo $?` returns `0`
3. Run `grove circular ./src/index.ts --exit-code circular:1` → `echo $?` returns `1`
4. Run on a project with no circulars and `--exit-code circular:1` → `echo $?` returns `0`
5. Run with invalid args → `echo $?` returns `2`

## CI Integration Example

```yaml
# GitHub Actions
- name: Check for circular dependencies
  run: grove circular ./src/index.ts --exit-code circular:1

# Only warn, don't fail
- name: Check for circular dependencies (advisory)
  run: grove circular ./src/index.ts
```

## Edge Cases

- `--exit-code circular:0` → valid but has no practical effect (always exit 0)
- `--exit-code circular:129` → error, code must be 0-128
- Multiple `--exit-code` flags → last one wins or error (keep it simple: error)
