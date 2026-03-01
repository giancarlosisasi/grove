# grove

----- WIP -----

> Inspect your dependency tree. Cut the dead wood.

Grove is a fast static analysis tool for JavaScript, TypeScript, and Node.js projects.
It detects unused exports, dead code, duplicate dependencies, and circular dependency
chains — powered by Rust, built for modern JS/TS codebases.

## Features

- Unused exports and dead code detection
- Circular dependency detection
- Duplicate and redundant dependency analysis
- Fast — built in Rust with oxc parser
- Zero config to get started, fully configurable when you need it
- Supports JavaScript, TypeScript, JSX, TSX, Node.js, and React projects

## Installation
```bash
npm install -g @giancarlosio/grove
```

## Usage
```bash
# Analyze current project
grove .

# Analyze specific directory
grove ./src

# Show only circular dependencies
grove --circular ./src

# Show only unused exports
grove --unused ./src

# Output as JSON
grove --format json .
```

## Configuration

Grove works out of the box but can be configured via `grove.config.json` or inside your `package.json`:
```json
{
  "grove": {
    "entry": ["src/index.ts"],
    "ignore": ["**/*.test.ts", "**/*.spec.ts"],
    "include": ["src/**/*"]
  }
}
```

## Roadmap

- [ ] Monorepo support
- [ ] VS Code extension
- [ ] GitHub Actions integration
- [ ] HTML report output

## License

MIT
