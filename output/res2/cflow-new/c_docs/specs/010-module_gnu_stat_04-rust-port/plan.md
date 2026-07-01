# Implementation Plan: module_gnu_stat_04

## Summary

This module ports the small GNU-style file descriptor and file status compatibility layer from C to Rust, covering `dup2`, `fcntl`, `fstat`, `open`, and `stat`-related behavior currently split across:

- `gnu/dup2.c`
- `gnu/fcntl.c`
- `gnu/fstat.c`
- `gnu/open.c`
- `gnu/stat-w32.c`
- `gnu/stat.c`

The Rust implementation should preserve the existing module scope and migrate the current functions as thin, platform-aware wrappers around OS file APIs. The primary approach is:

- use `std` types and OS extension traits where possible;
- use minimal `libc` bindings only where the C behavior depends on POSIX descriptors, flags, and `stat`/`fcntl`/`dup2` system calls;
- keep Unix and Windows behavior separated with `cfg(unix)` / `cfg(windows)` in the same module tree rather than introducing extra abstraction layers;
- represent C return/error conventions through `std::io::Result` internally, converting to C-style integer/status results only at the module boundary if required by the surrounding port.

The migration should stay close to the existing file/function boundaries and avoid expanding functionality beyond the current compatibility logic.

## Technical Context

### Language/Version

- Rust 1.78 or newer

### Primary Dependencies

Use the Rust standard library by default, plus only the minimal crate support needed for low-level OS interoperability:

- `libc` — for `dup2`, `fcntl`, `open`, `fstat`, `stat`, flags, mode values, and native `struct stat` interop on Unix
- No additional third-party crates are recommended based on the current input

### Testing

- `cargo test`

Testing focus:

- unit tests for flag translation and error propagation
- platform-gated tests for descriptor duplication and file status retrieval
- path-based tests for `open`/`stat` behavior on temporary files and directories

### Performance Goals

- preserve near-system-call overhead characteristics
- avoid unnecessary allocation when forwarding paths and file descriptor operations
- keep wrapper cost negligible relative to underlying OS calls
- maintain no extra copies of file metadata beyond required conversion between C-like and Rust/native representations

## Module Mapping

Recommended Rust module layout should mirror the current C file grouping without adding unrelated layers:

```text
src/
  module_cluster/
    module_gnu_stat_04/
      mod.rs
      dup2.rs
      fcntl.rs
      fstat.rs
      open.rs
      stat.rs
      stat_windows.rs   # only under cfg(windows) if needed
```

### C to Rust file mapping

| C File | Rust File | Notes |
|---|---|---|
| `gnu/dup2.c` | `dup2.rs` | Port `klibc_dup2` as a thin descriptor duplication wrapper |
| `gnu/fcntl.c` | `fcntl.rs` | Port `klibc_fcntl`; keep command/flag handling close to libc constants |
| `gnu/fstat.c` | `fstat.rs` | Port `orig_fstat` and `rpl_fstat`; central place for fd-based metadata retrieval |
| `gnu/open.c` | `open.rs` | Port `open`; translate flags/mode with minimal allocation |
| `gnu/stat.c` | `stat.rs` | Port `orig_stat`; path-based metadata/status wrapper |
| `gnu/stat-w32.c` | `stat_windows.rs` | Port `_gl_fstat_by_handle` and Windows-specific status handling behind `cfg(windows)` |

### Function mapping

| C Function | Rust Target | Implementation Notes |
|---|---|---|
| `klibc_dup2` | `module_gnu_stat_04::dup2::klibc_dup2` | Call through to `libc::dup2` on Unix; preserve descriptor/error semantics |
| `klibc_fcntl` | `module_gnu_stat_04::fcntl::klibc_fcntl` | Implement only the command paths used by existing C module behavior; use raw fd operations |
| `orig_fstat` | `module_gnu_stat_04::fstat::orig_fstat` | Native `fstat` wrapper returning Rust error type internally |
| `rpl_fstat` | `module_gnu_stat_04::fstat::rpl_fstat` | Replacement logic layered over `orig_fstat`, preserving current normalization behavior |
| `open` | `module_gnu_stat_04::open::open` | Low-level open wrapper using C-compatible flags and mode |
| `_gl_fstat_by_handle` | `module_gnu_stat_04::stat_windows::_gl_fstat_by_handle` | Windows-only helper for handle-based metadata extraction |
| `orig_stat` | `module_gnu_stat_04::stat::orig_stat` | Path-based `stat` wrapper, platform-gated where semantics differ |

### `mod.rs` responsibilities

- declare only the migrated submodules
- re-export the migrated functions if the rest of the crate expects a flat API
- keep platform gating local to this module

## Data Model

The input identifies only anonymous C data structures, which in this module are most likely temporary local structs, platform-native status records, or unnamed aggregate use around `stat`-family calls. The Rust plan should therefore minimize new named data types and rely on native OS structs where possible.

### Data-structure mapping

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous `struct stat` usage | `libc::stat` | Preferred for syscall-compatible layout on Unix |
| anonymous `struct fstat` output storage | `libc::stat` | `fstat` and `stat` share native storage where applicable |
| anonymous Windows file status storage | private Rust struct only if required, otherwise native Windows handle-derived fields | Create a private struct only if direct field grouping is necessary during port |
| anonymous flag aggregates | `libc::c_int` / small private helper functions | Prefer constants and helper translation functions over new structs |
| anonymous mode values | `libc::mode_t` | Preserve exact ABI-compatible type |
| anonymous file descriptor values | `libc::c_int` / `std::os::fd::RawFd` | Use `RawFd` in internal signatures when practical |
| anonymous handle values on Windows | platform raw handle type | Keep inside `cfg(windows)` code paths |

### Rust type decisions

- File descriptors:
  - internal APIs: `std::os::fd::RawFd` on Unix
  - external compatibility APIs: `libc::c_int` where matching C signatures is necessary
- Path inputs:
  - use `&CStr`/`CString` for direct syscall wrappers if preserving C-like behavior
  - avoid converting through `PathBuf` when not needed
- Metadata outputs:
  - use `libc::stat` for exact native layout where system calls write into caller-provided storage
- Errors:
  - internal: `std::io::Result<T>`
  - boundary compatibility: convert to `-1` and set errno implicitly via libc call behavior, or preserve surrounding crate convention if already established

### Memory management and safety approach

- avoid heap allocation except for temporary `CString` creation when required by path-based system calls
- use `MaybeUninit<libc::stat>` for syscall output buffers before successful initialization
- isolate all `unsafe` code to the direct syscall boundaries
- document each `unsafe` block with the pointer validity and initialization assumptions being preserved
- do not retain borrowed pointers or raw handles beyond the call scope

## Implementation Phases

### Phase 1: Establish module skeleton and lowest-level wrappers

Create the Rust module directory and migrate the simplest descriptor-based functions first:

- add `mod.rs`, `dup2.rs`, `fcntl.rs`, `fstat.rs`, `open.rs`, `stat.rs`
- add `stat_windows.rs` only if building on Windows requires separate compilation units
- port `klibc_dup2` as a direct syscall wrapper
- port `orig_fstat` as the baseline fd-to-`stat` wrapper
- define shared internal conventions for:
  - raw fd parameter types
  - `libc::stat` output handling with `MaybeUninit`
  - `std::io::Error::last_os_error()` conversion

Exit criteria:

- module compiles on the primary target platform
- `klibc_dup2` and `orig_fstat` behavior matches C return/error patterns
- unsafe syscall boundaries are contained and reviewed

### Phase 2: Port open/fcntl behavior and shared flag handling

Migrate the functions that depend on integer command and flag forwarding:

- port `open`
- port `klibc_fcntl`
- add minimal helper functions in `open.rs`/`fcntl.rs` for flag and mode forwarding only where needed
- keep variadic-C behavior flattened into explicit Rust helper paths based on command requirements
- ensure invalid command/flag combinations propagate OS errors consistently with the C version

Exit criteria:

- descriptor opening works for existing flag combinations used by the project
- `fcntl` command dispatch covers the current module’s actual behavior without speculative extensions
- tests confirm no extra allocations or semantic drift in simple file operations

### Phase 3: Port replacement/stat logic and platform-specific behavior

Complete status-related replacement logic and reconcile platform-specific differences:

- port `rpl_fstat`
- port `orig_stat`
- port `_gl_fstat_by_handle` into `stat_windows.rs` under `cfg(windows)`
- mirror the C replacement logic for metadata normalization or fallback behavior, staying close to the original control flow
- keep Unix and Windows branches local to status modules rather than creating cross-platform abstraction layers

Exit criteria:

- path-based and fd-based status calls produce compatible metadata for regular files and directories
- Windows-only handle-based code is isolated and does not affect Unix builds
- replacement logic compiles cleanly under target-specific cfgs

### Phase 4: Verification and cleanup

Validate the ported module as a direct replacement for the C implementation:

- add focused `cargo test` coverage for:
  - duplicate fd behavior
  - `fstat` on valid and invalid descriptors
  - `open` on existing and missing paths
  - `stat` on regular files and directories
  - platform-gated Windows handle path if applicable
- remove dead translation helpers introduced during migration
- confirm public function names and module exports match the expected integration points on branch `010-module_gnu_stat_04-rust-port`

Exit criteria:

- all module tests pass with `cargo test`
- code remains narrowly scoped to the original C module files and functions
- no extra support layers or unevidenced facilities remain in the implementation plan