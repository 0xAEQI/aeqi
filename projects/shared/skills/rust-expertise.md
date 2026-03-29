# Rust Expertise — Shared Knowledge Base

Domain knowledge for all Rust projects. Retrieved via `sigil_skills(action="get", name="rust-expertise")`.

## Safety

- No `.unwrap()` / `.expect()` in production paths — use `?` with `.context()` or `.map_err()`
- No `unsafe` without `// SAFETY:` comment documenting invariants
- No SQL injection — always parameterized queries
- No command injection — validate input before `std::process::Command`
- No path traversal — canonicalize + prefix check on user-controlled paths
- No hardcoded secrets in source
- No `panic!()` / `todo!()` / `unreachable!()` in production paths
- `Box<dyn Error>` in binaries is fine; in libraries use `thiserror` for typed errors

## Ownership & Lifetimes

- No unnecessary `.clone()` to satisfy the borrow checker — understand the root cause first
- `&str` over `String` when ownership isn't needed; `impl AsRef<str>` for generic APIs
- `&[T]` over `Vec<T>` for read-only access
- `Cow<'_, str>` when you sometimes need to allocate and sometimes don't
- Don't over-annotate lifetimes — use elision where the compiler allows it
- Watch for moves in loops — prefer `iter()` / `iter_mut()` over `into_iter()` when reusing

## Async & Concurrency

- No `std::sync::Mutex` in async code — use `tokio::sync::Mutex`
- No `std::thread::sleep` in async — use `tokio::time::sleep`
- No `std::fs` in async — use `tokio::fs`
- Prefer bounded channels (`tokio::sync::mpsc::channel(n)`) — unbounded needs justification
- NEVER bare `recv()` in `tokio::select!` — always `recv_timeout()`
- NEVER slow async work inside `select!` arms — defer with flag, handle after
- No crossbeam blocking calls inside `tokio::spawn` contexts
- Handle `PoisonError` from `Mutex::lock()` — don't ignore
- Consistent lock ordering to prevent deadlocks
- `Send`/`Sync` bounds on types shared across threads

## Performance

- No allocations in hot paths: `Vec::new()`, `String::new()`, `Box::new()`, `format!()`, `.clone()`, `.to_string()`, `.to_owned()`
- Use `Vec::with_capacity(n)` when size is known
- No repeated allocation in loops — hoist allocations out
- No `Mutex`/`RwLock` in hot paths — use `DashMap`, atomics, or lock-free structures
- Pre-allocated buffers for per-tick code
- `rust_decimal` for money, `f64` only on hot paths where precision trade-off is acceptable
- Watch for N+1 query patterns in database code

## SQL & Database

- `ON CONFLICT` requires a unique index — verify with `\d tablename` before writing upserts
- Correct SQL types: `i32` vs `i64`, proper decimal handling
- Parameterized queries only — never string interpolation
- Batch inserts (`COPY`) over individual `INSERT` for bulk writes
- Connection pool handles must be returned — watch for leaks

## Code Quality

- Functions under 50 lines
- Nesting under 4 levels
- No wildcard match (`_ =>`) on business enums — forces handling new variants
- No dead code — remove unused functions, imports, variables
- Clippy clean: `cargo clippy --workspace --all-targets -- -D warnings`
- Format clean: `cargo fmt --check`
- Derive order: `Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize`

## Error Handling Patterns

```rust
// Library errors — typed with thiserror
#[derive(Debug, thiserror::Error)]
enum MyError {
    #[error("connection failed: {0}")]
    Connection(#[from] std::io::Error),
    #[error("invalid config: {reason}")]
    Config { reason: String },
}

// Application errors — anyhow with context
use anyhow::Context;
let data = std::fs::read(path)
    .context("failed to read config file")?;

// Never silently discard #[must_use]
// BAD:  let _ = sender.send(msg);
// GOOD: sender.send(msg).ok();  // or log the error
```

## Zero-Allocation Patterns

```rust
// Cow for conditional allocation
use std::borrow::Cow;
fn normalize(input: &str) -> Cow<'_, str> {
    if input.contains(' ') {
        Cow::Owned(input.replace(' ', "_"))
    } else {
        Cow::Borrowed(input)
    }
}

// Pre-allocated buffer reuse
let mut buf = Vec::with_capacity(1024);
loop {
    buf.clear();  // reuse allocation
    // ... fill buf ...
}
```
