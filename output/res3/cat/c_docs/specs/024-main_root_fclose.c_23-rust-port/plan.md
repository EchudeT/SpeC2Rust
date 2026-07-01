# Implementation Plan

## Summary

Port `fclose.c` into a Rust module that preserves the existing close-and-status behavior represented by `fclose_nothrow` and `rpl_fclose`, with emphasis on matching C-side error propagation and resource finalization semantics rather than redesigning I/O abstractions.

The Rust implementation should stay narrowly scoped to the existing module responsibilities:
- provide Rust equivalents for the two functions,
- map C stream-closing behavior onto Rust file/stream ownership patterns,
- make close failures explicit through `Result`-based returns,
- avoid adding broader I/O utility layers beyond what is required to host the migrated logic.

The preferred technical approach is:
- represent owned closeable resources with standard-library types such as `std::fs::File` where the migrated call sites operate on real files,
- encapsulate any required explicit close operation through ownership-consuming helper functions,
- preserve the distinction between a non-throwing/internal close path and the replacement `fclose` path by keeping two Rust functions with separate responsibilities,
- convert C integer status codes into a small Rust return convention, ideally `io::Result<()>` internally and, where required by surrounding ported interfaces, a final integer status adapter.

Because Rust closes files via `Drop`, the port must explicitly force synchronization of the C semantics where `fclose` reports buffered-write or close-time errors. This means the implementation should consume the stream object, flush if needed, and surface close-related errors rather than relying only on destructor behavior.

## Technical Context

- **Language/Version:** Rust 1.75+
- **Primary Dependencies:** Rust standard library only (`std::fs`, `std::io`)
- **Testing:** `cargo test`
- **Performance Goals:**
  - no meaningful regression versus C for normal file-close paths,
  - no extra heap allocation in the close helpers,
  - preserve constant-time wrapper overhead around the underlying OS/file close operation,
  - ensure error handling remains direct and does not introduce retry loops or buffering beyond existing stream behavior.

## Module Mapping

| C File | C Function | Rust Module | Rust Item | Notes |
|---|---|---|---|---|
| `fclose.c` | `fclose_nothrow` | `src/main_cluster/fclose.rs` | `fn fclose_nothrow(...) -> ...` | Internal helper for explicit close without panic-based behavior. |
| `fclose.c` | `rpl_fclose` | `src/main_cluster/fclose.rs` | `fn rpl_fclose(...) -> ...` | Main replacement close routine preserving C-visible status semantics. |

Suggested Rust module exposure:

| Rust Path | Purpose |
|---|---|
| `src/main_cluster/mod.rs` | Re-export or declare `fclose` submodule if this cluster already exists. |
| `src/main_cluster/fclose.rs` | Direct migration target for `fclose.c`. |

If the current Rust port keeps a flatter layout, the minimum acceptable alternative is:
- `src/fclose.rs` for the implementation,
- one corresponding `mod` declaration in the crate root.

The file placement should follow the existing branch/project layout and should not introduce extra helper modules unless the current crate structure already requires them.

## Data Model

This module has no standalone C-defined data structures in the provided analysis, so the plan should keep the Rust data model minimal and function-oriented.

### C-to-Rust Type Mapping

| C Concept | Rust Mapping | Notes |
|---|---|---|
| `FILE *` | `std::fs::File` or another owned writer/stream type already used by the port | Prefer the concrete existing Rust stream abstraction used by surrounding migrated code. |
| `int` status return from `fclose`-style API | `std::io::Result<()>` internally; adapt to `i32` only if required by the surrounding ported interface | Keeps error handling idiomatic while preserving C-compatible status at boundaries. |
| `errno`-style failure signaling | `std::io::Error` | Preserve OS error codes where available via standard I/O errors. |

### Ownership and Memory Management

- C closes a `FILE *` explicitly; Rust should model this as consuming ownership of the stream/file in the close function.
- Do not rely solely on `Drop` for observable close success/failure, because C `fclose` returns status.
- Any buffering semantics present in the chosen Rust type must be finalized explicitly before ownership is dropped.
- Avoid raw pointers unless required by already-ported neighboring code; if unavoidable, isolate pointer handling at the function boundary and convert immediately into safe ownership/borrowing semantics.

## Implementation Phases

### Phase 1: Establish module skeleton and function signatures

- Create the Rust file corresponding to `fclose.c`.
- Add the two migrated functions:
  - `fclose_nothrow`
  - `rpl_fclose`
- Choose signatures based on the surrounding port:
  - preferred internal form: ownership-consuming parameter with `io::Result<()>`,
  - add thin status-code adaptation only if callers still expect C-style integer returns.
- Wire the module into the existing crate/module tree without creating unrelated abstractions.

**Exit criteria:**
- Rust module compiles with placeholder or straightforward implementations.
- Public/internal visibility matches expected call sites in the port.

### Phase 2: Port close semantics and error propagation

- Implement the actual close path using standard-library stream finalization.
- Ensure the logic explicitly handles:
  - final flush before close where relevant,
  - propagation of I/O failure as a returned error/status,
  - non-panicking behavior for the `fclose_nothrow` path.
- Preserve the intended distinction between helper and wrapper functions instead of collapsing them if the C module keeps them separate for behavior reasons.
- Where C returns `0`/`EOF`-style status, provide a narrow adapter from Rust `Result` to the expected integer convention.

**Exit criteria:**
- Both functions execute the expected close path.
- Error returns are explicit and not hidden in destructors.
- No leaked ownership or double-close behavior exists in the migrated API.

### Phase 3: Integrate with existing call sites and normalize types

- Update any module-local callers to use the new Rust close functions.
- Replace C-style assumptions about reusable stream pointers after close with Rust ownership consumption.
- Resolve mismatches between generic writers and concrete files only as needed by the existing call graph.
- Keep the adaptation local to this module and immediate callers; do not introduce general-purpose I/O trait layers unless already present in the branch.

**Exit criteria:**
- Callers compile cleanly against the migrated API.
- Close operations follow Rust ownership rules without unsafe lifetime workarounds.

### Phase 4: Add focused tests for success and failure paths

- Add unit tests covering:
  - successful close of a writable file,
  - write/flush failure surfaced through the replacement close routine where reproducible,
  - status-code mapping if the external signature uses integer results,
  - idempotency is *not* required unless the original C callers depend on it.
- Prefer temporary files and standard test utilities from the standard library.
- Keep tests focused on this module’s migrated behavior; do not broaden into system-level CLI coverage here.

**Exit criteria:**
- `cargo test` passes.
- Tests demonstrate both normal completion and error propagation behavior for the close wrapper.