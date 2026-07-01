# Implementation Plan: main_root_safe_rw_15

## Summary

This module ports the C `safe-read.c` implementation of `safe_rw` into Rust as a small, focused I/O utility within the main execution path of `cat`. The Rust version should preserve the existing low-level behavior: repeatedly attempting a read/write style operation until completion, a terminal result, or a non-retryable error condition is reached.

The implementation should stay narrowly aligned with the current C module layout and avoid introducing broader abstractions. The preferred Rust approach is:

- represent the retrying I/O logic as a dedicated module function,
- use Rust standard library I/O types and error handling where possible,
- keep interruption and partial-progress handling explicit,
- avoid heap allocation or new data structures unless required by the original logic.

## Technical Context

- **Language/Version**: Rust 1.77+ stable
- **Primary Dependencies**: Rust standard library only (`std::io`); no third-party crates are indicated by the source input
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve the C implementation’s low-overhead retry loop behavior,
  - avoid unnecessary allocations and buffering,
  - maintain efficient handling of partial reads/writes,
  - keep syscall-facing control flow straightforward and predictable.

## Module Mapping

### C to Rust File Mapping

- `safe-read.c` → `src/safe_read.rs`
- `include/safe-read.c` → functionality merged into `src/safe_read.rs` or exposed through the crate’s existing module declarations

### Function Mapping

- `safe_rw` → `pub(crate)` Rust function in `src/safe_read.rs`
- duplicated `safe_rw` listing in the analysis → validate whether this reflects declaration/definition duplication; implement a single canonical Rust function

### Rust Module Placement

- Add a single Rust module for this port:
  - `src/safe_read.rs`
- Expose only as needed by the existing main-cluster module tree:
  - `mod safe_read;` in the nearest existing module root
  - avoid adding extra wrapper modules or utility layers beyond what is needed to replace the original C call sites

## Data Model

No explicit C structs were identified for this module.

### Type and API Mapping

Because this module is function-oriented rather than struct-oriented, the main data mapping is at the parameter and result level:

- C file descriptor / low-level handle → Rust borrowed I/O object or raw descriptor-compatible representation, depending on surrounding port constraints
- C byte buffer pointer + length → Rust `&mut [u8]` for reads or `&[u8]` for writes
- C signed size result (`ssize_t`-style) → Rust `io::Result<usize>` where possible
- C errno-based retry/error behavior → Rust `std::io::Error` / `ErrorKind`

### Error Handling Mapping

- interrupted operation (`EINTR`) → retry loop in Rust
- partial transfer → continue until stopping condition defined by the original function
- hard error → return `Err(io::Error)`
- EOF / zero-progress terminal state → return `Ok(0)` or the accumulated count, matching the original control flow semantics

### Memory Management Notes

- use slice borrowing instead of raw pointer arithmetic where the surrounding port allows,
- if raw-descriptor compatibility is required by neighboring migrated code, isolate any unsafe interaction to the narrowest possible section,
- do not introduce owned buffers or extra copies beyond what the original function already implies.

## Implementation Phases

## Phase 1: Port Skeleton and Signature Stabilization

- Create `src/safe_read.rs`.
- Identify the exact `safe_rw` signature and call pattern from the C source.
- Define one Rust function matching the existing usage as closely as practical in Rust terms.
- Decide the minimal Rust-level interface based on the surrounding migrated code:
  - standard I/O trait-based if call sites already use Rust I/O objects, or
  - low-level descriptor-oriented if the main cluster still operates near C-style handles.
- Add module wiring only where required for current callers.

### Deliverables

- `src/safe_read.rs` added
- canonical Rust `safe_rw` function stub with final signature
- module declaration integrated into existing crate structure

## Phase 2: Core Retry Loop Migration

- Translate the C retry loop into Rust without expanding behavior.
- Preserve the original handling for:
  - interrupted system calls,
  - partial progress,
  - terminal zero-length outcomes,
  - propagation of non-retryable errors.
- Keep the implementation allocation-free.
- Minimize unsafe code; if unavoidable for raw OS interaction, keep it local and documented.

### Deliverables

- working Rust implementation of `safe_rw`
- explicit mapping from C errno-driven branches to Rust `io::ErrorKind` handling
- comments only where needed to explain non-obvious C-to-Rust behavior preservation

## Phase 3: Call-Site Integration and Behavioral Alignment

- Replace existing uses of the C module with the Rust module in the current branch scope.
- Verify that the return-value conventions expected by callers are preserved.
- Ensure no extra abstraction changes propagate outside this module.
- Reconcile any mismatch between declaration-style and definition-style occurrences of `safe_rw` from the source analysis.

### Deliverables

- Rust call sites updated to use `safe_rw`
- compile-clean integration with the main cluster
- no residual dependency on the original C implementation for this module

## Phase 4: Targeted Tests and Edge-Case Validation

- Add focused unit tests for the retry logic using standard library testing facilities.
- Cover the observable edge cases that matter for migration correctness:
  - interrupted call retry behavior,
  - partial transfer progression,
  - immediate hard error propagation,
  - zero-length terminal result.
- Run `cargo test` and confirm behavior remains aligned with the original module intent.

### Deliverables

- unit tests for `safe_rw`
- passing `cargo test`
- validated error and partial-progress semantics