# Implementation Plan: main_root_stat_03

## Summary

This module covers the migration of the C entry path in `cat.c` together with the local file-control helper in `fcntl.c` into a Rust binary-oriented implementation. The Rust port should preserve the existing command-flow shape and system-call-facing behavior as closely as practical while replacing manual C resource management with Rust ownership and `Result`-based error propagation.

The implementation approach is intentionally narrow:

- migrate `main` into the Rust binary entry point;
- migrate `klibc_fcntl` into a small Rust helper module;
- use the Rust standard library first for file descriptor and argument handling;
- only drop to low-level OS bindings if a direct `fcntl` operation is required and cannot be expressed through `std`.

The port should avoid adding new abstraction layers beyond what is needed to separate the translated `main` path from the translated `fcntl` helper.

## Technical Context

### Language / Version

- Rust 1.78 or newer
- Edition: 2021

### Primary Dependencies

Use the Rust standard library by default.

Recommended crates only if required by the exact `fcntl` behavior during translation:

- `libc`: only for direct `fcntl` constants or calls that are not available through `std::os::*`

No other third-party crates are planned from the provided input.

### Testing

- `cargo test`

Test focus:

- argument and entry-path behavior that can be exercised without expanding scope;
- unit tests for the translated `klibc_fcntl` wrapper behavior where practical;
- integration-style tests for process exit status and stderr/stdout behavior if the existing `main` logic exposes them.

### Performance Goals

- Preserve the operational profile of the original C implementation for startup and per-call file-control operations.
- Avoid unnecessary allocation in the `main` path beyond argument collection required by Rust.
- Keep file descriptor handling zero-copy and direct where possible.
- Match C behavior closely enough that no additional syscall or buffering layers are introduced unless required by Rust APIs.

## Module Mapping

### C to Rust File Mapping

| C File | Rust Target | Notes |
|---|---|---|
| `cat.c` | `src/main.rs` | Hosts the translated `main` entry point and only the minimal helper logic directly tied to startup flow. |
| `fcntl.c` | `src/fcntl.rs` | Contains the Rust translation of `klibc_fcntl` and any constant/flag handling needed for that function. |

### Function Mapping

| C Function | Rust Function | Notes |
|---|---|---|
| `main` | `fn main()` plus small internal helpers if needed | Keep logic close to original control flow; return path should map to process exit behavior. |
| `klibc_fcntl` | `pub(crate) fn klibc_fcntl(...) -> Result<..., std::io::Error>` or a narrow integer return if exact behavior requires it | Prefer `Result`; preserve raw descriptor semantics where needed. |

### Rust Module Layout

```text
src/
  main.rs
  fcntl.rs
```

`main.rs` should declare:

```rust
mod fcntl;
```

No additional modules are planned unless translation reveals a strictly necessary local helper type.

## Data Model

The analysis only identifies three anonymous C data structures without named definitions. Since no concrete fields are provided, the plan should keep data-structure migration minimal and driven by actual use sites in `cat.c` and `fcntl.c`.

### Data-Structure Mapping Strategy

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous | Inline local variables or a private `struct` only if the original anonymous aggregate has repeated use | Do not invent public types without direct need. |
| anonymous | Tuple/local binding or private `struct` | Choose the smallest representation matching usage. |
| anonymous | Private `struct` or enum if used for flags/state branching | Prefer explicit typed fields over raw mutable memory. |

### Conversion Rules

- C anonymous structs used only for temporary grouping should become local Rust bindings instead of standalone types.
- C flag-like integer fields should become:
  - `i32`/`u32` if exact raw OS values must be preserved;
  - a private enum only if there is clear branching logic and no loss of fidelity.
- Pointers to owned data should become owned Rust values (`String`, `Vec<u8>`, or dedicated structs) only when ownership is real.
- Borrowed C strings from argv-like inputs should map to `OsString`/`OsStr` first, converting to UTF-8 `String` only if the original logic requires text semantics.
- Raw file descriptors should remain raw OS descriptors at the helper boundary when interacting with `fcntl`.

### Memory Management and Error Handling

- Replace manual lifetime tracking with Rust ownership and scope-based cleanup.
- Avoid heap allocation for fixed temporary state that can remain on the stack.
- Use `std::io::Result` / `Result<T, io::Error>` for syscall-adjacent helpers.
- In `main`, translate recoverable helper failures into the same observable exit behavior expected from the original C path.
- Where exact errno-sensitive behavior matters, retain the underlying OS error via `std::io::Error::last_os_error()` or `libc` return checks.

## Implementation Phases

## Phase 1: Establish Binary Entry and File Mapping

### Goal

Create the Rust crate structure that mirrors the C module split and set up a direct translation path for the module entry point.

### Tasks

- Create `src/main.rs` for the translated `main`.
- Create `src/fcntl.rs` for the translated `klibc_fcntl`.
- Add `mod fcntl;` in `main.rs`.
- Translate C includes and macro dependencies into Rust `use` statements and private constants only where used by these two files.
- Identify whether `klibc_fcntl` can be implemented with only `std`; if not, add the minimal `libc` dependency.

### Deliverables

- Compiling Rust project skeleton on branch `004-main_root_stat_03-rust-port`
- Placeholder signatures matching the migrated functions
- Initial constant and type aliases required for buildability

## Phase 2: Port `klibc_fcntl` with Raw Descriptor Semantics

### Goal

Translate the low-level file-control helper first so the entry path can depend on a stable Rust equivalent.

### Tasks

- Inspect the exact `klibc_fcntl` call pattern and command variants used by the module.
- Implement the helper in `src/fcntl.rs` using:
  - standard library descriptor types where sufficient;
  - direct `libc::fcntl` only if necessary for exact command support.
- Preserve integer flag handling and command arguments with minimal reshaping.
- Convert C-style error returns into Rust `Result`, or keep raw return values internally if exact translation requires them and wrap at the module boundary.
- Add focused unit tests around supported command paths and failure handling.

### Deliverables

- Working `klibc_fcntl` translation
- Tests covering successful and failing helper calls
- Documented assumptions for any remaining raw integer command/flag mappings

## Phase 3: Port `main` Control Flow

### Goal

Migrate the `cat.c` entry logic into Rust while keeping behavior and output flow aligned with the original implementation.

### Tasks

- Translate `main` into `fn main()`.
- Map argc/argv handling to `std::env::args_os()`.
- Keep option/input traversal in the same order as the C source.
- Replace direct C error/reporting paths with Rust stderr writes and explicit process exit codes.
- Call the translated `klibc_fcntl` helper from `main` where the original C flow does.
- Minimize helper extraction; only split small private functions if needed to keep the translation readable and testable.

### Deliverables

- Functional Rust entry point reflecting the original C control flow
- Correct exit status propagation
- Preserved descriptor and argument handling behavior

## Phase 4: Verification and Behavioral Tightening

### Goal

Confirm that the Rust module is complete, minimal, and behaviorally aligned with the original files.

### Tasks

- Run `cargo test`.
- Add integration tests for executable behavior that directly exercise the migrated `main` path, limited to scenarios evident from the translated code.
- Review all translated paths for:
  - raw descriptor lifetime correctness;
  - no accidental ownership transfer of borrowed OS resources;
  - faithful error-path mapping;
  - removal of unused placeholder constants or temporary compatibility code.
- Ensure the final layout still consists only of the required Rust files for this module.

### Deliverables

- Passing test suite
- Cleaned, minimal Rust module port
- Final review confirming no unevidenced expansion beyond `cat.c` and `fcntl.c`