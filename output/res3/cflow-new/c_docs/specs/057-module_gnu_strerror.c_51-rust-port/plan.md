# Implementation Plan

## Summary
Port `gnu/strerror.c` into a focused Rust module that preserves the existing module boundary and behavior of the exported `strerror` functionality without adding broader error-framework features. The Rust implementation should center on translating an integer error code into a stable, human-readable message using Rust standard-library facilities where possible, with a small compatibility layer for GNU-style behavior where the C code would return textual fallbacks for unknown codes.

The implementation should remain narrow:
- migrate only the logic needed for `strerror`
- keep ownership and lifetime rules explicit in Rust
- avoid introducing new public APIs beyond what is needed to represent the migrated function in idiomatic Rust and, if required by the surrounding port, a compatibility entry point

## Technical Context
- **Language/Version**: Rust 1.76+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time or near-constant-time lookup for common known error codes
  - No unnecessary heap allocation for known static messages
  - Minimal allocation only when formatting fallback text for unknown error codes
  - Preserve lightweight call overhead comparable to the C implementation’s intent

## Module Mapping
- **C source**: `gnu/strerror.c`
- **Rust target**: `src/module_gnu_strerror_c_51.rs`

### Function mapping
- **C**: `strerror(int errnum)`
- **Rust**:
  - primary internal function: `fn strerror(errnum: i32) -> std::borrow::Cow<'static, str>`
  - if a stricter compatibility surface is required by the surrounding port, keep it as a thin wrapper over the internal Rust function rather than duplicating logic

### Mapping notes
- The Rust module should contain only the migrated logic from `gnu/strerror.c`.
- Do not split the implementation into additional helper modules unless a private helper function is needed inside the same file for fallback formatting or table lookup.
- Prefer returning borrowed static strings for known messages and owned strings only for unknown-code formatting.

## Data Model
This module has no declared C structs to migrate.

### Data mapping
- **C integer error code**: `int`
  - **Rust**: `i32`
- **C string result**: `char *` / static message buffer semantics
  - **Rust**: `Cow<'static, str>` for internal representation
  - Rationale:
    - known messages can be borrowed as `&'static str`
    - unknown messages can be formatted into an owned `String`
    - avoids unsafe mutable static buffers and clarifies ownership

### Memory and lifetime handling
- Replace any C static-storage string handling with Rust static string literals for known cases.
- Avoid global mutable buffers.
- For unknown error codes, allocate only the formatted fallback string required for the return value.
- Keep all returned data ownership explicit and free of aliasing hazards.

### Error-handling model
- The migrated `strerror` logic should not raise Rust errors for unknown input values.
- Unknown error numbers should map to a deterministic fallback message string, matching the C module’s compatibility intent rather than using `Result`.

## Implementation Phases

### Phase 1: Create module skeleton and function surface
- Add `src/module_gnu_strerror_c_51.rs`.
- Define the Rust `strerror(errnum: i32) -> Cow<'static, str>` function.
- Establish the exact module export path used by the project so callers can migrate with minimal disruption.
- Add a minimal private helper only if needed for fallback message formatting.

### Phase 2: Port lookup behavior from C logic
- Translate the C error-message selection logic into Rust.
- Use standard-library error text sources where they directly satisfy the required behavior; otherwise encode the necessary message mapping locally in the module.
- Ensure known messages are returned as borrowed static strings.
- Implement GNU-style unknown-code fallback formatting as owned text.
- Remove any C-style mutable buffer assumptions during translation.

### Phase 3: Validate behavior with unit tests
- Add tests covering:
  - a representative known error code returning a nonempty stable message
  - zero or success-adjacent input behavior if handled in the original logic
  - negative and out-of-range error codes producing fallback text
  - repeat calls to confirm no shared mutable buffer behavior is required
- Run `cargo test` and fix any message-selection mismatches exposed by the port.

### Phase 4: Final integration cleanup
- Wire the module into the project’s existing Rust module tree.
- Confirm the old C file’s responsibility is fully covered by the Rust file and no duplicate implementation remains in the Rust port path.
- Review for unnecessary allocation, unsafe code, or extra abstractions and reduce to the smallest faithful implementation.