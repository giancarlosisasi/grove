# User Story: Global CLI Arguments

## Story

**As a** developer,
**I want** standard global flags available across all Grove commands,
**So that** I can control common behaviors like verbosity, version display, and help without remembering command-specific flags.

## Commands

```
grove --help
grove --version
grove unused --verbose
grove circular --verbose --no-progress
```

## Global Flags

### `--help` / `-h`

Shows help text for the current command.

```
$ grove --help
Grove - Find unused code and circular dependencies in JS/TS projects

Usage:
  grove [command]

Available Commands:
  unused      Find unused files, exports, and dependencies
  circular    Find circular dependencies
  version     Print version information

Flags:
  -h, --help       Show help
  -v, --version    Show version
      --verbose    Enable verbose output
      --no-progress  Disable progress indicators

Use "grove [command] --help" for more information about a command.
```

Subcommand help:

```
$ grove unused --help
Find unused files, exports, and dependencies

Usage:
  grove unused [flags]

Flags:
      --entry <pattern>      Entry file patterns (repeatable)
      --project <pattern>    Project file patterns (repeatable)
      --ignore <pattern>     Ignore patterns (repeatable)
      --ignore-deps <name>   Ignore dependencies (repeatable)
      --config <path>        Path to config file
      --output <format>      Output format: text, json, github (default: text)
      --tsconfig <path>      Path to tsconfig.json
      --production           Analyze production code only
      --fix                  Auto-fix unused exports
      --dry-run              Show what --fix would do
      --include <type>       Include only specific issue types
      --exclude <type>       Exclude specific issue types
      --include-entry-exports  Report unused exports in entry files
      --no-exit-code         Always exit 0
  -h, --help                 Show help
```

### `--version` / `-v`

Prints the version and exits.

```
$ grove --version
grove v0.1.0 (darwin/arm64)
```

### `--verbose`

Enables verbose/debug output. Shows additional information:
- Configuration being used (resolved config file, entry patterns, project patterns)
- Number of files discovered
- Number of files parsed
- Time spent in each phase (discovery, parsing, graph building, analysis)
- Module resolution details (which imports resolved to which files)

Verbose output goes to **stderr** so it doesn't interfere with stdout.

```
$ grove unused --verbose
[debug] Config: grove.json
[debug] Entry patterns: ["src/index.ts"]
[debug] Project patterns: ["src/**/*.{ts,tsx}"]
[debug] Discovered 142 files in 0.03s
[debug] Parsed 142 files in 0.45s
[debug] Built graph: 142 nodes, 387 edges
[debug] Analysis complete in 0.02s

Unused Files (3)
  ...
```

### `--no-progress`

Disables the progress bar/indicator. Useful for:
- CI environments where progress bars create noise
- Piping output to a file
- Clean log output

Default: progress is shown when stderr is a TTY, hidden otherwise.

### `--directory` / `-d`

Sets the working directory for the command:

```
grove unused -d /path/to/project
```

Equivalent to `cd /path/to/project && grove unused`.

### `--concurrency` / `-j`

Sets the number of parallel workers for file parsing:

```
grove unused -j 4
```

Default: `runtime.NumCPU()` (number of CPU cores).

## Acceptance Criteria

- [ ] `--help` / `-h` shows help for root command and each subcommand
- [ ] `--version` / `-v` prints version string and exits
- [ ] `--verbose` enables debug output to stderr
- [ ] `--no-progress` suppresses progress indicators
- [ ] `--directory` changes the working directory
- [ ] `--concurrency` controls parallel worker count
- [ ] All global flags work with all subcommands
- [ ] Unknown flags produce a clear error with suggestion
- [ ] Flags can use `=` syntax: `--output=json`

## How to Test

1. Run `grove --help` → shows available commands and global flags
2. Run `grove unused --help` → shows unused-specific flags
3. Run `grove --version` → prints version string
4. Run `grove unused --verbose` → verbose output on stderr, results on stdout
5. Run `grove unused --no-progress` → no progress bar
6. Run `grove unused -d ./other-project` → analyzes other-project
7. Run `grove --invalid-flag` → error message suggesting valid flags

## Edge Cases

- `grove` with no command → show help (same as `--help`)
- `grove unknown-command` → error: "unknown command 'unknown-command'. Run 'grove --help' for available commands."
- `-v` short flag → version (not verbose; verbose is `--verbose` only)
- Combine `--verbose` with `--output json` → verbose on stderr, clean JSON on stdout
