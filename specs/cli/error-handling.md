# User Story: Error Handling and Messages

## Story

**As a** developer,
**I want** Grove to provide clear, actionable error messages when something goes wrong,
**So that** I can quickly understand and fix the issue instead of guessing what happened.

## Principles

1. **Errors should explain what went wrong and suggest what to do**
2. **Warnings don't stop execution; errors do**
3. **All error/warning output goes to stderr, results go to stdout**
4. **Exit code 2 for tool errors, exit code 1 for lint issues**

## Error Categories

### 1. Configuration Errors (exit code 2)

When the tool cannot start analysis due to configuration issues.

```
Error: Config file "grove.json" has invalid JSON at line 5, column 12.

  5 |   "entry": ["src/index.ts"
                                  ^ Expected ']' or ','

Fix: Correct the JSON syntax in grove.json
```

```
Error: Config file "grove.json" not found.

The --config flag was set but the file doesn't exist.
Available config files in this directory: none

Fix: Create grove.json or remove the --config flag to use defaults.
```

### 2. File System Errors (exit code 2)

```
Error: Cannot read file "src/index.ts": permission denied.

Fix: Check file permissions with 'ls -la src/index.ts'
```

```
Error: Directory "/nonexistent/path" does not exist.

Fix: Check the --directory flag value or run from the correct directory.
```

### 3. Invalid Arguments (exit code 2)

```
Error: Unknown flag "--ouput". Did you mean "--output"?

Run 'grove unused --help' for available flags.
```

```
Error: Invalid output format "xml". Valid formats: text, json, github.
```

```
Error: --exit-code value "circular:999" is invalid. Code must be between 0 and 128.
```

### 4. Parse Warnings (continue execution)

When a file cannot be parsed but the rest of the analysis can continue.

```
Warning: Could not parse "src/broken.ts" (syntax error at line 42).
         Skipping this file. Results may be incomplete.
```

### 5. Resolution Warnings (continue execution)

When an import cannot be resolved to a file.

```
Warning: Cannot resolve "./missing-module" (imported from src/index.ts).
```

These are collected and shown in a "Warnings" section at the end.

### 6. No Entry Points Found (exit code 2)

```
Error: No entry files found.

Grove looks for these files by default:
  - index.{ts,tsx,js,jsx}
  - src/index.{ts,tsx,js,jsx}
  - main.{ts,tsx,js,jsx}
  - src/main.{ts,tsx,js,jsx}
  - Files referenced in package.json (main, bin, exports)

None of these exist in the current directory.

Fix: Specify entry files with --entry "path/to/entry.ts"
     or create a config file with entry patterns.
```

### 7. No package.json Found (warning, continue)

```
Warning: No package.json found in the current directory.
         Dependency analysis will be skipped.
         File and export analysis will still run.
```

## Output Structure

All error messages follow a consistent structure:

```
<Type>: <What happened>

<Context/details if helpful>

Fix: <What the user should do>
```

Where `<Type>` is one of:
- `Error` — stops execution, exit code 2
- `Warning` — continues execution, shown at end

## Acceptance Criteria

- [ ] All errors include a clear description of what went wrong
- [ ] Fixable errors include a "Fix:" suggestion
- [ ] Misspelled flags show "Did you mean...?" suggestions
- [ ] Invalid values show the list of valid options
- [ ] Warnings don't stop execution
- [ ] Errors stop execution with exit code 2
- [ ] Parse/resolution warnings are collected and shown in a summary section
- [ ] All errors and warnings go to stderr
- [ ] Error messages include file paths and line numbers when relevant

## How to Test

1. Run `grove unused --config nonexistent.json` → clear error about missing config
2. Run `grove unused --ouput json` → "Did you mean --output?"
3. Run `grove unused` in an empty directory → "No entry files found" with suggestions
4. Create a JS file with syntax errors → warning about skipping it, analysis continues
5. Import a non-existent module → resolution warning in summary

## Edge Cases

- Multiple errors at startup (e.g., bad config AND missing directory) → show first error, stop
- Hundreds of parse warnings → show first 10, then "and N more warnings..."
- Warning about a file inside node_modules → suppress (user can't fix vendor code)
- Error message with very long file path → don't truncate, show full path
