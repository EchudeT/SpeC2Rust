# Implementation Plan: module_gnu_stat_04

## Summary

This module ports a small cluster of GNU/POSIX file-descriptor and file-status routines from C to Rust, covering `dup2`, `fcntl`, `fstat`, `stat`, and `open`-related behavior currently spread across:

- `gnu/dup2.c`
- `gnu/fcntl.c`
- `gnu/fstat.c`
- `gnu/open.c`
- `gnu/stat-w32.c`
- `gnu/stat.c`

The Rust implementation should preserve the current module scope and migrate existing behavior only, without adding higher-level abstractions or new capabilities. The technical approach is to:

- keep the implementation close to the C call boundaries,
- use Rust’s standard library types where they directly fit,
- use low-level OS bindings only where required to preserve exact descriptor/status semantics,
- centralize platform-specific logic for `stat`/`fstat` handling, especially for Windows-specific behavior now represented by `stat-w32.c`,
- convert C-style integer return/error patterns into idiomatic Rust `Result` values internally, while preserving external behavior expected by the surrounding port.

Memory management remains straightforward because the listed functions are syscall/wrapper oriented and do not imply ownership-heavy heap structures. The main migration risks are platform differences, exact errno propagation, file-descriptor validity checks, and matching the C module’s distinction between “original” and replacement entry points.

## Technical Context

### Language/Version

- Rust stable, edition 2021
- Minimum recommended compiler: Rust 1.76+

### Primary Dependencies

Prefer the Rust standard library first.

Recommended crates only where syscall parity is needed:

- `libc` — for POSIX constants, file descriptor types, `stat` layout interop, and low-level syscall access where `std` is insufficient.
- `windows-sys` — only if the Windows path in `stat-w32.c` requires direct Win32 handle/stat calls not expressible through `std`.

Do not add broader abstraction crates unless required by code discovered during implementation.

### Testing

- `cargo test`

Testing should focus on deterministic filesystem and descriptor behavior:
- duplicate descriptor behavior for valid and invalid descriptors,
- `fcntl` command coverage only for commands used by the current C implementation,
- `open` flag translation and error behavior,
- `fstat`/`stat` metadata population and failure paths,
- platform-specific tests gated with `cfg(unix)` / `cfg(windows)`.

### Performance Goals

- Preserve syscall-level performance characteristics close to the C implementation.
- Avoid unnecessary allocations in descriptor and metadata paths.
- Keep wrappers thin so overhead is limited to argument conversion and error mapping.
- Do not introduce caching or additional buffering not present in the source module.

## Module Mapping

Create a single Rust module for this migration cluster, with minimal internal sub-division only where platform separation is necessary.

### Proposed Rust file layout

- `src/module_gnu_stat_04/mod.rs`
- `src/module_gnu_stat_04/fd_ops.rs`
- `src/module_gnu_stat_04/stat_ops.rs`
- `src/module_gnu_stat_04/stat_windows.rs` (only compiled on Windows)

### C-to-Rust mapping

| C File | Rust Target | Notes |
|---|---|---|
| `gnu/dup2.c` | `src/module_gnu_stat_04/fd_ops.rs` | Port `klibc_dup2` as a thin descriptor-duplication wrapper. |
| `gnu/fcntl.c` | `src/module_gnu_stat_04/fd_ops.rs` | Port `klibc_fcntl`; keep command handling limited to existing C behavior. |
| `gnu/open.c` | `src/module_gnu_stat_04/fd_ops.rs` | Port `open`-related wrapper logic and flag translation. |
| `gnu/fstat.c` | `src/module_gnu_stat_04/stat_ops.rs` | Port `orig_fstat` and `rpl_fstat`; preserve distinction between direct and replacement paths. |
| `gnu/stat.c` | `src/module_gnu_stat_04/stat_ops.rs` | Port `orig_stat`; keep path-based metadata retrieval close to source behavior. |
| `gnu/stat-w32.c` | `src/module_gnu_stat_04/stat_windows.rs` | Port `_gl_fstat_by_handle` and any Windows-only helpers behind `cfg(windows)`. |

### Function mapping

| C Function | Rust Function | Return Style |
|---|---|---|
| `klibc_dup2` | `pub(crate) fn klibc_dup2(...)` | `Result<RawFd, io::Error>` internally; adapt if a C-style integer API is required by surrounding code. |
| `klibc_fcntl` | `pub(crate) fn klibc_fcntl(...)` | `Result<c_int, io::Error>` |
| `orig_fstat` | `pub(crate) fn orig_fstat(...)` | `Result<StatBuf, io::Error>` or `Result<(), io::Error>` depending on surrounding call pattern. |
| `rpl_fstat` | `pub(crate) fn rpl_fstat(...)` | same shape as `orig_fstat`; wrapper around validation/platform-specific path. |
| `open` | `pub(crate) fn open(...)` | `Result<RawFd, io::Error>` |
| `_gl_fstat_by_handle` | `pub(crate) fn gl_fstat_by_handle(...)` | Windows-only `Result<StatBuf, io::Error>` |
| `orig_stat` | `pub(crate) fn orig_stat(...)` | `Result<StatBuf, io::Error>` or equivalent out-parameter adaptation. |

If the surrounding port requires C-compatible signatures, keep an internal idiomatic layer and a narrow compatibility layer in the same files rather than introducing extra modules.

## Data Model

The input only identifies anonymous C structures, so the plan should avoid inventing new domain models. Use direct Rust representations that mirror the C storage actually needed by these functions.

### Data-structure mapping strategy

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous struct used for `stat` storage | `libc::stat` on Unix | Prefer direct OS layout to avoid field mismatch. |
| anonymous struct used for Windows file information | private Rust struct mirroring required Win32 fields, or `windows-sys` types directly | Only define a local struct if the Win32 API call pattern requires intermediate storage. |
| anonymous flag/argument bundles | plain function parameters | Do not introduce wrapper structs unless required by repeated use. |
| anonymous temporary buffers | stack locals / `MaybeUninit<T>` | Use only when syscall APIs require uninitialized output buffers. |
| anonymous integer-based mode/flag values | `libc::c_int`, `libc::mode_t`, `u32` as appropriate | Preserve exact widths at syscall boundaries. |

### Rust type choices

- File descriptors:
  - Unix: `std::os::fd::RawFd`
  - Windows equivalents only where necessary for handle-based logic
- Paths:
  - `&std::path::Path` internally where possible
  - convert to platform-specific string forms only at OS call boundaries
- Status buffers:
  - Unix: `libc::stat`
  - Windows: either native Win32 info structs through `windows-sys` or a small internal `StatBuf` compatibility struct if required by the current port architecture
- Optional out-parameters from C:
  - replace with return values in internal functions
  - preserve mutable output buffers only where compatibility with existing translated callers requires it

### Memory management and safety rules

- No manual heap management should be introduced for this module.
- Use stack allocation for syscall output structures.
- Use `MaybeUninit` only for FFI/syscall output buffers that are fully initialized by the OS.
- Validate descriptor and handle values before passing them to low-level APIs where the C code distinguishes invalid-input handling from syscall failure.
- Avoid borrowed data escaping syscall wrappers; convert path inputs at call time and return owned Rust errors/results.

### Error handling mapping

- Map `-1`/`errno` C patterns to `std::io::Result`.
- Use `std::io::Error::last_os_error()` immediately after low-level call failures.
- Where replacement functions must preserve exact errno-like behavior for callers, keep a thin adaptation layer near the exported function boundary.
- Do not collapse distinct failure modes if the C code treats them differently, especially for:
  - invalid descriptor,
  - unsupported `fcntl` command,
  - path conversion failure,
  - Windows handle/stat translation errors.

## Implementation Phases

## Phase 1: Scaffold module and port descriptor operations

Scope:
- Create `src/module_gnu_stat_04/mod.rs`
- Implement `fd_ops.rs`
- Port:
  - `klibc_dup2`
  - `klibc_fcntl`
  - `open`

Technical decisions:
- Use `libc` syscall bindings on Unix for exact flag and descriptor semantics.
- Keep open/fcntl flag handling as integer-based translation logic rather than wrapping in custom enums unless a direct one-to-one Rust type already exists.
- Expose only the functions required by the current module graph.

Validation:
- Unit tests for:
  - duplicating a valid descriptor,
  - failure on invalid descriptor,
  - `open` with read/write/create combinations used by the original code,
  - `fcntl` behavior for supported commands only.

## Phase 2: Port Unix stat/fstat paths

Scope:
- Implement `stat_ops.rs`
- Port:
  - `orig_fstat`
  - `rpl_fstat`
  - `orig_stat`

Technical decisions:
- Use `libc::stat`, `libc::fstat`, and path-based stat calls directly on Unix.
- Preserve separation between original and replacement functions:
  - `orig_*` should be thin syscall-facing wrappers,
  - `rpl_fstat` should contain any compatibility checks or normalization present in the C version.
- Keep metadata translation minimal; return raw/native stat storage unless surrounding translated code requires a normalized wrapper.

Validation:
- Tests covering:
  - metadata retrieval for regular files,
  - metadata retrieval through duplicated descriptors,
  - invalid descriptor/path failure mapping,
  - consistency between `orig_fstat` and `rpl_fstat` on supported cases.

## Phase 3: Port Windows-specific stat behavior

Scope:
- Implement `stat_windows.rs`
- Port:
  - `_gl_fstat_by_handle`
  - any directly required helpers from `stat-w32.c`

Technical decisions:
- Compile only on Windows via `cfg(windows)`.
- Prefer `std` where it can provide equivalent metadata; use `windows-sys` only if handle-based status extraction must match the C logic more closely than `std::fs::Metadata` permits.
- Keep the Windows path isolated so Unix code paths remain simple.

Validation:
- Windows-only tests for:
  - handle-based `fstat` on an open file,
  - failure mapping for invalid handles/descriptors,
  - parity between replacement and original paths where both exist.

## Phase 4: Integration cleanup and compatibility verification

Scope:
- Wire exports through `mod.rs`
- Adjust signatures to match the surrounding Rust port’s expected call sites
- Remove any temporary duplication between Unix and Windows stat handling

Technical decisions:
- Keep compatibility shims local to this module rather than creating new cross-cutting utility layers.
- Ensure all unsafe blocks are narrowly scoped and documented with syscall/OS invariants.
- Confirm no extra abstractions remain beyond what is needed to mirror the original files and functions.

Validation:
- Run full `cargo test`
- Verify platform gating compiles cleanly:
  - Unix build without Windows code,
  - Windows build without Unix-only assumptions
- Review error propagation and structure initialization for all syscall boundaries.