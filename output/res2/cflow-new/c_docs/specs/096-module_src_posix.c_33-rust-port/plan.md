# Implementation Plan

## Summary
This module ports the `src/posix.c` responsibilities related to symbol-type formatting and POSIX-style output handling into Rust on branch `096-module_src_posix.c_33-rust-port`.

The Rust implementation should stay narrowly aligned with the existing C behavior:
- migrate `print_symbol_type` into a Rust function that converts the source symbol/category information into the same textual output expected by the current module behavior;
- migrate `posix_output_handler` into a Rust output routine that writes formatted POSIX-style records in the same order and with the same conditional handling as the C code.

The technical approach is to:
- keep the implementation in a single Rust module corresponding closely to `src/posix.c`;
- replace C string and pointer-oriented logic with borrowed Rust string slices and explicit enums where the source category values are known;
- express output writing through standard-library formatting and `std::io::Write`-based sinks;
- preserve behavior while making ownership, nullability, and error paths explicit with `Option`/`Result`.

## Technical Context

### Language/Version
- Rust stable, edition 2021
- Minimum practical compiler target: Rust 1.76+

### Primary Dependencies
- Rust standard library only
  - `std::io` for output handling
  - `std::fmt` for formatting helpers
  - `std::borrow` / `std::string` as needed for string conversion

No third-party crates are recommended because the provided module scope does not show a need beyond standard formatting and I/O.

### Testing
- `cargo test`

Test coverage should focus on:
- symbol-type text mapping for all known categories handled by `print_symbol_type`;
- output equivalence of `posix_output_handler` for representative inputs;
- edge cases previously represented by C null checks, empty strings, or absent symbol metadata.

### Performance Goals
- Maintain behavior with performance comparable to the C implementation for line-oriented output generation.
- Avoid unnecessary heap allocation where borrowed string data or direct streaming writes are sufficient.
- Keep formatting passes minimal: prefer writing directly into the target sink instead of constructing intermediate strings unless required by control flow.

## Module Mapping

### C to Rust File Mapping
- `src/posix.c` → `src/posix.rs`

### Function Mapping
- `print_symbol_type` → `print_symbol_type(...)` in `src/posix.rs`
  - Implement as a focused helper that maps symbol/type information to the required textual representation.
  - Return either a borrowed `&'static str` or write directly into a sink, depending on how many call sites need the value.
- `posix_output_handler` → `posix_output_handler(...)` in `src/posix.rs`
  - Implement as the main formatting/output function for this module.
  - Accept explicit typed inputs instead of raw C pointers where possible within the Rust port.
  - Return `io::Result<()>` or a project-local `Result` if the surrounding Rust codebase already standardizes error types.

### Integration Boundary
This module should remain a direct port of the existing file responsibilities. It should not introduce extra formatting layers, registries, or generalized output frameworks beyond what is required to replace the current C implementation.

## Data Model

The analysis lists only an anonymous data structure. The Rust port should therefore derive its data model from actual field usage inside `print_symbol_type` and `posix_output_handler`, keeping the mapping minimal.

### Data-Structure Mapping
- C anonymous struct used by `src/posix.c` → Rust named struct in `src/posix.rs` or the nearest existing Rust module where the same data is already represented.

Recommended mapping strategy:
- If the anonymous C structure is only consumed locally by `posix_output_handler`, create a private Rust struct with fields matching only the accessed members.
- If the structure represents symbol classification values used by `print_symbol_type`, model the classification as a Rust `enum` when the set of values is closed and known from the C code.
- If some fields are optional in C via nullable pointers:
  - `char *` / `const char *` → `Option<&str>`, `Option<String>`, or `Cow<'a, str>` depending on ownership needs
  - object pointers used for required inputs → references `&T`
  - object pointers used for optional inputs → `Option<&T>`

### Primitive and Memory Mapping
- C string pointers → Rust `&str` / `String`
- C integer tag values for symbol kinds → Rust `enum` or integer constants only if exact value preservation is required by surrounding code
- C output target (`FILE *` or equivalent callback sink) → generic `W: std::io::Write` or a concrete writer used by the rest of the port

### Error Handling and Ownership
- Replace implicit C write failures with `Result` returns.
- Eliminate raw ownership transfer; borrowed inputs should remain borrowed for formatting.
- Any temporary converted strings should have scope limited to the output call path.
- Avoid `unsafe` unless forced by surrounding not-yet-ported interfaces; if unavoidable, isolate it to boundary conversion code and keep this module’s core logic safe.

## Implementation Phases

### Phase 1: Extract and Map C Behavior
- Inspect `src/posix.c` and identify:
  - all branches in `print_symbol_type`;
  - all fields read by `posix_output_handler`;
  - exact output token order, separators, terminators, and conditional omissions.
- Define the Rust file `src/posix.rs`.
- Introduce the minimal Rust representations needed for the anonymous C data used by these two functions.
- Decide whether `print_symbol_type` should return a textual token or write directly, based strictly on the call pattern in this module.

### Phase 2: Port Core Formatting Logic
- Implement `print_symbol_type` first, preserving:
  - exact mapping of source type/category to output text;
  - fallback behavior for unknown or absent values, if present in C.
- Implement `posix_output_handler` next using `std::io::Write` and explicit formatting.
- Preserve C control flow order so output remains behaviorally equivalent.
- Convert null/empty checks into `Option`/string checks without changing emitted output semantics.

### Phase 3: Integrate Error Paths and Validate Equivalence
- Replace C-style implicit success/failure with explicit `Result` propagation from output operations.
- Ensure no unnecessary allocation is introduced in the final formatting path.
- Add unit tests covering:
  - each symbol type branch;
  - representative complete output lines;
  - missing optional data cases;
  - write failure propagation using a failing test writer if needed.
- Verify the Rust module compiles cleanly and is invoked from the corresponding ported call path in place of the C module.

### Phase 4: Final Review and Narrow Cleanup
- Compare emitted output against the C implementation for fixture-like examples from the original code path.
- Remove any temporary compatibility helpers that are not required by the final migrated module.
- Confirm the final Rust module remains limited to the original file scope and does not introduce extra abstractions beyond the migrated functions.