# Implementation Plan: `main_root_pwd.c_23`

## Summary

This module ports the `pwd.c` main-cluster logic associated with `usage` and `nth_parent` into Rust on branch `023-main_root_pwd.c_23-rust-port`.

The Rust implementation should remain narrowly aligned with the existing C file layout and behavior:

- migrate the command-support logic currently in `pwd.c`
- preserve the current control-flow role of `usage`
- reimplement `nth_parent` using safe Rust path/string handling where possible
- keep behavior local to the existing module scope instead of introducing new subsystem layers

Technical approach:

- use `std::path::{Path, PathBuf}` for parent-path traversal logic
- use owned UTF-8 aware strings where valid, but avoid assuming more than the original C code guarantees; where path handling must remain platform-correct, prefer `Path`/`OsStr` over raw `String`
- represent fallible operations with `Result` and explicit error propagation instead of sentinel returns
- keep module boundaries minimal: map `pwd.c` into a single Rust source file in the existing binary crate structure, with helper functions corresponding directly to the C functions

The migration should focus on replacing C memory and pointer manipulation with Rust ownership and borrowing, without adding extra capabilities beyond the original file’s responsibilities.

## Technical Context

- **Language/Version:** Rust 1.78+
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - equivalent asymptotic behavior to the C implementation
  - no unnecessary path cloning in repeated parent traversal
  - bounded allocations, limited to path/result construction that replaces C buffer handling
  - startup and command execution overhead should remain negligible for a small CLI utility

## Module Mapping

| C Module/File | Rust Module/File | Notes |
|---|---|---|
| `pwd.c` | `src/bin/pwd.rs` or the project’s existing Rust binary entry file for `pwd` | Keep this migration in the existing binary target structure; do not split unless already required by the repository layout. |
| `usage` | `fn usage(...) -> !` or `fn usage(...)` | Match current call sites. If the function always terminates in C, model it with a diverging function `-> !`; otherwise return normally and let caller decide exit behavior. |
| `nth_parent` | `fn nth_parent(...) -> Option<PathBuf>` or `Result<PathBuf, _>` | Use `Path`/`PathBuf` to model repeated parent lookup. Final signature should follow observed C success/failure semantics. |

## Data Model

The analysis lists only anonymous C data structures and no named structs directly tied to this module’s exported logic. For this port, data mapping should stay minimal and only introduce Rust types where required by the migrated functions.

| C Data Shape | Rust Mapping | Usage |
|---|---|---|
| anonymous local struct/temporary state | local variables / tuples | Prefer function-local bindings instead of inventing named structs. |
| C string pointers (`char *`, `const char *`) | `&str`, `String`, `&OsStr`, or `PathBuf` | Use `&str` only for text known to be UTF-8; use `Path`/`OsStr` for filesystem paths. |
| C integer counters | `usize` or `u32`/`i32` as appropriate | Parent-depth counters should default to `usize` unless interoperability with existing signatures requires another type. |
| C boolean-like flags | `bool` | Replace integer flag checks with explicit booleans. |
| C sentinel error returns | `Option<T>` / `Result<T, E>` | Choose `Option` for simple absence, `Result` when caller must distinguish failures. |

### Planned Rust Representations

#### `usage`
No dedicated data structure is required. The Rust function should accept only the minimal argument set needed by existing call sites, typically:

- program name or command name as `&str`
- possibly an output stream choice encoded through call-site control rather than a custom type

If the C implementation formats static help text only, preserve that as string literals.

#### `nth_parent`
Model inputs and outputs with path-safe types:

```rust
fn nth_parent(path: &Path, n: usize) -> Option<PathBuf>
```

Alternative signature if the original behavior needs richer diagnostics:

```rust
fn nth_parent(path: &Path, n: usize) -> Result<PathBuf, NthParentError>
```

Only introduce a small local error enum if the caller must react differently to invalid depth vs missing parent; otherwise prefer `Option<PathBuf>` to avoid unnecessary type expansion.

## Implementation Phases

## Phase 1: Establish file-level Rust skeleton

- identify the Rust binary file corresponding to `pwd.c`
- create or update the target Rust file without introducing extra modules
- add direct Rust equivalents for `usage` and `nth_parent` as stubs
- map existing C includes/macros used by these functions to standard-library imports
- determine whether `usage` should return normally or terminate, based on existing C call behavior

### Deliverables
- Rust file updated in the `pwd` binary target
- function signatures fixed for `usage` and `nth_parent`
- compilation succeeds with placeholder logic where needed

## Phase 2: Port `nth_parent` logic

- translate the parent traversal algorithm from pointer/buffer logic into `Path`/`PathBuf` operations
- preserve edge-case handling from the C code:
  - zero-depth requests
  - paths with insufficient ancestor depth
  - root-directory behavior
  - relative versus absolute path handling, if present in the source
- eliminate manual memory management by relying on owned `PathBuf` and borrowed `Path`
- replace any mutable C buffer rewriting with explicit path reconstruction only where necessary

### Memory and error-handling decisions
- no raw allocation APIs
- return `Option`/`Result` instead of null pointers or status integers
- avoid panics for ordinary invalid-input paths or ancestor underflow
- keep cloning constrained to the final returned path value

### Deliverables
- working Rust implementation of `nth_parent`
- unit tests for normal and boundary cases

## Phase 3: Port `usage` behavior and integrate call sites

- translate help/usage output text into Rust string literals
- route output through standard error or standard output to match existing behavior
- preserve exit semantics from the C implementation
- update calling code in the same file so error handling and usage display follow Rust control flow rather than `goto`/status patterns

### Memory and error-handling decisions
- static help text remains borrowed string data
- terminal exit paths use `std::process::exit` only if the original C function unconditionally terminates
- otherwise propagate an error/status to the caller

### Deliverables
- `usage` fully implemented
- all local call sites compiled against final signatures
- behavior-aligned output path established

## Phase 4: Validation and cleanup

- add focused `cargo test` coverage for:
  - `nth_parent` depth traversal
  - insufficient parent cases
  - root and relative path edge conditions relevant to the original code
- verify no unnecessary unsafe code is present
- remove temporary compatibility scaffolding introduced during migration
- confirm the Rust module remains a direct port of `pwd.c` responsibilities only

### Deliverables
- passing tests via `cargo test`
- final Rust implementation with minimal module surface
- code review pass for ownership, borrowing, and error propagation consistency

## Notes and Constraints

- Keep the migration confined to the logic currently present in `pwd.c`.
- Do not introduce helper crates unless later source inspection shows a concrete unmet requirement.
- Prefer safe Rust throughout; `unsafe` should not be used for these functions unless a proven source-level constraint requires it.
- Preserve observable behavior first; only simplify internals where Rust ownership clearly replaces C buffer and pointer management without changing results.