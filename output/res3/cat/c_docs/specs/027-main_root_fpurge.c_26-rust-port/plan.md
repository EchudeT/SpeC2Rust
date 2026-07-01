# Implementation Plan

## Summary
Port `fpurge.c` into the Rust branch `027-main_root_fpurge.c_26-rust-port` as a narrowly scoped module migration for the `fpurge` function only.

The Rust implementation should preserve the existing module boundary and behavior intent of the C source while adapting to Rust’s ownership and error-handling model. Because `fpurge` is inherently tied to C `FILE*` stream buffering semantics and the input does not indicate any broader redesign, the implementation approach should stay minimal:

- migrate the function into a Rust module dedicated to this source file,
- keep behavior aligned with the existing call surface used by the project,
- represent success/failure with Rust result types internally,
- isolate any unavoidable low-level stream interaction behind a small, explicit function boundary.

The plan should avoid introducing new abstractions beyond what is required to replace the existing file and function.

## Technical Context

- **Language/Version**: Rust 1.78+ edition 2021
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended from the provided evidence
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Maintain behavior with no meaningful regression relative to the C implementation
  - Keep the implementation allocation-free for the core purge path
  - Preserve constant-time control flow around stream state handling, aside from underlying platform I/O behavior

## Module Mapping

| C Source File | C Function | Rust Module/File | Rust Item |
|---|---|---|---|
| `fpurge.c` | `fpurge` | `src/main_root_fpurge.rs` or project-equivalent standard module path | `fpurge` |

### Mapping Notes
- Keep the migration at file/function granularity: one C file maps to one Rust module.
- Do not split helper logic into extra modules unless required by the existing crate layout.
- If the project already has a `main_cluster` area in Rust, place this module there without adding new architectural layers.

## Data Model

No module-specific data structures are identified in the input.

### C-to-Rust Type Mapping
| C Type/Concept | Rust Mapping | Notes |
|---|---|---|
| `FILE *` stream handle | Narrow wrapper over raw stream handle or existing project stream abstraction | Use the smallest compatible representation already present in the Rust port |
| integer status return | `Result<(), std::io::Error>` internally, with outward adaptation as needed | Preserve caller-visible behavior at the module boundary |
| `errno`-style failure | `std::io::Error` | Convert low-level failure into standard Rust error representation |

### Memory and Ownership
- Do not take ownership of external stream resources unless the existing Rust call path already does so.
- Accept borrowed access to stream state where possible.
- Avoid storing stream handles beyond the call.
- Keep unsafe code, if needed for low-level stream manipulation, tightly scoped and documented with invariants.

## Implementation Phases

## Phase 1: Inspect and map the existing C behavior
- Review `fpurge.c` to identify:
  - exact function signature,
  - return-value conventions,
  - whether behavior differs for readable vs writable streams,
  - any platform- or libc-specific assumptions.
- Identify the existing Rust crate location where this migrated function belongs.
- Determine whether the broader port already defines a stream-handle abstraction; if so, reuse it instead of creating a new one.

### Deliverables
- Rust module file created in the appropriate standard source location
- Function signature decision documented in code comments where adaptation is necessary

## Phase 2: Implement the Rust replacement for `fpurge`
- Port the core logic of `fpurge` into Rust with a direct control-flow translation.
- Use standard library error types for internal failure handling.
- If direct stream-buffer interaction requires unsafe code:
  - confine it to the smallest possible region,
  - document pointer validity and aliasing assumptions,
  - avoid extra wrappers not required by this module migration.
- Preserve the original function’s observable return semantics expected by the rest of the project.

### Deliverables
- Compiling Rust implementation of `fpurge`
- Minimal integration into the crate/module tree
- Inline safety notes for any unsafe block

## Phase 3: Replace usage and validate behavior
- Wire existing callers to the Rust implementation at the same module boundary used by the C code migration.
- Add unit tests covering:
  - successful purge path,
  - failure propagation for invalid/unusable stream state,
  - any distinct read-buffer vs write-buffer handling visible in the original implementation.
- Run `cargo test` and fix integration mismatches without widening scope.

### Deliverables
- Caller path updated to use the Rust module
- Test coverage for the migrated function’s expected behavior
- Passing `cargo test`

## Phase 4: Final cleanup and parity review
- Compare the Rust implementation against `fpurge.c` for control-flow and error-path parity.
- Remove any temporary migration scaffolding introduced during the port.
- Ensure the final module remains limited to the original file/function scope and does not introduce unrelated helpers or facilities.

### Deliverables
- Final parity review completed
- Module limited to the migrated `fpurge` functionality only