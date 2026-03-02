# User Story: Circular Output Format

## Story

**As a** developer or CI system,
**I want to** choose the output format for circular dependency results,
**So that** I can read them in the terminal, parse them in scripts, or integrate with GitHub Actions.

## Command

```
grove circular ./src/index.ts --output text      # default
grove circular ./src/index.ts --output json       # machine-readable
grove circular ./src/index.ts --output github     # GitHub Actions
```

## Available Formats

### 1. Text (default)

```
Circular Dependencies (2)
  1) src/a.ts → src/b.ts → src/c.ts → src/a.ts
  2) src/x.ts → src/y.ts → src/x.ts

Warnings (1)
  Cannot resolve: ./missing (imported from src/index.ts)

✓ Scanned 47 files in 0.12s
```

### 2. JSON

```json
{
  "circular": [
    ["src/a.ts", "src/b.ts", "src/c.ts", "src/a.ts"],
    ["src/x.ts", "src/y.ts", "src/x.ts"]
  ],
  "warnings": [
    {
      "type": "unresolved",
      "specifier": "./missing",
      "importedFrom": "src/index.ts"
    }
  ],
  "tree": {
    "src/index.ts": [
      {"path": "src/a.ts", "kind": "static"},
      {"path": "src/b.ts", "kind": "static"}
    ]
  },
  "summary": {
    "filesScanned": 47,
    "circularCount": 2,
    "warningCount": 1,
    "elapsedMs": 120
  }
}
```

The `tree` field is only included when `--tree` is also set.

### 3. GitHub Actions

```
::warning file=src/a.ts::Circular dependency: src/a.ts → src/b.ts → src/c.ts → src/a.ts
::warning file=src/x.ts::Circular dependency: src/x.ts → src/y.ts → src/x.ts
```

### 4. JSON File Output

```
grove circular ./src/index.ts --output-file results.json
```

Writes the full dependency tree and circulars as JSON to a file (separate from `--output` which controls stdout format).

## Acceptance Criteria

- [ ] `--output text` produces human-readable numbered list (default)
- [ ] `--output json` produces valid JSON to stdout
- [ ] `--output github` produces GitHub Actions annotation format
- [ ] `--output-file` writes JSON to a specified file path
- [ ] JSON includes circular chains, warnings, and optionally the full tree
- [ ] Each format correctly handles zero circular dependencies
- [ ] Progress output goes to stderr (doesn't pollute stdout)

## How to Test

1. Create a project with known cycles
2. Run `grove circular ./src/index.ts --output json | python -m json.tool` → valid JSON
3. Run `grove circular ./src/index.ts --output text` → numbered list
4. Run `grove circular ./src/index.ts --output-file out.json` → file created with valid JSON
5. Run on a project with no cycles → each format shows appropriate "no cycles" message

## Edge Cases

- `--output json` with `--tree` → include tree in JSON
- `--output json` without `--tree` → omit tree field from JSON
- `--output-file` to a read-only path → clear error message
- `--output` with invalid value → error listing valid options
