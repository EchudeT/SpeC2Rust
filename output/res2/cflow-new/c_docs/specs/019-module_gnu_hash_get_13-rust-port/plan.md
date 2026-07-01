# Implementation Plan

## Summary

This module ports the GNU hash count/access helper logic from `gnu/hash.c` into a focused Rust module that exposes the three existing query functions:

- `hash_get_n_buckets`
- `hash_get_n_buckets_used`
- `hash_get_n_entries`

The Rust implementation should preserve the current scope of the C code: reading hash-table metadata and computing/reporting counts without adding new parsing features or broader hash-table services.

Technical approach:

- Migrate the logic into a single Rust source module that mirrors the existing C file’s responsibility.
- Replace pointer-based access with borrowed Rust views over already-validated binary data structures.
- Keep integer conversions explicit to avoid truncation or sign issues.
- Model absent/invalid state with `Result` where internal validation is required, while keeping public function behavior aligned with the existing C semantics as closely as possible.
- Use slices and standard library iteration for bucket and entry counting instead of raw pointer arithmetic.

## Technical Context

- **Language/Version**: Rust 1.78 or newer
- **Primary Dependencies**:
  - Rust standard library only
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Match the C implementation’s asymptotic behavior for all three functions.
  - Avoid heap allocation in the normal counting/query path.
  - Operate on borrowed data and contiguous slices where possible.
  - Keep per-call overhead limited to bounds checks and simple iteration already implied by the C logic.

## Module Mapping

### C to Rust File Mapping

- `gnu/hash.c` → `src/gnu/hash.rs`

If the Rust project already groups GNU-related code under a module tree, expose this file with the minimal required declaration:

- `src/gnu/mod.rs` → `pub mod hash;`

No additional module split is planned because the source scope is limited to three related functions.

### Function Mapping

- `hash_get_n_buckets` → `pub fn hash_get_n_buckets(...) -> ...`
- `hash_get_n_buckets_used` → `pub fn hash_get_n_buckets_used(...) -> ...`
- `hash_get_n_entries` → `pub fn hash_get_n_entries(...) -> ...`

Notes for migration:

- Preserve naming intent closely; exact Rust signatures should be chosen based on the existing project’s surrounding types.
- If the C functions currently accept pointers to an owning/hash descriptor structure, the Rust versions should accept shared references to the corresponding Rust struct.
- If the C functions may currently return sentinel values on invalid input, decide at the module boundary whether to:
  - preserve raw numeric return behavior internally, or
  - expose `Result<usize, HashError>` and convert at the integration boundary if needed.

## Data Model

The analysis reports only anonymous C data structures, so the plan should avoid inventing new domain abstractions beyond what is required to represent the existing memory layout and function inputs.

### Data-Structure Mapping

Because the C source references anonymous structures, map them by role rather than by assumed names:

- Anonymous C hash header/layout data
  - → Rust struct such as `GnuHashHeader` or a private module-local struct
  - Fields should use fixed-width integers where the binary layout is known, for example:
    - `u32` for bucket/count/header fields
    - `usize` only for derived indexing and returned counts

- Anonymous C container holding GNU hash state
  - → Rust struct such as `GnuHashView<'a>`
  - Should contain borrowed slices instead of raw pointers, for example:
    - `buckets: &'a [u32]`
    - `chains: &'a [u32]`
    - optional metadata fields required by the three query functions

- Anonymous C nullable or optional pointer members
  - → `Option<&'a T>` or `Option<&'a [T]>` when absence is valid in the original logic

- Anonymous C integer flags or mode values
  - → plain integer fields, or a small private `enum` only if the C logic clearly branches on a closed set of states needed by these functions

### Memory Management Decisions

- No manual allocation should be introduced for this module if the C logic only inspects existing structures.
- Prefer borrowed views with lifetimes to represent non-owning access to already-loaded hash data.
- Eliminate pointer arithmetic in favor of indexed slice access.
- Guard all derived indices with length checks before iteration.

### Error Handling Decisions

- Invalid layout, out-of-range counts, or inconsistent slice lengths should be detected explicitly.
- Use a small module-local error type only if required to separate malformed data from valid zero-count results.
- Avoid `unsafe` unless the surrounding codebase already requires binary layout reinterpretation; if unavoidable, isolate it to a narrow constructor and keep query functions safe.

## Implementation Phases

## Phase 1: Establish Rust Module Skeleton and Input Types

Goals:

- Create `src/gnu/hash.rs`.
- Identify the exact C inputs used by the three functions in `gnu/hash.c`.
- Define the minimal Rust structs/enums needed to represent those inputs and related hash metadata.

Tasks:

- Translate the C file’s local data access pattern into Rust field/slice access.
- Introduce a private borrowed view struct for GNU hash data if the functions currently traverse raw memory.
- Map anonymous C structures to private Rust structs with only the fields used by:
  - `hash_get_n_buckets`
  - `hash_get_n_buckets_used`
  - `hash_get_n_entries`
- Decide the public function signatures based on existing project conventions and the smallest necessary adaptation from C.

Completion criteria:

- The Rust module compiles with placeholder bodies.
- All fields required by the original C logic are represented without adding extra capability.

## Phase 2: Port Core Counting Logic

Goals:

- Implement the three functions with behavior matching the C logic.

Tasks:

- Port `hash_get_n_buckets` first as the direct metadata/count accessor.
- Port `hash_get_n_buckets_used` next, translating bucket traversal from pointer iteration to slice iteration.
- Port `hash_get_n_entries` last, preserving the original counting rules for entries/chains.
- Replace implicit C integer promotion with explicit Rust conversions.
- Add validation at slice boundaries and on derived offsets used during counting.

Key technical decisions:

- Return `usize` for final Rust-side counts where natural, converting from stored `u32` values explicitly.
- Keep count computation single-pass where the C code is single-pass.
- Do not introduce caching or precomputation not present in the original module.

Completion criteria:

- The three functions are fully implemented.
- No unchecked indexing remains in the normal query path.

## Phase 3: Validate Semantics with Unit Tests

Goals:

- Confirm that Rust behavior matches the migrated C logic for normal and boundary cases.

Tasks:

- Add focused unit tests in `src/gnu/hash.rs` or adjacent test modules.
- Cover:
  - empty or zero-count layouts if permitted by the C logic
  - non-zero bucket counts
  - mixtures of used and unused buckets
  - entry-count derivation edge cases implied by chain termination rules or metadata limits
  - malformed/inconsistent inputs if the Rust API validates them
- Use small handcrafted fixtures that mirror the binary/hash arrays needed by the functions.

Completion criteria:

- `cargo test` passes.
- Tests cover both direct metadata retrieval and computed counts.

## Phase 4: Integration Cleanup and API Alignment

Goals:

- Align the new module with the rest of the Rust port branch and remove migration scaffolding.

Tasks:

- Adjust visibility (`pub`, `pub(crate)`, private) to match actual usage.
- Ensure naming and signatures fit the existing Rust project module tree.
- Remove any temporary helpers not required by the final three-function scope.
- Verify that error handling is consistent with neighboring migrated modules, without widening scope.

Completion criteria:

- The module is integrated into the project tree cleanly.
- Only the migrated functionality from `gnu/hash.c` remains exposed.
- `cargo test` passes for the branch.