# User Story: Configuration File

## Story

**As a** developer,
**I want to** store my Grove settings in a configuration file,
**So that** I don't have to type long CLI commands every time and my team shares the same analysis settings.

## Command

```
grove unused
```

(Reads from config file automatically)

Or with explicit config path:

```
grove unused --config grove.json
```

## Why This Is Needed

Projects have stable configurations — the same entry points, the same ignore patterns, the same output format. A config file avoids repeating these on every run and ensures consistency across the team.

## Expected Behavior

### Config File Discovery (searched in order)
1. `grove.json` in project root
2. `grove.jsonc` in project root (JSON with comments)
3. `.grove.json` in project root
4. `"grove"` key in `package.json`

If `--config <path>` is provided, only that path is used (no auto-discovery).

### Config File Format

```json
{
  "entry": ["src/index.ts", "src/worker.ts"],
  "project": ["src/**/*.{ts,tsx}"],
  "ignore": ["**/*.test.ts", "**/*.spec.ts", "scripts/**"],
  "ignoreDependencies": ["webpack", "@types/node"],
  "output": "text",
  "tsconfig": "tsconfig.json"
}
```

### Merging with CLI Flags
- CLI flags **override** config file values (they don't merge)
- If `--entry` is passed on CLI, the config file's `entry` is completely replaced
- If `--ignore` is passed on CLI, the config file's `ignore` is completely replaced
- This keeps the behavior predictable: CLI always wins

### All Config Options

| Key | Type | CLI Equivalent | Description |
|-----|------|----------------|-------------|
| `entry` | `string[]` | `--entry` | Entry file patterns |
| `project` | `string[]` | `--project` | Project file patterns |
| `ignore` | `string[]` | `--ignore` | File ignore patterns |
| `ignoreDependencies` | `string[]` | `--ignore-deps` | Dependencies to skip |
| `output` | `string` | `--output` | Output format (text, json) |
| `tsconfig` | `string` | `--tsconfig` | Path to tsconfig.json |
| `includeEntryExports` | `boolean` | `--include-entry-exports` | Report unused exports in entry files |

## Acceptance Criteria

- [ ] Grove auto-discovers config files in the order listed above
- [ ] `--config` flag overrides auto-discovery
- [ ] Config file is parsed as JSON (with comments support for `.jsonc`)
- [ ] Config values in `package.json` under `"grove"` key are supported
- [ ] CLI flags override corresponding config values completely
- [ ] Unknown keys in config file produce a warning (not an error)
- [ ] Invalid JSON produces a clear error message with file path and line number
- [ ] If no config file is found, defaults are used (no error)

## How to Test

1. Create `grove.json` with `{"entry": ["src/app.ts"], "ignore": ["tests/**"]}`
2. Run `grove unused` (no CLI flags)
3. Verify it uses `src/app.ts` as entry (not default patterns)
4. Verify files in `tests/` are not analyzed
5. Run `grove unused --entry "src/other.ts"`
6. Verify it uses `src/other.ts` as entry (overrides config)

## Edge Cases

- Config file has a syntax error → clear error message, tool stops
- Config file has unknown keys → warning, rest of config still used
- Both `grove.json` and `"grove"` in `package.json` exist → `grove.json` wins (first in search order)
- Config file specifies relative paths → resolved relative to the config file's directory
- Empty config file (`{}`) → all defaults used, no error
