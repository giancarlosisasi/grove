# User Story: Output Formats

## Story

**As a** developer or CI system,
**I want to** choose how Grove formats its results,
**So that** I can read them easily in the terminal, parse them in scripts, or display them in GitHub Actions.

## Command

```
grove unused --output text       # default, human-readable
grove unused --output json       # machine-readable JSON
grove unused --output github     # GitHub Actions annotations
```

## Available Formats

### 1. Text (default)

Human-readable, grouped by category, with counts.

```
Unused Files (2)
  src/utils/old-helper.ts
  src/components/DeprecatedButton.tsx

Unused Exports (3)
  src/utils/math.ts
    - multiply (function)
    - divide (function)
  src/api/client.ts
    - debugMode (variable)

Unused Dependencies (1)
  moment

Found 6 issues.
```

When no issues are found:
```
No issues found.
```

### 2. JSON

Machine-readable JSON to stdout. Suitable for piping to other tools, storing as artifacts, or custom reporting.

```json
{
  "unusedFiles": [
    "src/utils/old-helper.ts",
    "src/components/DeprecatedButton.tsx"
  ],
  "unusedExports": [
    {
      "file": "src/utils/math.ts",
      "exports": [
        { "name": "multiply", "kind": "function", "line": 15 },
        { "name": "divide", "kind": "function", "line": 22 }
      ]
    }
  ],
  "unusedDependencies": ["moment"],
  "unusedDevDependencies": ["@types/express"],
  "summary": {
    "totalIssues": 6,
    "unusedFiles": 2,
    "unusedExports": 3,
    "unusedDependencies": 1,
    "unusedDevDependencies": 0
  }
}
```

### 3. GitHub Actions

Uses GitHub Actions workflow commands to create inline annotations on files.

```
::warning file=src/utils/old-helper.ts::Unused file - not imported by any other file
::warning file=src/utils/math.ts,line=15::Unused export: multiply (function)
::warning file=src/utils/math.ts,line=22::Unused export: divide (function)
```

These show as annotations directly on the file in GitHub PR reviews.

## Acceptance Criteria

- [ ] `--output text` produces human-readable grouped output (default)
- [ ] `--output json` produces valid JSON to stdout
- [ ] `--output github` produces GitHub Actions annotation format
- [ ] JSON output includes file paths, export names, kinds, and line numbers
- [ ] Text output includes issue counts per category and total
- [ ] When no issues found, each format has an appropriate "clean" output
- [ ] Output goes to stdout (can be redirected to a file with `> output.txt`)
- [ ] Progress indicators go to stderr (so they don't interfere with stdout redirection)

## How to Test

1. Create a project with known unused files and exports
2. Run `grove unused --output json | python -m json.tool` → verify valid JSON
3. Run `grove unused --output text` → verify readable, grouped output
4. Run `grove unused --output github` → verify `::warning` format

## Edge Cases

- Very large output (hundreds of unused items) → text format should still be readable (consider truncation with "and N more...")
- JSON with special characters in file paths → properly escaped
- No issues found → JSON returns empty arrays, text says "No issues found"
- `--output` with invalid value → error message listing valid options
