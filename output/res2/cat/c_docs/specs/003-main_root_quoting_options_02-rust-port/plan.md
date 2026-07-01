# Implementation Plan

## Summary

Port the `quotearg.c` functionality required by `quotearg_n_custom_mem` into a focused Rust module that preserves existing behavior needed by the `cat` main cluster. The Rust implementation should migrate only the logic directly involved in custom-memory quoting, including byte-slice handling, option selection, and returned quoted data construction.

Technical approach:

- Implement a Rust module centered on a safe function operating on `&[u8]` input and producing owned output.
- Preserve the C routine’s semantics around custom quoting rules and explicit memory-length inputs rather than relying on NUL-terminated strings.
- Replace C global/static option patterns with explicit Rust data structures and function parameters where possible within the existing call shape.
- Use standard-library byte buffers (`Vec<u8>`, `String` only when text validity is guaranteed) to avoid manual allocation management.
- Keep the port narrowly scoped to the existing file/function set and avoid introducing unrelated quoting modes or infrastructure not required by this module slice.

## Technical Context

- **Language/Version:** Rust 1.75+
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - Maintain linear-time processing with respect to input length.
  - Avoid unnecessary intermediate allocations beyond the final quoted buffer.
  - Preserve explicit byte-oriented handling so non-UTF-8 inputs do not require transcoding.
  - Match C behavior closely enough that existing callers do not observe regressions in output content or memory ownership expectations.

## Module Mapping

| C File | C Function | Rust Target |
|---|---|---|
| `quotearg.c` | `quotearg_n_custom_mem` | `src/quotearg.rs` -> `pub(crate) fn quotearg_n_custom_mem(...)` |

Recommended Rust file layout:

| Rust File | Responsibility |
|---|---|
| `src/quotearg.rs` | Port of the minimal quoting logic and supporting option/data types needed by `quotearg_n_custom_mem` |

Notes:

- Keep the implementation in a single Rust module unless existing project structure already contains a quoting module to extend.
- Do not split helpers into additional modules unless required by existing crate organization.
- If the original C function depends on internal helper routines from `quotearg.c`, migrate only those helpers that are directly necessary and keep them private in `src/quotearg.rs`.

## Data Model

The analysis lists only anonymous C data structures, so the Rust plan should derive data types from actual usage inside `quotearg_n_custom_mem` and its immediate helpers rather than mirroring unnamed declarations mechanically.

| C Data Shape | Rust Mapping | Notes |
|---|---|---|
| Anonymous option/config struct(s) used by quoting logic | `struct QuotingOptions` | Collect only fields referenced by the migrated function path. |
| Anonymous custom quoting callback/config payload | `struct CustomQuoting` or fields inside `QuotingOptions` | Represent custom left/right quote markers or equivalent custom byte patterns as owned or borrowed byte slices. |
| Anonymous flag/group fields | `bitflags`-like behavior using plain integer/boolean fields | Prefer `u32`/`bool` fields with constants from std-only approach; avoid extra crates. |
| Anonymous temporary output buffer state | `Vec<u8>` | Replaces manual allocation and growth logic. |
| Anonymous input memory region (`char *` + length) | `&[u8]` | Preserves explicit-length semantics and supports non-UTF-8 data. |
| Anonymous returned C string storage | `Vec<u8>` or `String` depending on caller contract | Prefer `Vec<u8>` internally; convert only at boundary if current Rust crate API requires UTF-8. |
| Anonymous index/count types | `usize` | Use checked conversions when translating from C integer types. |
| Anonymous error/sentinel state | `Result<T, E>` only if current call chain supports it; otherwise deterministic infallible construction | Prefer infallible API if C behavior assumes allocation abort/panic semantics in the surrounding port. |

Rust type decisions:

- **Input data:** `&[u8]`
- **Custom quote delimiters:** `&[u8]` or small owned `Vec<u8>` if lifetime decoupling is needed
- **Output buffer:** `Vec<u8>`
- **Public surface:** keep `pub(crate)` visibility unless broader exposure already exists in the Rust crate
- **Error handling:** avoid introducing new recoverable error variants unless the surrounding Rust port already uses them; allocation failures remain standard Rust allocation failures

## Implementation Phases

### Phase 1: Extract and Define the Minimal Rust Surface

- Create or extend `src/quotearg.rs`.
- Identify the exact data dependencies of `quotearg_n_custom_mem` within `quotearg.c`.
- Define the minimal Rust function signature matching the existing crate’s calling style.
- Introduce the smallest set of supporting structs/constants required for custom-memory quoting.
- Map C pointer-and-length inputs to byte-slice parameters.
- Decide return type based on surrounding Rust code:
  - use `Vec<u8>` if byte-preserving behavior is required end-to-end;
  - use `String` only if all existing Rust callers require valid UTF-8 and the quoting path guarantees it.

Exit criteria:

- Rust module compiles with stubbed or partial logic.
- All required supporting types for this function path are declared.

### Phase 2: Port Core Quoting Logic

- Translate the body of `quotearg_n_custom_mem` and any directly required private helpers from `quotearg.c`.
- Replace manual buffer sizing/reallocation with `Vec<u8>` growth.
- Preserve explicit handling of input length and embedded non-text bytes.
- Implement custom delimiter insertion and escaping behavior exactly as required by this function path.
- Convert C branching on flags/options into Rust `match`/`if` over the reduced option struct.
- Remove any dependency on unnamed shared mutable static storage by passing state explicitly or containing it in local/module-private immutable defaults if present in the original path.

Memory-management considerations:

- No raw allocation APIs.
- No returned borrowed data tied to temporary buffers.
- Avoid `unsafe` unless a direct byte-level optimization is truly necessary; safe slice iteration should be the default.

Exit criteria:

- `quotearg_n_custom_mem` is fully implemented in Rust.
- Private helper logic used by the function path is migrated and compiling.

### Phase 3: Behavioral Validation

- Add unit tests in the same module or crate test layout covering:
  - empty input
  - simple ASCII input
  - embedded special characters requiring quoting/escaping
  - non-UTF-8 byte input
  - custom left/right quote delimiter behavior
  - explicit-length behavior independent of trailing NUL bytes
- Add comparison-style tests against known expected outputs derived from the C behavior for this specific function path.
- Verify no accidental UTF-8 assumptions remain in internal processing.

Exit criteria:

- `cargo test` passes.
- Tests cover the main edge cases for byte-oriented custom quoting.

### Phase 4: Integration Cleanup

- Wire the new Rust function into the existing `cat` main-cluster call path on branch `003-main_root_quoting_options_02-rust-port`.
- Remove or disable the migrated C entry point usage for this function path if the branch structure expects Rust replacement.
- Confirm naming and visibility match the rest of the Rust port.
- Perform a final pass to eliminate unused translated constants/fields not actually needed by `quotearg_n_custom_mem`.

Exit criteria:

- The Rust implementation is the active path for this module slice.
- Build and tests succeed without dead migration scaffolding.

## Notes and Constraints

- Keep scope limited to `quotearg_n_custom_mem` and the directly supporting logic it needs from `quotearg.c`.
- Do not generalize the quoting subsystem beyond what this function already uses.
- Prefer byte-oriented APIs to preserve original semantics.
- Anonymous C structures should be reconstructed only from accessed fields; do not create broad catch-all Rust models for the whole source file.
- Maintain close behavioral parity with the C implementation while using safe ownership and allocation patterns from the Rust standard library.