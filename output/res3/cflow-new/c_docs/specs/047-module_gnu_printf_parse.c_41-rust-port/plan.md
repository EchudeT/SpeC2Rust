# Implementation Plan: module_gnu_printf-parse.c_41

## Summary

Port `gnu/printf-parse.c` into an idiomatic Rust module that preserves the current parsing behavior of `PRINTF_PARSE` without adding new formatting features or broader infrastructure. The Rust implementation should keep the parsing logic close to the C control flow so behavior remains comparable, while replacing pointer-based state updates with slice/index traversal and explicit result types.

The implementation approach is to:
- migrate the parsing routine into a single Rust module with a narrow public surface,
- represent parser state and intermediate results with Rust structs/enums only where the C code requires structured state,
- use owned or borrowed string data as appropriate to avoid manual memory management,
- translate C error paths into `Result`-based returns or explicit parse-status values consistent with current caller expectations.

## Technical Context

- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain linear parsing behavior over the format string input.
  - Avoid unnecessary string allocations during scanning.
  - Keep data movement minimal by parsing via byte slices and indices where valid.
  - Stay close to the original routine’s operational cost profile rather than introducing abstraction-heavy parsing layers.

## Module Mapping

- **C source file**: `gnu/printf-parse.c`
- **Rust target module**: `src/gnu/printf_parse.rs`

Suggested crate module exposure:
- `src/gnu/mod.rs`
  - `pub mod printf_parse;`

Function migration:
- **C**: `PRINTF_PARSE`
- **Rust**: `pub(crate) fn printf_parse(...) -> Result<..., ...>` or equivalent internal function signature matching existing project needs

Mapping guidance:
- Keep the Rust module focused on the single migrated parser routine from `gnu/printf-parse.c`.
- Do not split the parser into extra helper modules unless required by borrow-checking or basic readability during direct migration.
- Preserve the original parsing order and branch structure as much as practical to reduce behavioral drift.

## Data Model

No explicit C data structures were provided in the analysis input. The Rust data model should therefore be introduced only as needed to support the migrated `PRINTF_PARSE` logic.

Expected mapping strategy:

- **C pointer-based parser state**
  - **Rust**: local index over `&[u8]` or `&str`
  - Rationale: replaces pointer arithmetic with bounds-checked traversal.

- **C output parameters for parsed components**
  - **Rust**: a dedicated result struct if the original function populates multiple fields
  - Example shape:
    - `struct PrintfParseResult { ... }`
  - Use only if the original function naturally returns multiple parsed fields.

- **C integer status/error returns**
  - **Rust**: `Result<T, ParseError>` when parse failure is semantically exceptional
  - Alternatively, a small internal enum if the original behavior distinguishes multiple non-fatal parser outcomes.

- **C flags / conversion categories**
  - **Rust**: primitive integer/bitflag fields or small enums
  - Prefer plain `u32`/`usize`/`i32` fields if this best preserves direct behavior and avoids unnecessary redesign.

- **C dynamically managed character buffers**
  - **Rust**: `String` or `Vec<u8>` only where the original routine truly materializes substrings
  - Prefer borrowed slices (`&str`, `&[u8]`) during scanning to minimize allocation.

- **C nullability**
  - **Rust**: `Option<T>`

- **C struct fields that may be partially initialized across branches**
  - **Rust**: initialize through a single constructor/default path, then mutate explicitly
  - This avoids unsafe uninitialized-state patterns.

## Implementation Phases

### Phase 1: Establish module skeleton and direct parser translation

- Create `src/gnu/printf_parse.rs`.
- Add the corresponding `mod` declaration from `src/gnu/mod.rs`.
- Translate `PRINTF_PARSE` into Rust with a direct control-flow-preserving implementation.
- Replace C pointer arithmetic with:
  - input as `&str` or `&[u8]`,
  - a mutable cursor index,
  - explicit bounds checks before reads.
- Keep helper logic local to the module and avoid introducing extra subsystem abstractions.
- Define the minimum required Rust result types to represent parser output and failure.

**Exit criteria**:
- The parser compiles.
- The migrated function covers the same parse branches as the C routine at a structural level.

### Phase 2: Data-model tightening and memory/error handling alignment

- Review all translated parser outputs and consolidate them into the smallest necessary Rust struct/enum set.
- Replace any C-style sentinel values with `Option` or `Result` where that does not alter externally required behavior.
- Ensure substring handling does not rely on unchecked indexing into UTF-8 `&str`; use byte-oriented parsing if the C routine is byte-based.
- Remove any temporary allocation introduced during initial translation unless strictly necessary.
- Verify that all formerly implicit C memory assumptions are expressed safely:
  - no dangling references,
  - no out-of-bounds cursor movement,
  - no partially initialized output state escaping the function.

**Exit criteria**:
- Parser state and outputs are represented safely and minimally.
- Error paths are explicit and compile-time checked.

### Phase 3: Behavioral test coverage against parsing cases

- Add unit tests for the migrated parser in the same module or standard Rust test layout.
- Cover representative cases implied by `PRINTF_PARSE`, including:
  - ordinary text with no conversion,
  - valid conversion introducers,
  - flag/width/precision/length scanning as applicable,
  - malformed or truncated format input,
  - boundary cases at start/end of input.
- Write tests to assert returned parse state and cursor advancement, not just success/failure.
- Where C behavior is permissive or unusual, preserve that behavior in tests rather than normalizing it.

**Exit criteria**:
- `cargo test` passes.
- Core parser branches and edge conditions are exercised.

### Phase 4: Final parity review and integration cleanup

- Compare the Rust routine branch-by-branch with `gnu/printf-parse.c` to confirm no accidental behavior expansion.
- Rename local variables and internal helpers to stay recognizable relative to the C source where that aids maintenance.
- Remove translation scaffolding, dead code, and redundant conversions.
- Confirm the module remains narrowly scoped to the original file/function migration.

**Exit criteria**:
- Final Rust module is limited to the migrated parser logic.
- The implementation is safe, test-covered, and aligned with the source module’s behavior.