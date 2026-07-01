# Implementation Plan

## Summary

Port `xbinary-io.c` into a small Rust module that preserves the existing role of `xset_binary_mode_error` within the `cat` main cluster. The Rust implementation should stay narrowly scoped to the current file and function set, using standard library facilities and platform-conditional compilation where needed.

The technical approach is to migrate the C function behavior into a Rust function with explicit error propagation via `Result`, avoiding manual memory management and preserving the current module boundary. Because this module appears to be focused on binary I/O mode error handling, the Rust port should model any platform-specific behavior explicitly and keep the public surface minimal.

## Technical Context

- **Language/Version:** Rust 1.78+ stable
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - No meaningful regression versus the C implementation for startup-path or error-path execution
  - Zero heap allocation unless required by message construction already inherent in the behavior
  - Keep binary-mode setup/error reporting logic constant-time and lightweight

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `xbinary-io.c` | `src/main_root/xbinary_io.rs` | Direct port of the file-level responsibility with no scope expansion |
| `xset_binary_mode_error` | `xset_binary_mode_error` | Preserve function intent and keep signature minimal and idiomatic for Rust integration |

## Data Model

This module analysis shows no dedicated C data structures.

| C Construct | Rust Construct | Notes |
|---|---|---|
| No struct/enum definitions | None required | Keep implementation function-based unless migration reveals hidden constants or internal enums strictly needed for control flow |

## Implementation Phases

### Phase 1: Establish Rust module and API surface

- Create `src/main_root/xbinary_io.rs` for the direct port of `xbinary-io.c`.
- Add the module declaration from the existing crate root or parent module using standard Rust project structure.
- Define the Rust version of `xset_binary_mode_error` with an idiomatic signature:
  - Prefer returning `std::io::Result<()>` if the function participates in operational error propagation.
  - If the original role is strictly diagnostic/terminating, represent that explicitly without introducing broader abstractions.
- Identify all call sites that depend on this C function and map them to the Rust module without changing surrounding behavior.

### Phase 2: Port core logic and platform-specific behavior

- Translate the function body from C control flow into Rust using:
  - `std::io::Error` / `std::io::ErrorKind` for I/O-related failures
  - borrowed string types (`&str`) where inputs are textual
  - conditional compilation for platform-specific binary-mode behavior if the original implementation distinguishes Windows from Unix-like systems
- Remove any C-style manual error-state handling in favor of explicit return values.
- Preserve current semantics for:
  - when an error is raised
  - what context is attached to the error
  - whether the function only reports, or reports and returns failure
- Avoid introducing helper modules unless a tiny private helper is necessary inside the same file.

### Phase 3: Integrate with existing error flow

- Update the Rust port of the surrounding main-cluster code to call `xset_binary_mode_error` from `src/main_root/xbinary_io.rs`.
- Ensure ownership and lifetimes remain simple:
  - pass borrowed inputs where possible
  - avoid unnecessary `String` creation
- Align the function with the crate’s existing error handling style, but do not widen the module’s responsibility beyond binary I/O mode error processing.
- Confirm that no unsafe code is needed; if platform behavior forces a non-std call later, isolate that decision and keep the current plan std-first.

### Phase 4: Add focused tests and finalize migration

- Add unit tests covering the Rust function’s observable behavior:
  - success path if applicable
  - failure path and returned error shape/message context
  - platform-conditional expectations only where behavior differs materially
- Verify build integration with `cargo test`.
- Remove or mark the original C implementation as replaced in the migration branch once all call sites are switched.
- Perform a final pass to ensure the Rust file remains a direct migration of `xbinary-io.c` and has not accumulated unrelated utilities.