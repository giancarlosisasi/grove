
# CLAUDE.md — Go Best Practices

## Code Style

- We are building a golang cli project that is going to be distributed on npm.
- We are using go version 1.26.0
- Write idiomatic Go — prefer simplicity over cleverness
- No global state; pass dependencies explicitly
- Return errors, don't panic (except truly unrecoverable cases)
- Keep functions short (~20-40 lines max); extract when complexity grows
- Accept interfaces, return structs
- Define interfaces where they're used, not where implemented
- Keep interfaces small (1-3 methods)

## Naming

- Use short, clear names: `cfg` not `configuration`, `buf` not `buffer`
- Avoid stuttering: `parser.Parser` → `parser.Instance`
- Acronyms in caps: `ID`, `URL`, `AST`; mixed case: `userID`, `httpClient`

## Error Handling

```go
// Always wrap errors with context
if err != nil {
    return fmt.Errorf("doing X: %w", err)
}

// Define sentinel errors for expected conditions
var ErrNotFound = errors.New("not found")

// Check with errors.Is/As
if errors.Is(err, ErrNotFound) { ... }
```

## Struct Design

```go
// Use functional options for flexible configuration
type Option func(*Service)

func WithLogger(l *slog.Logger) Option {
    return func(s *Service) { s.logger = l }
}

func New(opts ...Option) *Service {
    s := &Service{logger: slog.Default()}
    for _, opt := range opts {
        opt(s)
    }
    return s
}
```

## Concurrency

- Use `errgroup.Group` for parallel tasks with error propagation
- Limit concurrency with semaphores for I/O-bound work
- Prefer channels for communication, mutexes for state protection
- Always propagate and check `context.Context`

```go
g, ctx := errgroup.WithContext(ctx)
sem := make(chan struct{}, runtime.NumCPU())

for _, item := range items {
    item := item
    g.Go(func() error {
        sem <- struct{}{}
        defer func() { <-sem }()
        return process(ctx, item)
    })
}
return g.Wait()
```

## Performance

- Profile before optimizing: `go test -bench=. -cpuprofile=cpu.out`
- Preallocate slices: `make([]T, 0, expectedLen)`
- Use `sync.Pool` for frequently allocated objects
- Stream large data; avoid loading entirely into memory
- Reuse buffers; avoid allocations in hot paths
- Use `strings.Builder` for string concatenation

## Testing

- Table-driven tests for multiple cases
- Use `t.Parallel()` where safe
- Test public API primarily
- Use `testdata/` for fixtures

```go
func TestFoo(t *testing.T) {
    t.Parallel()
    tests := []struct {
        name    string
        input   string
        want    string
        wantErr bool
    }{
        {"valid", "input", "expected", false},
        {"error case", "bad", "", true},
    }
    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            t.Parallel()
            got, err := Foo(tt.input)
            if (err != nil) != tt.wantErr {
                t.Fatalf("error = %v, wantErr = %v", err, tt.wantErr)
            }
            if got != tt.want {
                t.Errorf("got %q, want %q", got, tt.want)
            }
        })
    }
}
```

## Logging

- Use `log/slog` for structured logging
- Debug for verbose, Info for user-facing

```go
slog.Debug("processing", "key", value)
slog.Error("failed", "err", err)
```

## Avoid

- `init()` for setup — use explicit initialization
- Returning `interface{}` or `any` — define concrete types
- Ignoring context cancellation
- `ioutil` (deprecated) — use `os` and `io`
- Shadowing `err` in nested scopes
- Ignoring errors from `Close()` on writers
- Bare `panic` for error handling