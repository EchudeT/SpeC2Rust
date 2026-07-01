# Implementation Plan

## Summary

This module is the Rust port of `src/main.c`, which appears to contain the program entry path, option parsing helpers, runtime initialization, rc/config parsing, symbol filtering helpers, allocation-failure termination, and the top-level `main` routine.

The Rust implementation should keep the migration narrowly aligned with the existing C file rather than redesigning behavior. The preferred approach is to port the file into a single Rust source module that preserves the current control flow and helper boundaries as closely as practical:

- translate command-line and startup logic into Rust functions with matching responsibilities,
- replace C string and pointer manipulation with `String`, `OsString`, `PathBuf`, slices, and enums,
- convert process-terminating error paths into explicit `Result` returns internally, with final process exit handled in `main`,
- represent parser and option-state data with small Rust structs/enums derived from the anonymous C structures actually used by this file.

Because this module is the executable entry cluster, the implementation should prioritize behavioral equivalence, especially around argument parsing, config parsing, symbol inclusion decisions, and initialization ordering.

## Technical Context

### Language / Version

- Rust stable, edition 2021
- Minimum recommended toolchain: Rust 1.75+

### Primary Dependencies

Use the Rust standard library by default.

Recommended crates only where they directly match the apparent responsibilities of this file:

- `clap` only if the existing C option behavior is sufficiently complex that a manual `std::env::args_os` port becomes less maintainable; otherwise avoid it.
- `shellexpand` only if `tildexpand` requires shell-like home expansion semantics beyond a minimal `$HOME`-based implementation; otherwise implement directly with `std::env`.
- No other third-party crates should be introduced without confirmed need during migration.

Preferred default for this module: no external dependencies unless the existing option/config behavior proves too irregular to preserve cleanly with the standard library.

### Testing

- `cargo test`

Testing focus:

- unit tests for option-type detection and character-to-state/category mapping,
- unit tests for numeric parsing and level-string parsing edge cases,
- unit tests for tilde expansion behavior,
- unit tests for rc/config line parsing and symbol inclusion filtering,
- smoke test for top-level startup path using controlled argument vectors where practical.

### Performance Goals

- Maintain behavior and startup responsiveness comparable to the C implementation.
- Avoid unnecessary string cloning in argument and config parsing.
- Use borrowed string slices during parsing where possible.
- Keep initialization and filtering logic linear in input size.
- No additional performance work beyond preserving expected CLI startup efficiency.

## Module Mapping

C source to Rust source mapping should remain compact and centered on the executable entry module.

| C File | Rust File | Notes |
|---|---|---|
| `src/main.c` | `src/main.rs` | Primary port target containing top-level migration of startup logic and helpers. |

Function mapping should follow the original file layout as closely as possible:

| C Function | Rust Target | Notes |
|---|---|---|
| `CHAR_TO_SM` | `fn char_to_sm(...) -> ...` | Convert macro/helper into a small Rust function or `const fn` if practical. |
| `find_option_type` | `fn find_option_type(...) -> OptionType` | Replace ad hoc C classification with enum-based return. |
| `symbol_override` | `fn symbol_override(...) -> ...` | Keep current override logic; avoid redesign. |
| `number` | `fn number(...) -> Result<..., ParseError>` | Use checked numeric parsing. |
| `parse_level_string` | `fn parse_level_string(...) -> Result<..., ParseError>` | Preserve accepted syntax and mapping. |
| `tildexpand` | `fn tildexpand(...) -> PathBuf/String` | Implement direct home expansion semantics only. |
| `parse_rc` | `fn parse_rc(...) -> Result<(), Error>` | Port config parsing with explicit I/O and parse errors. |
| `globals_only` | `fn globals_only(...) -> bool/...` | Preserve predicate semantics. |
| `include_symbol` | `fn include_symbol(...) -> bool` | Keep symbol filtering logic isolated. |
| `xalloc_die` | `fn xalloc_die(...) -> !` or folded into error handling | In Rust, allocation failure is generally handled by panic/abort; only preserve explicit fatal reporting if observable. |
| `init` | `fn init(...) -> Result<AppState, Error>` or `fn init(...) -> Result<(), Error>` | Centralize migrated initialization sequence. |
| `main` | `fn main()` + inner `fn run() -> Result<(), Error>` | Keep process exit/reporting in outer boundary only. |

If the existing project already has adjacent Rust modules for shared state or parser behavior, `src/main.rs` may call into them, but this plan should not create new abstractions beyond what is needed to land the migrated file.

## Data Model

The analysis only exposes anonymous C data structures. The Rust port should convert them into named, file-local structs/enums based on actual usage in `src/main.c`.

### Data-structure mapping strategy

| C Shape | Rust Mapping | Notes |
|---|---|---|
| Anonymous struct used for option metadata | `struct OptionSpec` | Holds option spelling, type/category, and any associated action data. |
| Anonymous struct used for runtime flags/state | `struct AppState` | Central mutable state replacing scattered globals where feasible within this file. |
| Anonymous struct used for rc/config parse context | `struct RcParseState` | Tracks current file/path and parse-side settings. |
| Anonymous struct used for symbol filter/include rules | `struct SymbolRule` or `struct SymbolFilter` | Keeps include/exclude/override data explicit. |
| Anonymous struct used for level parsing result | `enum Level` / `struct LevelSpec` | Use enum for discrete levels, struct if bitmask/compound values are needed. |
| Anonymous struct used for option lookup result | `enum OptionType` | Replace integer/tag return values with a closed enum. |
| Anonymous struct used for fatal/error reporting context | `enum AppError` | Consolidate parse, I/O, argument, and startup failures. |

### C-to-Rust type conversions

Use the following default mappings unless the original field usage requires otherwise:

| C Type Pattern | Rust Type |
|---|---|
| `char *` string data | `String` |
| borrowed string input | `&str` |
| path-like `char *` | `PathBuf` or `&Path` |
| integer option codes / tags | `i32`, `u32`, or enum |
| boolean flags / int predicates | `bool` |
| arrays of records | `Vec<T>` |
| nullable pointer references | `Option<T>` / `Option<&T>` |
| mutable global state | fields on `AppState`, passed by `&mut` where practical |

### Memory management and ownership

- Eliminate manual allocation/free ownership paths from `src/main.c`.
- Replace temporary allocated C strings with scoped `String` or `PathBuf`.
- For helper routines that formerly returned allocated strings, return owned Rust values directly.
- Avoid introducing `unsafe` unless a concrete interoperability requirement is discovered in the surrounding codebase.
- Treat `xalloc_die` as a behavioral artifact rather than an allocation strategy; Rust should rely on normal ownership and standard allocation behavior.

### Error handling model

Adopt explicit typed errors for internal helpers:

```rust
type AppResult<T> = Result<T, AppError>;
```

Suggested shape:

- `AppError::Io`
- `AppError::InvalidOption`
- `AppError::InvalidNumber`
- `AppError::InvalidLevel`
- `AppError::RcParse`
- `AppError::Init`

`main` should remain the only place that converts these into user-visible exit behavior. Helpers that are pure predicates should keep boolean returns rather than overusing `Result`.

## Implementation Phases

## Phase 1: Port local types and pure parsing helpers

Scope:

- Create `src/main.rs` migration target if not already present on the Rust branch.
- Define the minimum named Rust replacements for the anonymous C data used directly by this file.
- Port pure/local helpers first:
  - `CHAR_TO_SM`
  - `find_option_type`
  - `number`
  - `parse_level_string`
  - `globals_only`
  - `include_symbol`
  - `symbol_override`

Technical decisions:

- Replace macro-like behavior with functions or enums.
- Convert integer/status-code parsing into `Result`-based helpers.
- Keep signatures small and oriented around slices/owned strings instead of raw pointers.
- Preserve original matching and classification order to minimize behavioral drift.

Exit criteria:

- Helper logic compiles independently.
- Unit tests cover valid and invalid numeric, level, option-type, and symbol-filter cases.

## Phase 2: Port path/config processing and initialization support

Scope:

- Port `tildexpand`.
- Port `parse_rc`.
- Port `init`.
- Introduce the minimal `AppState` and config parse state necessary to support the above logic.

Technical decisions:

- Use `std::env` for home-directory expansion unless actual behavior requires more.
- Use `std::fs` and buffered reading for rc/config processing.
- Preserve line-by-line parse behavior and existing precedence ordering between config-derived and command-line-derived values.
- Convert implicit C global mutation into explicit updates on `AppState` where practical, while avoiding broad architectural changes.

Exit criteria:

- Config parsing and initialization paths compile and are testable.
- Unit tests cover rc parsing edge cases, missing files behavior if applicable, and tilde expansion.

## Phase 3: Port top-level argument handling and program entry

Scope:

- Port `main`.
- Introduce `run() -> AppResult<()>` as the internal execution boundary.
- Integrate all helper functions into the startup path in the same order as the C implementation.

Technical decisions:

- Parse arguments using `std::env::args_os` by default.
- Keep option dispatch close to the original control flow rather than re-modeling the CLI.
- Restrict process termination and message emission to the outer entry path.
- Preserve return codes and fatal-error behavior as closely as possible.

Exit criteria:

- End-to-end executable path compiles.
- Smoke tests validate representative command-line paths and failure exits.

## Phase 4: Stabilization and behavioral alignment

Scope:

- Compare the Rust startup/config/filtering behavior against the C source for edge cases.
- Tighten naming and field types only where needed to reflect real usage from the migrated file.
- Remove any temporary compatibility scaffolding introduced during earlier phases.

Technical decisions:

- Do not add new features or abstractions.
- Prefer local refactoring only when it reduces ambiguity in ownership or error propagation.
- Keep the file/module boundary restrained to the original `src/main.c` responsibility set.

Exit criteria:

- `cargo test` passes.
- The migrated `src/main.rs` behavior is aligned with the original file’s observable responsibilities.
- No unnecessary dependencies or unused compatibility code remain.