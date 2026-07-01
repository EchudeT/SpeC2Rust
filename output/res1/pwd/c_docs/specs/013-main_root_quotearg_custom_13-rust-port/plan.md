# Implementation Plan

## Summary
This module ports the `quotearg.c` entry points `quotearg_custom` and `quotearg_custom_mem` into Rust, preserving their existing behavior and call patterns within the main cluster of the `pwd` project.

The Rust implementation should stay narrowly scoped to the existing functionality:
- migrate the quoting logic needed by these two functions,
- preserve byte-oriented behavior where the C code operates on raw buffers,
- model custom quoting rules explicitly in Rust types,
- avoid introducing broader quoting APIs beyond what is required for these functions.

The technical approach is to translate the relevant quoting state and option handling from C into a small Rust module with:
- borrowed byte-slice inputs for `_mem` behavior,
- a thin string-oriented wrapper for the non-`_mem` function,
- explicit result construction using owned `String` or `Vec<u8>` as appropriate for the calling pattern already present in the Rust port.

Memory safety will come from replacing manual buffer management with Rust-owned allocations and slice-based processing. Error handling should remain minimal and behavior-preserving: where the C code assumes successful quoting of in-memory data, the Rust code should avoid unnecessary fallible interfaces unless UTF-8 conversion or API boundary constraints require one.

## Technical Context
- **Language/Version**: Rust 1.74+
  A stable toolchain in this range is sufficient for standard-library-based slice, string, and iterator handling.

- **Primary Dependencies**:
  - Rust standard library only (`std`)
  No third-party crates are recommended because the input provides no evidence that external parsing, escaping, or compatibility crates are necessary.

- **Testing**:
  - `cargo test`

- **Performance Goals**:
  - Match the C implementation’s practical performance for typical short-to-medium argument quoting.
  - Preserve linear-time processing over the input buffer.
  - Avoid repeated reallocations by reserving output capacity conservatively when input length is known.
  - Do not introduce extra copying beyond what is required to produce the final quoted output.

## Module Mapping
### C to Rust File Mapping
- `quotearg.c` → `src/quotearg.rs`

### Function Mapping
- `quotearg_custom` → `pub fn quotearg_custom(...) -> String` or project-local equivalent string return type
- `quotearg_custom_mem` → `pub fn quotearg_custom_mem(input: &[u8], ...) -> String` or byte-preserving project-local equivalent

### Integration Scope
- Keep both migrated functions in the same Rust module unless the existing Rust branch already has a `quotearg` module with partial functionality.
- If a partial Rust `quotearg` implementation already exists, add only the missing custom-quoting functions and the minimum supporting types they require.
- Do not split helper logic into extra modules unless required by the existing crate layout.

## Data Model
The source analysis reports only anonymous C data structures. For planning purposes, these should be mapped by role rather than by C identifier, and only when needed by `quotearg_custom` and `quotearg_custom_mem`.

### Data Structure Mapping
- Anonymous quoting-options struct(s) in `quotearg.c`
  - → `struct QuotingOptions`
  - Holds the minimum option state required to represent custom left/right quote delimiters and any directly referenced flags used by these two functions.

- Anonymous style or mode constants
  - → `enum QuotingStyle`
  - Include only variants required to express the custom quoting path already used by these functions.

- Anonymous character-class or bitset tables
  - → one of:
    - fixed-size Rust arrays, such as `[u32; N]` / `[bool; N]`, or
    - compact helper functions over `u8`
  - Choose the smallest representation that preserves the C logic actually exercised by the custom quoting path.

- Anonymous buffer/output state structs
  - → no direct struct where unnecessary
  - Prefer local variables with `String` or `Vec<u8>` unless persistent option state is required across calls.

### Ownership and Representation Decisions
- C `char *` input with NUL termination
  - → `&str` for wrapper-style entry points when the Rust caller already has valid UTF-8
  - → delegate internally to byte-oriented logic if behavior must remain based on raw bytes

- C `(char *, size_t)` memory input
  - → `&[u8]`

- C output buffer management
  - → `String` if resulting output is expected to be textual in the current Rust port
  - → `Vec<u8>` internally if byte-accurate assembly is easier, with final conversion only at the API boundary if required

- C function pointers or callback-like custom quote delimiters
  - → plain fields in `QuotingOptions` such as borrowed or owned delimiter values
  - Avoid dynamic dispatch unless it already exists in the surrounding Rust code

### Error Handling and Safety
- Replace all implicit C buffer lifetime assumptions with owned Rust outputs.
- Avoid unchecked indexing where delimiter or input scanning occurs.
- If the implementation must convert bytes to `String`, use a behavior-preserving policy already established in the Rust port; otherwise keep processing byte-oriented until the final step.
- Do not introduce `Result` return types unless required by the existing crate API or by unavoidable UTF-8 constraints.

## Implementation Phases

### Phase 1: Establish Rust Module Skeleton and Core Type Mapping
- Create or update `src/quotearg.rs`.
- Identify the exact C-local state used by `quotearg_custom` and `quotearg_custom_mem`.
- Define the minimal Rust equivalents for:
  - custom quoting options,
  - quoting style/mode representation,
  - any small helper constants or tables needed by these functions.
- Confirm the intended Rust return types from surrounding project usage before coding the function bodies.
- Keep type definitions local to this module unless they are already shared elsewhere in the branch.

### Phase 2: Port `quotearg_custom_mem` as the Core Byte-Oriented Implementation
- Implement `quotearg_custom_mem` first, since it exposes the underlying buffer-length semantics directly.
- Translate the C quoting loop into slice-based Rust logic.
- Preserve delimiter insertion and escaping decisions exactly as in the C path used for custom quoting.
- Use owned output construction with preallocation based on input size and delimiter overhead.
- Add focused unit tests covering:
  - empty input,
  - input with ordinary ASCII bytes,
  - input containing bytes that require escaping or quoting treatment,
  - custom left/right delimiter combinations,
  - embedded NUL bytes if the C function supports arbitrary memory regions.

### Phase 3: Port `quotearg_custom` as the Thin Wrapper
- Implement `quotearg_custom` on top of `quotearg_custom_mem`.
- Map C string-based input semantics to Rust input handling with minimal transformation.
- Ensure wrapper behavior matches the C function for delimiter handling and output formation.
- Add unit tests that verify parity between:
  - the wrapper function and
  - the underlying `_mem` implementation for equivalent inputs.

### Phase 4: Final Alignment, Cleanup, and Regression Coverage
- Compare the Rust output against the C behavior for representative cases derived from the original module.
- Remove any temporary translation artifacts that are not needed after the port stabilizes.
- Confirm no unnecessary heap copies or redundant UTF-8 conversions remain in hot paths.
- Run `cargo test` and resolve any integration issues with existing callers in the main cluster.
- Keep the final implementation limited to the migrated functions and their direct supporting types only.