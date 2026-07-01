# Implementation Plan

## Summary

Port the POSIX-oriented output logic from `src/posix.c` into a single Rust module that preserves existing behavior and output formatting for the functions `print_symbol_type` and `posix_output_handler`. The Rust implementation should stay narrowly scoped to the current module responsibilities: symbol-type rendering and POSIX-style output emission.

The technical approach is to translate the current control flow and formatting logic directly into idiomatic Rust while keeping the module boundary small. C string handling, nullable values, and implicit output-side effects should be converted into explicit Rust types and function signatures. Memory ownership should be made explicit, with borrowed string data used where possible and owned `String` values introduced only where formatting requires allocation. Error handling should replace C-style implicit failure paths with `std::io::Result` or similarly narrow standard-library result types when output operations can fail.

## Technical Context

- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates are recommended based on the available module analysis
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Match the existing C module’s asymptotic behavior
  - Avoid unnecessary heap allocation in symbol-type printing and output emission paths
  - Prefer writing directly to output sinks instead of building large intermediate strings unless the original control flow requires staging
  - Keep per-call overhead minimal and predictable

## Module Mapping

- **C source file**
  - `src/posix.c`

- **Rust target module**
  - `src/posix.rs`

- **Function mapping**
  - `print_symbol_type` -> `print_symbol_type`
    - Implement as a focused helper in `src/posix.rs`
    - Convert C output operations into writes against a Rust output target
  - `posix_output_handler` -> `posix_output_handler`
    - Implement in `src/posix.rs`
    - Preserve the existing ordering and formatting behavior
    - Accept explicit Rust references instead of relying on implicit global/stateful C conventions where adaptation at the call boundary is sufficient

The Rust module should remain a direct port of the existing file rather than being split into additional submodules.

## Data Model

The analysis only identifies an **anonymous** data structure, so the Rust data model should be derived strictly from the fields actually consumed by `print_symbol_type` and `posix_output_handler`.

### Mapping approach

- **Anonymous C struct(s)** -> **Named Rust struct or borrowed view struct**
  - Introduce a minimal named Rust struct only if the C code accesses grouped fields through a common record
  - If the anonymous structure is only passed through and partially read, define the narrowest Rust representation necessary for these two functions
  - Use:
    - `&str` for borrowed textual data when valid UTF-8 is already guaranteed by surrounding project assumptions
    - `String` only where owned formatted output is required
    - `Option<T>` for nullable pointers
    - Rust enums for symbolic C integer categories if `print_symbol_type` switches on a fixed symbol-type domain
    - integer primitives (`i32`, `u32`, `usize`) chosen to reflect the original semantic range rather than preserving C type names mechanically

### Memory and ownership decisions

- Replace raw pointer field access with borrowed references where the caller can guarantee lifetime validity
- Convert null checks into `Option`
- Keep formatting helpers free of hidden allocation where possible
- If the original module writes to `FILE *` or similar output handles, represent the destination as a generic writer parameter such as `&mut impl std::io::Write` or a concrete project-local sink type already in use elsewhere in the port

### Error handling decisions

- Any function performing output should return `std::io::Result<()>` unless the wider port already standardizes on another project-local result type
- Pure symbol mapping logic in `print_symbol_type` may return:
  - a borrowed static string, or
  - write directly into the output sink and propagate I/O errors
- Invalid or unmapped symbol categories should be handled explicitly rather than relying on C fallthrough or undefined formatting

## Implementation Phases

### Phase 1: Establish the Rust module skeleton

- Create `src/posix.rs`
- Identify the exact C signatures, referenced types, constants, and output dependencies used by:
  - `print_symbol_type`
  - `posix_output_handler`
- Define the minimal Rust equivalents for any anonymous or implicit C data used by these functions
- Decide the final Rust function signatures based on actual call-site and output requirements
- Add the module to the crate using standard Rust project conventions without introducing extra abstraction layers

### Phase 2: Port symbol-type rendering logic

- Implement `print_symbol_type` first as the smaller, dependency-revealing unit
- Translate C branching or switch logic into:
  - `match` for fixed symbolic categories, or
  - narrow conditional logic if the C code uses flags or sentinel values
- Preserve exact textual output expected by the existing module behavior
- Replace any pointer/null-dependent logic with `Option`-based handling
- Add focused unit tests for each known symbol-type branch and any fallback path

### Phase 3: Port the POSIX output handler

- Implement `posix_output_handler` in `src/posix.rs`
- Translate the existing formatting sequence directly, keeping output ordering stable
- Pass output sinks explicitly and propagate write failures through `Result`
- Reuse `print_symbol_type` rather than duplicating symbol formatting logic
- Resolve any remaining anonymous-structure field access through the minimal Rust data model defined earlier
- Add tests covering representative output records and edge cases such as absent optional fields

### Phase 4: Validate behavior and complete integration

- Confirm that the Rust implementation compiles cleanly on branch `096-module_src_posix.c_33-rust-port`
- Run `cargo test`
- Compare the Rust output against the C behavior for the covered cases, especially:
  - symbol-type formatting
  - delimiter placement
  - newline and spacing rules
  - handling of missing or optional data
- Remove any temporary compatibility code introduced during translation so the final module remains minimal and directly maintainable