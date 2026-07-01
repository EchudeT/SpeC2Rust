# Implementation Plan

## Summary

Port the `quotearg.c` functionality for `quotearg_custom` and `quotearg_custom_mem` into Rust on branch `013-main_root_quotearg_custom_12-rust-port`, keeping behavior aligned with the existing C implementation and limiting scope to the functions in this module slice.

The Rust implementation should:
- migrate the existing quoting entry points only;
- preserve the current custom-quoting semantics, especially handling of caller-provided delimiters or replacement strings used by the custom quoting path;
- use owned and borrowed byte/string types from the standard library to replace raw pointer and length handling where possible;
- keep allocation behavior explicit and localized, avoiding global mutable state unless the surrounding port already requires a direct equivalent.

The technical approach is to translate the two functions into a Rust module that:
- accepts byte slices for `_mem` variants;
- converts C-style pointer/length processing into safe slice-based logic;
- returns owned quoted output or writes into the project’s existing quoting result abstraction if one already exists in the Rust port;
- preserves error-visible behavior around invalid inputs through narrow validation and deterministic panic-free handling.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - match the C path’s asymptotic behavior for input traversal and output construction;
  - avoid unnecessary intermediate allocations beyond the final quoted buffer;
  - preserve efficient handling of non-NUL-terminated input in `quotearg_custom_mem`;
  - keep per-call overhead low by using slices and `String`/`Vec<u8>` capacity reservation where output growth is predictable.

## Module Mapping

| C File | C Functions | Rust Module | Rust Items |
|---|---|---|---|
| `quotearg.c` | `quotearg_custom` | `src/quotearg.rs` or existing `quotearg` module file | `pub fn quotearg_custom(...)` |
| `quotearg.c` | `quotearg_custom_mem` | `src/quotearg.rs` or existing `quotearg` module file | `pub fn quotearg_custom_mem(...)` |

### Mapping Notes
- Place the Rust code in the existing quoting module used by the port if `quotearg.c` has already been partially migrated; do not introduce a new subsystem.
- If the current Rust port already exposes shared quoting helpers, these two functions should be thin adapters onto that shared implementation rather than duplicating escape/quoting logic.
- `quotearg_custom` should remain the string-oriented wrapper over the length-aware `quotearg_custom_mem` path, mirroring the C layering.

## Data Model

The analysis only identifies anonymous C data structures, which is typical for internal option/state objects in `quotearg.c`. For this module plan, only structures directly touched by `quotearg_custom` and `quotearg_custom_mem` should be mapped.

| C Data Shape | Rust Mapping | Notes |
|---|---|---|
| Anonymous quoting/options struct used internally by custom quoting | Reuse existing Rust `struct` for quoting options, or introduce a minimal private `QuotingOptions` struct in `src/quotearg.rs` | Do not generalize beyond fields needed by the two target functions. |
| Anonymous pair of custom quote delimiters / left-right strings | `struct CustomQuoting<'a> { left: &'a [u8], right: &'a [u8] }` or equivalent borrowed fields inside quoting options | Use borrowed byte slices to reflect non-owning C pointers. |
| Anonymous flags / enum-like mode selectors | `enum` or integer-backed private fields matching existing Rust port conventions | Prefer `enum` when variants are already clear in the current port; otherwise keep a narrow integer/flag translation. |
| Raw C string pointer input | `&str` for validated UTF-8 wrappers, or `&[u8]` for exact byte-preserving behavior | `quotearg_custom` may accept `&str` at API edge only if the surrounding port is already string-based; otherwise use `&CStr`/`&[u8]` internally. |
| Raw pointer + explicit length input | `&[u8]` | Primary representation for `quotearg_custom_mem`. |
| Heap-allocated returned quoted buffer | `String` or `Vec<u8>` depending on existing module conventions | Use `Vec<u8>` if quoting must preserve arbitrary bytes; convert to `String` only when guaranteed valid UTF-8. |

### Memory Management and Error Handling
- Replace raw pointer arithmetic with slice indexing and iterator traversal.
- Treat custom left/right quote arguments as borrowed inputs with lifetime-bound storage; do not clone unless needed for the final result.
- Avoid panics from indexing or UTF-8 conversion; byte-oriented processing is preferred for exact C behavior.
- If the C implementation assumes non-null custom quote pointers, represent this in Rust with required references/slices instead of optional pointers.
- If null/invalid inputs are observable at call boundaries due to broader port compatibility, validate at the boundary and map to the project’s established error strategy rather than introducing new error types unnecessarily.

## Implementation Phases

### Phase 1: Establish Module Placement and Type Mapping
- Identify the existing Rust destination for `quotearg.c` translations.
- Locate any already-ported quoting option/state types and reuse them instead of creating parallel abstractions.
- Define the minimal Rust representation for the custom quoting parameters required by `quotearg_custom` and `quotearg_custom_mem`.
- Decide the exact public signatures based on surrounding migrated code:
  - wrapper form for `quotearg_custom`;
  - byte-slice/length-aware form for `quotearg_custom_mem`.
- Document invariants previously implicit in C pointers, especially required custom delimiter inputs.

### Phase 2: Port `quotearg_custom_mem` Core Logic
- Translate the length-aware custom quoting path first, since it is the behavioral core.
- Convert all pointer/length reads into safe slice operations.
- Reuse existing quote/escape helpers from the Rust port if already present; otherwise implement only the minimal internal helper logic needed by this function.
- Ensure output construction preserves byte content and delimiter placement exactly as expected by the C path.
- Keep allocation localized to the returned/result buffer and reserve capacity when practical.

### Phase 3: Port `quotearg_custom` as Wrapper
- Implement `quotearg_custom` as the wrapper over the `_mem` variant, mirroring the C relationship.
- Replace C NUL-terminated string length discovery with Rust boundary-safe input handling according to the surrounding API style.
- Ensure wrapper behavior does not duplicate core quoting logic.
- Verify that custom delimiter handling is identical between the wrapper and `_mem` entry point.

### Phase 4: Validation and Test Migration
- Add focused unit tests in the same Rust module or its corresponding test module.
- Cover:
  - empty input;
  - inputs requiring no escaping;
  - inputs requiring custom quote insertion/escaping;
  - embedded NUL or non-text bytes for `_mem`;
  - distinct left/right custom delimiters;
  - wrapper consistency between `quotearg_custom` and `quotearg_custom_mem`.
- Run `cargo test` and adjust for behavioral parity with the C implementation and the broader `cat` Rust port conventions.