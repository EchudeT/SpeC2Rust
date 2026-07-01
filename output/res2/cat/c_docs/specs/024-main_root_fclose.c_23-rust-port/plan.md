# Implementation Plan

## Summary

Port `fclose.c` into a Rust module that preserves the existing close-and-status behavior of the C implementation while fitting the current `cat` codebase structure. The Rust work should focus only on the two existing functions, `fclose_nothrow` and `rpl_fclose`, and should not introduce broader I/O abstractions.

The implementation approach is to map the C file-level close logic onto Rust file-handle closing patterns using the standard library. Because Rust closes files through ownership and `Drop` rather than a direct `fclose` equivalent, the port should represent explicit close operations by consuming owned file objects and forcing buffered output to be flushed before release where needed. Return values should be translated into Rust `Result` forms internally, with thin compatibility-oriented wrappers if the surrounding port still expects C-style integer status codes.

Special care is required for behavioral parity around:
- explicit close timing,
- flush-before-close semantics for writable streams,
- preservation of error information from failed flush/close paths,
- avoiding double-close or use-after-close through ownership transfer.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s practical overhead for normal file close paths.
  - Avoid unnecessary allocations or buffering layers beyond those already required by Rust I/O types.
  - Keep error-path handling minimal and synchronous.
  - Preserve O(1) close-path logic aside from required OS flush/close behavior.

## Module Mapping

| C Source File | C Function | Rust Module/File | Rust Item | Notes |
|---|---|---|---|---|
| `fclose.c` | `fclose_nothrow` | `src/main_root_fclose.rs` | `fn fclose_nothrow(...) -> ...` | Direct migration target; keep behavior limited to close without propagating panic conditions. |
| `fclose.c` | `rpl_fclose` | `src/main_root_fclose.rs` | `fn rpl_fclose(...) -> ...` | Wrapper-level close routine preserving C status semantics as needed by callers. |

### Proposed Rust placement

- Add a single Rust source file for this module:
  - `src/main_root_fclose.rs`

- Expose only what the existing ported call graph requires:
  - module-private helpers where possible,
  - `pub(crate)` visibility if called from other translated modules.

This keeps the port aligned to the original file boundary and avoids inventing new utility layers.

## Data Model

No named C data structures were identified in this module.

### Type and API mapping

| C Concept | Rust Mapping | Notes |
|---|---|---|
| `FILE *` | Owned standard-library file handle, likely `std::fs::File` or a crate-local stream abstraction already used by the port | Use the narrowest existing Rust type that matches how the wider project represents opened streams. |
| `int` status return from close routines | `Result<(), std::io::Error>` internally; compatibility return code at module boundary if required | Prefer Rust error typing internally, then convert to `0` / `EOF`-style status only where integration requires it. |
| `errno`-based failure reporting | `std::io::Error` / `ErrorKind` | Preserve underlying OS error when available. |

### Ownership and memory management

- C `fclose(FILE *)` semantics should map to consuming ownership of the Rust file object.
- The Rust implementation must not leave an accessible handle after a successful explicit close path.
- If buffering is involved, flush must occur before the handle is released so write errors are observed deterministically.
- No manual memory management structures are needed; rely on Rust ownership and `Drop`.

## Implementation Phases

## Phase 1: Establish module skeleton and signature mapping

- Create `src/main_root_fclose.rs`.
- Add Rust equivalents for `fclose_nothrow` and `rpl_fclose`.
- Determine the exact stream type used by the current Rust port of `cat` for open files/stdio handles, and bind these functions to that type rather than introducing a new abstraction.
- Decide the outward-facing signature style:
  - use `Result` internally,
  - convert to integer status codes only if existing translated callers require C-compatible returns.

### Deliverables
- New Rust module file with function stubs and imports.
- Initial compileable signatures aligned with the current project call sites.

## Phase 2: Port close semantics and error handling

- Implement the actual close path for owned file/stream values.
- Ensure writable buffered streams are flushed before final release.
- Preserve the first meaningful I/O error from flush/close-related operations.
- Keep `fclose_nothrow` free of panic-based control flow; return ordinary error/status information instead.
- Implement `rpl_fclose` as the behavioral wrapper corresponding to the original C layering, without extending functionality.

### Rust-specific handling decisions
- Use ownership-consuming functions to prevent reuse after close.
- Avoid `unwrap`, `expect`, or panic-driven error propagation.
- If the wider port uses `BufWriter` or similar wrappers, explicitly call `flush`.
- If only `std::fs::File` is involved, rely on `sync`/flush behavior only when the original C intent requires visible write error detection; do not add stronger durability semantics.

### Deliverables
- Functional close logic in both functions.
- Error/status conversion logic matching expected caller behavior.

## Phase 3: Integrate with callers and preserve module boundaries

- Replace references to the C-originated close helpers with calls into `main_root_fclose`.
- Keep integration local to the existing translated call graph; do not refactor unrelated I/O code.
- Verify visibility (`pub(crate)` vs private) based on actual use sites.
- Confirm that there is no duplicate close logic elsewhere that should remain authoritative for this migrated file.

### Deliverables
- Module wired into the Rust branch build.
- Clean compilation with no dead duplicate wrappers introduced by the migration.

## Phase 4: Add focused tests for close-path behavior

- Add unit tests and, if needed, narrow integration tests for:
  - successful close of a readable file,
  - successful flush-and-close of a writable file,
  - error propagation when flushing buffered output fails, where testable,
  - status-code mapping if a C-style integer return is exposed.
- Use temporary files from the standard library test ecosystem already present in the project; do not add new dependencies solely for this module.
- Run `cargo test` and fix any ownership or lifetime mismatches revealed during integration.

### Deliverables
- Test coverage for the two migrated functions.
- Passing `cargo test` for the module and its immediate integration points.