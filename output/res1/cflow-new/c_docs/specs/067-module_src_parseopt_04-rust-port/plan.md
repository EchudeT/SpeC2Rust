# Implementation Plan: module_src_parseopt_04

## Summary

This module ports the option-parsing support currently implemented in `src/main.c` into Rust for the `cflow-new` project branch `067-module_src_parseopt_04-rust-port`.

The C functions in scope:

- `parseopt_from_env`
- `fromfile_error`
- `fromfile`
- `optset_profile`
- `init_hook`

form a small cluster around initialization-time option sourcing and application, especially reading options from environment variables and files, reporting related errors, and applying profile-driven option sets.

The Rust implementation should preserve the existing control flow and behavior as closely as possible, with a direct migration of these functions into a focused Rust module rather than introducing new abstraction layers. The technical approach is:

- move the logic from `src/main.c` into a Rust source file under standard Cargo layout,
- represent C string/file handling with `String`, `PathBuf`, and standard I/O readers,
- convert C-style status/error signaling into `Result` and explicit error enums,
- keep initialization ordering explicit so `init_hook` remains a direct translation target,
- avoid capability expansion beyond the existing parsing and setup behavior.

## Technical Context

- **Language/Version:** Rust 1.78+
- **Primary Dependencies:** Rust standard library only (`std::env`, `std::fs`, `std::io`, `std::path`, `std::ffi` only if needed for argument/environment boundaries)
- **Testing:** `cargo test`
- **Performance Goals:**
  - Maintain behaviorally equivalent startup-time performance for environment/file option parsing.
  - Avoid unnecessary heap allocations beyond unavoidable `String`/line buffering during file reads.
  - Keep file parsing single-pass and streaming where the C implementation reads sequentially.
  - No additional background processing or caching.

## Module Mapping

### C to Rust File Mapping

| C File | Rust File | Notes |
|---|---|---|
| `src/main.c` | `src/main.rs` or `src/parseopt.rs` with invocation from `src/main.rs` | Keep migration constrained to the functions in scope; if the existing Rust port already separates logic, place the ported functions in a single dedicated module and call them from `main.rs`. |

### Function Mapping

| C Function | Rust Target | Migration Notes |
|---|---|---|
| `parseopt_from_env` | `fn parseopt_from_env(...) -> Result<..., ParseOptError>` | Read environment variables via `std::env`; preserve conditional behavior and option-token handling order. |
| `fromfile_error` | `fn fromfile_error(...) -> ParseOptError` or `fn fromfile_error(..., ...)` | Centralize translation of file/parse failures into a stable internal error type; preserve message content used by callers. |
| `fromfile` | `fn fromfile(path: &Path, ...) -> Result<..., ParseOptError>` | Replace C `FILE *` processing with `std::fs::File` and buffered reading; preserve tokenization/line semantics from C. |
| `optset_profile` | `fn optset_profile(...) -> Result<..., ParseOptError>` | Port profile option application logic directly; preserve mutation order and any overwrite/append semantics. |
| `init_hook` | `fn init_hook(...) -> Result<..., ParseOptError>` | Keep as the orchestrator for initialization-time setup; translate side effects in original order. |

### Rust Placement Decision

Prefer the smallest standard layout that fits the current migration stage:

- If these functions are only consumed by startup logic: implement them in `src/main.rs`.
- If `main.rs` would become too dense, place them in `src/parseopt.rs` and expose them with `mod parseopt;`.

This plan intentionally avoids creating additional modules beyond what is needed to host the migrated code.

## Data Model

The analysis identifies only anonymous C data structures, so the Rust plan should map by usage rather than by inferred names.

### Data-Structure Mapping Strategy

| C Construct | Rust Mapping | Notes |
|---|---|---|
| Anonymous structs used only as grouped local state | Local Rust struct with a descriptive module-local name | Introduce only when needed to keep function signatures manageable. |
| Anonymous flag/config aggregates | `struct` with explicit field types (`bool`, integer types, `Option<String>`) | Preserve field ownership and initialization defaults. |
| Anonymous tagged-state patterns | `enum` | Use only when the C logic clearly branches on state kind. |
| C strings (`char *`, `const char *`) | `String`, `&str`, `Option<String>` | Borrow where possible inside a call; own strings when values outlive source buffers. |
| C file handles (`FILE *`) | `std::fs::File` + `std::io::BufRead` | Scoped ownership removes manual close paths. |
| C arrays of option text | `Vec<String>` or `Vec<OsString>` | Use `OsString` only if exact non-UTF-8 environment fidelity is required by surrounding code. |
| Integer status codes | `Result<T, ParseOptError>` | Replace sentinel return values with typed error propagation. |

### Required Rust Types

At minimum, define a narrow internal error type for this migrated cluster:

```rust
enum ParseOptError {
    Io { path: Option<std::path::PathBuf>, message: String },
    Parse { source: String, message: String },
    Env { name: String, message: String },
}
```

This error type should remain module-local unless existing Rust code requires wider visibility.

If the original C code mutates a shared option/config object, map that object directly into an existing Rust config struct. If no Rust equivalent exists yet, define one minimally with only the fields touched by:

- environment-derived options,
- file-derived options,
- profile application,
- init-time hook behavior.

Do not generalize beyond fields actually required by these five functions.

## Implementation Phases

## Phase 1: Establish Rust module boundary and signatures

- Identify the exact section of `src/main.c` containing the five target functions and their immediate local dependencies.
- Decide whether the migration lands in:
  - `src/main.rs`, or
  - `src/parseopt.rs` called from `src/main.rs`.
- Create Rust function skeletons for:
  - `parseopt_from_env`
  - `fromfile_error`
  - `fromfile`
  - `optset_profile`
  - `init_hook`
- Define the minimal shared Rust types required by these signatures:
  - internal error enum,
  - any small config/state struct directly touched by these functions.
- Preserve existing call order and mutability patterns from C instead of redesigning interfaces.

**Deliverable:** compiling stubs with stable signatures and TODO-free type definitions for the migrated path.

## Phase 2: Port file and environment option sourcing

- Implement `parseopt_from_env` using `std::env` accessors matching the C lookup behavior.
- Implement `fromfile` using:
  - `std::fs::File`
  - `std::io::BufRead`
  - path-aware error propagation
- Implement `fromfile_error` as the single point converting low-level read/parse failures into module errors.
- Preserve original parsing rules:
  - whitespace/comment handling,
  - line continuation if present in C,
  - token splitting and ordering,
  - empty-input behavior.
- Ensure ownership is explicit:
  - no borrowed data escapes local buffers,
  - line/token strings are copied only when needed by downstream option application.

**Deliverable:** environment and file option ingestion working under `cargo test` with unit coverage for success and failure cases.

## Phase 3: Port profile application and initialization flow

- Implement `optset_profile` by directly translating profile-to-option assignment logic from C.
- Implement `init_hook` as the ordered orchestrator that invokes:
  - profile setup,
  - environment parsing,
  - file parsing,
  - any remaining initialization touched by these functions.
- Preserve original precedence rules among profile, environment, and file inputs.
- Convert C early-return/error-code branches into `Result` propagation without changing visible behavior.
- Keep side effects localized to the same config/state object used by the migrated functions.

**Deliverable:** end-to-end initialization path migrated for this module cluster and callable from the Rust startup path.

## Phase 4: Behavioral verification and cleanup

- Add focused tests for:
  - environment variable present/absent behavior,
  - file open failure,
  - malformed file content,
  - profile selection/application,
  - initialization ordering and precedence.
- Compare Rust behavior against the C implementation for representative inputs from these functions.
- Remove now-redundant C-path references for this function cluster only when the Rust path is confirmed equivalent.
- Keep cleanup limited to migrated code; do not restructure unrelated startup or option-parsing areas.

**Deliverable:** passing `cargo test`, equivalent behavior for the migrated functions, and a contained Rust replacement for this module scope.

## Notes on Memory Management and Error Handling

- Replace manual C lifetime management for file buffers and strings with Rust ownership.
- Avoid `unwrap`/`expect` in migrated logic; all I/O and parse failures should be returned as structured errors.
- Use borrowed `&str` during parsing where practical, promoting to `String` only for stored configuration or error payloads.
- Keep mutation through `&mut` references explicit to reflect the original in-place configuration updates.
- Preserve user-visible error text where the C implementation formats diagnostics, especially for file-originated option errors.