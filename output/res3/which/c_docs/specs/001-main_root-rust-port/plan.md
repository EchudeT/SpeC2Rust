# Implementation Plan

## Summary

Port the `main_root` C module into a Rust binary crate entry path for the `which` project, preserving the current command-line behavior and path-resolution flow without adding new features. The Rust implementation should migrate the existing responsibilities in a direct way:

- command-line option parsing currently provided by `getopt.c` and `getopt1.c`
- environment and user-context access from `bash.c`
- executable lookup and status checks from `which.c`
- top-level program flow from `main`

Technical approach:

- Use a single Rust binary entry (`src/main.rs`) with a small number of internal modules only where they directly map to current C file responsibilities.
- Prefer Rust standard library APIs for path handling, environment access, argument iteration, and filesystem metadata.
- Replace C pointer/string manipulation with owned `String`, borrowed `&str`, and `PathBuf`.
- Replace integer/status-style error signaling with explicit `Result` where internal, while keeping process exit behavior in `main`.
- Preserve migration boundaries by implementing Rust equivalents for the existing functions first, then collapsing or simplifying only where the C helper has no meaningful standalone Rust need.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain linear scanning over `PATH` entries and candidate files.
  - Avoid unnecessary path/string allocations where borrowed slices are sufficient.
  - Use standard filesystem metadata calls directly without caching or extra abstraction.
  - Keep startup overhead minimal and comparable to the C implementation.

## Module Mapping

C-to-Rust file/module mapping should stay close to the source layout and migration order.

| C File | Rust Target | Notes |
|---|---|---|
| `which.c` | `src/which.rs` | Core executable lookup logic, path element parsing, file status checks, and helper routines. |
| `bash.c` | `src/env.rs` | User/group/environment helpers that are actually used by the program flow. |
| `getopt.c` | `src/getopt.rs` | Internal short-option parsing support if needed to preserve current option behavior. |
| `getopt1.c` | `src/getopt.rs` | Long-option parsing support merged into the same Rust module. |
| `main` from C | `src/main.rs` | Program entrypoint, argument dispatch, exit code mapping. |

Recommended crate layout:

```text
src/
  main.rs
  which.rs
  env.rs
  getopt.rs
```

If option parsing can be expressed cleanly with `std::env::args_os()` and fixed manual parsing while preserving current semantics, keep `getopt.rs` as a migration shim and reduce it after parity is reached. Do not introduce a third-party CLI crate.

## Data Model

The input analysis lists only anonymous C data structures, so the Rust plan should define only the minimum explicit types required to replace the C global/stateful parsing and helper behavior.

### C anonymous structures -> Rust mappings

| C Structure Kind | Rust Mapping | Usage |
|---|---|---|
| anonymous parser state structs/statics from `getopt.c` / `getopt1.c` | `struct GetoptState` | Holds parser cursor, current option argument, index tracking, and ordering mode if direct migration is required. |
| anonymous option descriptor data | `struct LongOption` or `enum LongOptionKind` | Represents long-option name, argument requirement, and returned discriminator if long-option logic is preserved. |
| anonymous user/group helper state | `struct UserInfo` | Stores current user identifiers, home directory, and group membership data actually needed by `group_member` / `sh_get_home_dir` / `get_current_user_info`. |
| anonymous file-status helper data | `enum FileStatus` | Replaces integer/flag-style return values for executable existence/type checks. |
| anonymous temporary string/range state | `&str`, `String`, `PathBuf`, `std::ops::Range<usize>` | Replaces manual substring extraction and mutable C buffer management. |

### Planned Rust data structures

#### `UserInfo`
Maps logic currently spread across `uidget`, `getmaxgroups`, `initialize_group_array`, `group_member`, and `get_current_user_info`.

```rust
struct UserInfo {
    uid: u32,
    euid: u32,
    gid: u32,
    groups: Vec<u32>,
    home_dir: Option<std::path::PathBuf>,
}
```

Notes:
- Use `Vec<u32>` instead of manual group arrays.
- If exact UID/GID/group discovery cannot be done with std alone on all targets, keep the interface narrow and isolate platform-specific code behind `cfg(unix)` in `env.rs`.
- No extra platform abstraction layer beyond what is required for current behavior.

#### `FileStatus`
Maps integer return conventions from `file_status`.

```rust
enum FileStatus {
    NotFound,
    NotExecutable,
    ExecutableFile,
    Directory,
    Other,
}
```

Notes:
- Exact variant names can be adjusted to the C semantics during implementation.
- Use this internally; convert back to process behavior in `main`.

#### `GetoptState`
Used only if direct migration of getopt semantics is necessary.

```rust
struct GetoptState {
    optind: usize,
    optarg: Option<String>,
    optopt: Option<char>,
    current_pos: usize,
    ordering: OrderingMode,
}
```

#### `OrderingMode`
```rust
enum OrderingMode {
    RequireOrder,
    Permute,
    ReturnInOrder,
}
```

#### `LongOption`
Used for `getopt_long` / `getopt_long_only` migration.

```rust
struct LongOption {
    name: &'static str,
    has_arg: HasArg,
    val: i32,
}
```

#### `HasArg`
```rust
enum HasArg {
    No,
    Required,
    Optional,
}
```

## Implementation Phases

## Phase 1: Establish binary entry and migrate core lookup helpers

Scope:
- Create Rust crate structure for the branch `001-main_root-rust-port`.
- Implement `src/main.rs` and `src/which.rs`.
- Migrate the path and file-resolution helpers from `which.c`.

Functions to migrate in this phase:
- `file_status`
- `absolute_program`
- `substring`
- `extract_colon_unit`
- `get_next_path_element`
- `make_full_pathname`
- `main` skeleton with delegated lookup flow

Technical decisions:
- Replace raw C strings with `OsString`, `String`, `&str`, `Path`, and `PathBuf` as appropriate.
- Use `std::env::var_os("PATH")` for path retrieval.
- Use `std::fs::metadata` / `symlink_metadata` and Unix permission bits under `cfg(unix)` for executability checks.
- Remove manual allocation/free behavior by expressing helpers as safe-returning Rust functions.
- Treat colon-separated `PATH` parsing with slice iteration rather than mutable buffer walking, but keep helper boundaries corresponding to the C functions until parity is validated.

Error handling:
- Internal helpers return `Option` or `Result` instead of sentinel pointers/integers.
- `main` remains responsible for exit codes and user-facing output.

Testing targets:
- path element extraction for empty, leading-colon, trailing-colon, and repeated-colon cases
- absolute vs non-absolute program name detection
- file status classification for regular files, directories, and missing paths

## Phase 2: Migrate environment and user-context helpers

Scope:
- Implement `src/env.rs`.
- Port only the environment and user/group functions referenced by the main lookup flow.

Functions to migrate in this phase:
- `uidget`
- `getmaxgroups`
- `initialize_group_array`
- `group_member`
- `get_current_user_info`
- `sh_get_env_value`
- `sh_get_home_dir`
- `store_args_and_env`

Technical decisions:
- Use `std::env` for environment value access and argument/environment capture.
- Use `std::env::home_dir` replacement approach via environment inspection only if needed by existing logic; prefer `HOME` lookup because no extra crate should be introduced.
- For Unix UID/GID/group logic, isolate platform-specific implementation in `cfg(unix)` code paths and use standard OS extension APIs where available.
- Represent group lists with `Vec<u32>` instead of fixed-size arrays.
- If some C helpers are only setup utilities for global state, collapse them into `UserInfo` construction while retaining function-level mapping comments for traceability.

Memory and safety considerations:
- No global mutable buffers.
- No retained raw environment pointers.
- Clone environment data only when required by function behavior.

Testing targets:
- environment lookup behavior for present and missing variables
- home directory resolution fallback order as implemented
- group membership checks from initialized user info on supported targets

## Phase 3: Migrate getopt behavior with minimal stateful parser

Scope:
- Implement `src/getopt.rs`.
- Port command-line parsing behavior now provided by `getopt.c` and `getopt1.c` only to the extent used by `main_root`.

Functions to migrate in this phase:
- `exchange`
- `_getopt_internal`
- `getopt`
- `getopt_long`
- `getopt_long_only`

Technical decisions:
- Do not introduce `clap`, `pico-args`, or similar crates.
- Prefer a small internal parser over recreating every GNU getopt edge case unless the current `main` logic depends on them.
- Keep parser state explicit in `GetoptState` rather than emulating C globals.
- If argument permutation is needed, implement it with `Vec<OsString>` index swaps corresponding to `exchange`.

Behavior constraints:
- Preserve existing short and long option handling used by the current binary.
- Preserve distinction between option arguments and positional program names.
- Keep unknown-option and missing-argument behavior aligned with current exit/reporting expectations.

Testing targets:
- short options with and without arguments
- long options and long-only forms if used
- mixed option/positional ordering
- unknown and malformed option cases

## Phase 4: Integrate, trim migration shims, and finalize parity

Scope:
- Wire `main.rs`, `which.rs`, `env.rs`, and `getopt.rs` together.
- Remove only those migration helpers that become dead code after direct function parity is achieved.
- Finalize process exit behavior and module-local tests.

Integration tasks:
- Connect parsed options to executable lookup flow.
- Ensure environment/user helpers feed path and home resolution exactly where needed.
- Normalize helper signatures to idiomatic Rust while keeping one-to-one traceability in comments or commit structure.

Validation goals:
- `cargo test` passes for all module tests.
- End-to-end command behavior is stable for representative invocation forms.
- No unsafe code unless a platform-specific user/group lookup absolutely requires it; if required, keep it isolated and documented in `env.rs`.

## Memory Management and Error Handling Notes

- All C heap/manual-buffer logic should be replaced with Rust ownership and borrowing.
- Avoid lossy UTF-8 assumptions for command-line arguments and environment values where path semantics matter; use `OsStr`/`OsString` and `PathBuf`.
- Convert C integer status returns into small enums or `Result` internally.
- Keep user-visible failures and exit codes centralized in `main`.
- Avoid global mutable parser/environment state; pass state through function arguments or dedicated structs.

## Migration Order Rationale

1. Port `which.c` first because it contains the executable lookup core.
2. Port `bash.c` next because environment and user-context helpers support lookup behavior.
3. Port `getopt.c` and `getopt1.c` after core logic so parser requirements are constrained by actual `main` usage.
4. Finish with integration in `main.rs`, preserving current behavior without adding new CLI or platform features.