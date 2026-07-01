# Implementation Plan: module_src_parseopt_04

## Summary

This module migration ports the option-parsing support currently embedded in `src/main.c` into Rust, focusing only on the existing functions:

- `parseopt_from_env`
- `fromfile_error`
- `fromfile`
- `optset_profile`
- `init_hook`

The Rust implementation should preserve current control flow and side effects while replacing C-style string handling, pointer-based state access, and ad hoc error propagation with idiomatic Rust equivalents based on owned data, borrowing, and `Result`.

The technical approach is to extract the option/profile/from-file parsing behavior into a focused Rust module that mirrors the C responsibilities without adding new features. Parsing from environment variables and files should use the Rust standard library (`std::env`, `std::fs`, `std::io`) and map C error-reporting paths into explicit Rust error types or localized reporting helpers, depending on how the surrounding port handles process-level diagnostics.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates required based on current evidence
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Maintain behaviorally equivalent startup-time parsing cost
  - Avoid unnecessary intermediate allocations beyond what is needed for safe string and path handling
  - Preserve linear parsing behavior for environment- and file-driven option loading

## Module Mapping

### C to Rust File Mapping

| C File | Rust File | Notes |
|---|---|---|
| `src/main.c` | `src/main.rs` and/or a closely scoped helper module such as `src/parseopt.rs` | Migrate only the option/profile/from-file parsing logic required by the listed functions; keep call sites near existing startup flow. |

### Function Mapping

| C Function | Rust Target | Migration Notes |
|---|---|---|
| `parseopt_from_env` | `fn parseopt_from_env(...) -> Result<..., ...>` | Read environment-provided option input using `std::env`; preserve current parsing order and failure semantics. |
| `fromfile_error` | `fn fromfile_error(...)` or `fn fromfile_error(...) -> ParseOptError` | Centralize file-originated parse diagnostics; decide whether it emits directly or constructs structured errors based on surrounding port style. |
| `fromfile` | `fn fromfile(...) -> Result<..., ...>` | Read and parse options from a file using `std::fs`/`std::io`; preserve tokenization and error points from C behavior. |
| `optset_profile` | `fn optset_profile(...) -> Result<..., ...>` | Apply profile-specific option settings; convert string/profile matching logic into Rust enums or validated strings as appropriate. |
| `init_hook` | `fn init_hook(...) -> Result<..., ...>` or `fn init_hook(...)` | Keep startup hook behavior minimal and equivalent, invoking migrated setup/parsing steps in original order. |

## Data Model

The input only identifies multiple anonymous C data structures, so the Rust data model should be derived narrowly from actual usage inside the listed functions rather than introducing speculative types.

### Data-Structure Mapping Strategy

| C Representation | Rust Representation | Notes |
|---|---|---|
| anonymous struct used only locally | Local Rust struct | Introduce only if the C code groups related fields that remain meaningful in Rust. |
| anonymous struct carrying option parse state | Named Rust struct such as `ParseOptState` | Use explicit field names from migrated code; prefer owned `String`, `PathBuf`, slices, and enums over raw buffers and flags. |
| anonymous struct for file context/error context | Named Rust struct such as `FromFileContext` | Include filename/path, line/offset if present in C usage, and any parser state needed for diagnostics. |
| anonymous enum/flag-like integer fields | Rust `enum` or `bool`/integer fields | Use enums where C code switches over discrete states; otherwise keep primitive fields for exact behavior. |
| C strings (`char *`, fixed buffers) | `String`, `&str`, `OsString`, `PathBuf` | Use `OsString`/`PathBuf` for environment variables and file paths where UTF-8 cannot be assumed. |
| pointer-linked or nullable references | `Option<T>` / `Option<&T>` | Replace null checks directly with `Option`. |
| integer error codes / side-effect error paths | `Result<T, ParseOptError>` | Keep a small module-local error enum; do not generalize beyond this migration scope. |

### Expected Rust Types

The exact shapes should be finalized during code inspection, but the following minimal types are expected to be sufficient:

- `ParseOptError`
  - file read failures
  - invalid option syntax
  - invalid profile selection
  - environment lookup/parsing failures where the C path treats them as hard errors
- `ParseOptState`
  - parser inputs and mutable state currently carried through the C functions
- `FromFileContext`
  - source filename/path and any line/token position data needed by `fromfile_error`

Memory ownership should be explicit:
- file contents owned as `String` or `Vec<u8>` depending on current C parsing assumptions
- transient token views as `&str` slices where possible
- no raw pointers or manual lifetime management

## Implementation Phases

### Phase 1: Extract and Model Existing Parse State

- Inspect `src/main.c` and isolate the exact state, globals, and helper interactions used by:
  - `parseopt_from_env`
  - `fromfile_error`
  - `fromfile`
  - `optset_profile`
  - `init_hook`
- Define minimal Rust types needed to represent:
  - parse state
  - file-source context
  - parse/profile errors
- Map nullable and buffer-based C fields into:
  - `Option`
  - `String` / `OsString`
  - `PathBuf`
- Preserve existing control flow and call ordering; do not redesign startup configuration handling.

### Phase 2: Port Environment and File Parsing Functions

- Implement `parseopt_from_env` using `std::env`.
- Implement `fromfile` using `std::fs::File`/`std::fs::read_to_string` or buffered reading, depending on how closely tokenization must match the C logic.
- Implement `fromfile_error` as the single path for file-related diagnostic construction/reporting.
- Keep parsing logic behaviorally aligned with C:
  - same source precedence
  - same validation points
  - same distinction between recoverable and fatal cases, if present
- Replace C-style return/error conventions with `Result`, while preserving the surrounding externally visible behavior.

### Phase 3: Port Profile Application and Initialization Hook

- Implement `optset_profile` with direct mapping of existing profile selection and option application logic.
- Implement `init_hook` so that it invokes the migrated parsing/profile setup in the same sequence as the C startup path.
- Integrate the migrated functions back into `src/main.rs` or the nearest equivalent startup module without broad refactoring.
- Ensure all original side effects that matter to startup option state still occur.

### Phase 4: Verification and Behavior Lock-In

- Add focused unit tests for:
  - environment-driven option parsing
  - file-driven option parsing
  - profile selection behavior
  - expected error formatting/propagation for file parse failures
- Add regression-style tests for edge cases visible from the C logic, such as:
  - missing environment variable
  - unreadable option file
  - malformed file content
  - invalid profile name
- Run `cargo test` and confirm the Rust port preserves the original startup parsing behavior without introducing broader architectural changes.