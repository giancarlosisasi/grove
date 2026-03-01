# User Story: npm Distribution

## Story

**As a** JavaScript/TypeScript developer,
**I want to** install Grove via npm,
**So that** I can use it like any other dev tool in my project without needing to install Go or compile anything.

## Command

```bash
# Install globally
npm install -g grove

# Install as devDependency
npm install -D grove

# Run via npx (no install)
npx grove unused
```

## How It Works

Grove is a Go binary but distributed as an npm package. When a user runs `npm install grove`, a post-install script downloads the correct pre-compiled binary for their platform.

### Install Flow

1. User runs `npm install grove`
2. npm downloads the package (small, just the install script + package.json)
3. The `postinstall` script runs
4. The script detects the platform (OS + architecture)
5. It downloads the pre-compiled Go binary from GitHub Releases
6. The binary is placed in `node_modules/.bin/grove`
7. User can now run `grove` or `npx grove`

### Supported Platforms

| Platform | Architecture | Binary Name |
|---|---|---|
| macOS | Intel (x64) | `grove-darwin-amd64` |
| macOS | Apple Silicon (arm64) | `grove-darwin-arm64` |
| Linux | x64 | `grove-linux-amd64` |
| Linux | arm64 | `grove-linux-arm64` |
| Windows | x64 | `grove-windows-amd64.exe` |

### npm Package Structure

```
grove/
├── package.json
├── install.js         # postinstall script
├── bin/
│   └── grove          # placeholder, replaced by actual binary on install
└── README.md
```

### package.json

```json
{
  "name": "grove",
  "version": "0.1.0",
  "description": "Find unused code and circular dependencies in JS/TS projects",
  "bin": {
    "grove": "./bin/grove"
  },
  "scripts": {
    "postinstall": "node install.js"
  },
  "os": ["darwin", "linux", "win32"],
  "cpu": ["x64", "arm64"],
  "engines": {
    "node": ">=14"
  }
}
```

### install.js Logic

1. Detect `process.platform` → `darwin`, `linux`, `win32`
2. Detect `process.arch` → `x64`, `arm64`
3. Map to binary name: `grove-{os}-{arch}`
4. Construct download URL: `https://github.com/{owner}/grove/releases/download/v{version}/grove-{os}-{arch}`
5. Download the binary using `https` module (no external dependencies)
6. Save to `./bin/grove` (or `./bin/grove.exe` on Windows)
7. Set executable permissions (`chmod +x`)

### Fallback: No Network

If the download fails (corporate proxy, no internet), the install script should:
1. Print a clear error with the download URL so the user can download manually
2. NOT fail `npm install` (use `||` true or catch the error) — some CI environments have restricted network
3. Provide instructions for manual binary placement

## Alternative Distribution Strategy: Optional Dependencies

An alternative to a postinstall script is using npm optional dependencies with platform-specific packages:

```json
{
  "optionalDependencies": {
    "grove-darwin-arm64": "0.1.0",
    "grove-darwin-x64": "0.1.0",
    "grove-linux-arm64": "0.1.0",
    "grove-linux-x64": "0.1.0",
    "grove-win32-x64": "0.1.0"
  }
}
```

Each platform package contains the binary. npm automatically installs only the matching platform. This is how esbuild, swc, and turbo distribute their binaries.

This approach is more robust (no postinstall network requests, works with all package managers) but requires publishing 6 npm packages instead of 1.

## Acceptance Criteria

- [ ] `npm install grove` works on macOS (Intel + ARM), Linux (x64 + ARM), Windows (x64)
- [ ] After install, `grove --version` works
- [ ] `npx grove unused` works without prior install
- [ ] Failed download produces a helpful error message, not a cryptic crash
- [ ] No Go runtime needed on the user's machine
- [ ] Binary is the correct one for the platform (no wrong-architecture binaries)
- [ ] Package size is minimal (just the install script, not all platform binaries)

## How to Test

1. On macOS ARM: `npm install grove && grove --version` → prints version
2. On Linux x64: same test
3. Kill network, run `npm install grove` → clear error with manual download instructions
4. Run `npx grove unused` in a project → works without prior install
5. Check `ls -la node_modules/.bin/grove` → executable permissions set

## Release Process

1. Build Go binaries for all platforms
2. Create GitHub Release with all binaries attached
3. Update version in npm `package.json`
4. Publish to npm: `npm publish`
5. Verify: `npx grove@latest --version`

## Edge Cases

- User on unsupported platform (e.g., FreeBSD) → clear error: "Unsupported platform. Build from source with: go install github.com/..."
- User behind corporate proxy → install script respects `HTTP_PROXY` / `HTTPS_PROXY` environment variables
- `--ignore-scripts` flag in npm → binary not downloaded, provide docs for manual install
- Multiple package managers (npm, yarn, pnpm, bun) → all should work (postinstall is standard)
- Lockfile pinning → version in lockfile matches the binary version downloaded
