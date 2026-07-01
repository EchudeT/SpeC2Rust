# Implementation Plan: module_gnu_stat_04

## Summary

This module ports the existing GNU-style file descriptor and file status support from the C sources into Rust, preserving the current behavior and migration boundaries of the listed files and functions only.

The implementation centers on translating low-level descriptor and metadata operations from:

- `gnu/dup2.c`
- `gnu/fcntl.c`
- `gnu/fstat.c`
- `gnu/open.c`
- `gnu/stat-w32.c`
- `gnu/stat.c`

into a Rust module that exposes Rust equivalents for:

- `klibc_dup2`
- `klibc_fcntl`
- `orig_fstat`
- `rpl_fstat`
- `open`
- `_gl_fstat_by_handle`
- `orig_stat`

Technical approach:

- Use `std::fs`, `std::os::{unix,windows}` platform extensions, and `std::io` as the first choice.
- Use direct OS bindings through `libc` only where the Rust standard library does not expose the required descriptor- or stat-level operation cleanly.
- Preserve platform split already present in the C sources, especially the Windows-specific logic from `stat-w32.c`.
- Keep the Rust port narrow: translate existing responsibilities and call patterns without introducing new abstractions beyond what is necessary to express ownership, error propagation, and platform-specific branching safely.

## Technical Context

### Language/Version

- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.75+`

### Primary Dependencies

Use the Rust standard library by default.

Recommended crates:

- `libc` — for `dup2`, `fcntl`, `fstat`, `stat`, flags, and platform C ABI constants not fully covered by `std`
- No additional third-party crates are planned unless required by already-existing project constraints outside this module

### Testing

- `cargo test`

Test scope should include:

- descriptor duplication behavior
- open flag handling coverage that matches existing migrated behavior
- `fstat`/`stat` result population for regular files and directories
- platform-conditional tests for Unix and Windows code paths
- error propagation equivalence for invalid descriptors/paths and unsupported flag combinations handled by the original code

### Performance Goals

- Maintain near-system-call-level overhead for descriptor and metadata operations
- Avoid unnecessary allocations in wrapper paths
- Preserve direct pass-through behavior where C code primarily delegates to OS facilities
- Keep per-call overhead limited to Rust error conversion and minimal structure translation

## Module Mapping

Proposed Rust placement should mirror the current migration surface closely and avoid expanding unrelated structure.

### Source File Mapping

| C File | Rust File | Notes |
|---|---|---|
| `gnu/dup2.c` | `src/module_gnu_stat_04/dup2.rs` | Port `klibc_dup2` with direct descriptor duplication logic |
| `gnu/fcntl.c` | `src/module_gnu_stat_04/fcntl.rs` | Port `klibc_fcntl`; keep command/flag translation localized |
| `gnu/fstat.c` | `src/module_gnu_stat_04/fstat.rs` | Port `orig_fstat` and `rpl_fstat` |
| `gnu/open.c` | `src/module_gnu_stat_04/open.rs` | Port `open`; preserve low-level flag-based open behavior |
| `gnu/stat-w32.c` | `src/module_gnu_stat_04/stat_windows.rs` | Port `_gl_fstat_by_handle` and Windows-specific stat helpers |
| `gnu/stat.c` | `src/module_gnu_stat_04/stat.rs` | Port `orig_stat`; coordinate shared stat translation |
| module aggregation | `src/module_gnu_stat_04/mod.rs` | Re-export only migrated items required by the project |

### Function Mapping

| C Function | Rust Target | Migration Notes |
|---|---|---|
| `klibc_dup2` | `pub(crate) fn klibc_dup2(...) -> io::Result<_>` | Use OS fd/handle primitives; return Rust `Result` instead of errno-based status |
| `klibc_fcntl` | `pub(crate) fn klibc_fcntl(...) -> io::Result<_>` | Restrict implementation to command variants present in existing call sites and C code responsibilities |
| `orig_fstat` | `pub(crate) fn orig_fstat(...) -> io::Result<_>` | Thin wrapper over platform `fstat` |
| `rpl_fstat` | `pub(crate) fn rpl_fstat(...) -> io::Result<_>` | Replacement path preserving original normalization/fixup behavior |
| `open` | `pub(crate) fn open(...) -> io::Result<_>` | Use `OpenOptions` only when flags map exactly; otherwise call OS open APIs via `libc` |
| `_gl_fstat_by_handle` | `pub(crate) fn gl_fstat_by_handle(...) -> io::Result<_>` | Windows-only implementation in `stat_windows.rs` |
| `orig_stat` | `pub(crate) fn orig_stat(...) -> io::Result<_>` | Thin wrapper over path-based stat call with structure translation |

### Module Organization

```text
src/
  module_gnu_stat_04/
    mod.rs
    dup2.rs
    fcntl.rs
    fstat.rs
    open.rs
    stat.rs
    stat_windows.rs
```

`mod.rs` should provide the minimal public surface needed by the consuming crate and keep platform-specific modules behind `cfg` gates.

## Data Model

The input identifies only anonymous C data structures. For this module, the practical data model is driven by system structs and transient flag/command values rather than stable project-defined records.

### Data Structure Mapping

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous struct/union used for `stat` interop | `libc::stat` or platform-specific raw stat type | Preserve binary-compatible layout by using OS-provided types rather than recreating layout manually |
| anonymous temporary flag containers | primitive integers (`i32`, `u32`, `c_int`) | Keep command/flag transport in raw integer form where required by syscalls |
| anonymous descriptor/handle holders | `std::os::fd::RawFd` / `std::os::windows::io::RawHandle` | Use platform-native raw handle types |
| anonymous result normalization state | small private Rust structs only if needed locally | Only introduce private structs when translation logic becomes unreadable with tuples/primitives |

### Rust Representation Guidance

#### File Descriptor and Handle Types

- Unix:
  - `RawFd` for descriptor parameters and returns
- Windows:
  - `RawHandle` or raw OS handle-compatible integer/pointer types where required
- Do not wrap descriptors in owning types unless ownership transfer is explicit in the original function behavior

#### Stat Results

Where the original C functions fill caller-provided `struct stat` memory:

- Internal Rust logic should operate on:
  - `libc::stat` for syscall compatibility, or
  - `std::fs::Metadata` only when sufficient to populate the same required fields
- If the wider Rust port requires a stable internal representation, define one small private mirror struct containing only fields actually consumed by this project; otherwise keep raw OS struct usage to minimize divergence

Example constrained internal representation if needed:

```rust
pub(crate) struct StatInfo {
    pub mode: u32,
    pub size: i64,
    pub mtime_sec: i64,
    pub atime_sec: i64,
    pub ctime_sec: i64,
}
```

This struct should only be introduced if multiple translated functions need a shared normalized view. Otherwise, prefer direct raw struct filling.

#### Fcntl Command Mapping

Use a private enum only if it simplifies branching over supported command values:

```rust
enum FcntlCmd {
    DupFd,
    GetFd,
    SetFd,
    GetFl,
    SetFl,
}
```

If the C implementation is already mostly numeric pass-through, raw constants via `libc` are preferable to avoid changing semantics.

### Memory Management

- Replace caller-managed output buffers with mutable references where possible in Rust internals.
- Avoid heap allocation for descriptor and stat operations.
- Keep unsafe code limited to syscall boundaries and raw struct initialization.
- Use `MaybeUninit<libc::stat>` for syscall-filled structures to avoid undefined behavior from uninitialized memory.
- Document ownership expectations clearly for any function accepting or returning raw descriptors/handles.

### Error Handling

- Convert OS failures into `std::io::Error`
- Preserve error boundaries close to the syscall layer
- Do not silently normalize unrelated error codes
- Keep platform-specific error mapping inside the corresponding file (`fstat.rs`, `open.rs`, `stat_windows.rs`) instead of spreading conversion logic across the module

## Implementation Phases

## Phase 1: Skeleton and Raw syscall migration

### Goal

Create the Rust module structure and port the direct syscall-style wrappers with the least semantic change.

### Tasks

- Create `src/module_gnu_stat_04/mod.rs`
- Add `dup2.rs`, `fcntl.rs`, `fstat.rs`, `open.rs`, `stat.rs`, and `stat_windows.rs`
- Port:
  - `klibc_dup2`
  - `klibc_fcntl`
  - `orig_fstat`
  - `orig_stat`
- Introduce minimal platform imports:
  - `std::io`
  - `std::os::fd::RawFd`
  - `std::os::windows::io::RawHandle`
  - `libc`
- Implement syscall-adjacent unsafe blocks with narrow scope and inline safety comments
- Preserve original integer flag handling rather than redesigning APIs

### Deliverables

- Compiling module skeleton
- Direct wrappers for dup/fcntl/fstat/stat paths
- Basic unit tests for invalid input and successful regular-file cases on supported platforms

## Phase 2: Replacement behavior and open path migration

### Goal

Port the replacement logic that sits above the raw wrappers, especially where the C code adjusts or supplements base OS behavior.

### Tasks

- Port `rpl_fstat` into `fstat.rs`
- Port `open` into `open.rs`
- Reproduce existing flag interpretation and mode forwarding from the C implementation
- Use `OpenOptions` only for straightforward mappings; keep raw `libc::open` path for exact flag parity when needed
- Ensure descriptor return semantics match the original C behavior
- Add tests covering:
  - read-only/open-create combinations as supported by the original module
  - invalid flags or invalid path handling
  - `rpl_fstat` behavior on descriptors obtained from the migrated `open`

### Deliverables

- Compiling replacement layer for `rpl_fstat`
- Compiling migrated `open`
- Test coverage for descriptor creation and post-open metadata inspection

## Phase 3: Windows stat-specific migration

### Goal

Translate the Windows-specific stat support without altering the existing platform split.

### Tasks

- Port `_gl_fstat_by_handle` into `stat_windows.rs`
- Move Windows-only metadata translation logic from `gnu/stat-w32.c` into private helper functions
- Use `cfg(windows)` gates to isolate Windows code from Unix builds
- Keep raw handle processing and structure filling close to the C flow to reduce behavioral drift
- Add Windows-only tests for:
  - handle-based metadata retrieval for files
  - error handling for invalid handles
  - parity between path-based and handle-based metadata where both exist

### Deliverables

- Windows-specific stat module compiled behind feature gating by platform
- Tests for Windows handle/stat behavior

## Phase 4: Integration cleanup and equivalence verification

### Goal

Finalize module integration and reduce migration risk by validating consistent behavior across the translated files.

### Tasks

- Re-export only the required functions from `mod.rs`
- Remove duplicated flag/stat translation code where the translated files share exact logic
- Verify all unsafe blocks are limited to syscall boundaries and raw struct access
- Confirm consistent `io::Result` usage across:
  - `klibc_dup2`
  - `klibc_fcntl`
  - `orig_fstat`
  - `rpl_fstat`
  - `open`
  - `_gl_fstat_by_handle`
  - `orig_stat`
- Run `cargo test` across supported host platforms
- Adjust naming only where required by Rust keyword conflicts or module clarity; keep original function identity recognizable

### Deliverables

- Integrated Rust module aligned with the listed C files
- Passing tests
- Finalized narrow-scope port with no added facilities beyond the original migration surface