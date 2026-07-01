# Implementation Plan: `main_root`

## Summary

Port the C `main_root` module into a Rust binary entry module that preserves the current command-line behavior and path lookup flow without adding new capabilities. The implementation should migrate the existing responsibilities from `which.c`, `getopt.c`, `getopt1.c`, and `bash.c` into a restrained Rust layout centered on:

- argument parsing equivalent to the current getopt-based behavior,
- environment and user/group lookup helpers,
- path element iteration and pathname construction,
- file executability/status checks,
- binary entrypoint orchestration in `main`.

The preferred technical approach is to use the Rust standard library for path handling, environment access, string slicing where safe, and process exit behavior. Low-level user/group membership behavior that has no standard-library equivalent should be handled through minimal libc-backed calls, keeping the logic close to the C flow. The migration should preserve the existing function boundaries where useful so behavior can be validated incrementally against the C implementation.

## Technical Context

### Language/Version
- Rust 1.78 or newer

### Primary Dependencies
Use the Rust standard library by default.

Recommended crates:
- `libc` — for user ID, group ID, supplementary group access, and `access`/`stat`-adjacent Unix behavior where standard library support is insufficient.

Do not introduce argument-parser crates unless the getopt behavior proves too coupled to preserve with a small in-tree port. The default plan is to migrate the existing getopt logic into Rust module code instead of replacing it with a higher-level parser.

### Testing
- `cargo test`

Test focus:
- unit tests for path parsing helpers,
- unit tests for getopt state transitions and long-option behavior,
- unit tests for pathname construction and absolute path detection,
- integration-style tests for main argument combinations and exit codes,
- Unix-only tests for file permission and executable detection.

### Performance Goals
- Match the current C utility’s expected single-process CLI performance profile.
- Avoid unnecessary allocations while scanning `PATH` and constructing candidate pathnames.
- Keep argument parsing linear in input size.
- Preserve low-overhead filesystem checks during command lookup.

## Module Mapping

Map the current C files into a small Rust binary-oriented layout without expanding scope.

### C to Rust File Mapping

| C File | Rust Target | Notes |
|---|---|---|
| `which.c` | `src/main.rs` and `src/main_root.rs` | Move entrypoint flow, path search logic, file status checks, and command lookup helpers here. |
| `bash.c` | `src/main_root.rs` | Keep only the environment/user/group helper functions actually used by this module. Do not port unrelated shell facilities. |
| `getopt.c` | `src/getopt.rs` | Port core getopt state machine and option scanning behavior. |
| `getopt1.c` | `src/getopt.rs` | Port long-option support into the same Rust module to keep parser state together. |

### Rust Module Layout

- `src/main.rs`
  - thin binary entrypoint,
  - delegates to `main_root::main_entry()` or equivalent function returning an exit code.

- `src/main_root.rs`
  - migrated equivalents of:
    - `uidget`
    - `getmaxgroups`
    - `initialize_group_array`
    - `group_member`
    - `file_status`
    - `absolute_program`
    - `substring`
    - `extract_colon_unit`
    - `get_next_path_element`
    - `make_full_pathname`
    - `get_current_user_info`
    - `sh_get_env_value`
    - `sh_get_home_dir`
    - `store_args_and_env`
    - `main`

- `src/getopt.rs`
    - `exchange`
    - `_getopt_internal`
    - `getopt`
    - `getopt_long`
    - `getopt_long_only`

This keeps migration close to the original source split while using idiomatic Rust module boundaries.

## Data Model

The source analysis only reports anonymous C structures, so the Rust plan should reconstruct only the concrete shapes required by the migrated functions rather than inventing broader abstractions.

### Data-Structure Mapping

| C Structure | Rust Mapping | Purpose |
|---|---|---|
| anonymous getopt option records | `struct LongOption` | Rust representation of long-option entries used by `getopt_long` and `getopt_long_only`. |
| anonymous getopt parser globals/state | `struct GetoptState` | Holds `optind`, `optarg`, `opterr`, `optopt`, ordering mode, scan position, and internal bookkeeping that was global/static in C. |
| anonymous user/group-related records | `struct UserInfo` | Stores effective/real uid/gid and supplementary group list needed by access checks. |
| anonymous file-status outputs | `enum FileStatus` or `struct FileStatus` | Encodes executable/not found/not executable/directory-like outcomes as needed by migrated logic. |
| anonymous environment/argv storage | `struct StoredContext` | Stores copied argument vector and environment values only if required by the original behavior. |

### Function-Level Type Mapping

- `char *` / `const char *` -> `&str`, `String`, or `&OsStr`/`OsString`
  - Use `OsStr`/`Path` for filesystem and environment values that may not be valid UTF-8.
  - Use `String` only where option parsing and internal text logic are clearly text-oriented.

- `int`, option return codes, status codes -> `i32`
  - Keep explicit numeric compatibility where behavior depends on sentinel values like `-1`, `?`, or `:`.

- C global parser variables -> fields on `GetoptState`
  - Avoid mutable global state; parser state is owned and passed explicitly.

- uid/gid arrays -> `Vec<libc::gid_t>`
  - Replace manual allocation/free with owned vectors.

- pointer-returned substrings -> owned `String` or borrowed slices
  - Prefer borrowed slices where the lifetime is straightforward.
  - Use owned strings where the C code created fresh allocated substrings.

### Memory Management and Error Handling

- Replace all manual allocation/free patterns with `Vec`, `String`, `PathBuf`, and scoped ownership.
- Remove null-pointer signaling in favor of:
  - `Option<T>` for “not found”/missing values,
  - `Result<T, E>` for system-call or parsing failures that must propagate,
  - explicit integer exit codes only at the binary boundary.
- Preserve externally visible command behavior by translating internal Rust errors into the same output and exit status patterns as the C program.
- Keep Unix-specific unsafe code narrowly contained in helper functions around `libc` calls, with safe wrappers returning Rust types.

## Implementation Phases

## Phase 1: Establish Binary Skeleton and Core Lookup Helpers

### Goals
Create the Rust binary structure and migrate the path/file logic from `which.c` first so the main command lookup flow can be exercised early.

### Tasks
- Create `src/main.rs` with a minimal entrypoint calling a Rust main-root function.
- Create `src/main_root.rs`.
- Port:
  - `absolute_program`
  - `substring`
  - `extract_colon_unit`
  - `get_next_path_element`
  - `make_full_pathname`
  - `file_status`
- Implement Rust equivalents using:
  - `std::path::{Path, PathBuf}`
  - `std::env`
  - `std::fs::metadata`
  - Unix permission inspection via `std::os::unix::fs::PermissionsExt` and/or `libc` where exact access semantics are needed.
- Define `FileStatus` and any small helper enums/constants needed for status preservation.
- Add unit tests for:
  - absolute vs relative program detection,
  - colon-separated path extraction,
  - empty path elements,
  - pathname joining behavior,
  - executable status classification on temporary files.

### Deliverable
A compilable Rust binary skeleton with helper logic validated independently of getopt migration.

## Phase 2: Migrate Environment and User/Group Support

### Goals
Port only the user, group, and environment support functions actually consumed by the main lookup flow.

### Tasks
- In `src/main_root.rs`, port:
  - `uidget`
  - `getmaxgroups`
  - `initialize_group_array`
  - `group_member`
  - `get_current_user_info`
  - `sh_get_env_value`
  - `sh_get_home_dir`
  - `store_args_and_env`
- Introduce `UserInfo` and `StoredContext` only with fields required by the migrated code.
- Wrap `libc` calls in narrow helper functions for:
  - effective/real uid and gid retrieval,
  - supplementary group enumeration.
- Use `std::env::var_os` for environment lookup, keeping non-UTF-8 values representable where relevant.
- Keep any argv/environment storage behavior as a direct migration of the original responsibility; do not generalize it into a reusable subsystem.
- Add tests for:
  - environment lookup wrappers,
  - home-directory resolution behavior when environment values are present or absent,
  - group-membership helper behavior where deterministic tests are feasible.

### Deliverable
Main-root support code can evaluate environment and user/group conditions without C-style memory management.

## Phase 3: Port Getopt Parsing Logic

### Goals
Migrate the current option parser behavior closely enough to preserve existing command-line semantics.

### Tasks
- Create `src/getopt.rs`.
- Port:
  - `exchange`
  - `_getopt_internal`
  - `getopt`
  - `getopt_long`
  - `getopt_long_only`
- Replace global/static parser variables with a `GetoptState` struct.
- Define `LongOption` matching the C option table shape closely.
- Preserve:
  - short option scanning,
  - argument-taking options,
  - permutation/exchange behavior,
  - long option matching,
  - long-only behavior,
  - return-code conventions for invalid or ambiguous options.
- Keep signatures and control flow close to the C logic where that reduces migration risk.
- Add focused parser tests covering:
  - short options,
  - clustered short options,
  - required option arguments,
  - long options with and without `=value`,
  - ambiguous long options,
  - operand permutation behavior.

### Deliverable
A Rust getopt module that can replace the C parser without introducing a new CLI contract.

## Phase 4: Integrate Main Flow and Finalize Behavioral Parity

### Goals
Connect the migrated parser and helper logic into the Rust `main` flow and verify observable behavior against the original program.

### Tasks
- Port `main` from `which.c` into `main_root::main_entry()` and invoke it from `src/main.rs`.
- Wire getopt parsing into the option-processing loop using `GetoptState`.
- Connect environment retrieval, path iteration, and file status checks into the final command search flow.
- Ensure output formatting and exit code handling match current behavior.
- Remove any remaining placeholder logic from earlier phases.
- Add integration-style tests that execute the compiled binary with representative arguments and verify:
  - success/failure exit codes,
  - lookup of explicit pathnames,
  - lookup through `PATH`,
  - option parsing behavior,
  - handling of missing commands.

### Deliverable
A complete Rust port of the `main_root` module on branch `001-main_root-rust-port`, limited to the behavior present in the listed C files and functions.