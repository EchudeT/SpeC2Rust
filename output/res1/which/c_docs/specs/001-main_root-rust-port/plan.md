# Implementation Plan: main_root Rust Port

## Summary

Port the C `main_root` module for the `which` project into a Rust binary crate entry path on branch `001-main_root-rust-port`, preserving the current command-line behavior and file lookup flow without adding new capabilities.

The Rust implementation should migrate the existing logic in place by:
- consolidating the executable entrypoint from `which.c` into `src/main.rs`,
- moving path inspection, executable resolution, user/group checks, and environment access into a small internal module layout under `src/`,
- replacing C string and buffer manipulation with `String`, `OsString`, `Path`, and `PathBuf`,
- replacing integer/status-code-driven control flow with `Result`, `Option`, and explicit exit-code handling in `main`,
- minimizing unsafe code and using the Rust standard library for filesystem, environment, and argument handling.

The scope should remain limited to the existing files and functions:
- command-line parsing from `getopt.c` / `getopt1.c`,
- shell/environment helpers from `bash.c`,
- top-level search and reporting logic from `which.c`.

## Technical Context

### Language/Version
- Rust 1.78+ stable

### Primary Dependencies
Prefer the Rust standard library only.

Recommended crates:
- None required by the input.

Rationale:
- argument parsing can be migrated directly from the existing `getopt`-style behavior into manual parsing with `std::env::args_os`,
- filesystem and path handling are available in `std::fs` and `std::path`,
- environment access is available in `std::env`,
- Unix-specific permission/group checks, if needed to preserve behavior, can use `std::os::unix` and a small amount of localized platform-specific code.

### Testing
- `cargo test`

Testing focus:
- unit tests for path splitting and pathname construction,
- unit tests for option parsing compatibility,
- integration-style tests for executable lookup behavior using temporary directories and controlled `PATH`,
- exit-code verification for success/failure cases.

### Performance Goals
- Match the C implementation’s practical performance for typical `PATH` scanning workloads.
- Avoid unnecessary string cloning while iterating through path elements.
- Use `Path`/`PathBuf` operations directly rather than repeated manual concatenation where possible.
- Keep startup overhead minimal; no dynamic subsystem initialization beyond existing runtime needs.

## Module Mapping

### Source File Mapping

| C File | Rust Target | Notes |
|---|---|---|
| `which.c` | `src/main.rs` and `src/which_logic.rs` | `main` stays in `src/main.rs`; executable search and status helpers move into one internal module. |
| `bash.c` | `src/env_compat.rs` | Migrate only the referenced shell/environment helper functions used by this module. |
| `getopt.c` | `src/getopt_compat.rs` | Port only the parsing behavior needed by current `main`. |
| `getopt1.c` | `src/getopt_compat.rs` | Keep long-option support together with getopt compatibility code. |

### Function Mapping

| C Function | Rust Location | Rust Shape |
|---|---|---|
| `main` | `src/main.rs` | `fn main()` with explicit exit handling |
| `file_status` | `src/which_logic.rs` | helper returning status enum / `Result` |
| `absolute_program` | `src/which_logic.rs` | helper on `&Path` / `&OsStr` |
| `substring` | `src/which_logic.rs` or removed | replace with slice/owned string helper only if still needed after refactor |
| `extract_colon_unit` | `src/which_logic.rs` | path-element extraction helper over `OsStr`/string |
| `get_next_path_element` | `src/which_logic.rs` | iterator-style helper or loop-local function |
| `make_full_pathname` | `src/which_logic.rs` | `PathBuf` join helper |
| `uidget` | `src/env_compat.rs` | Unix user-id helper |
| `getmaxgroups` | `src/env_compat.rs` | migrate only if still required by group membership logic |
| `initialize_group_array` | `src/env_compat.rs` | internal helper for group resolution |
| `group_member` | `src/env_compat.rs` | Unix-specific membership helper |
| `get_current_user_info` | `src/env_compat.rs` | current-user lookup helper |
| `sh_get_env_value` | `src/env_compat.rs` | thin wrapper over `std::env::var_os` |
| `sh_get_home_dir` | `src/env_compat.rs` | home-dir helper using environment first |
| `store_args_and_env` | `src/env_compat.rs` or `src/main.rs` | only preserve if required by current logic |
| `exchange` | `src/getopt_compat.rs` | internal argument permutation helper |
| `_getopt_internal` | `src/getopt_compat.rs` | internal parser state machine |
| `getopt` | `src/getopt_compat.rs` | public compatibility wrapper |
| `getopt_long` | `src/getopt_compat.rs` | public compatibility wrapper |
| `getopt_long_only` | `src/getopt_compat.rs` | public compatibility wrapper |

### Proposed Rust File Layout

```text
src/
  main.rs
  which_logic.rs
  env_compat.rs
  getopt_compat.rs
```

Keep this layout small and directly tied to the current C file split.

## Data Model

The C analysis reports only anonymous structures, so the Rust plan should introduce only the minimum named internal types needed to replace implicit C state.

### Data Structure Mapping

| C Structure | Rust Type | Purpose |
|---|---|---|
| anonymous getopt state structs | `struct GetoptState` | Holds parser indices, current option argument, ordering mode, and parser state previously kept in C globals/statics. |
| anonymous long option descriptors | `struct LongOption` | Rust representation of long-option metadata used by `getopt_long` and `getopt_long_only`. |
| anonymous file/user/group helper structs | `struct UserInfo` | Holds current-user identifiers and home directory if the C code requires grouped access. |
| anonymous status/result carriers | `enum FileStatus` | Encodes executable/non-executable/not-found style outcomes instead of raw C ints. |
| anonymous temporary buffers | `String`, `OsString`, `PathBuf`, `Vec<_>` | Replace manual allocation and mutable char buffers. |

### Rust Type Decisions

#### `GetoptState`
Use a dedicated state struct rather than mutable globals where possible:
- `optind: usize`
- `opterr: bool`
- `optopt: Option<char>`
- `optarg: Option<OsString>`
- internal cursor/permutation fields

This keeps the port close to C semantics while making mutation explicit and testable.

#### `LongOption`
Represent long-option definitions as a small Rust struct mirroring the C table-driven parser:
- name
- has-argument mode
- return value / equivalent short option

Only fields required by current parsing behavior should be included.

#### `UserInfo`
Use a compact struct only if multiple helpers share the same data:
- uid/euid as integer types appropriate for Unix
- optional home directory
- optional group list

Do not generalize beyond what `group_member`, `uidget`, and `get_current_user_info` require.

#### `FileStatus`
Replace raw integer result codes with an enum such as:
- `FoundExecutable`
- `FoundNonExecutable`
- `NotFound`
- `Error`

Map back to the original process exit semantics only at the CLI boundary.

## Implementation Phases

## Phase 1: Establish Rust Entry Point and Core Search Logic

### Goals
- Create the Rust binary entrypoint.
- Port the non-parser executable lookup path from `which.c`.
- Preserve current process exit behavior.

### Tasks
- Create `src/main.rs` with a thin `main` function that gathers arguments and delegates to internal logic.
- Create `src/which_logic.rs` and migrate:
  - `file_status`
  - `absolute_program`
  - `substring` if still needed
  - `extract_colon_unit`
  - `get_next_path_element`
  - `make_full_pathname`
- Replace C string traversal with:
  - `OsStr` / `OsString` for argv and environment values,
  - `Path` / `PathBuf` for filesystem joins and checks.
- Define a minimal `FileStatus` enum and convert status-based branching to idiomatic Rust.
- Implement exit-code mapping at the top level rather than leaking integer codes through internal helpers.

### Notes
- Keep path iteration logic behaviorally close to the C code, including handling empty `PATH` elements if the original code depends on them.
- Avoid introducing generalized search abstractions; one module-local implementation is sufficient.

## Phase 2: Port Environment and User/Group Compatibility Helpers

### Goals
- Migrate only the `bash.c` helper functions referenced by this module.
- Preserve behavior around environment lookups, home directory access, and user/group membership checks.

### Tasks
- Create `src/env_compat.rs` and migrate:
  - `uidget`
  - `getmaxgroups`
  - `initialize_group_array`
  - `group_member`
  - `get_current_user_info`
  - `sh_get_env_value`
  - `sh_get_home_dir`
  - `store_args_and_env` if required by the ported control flow
- Replace global/process-wide C assumptions with explicit function inputs/outputs where feasible.
- Use standard library environment APIs first:
  - `std::env::var_os`
  - `std::env::home_dir` should not be used; instead prefer environment-based lookup and any logic already implied by the original code
- For Unix-only identity/group checks, isolate platform-specific code behind small helper functions and keep it minimal.

### Memory and Error Handling
- Replace returned borrowed C pointers with owned `OsString`, `String`, or `PathBuf`.
- Replace sentinel null/error returns with `Option`/`Result`.
- Keep any unavoidable platform-specific low-level interactions localized so the rest of the crate stays safe Rust.

## Phase 3: Port `getopt` Compatibility Layer

### Goals
- Preserve the current command-line parsing behavior used by `main`.
- Avoid bringing in external parsing crates or redesigning the CLI surface.

### Tasks
- Create `src/getopt_compat.rs` and migrate:
  - `exchange`
  - `_getopt_internal`
  - `getopt`
  - `getopt_long`
  - `getopt_long_only`
- Introduce a `GetoptState` struct to contain parser state that was global/static in C.
- Introduce a minimal `LongOption` struct and any small supporting enums needed to represent:
  - no-argument
  - required-argument
  - optional-argument
- Adapt `main` to call the Rust parser in the same order/shape expected by the C implementation.
- Preserve option permutation behavior only to the extent used by the current program logic.

### Notes
- Do not replace this with Clap/Pico-args/etc.; the input points to existing `getopt` sources that should be ported directly.
- Keep parser output close to the C semantics so that later code migration stays mechanical.

## Phase 4: Integration, Compatibility Cleanup, and Tests

### Goals
- Wire all migrated pieces together.
- Remove dead compatibility code not used by the final Rust module.
- Lock in behavior with tests.

### Tasks
- Connect `main.rs`, `which_logic.rs`, `env_compat.rs`, and `getopt_compat.rs`.
- Eliminate any helper that became unnecessary after safe Rust refactoring, but only where removal does not change behavior.
- Add unit tests for:
  - colon-separated path element extraction,
  - full pathname construction,
  - absolute-path detection,
  - getopt short and long option cases actually used by the program.
- Add integration tests covering:
  - executable found in `PATH`,
  - missing executable,
  - direct absolute/relative program input,
  - non-executable file handling if present in current behavior,
  - environment-dependent behavior using temporary `PATH` values.
- Verify that error messages and exit statuses remain aligned with the existing module behavior.

### Completion Criteria
- All listed C functions are either:
  - ported into the mapped Rust modules, or
  - intentionally removed because equivalent Rust control flow made them unnecessary, with no behavior change.
- `cargo test` passes.
- The binary builds and runs from the Rust branch as the module’s main entrypoint.