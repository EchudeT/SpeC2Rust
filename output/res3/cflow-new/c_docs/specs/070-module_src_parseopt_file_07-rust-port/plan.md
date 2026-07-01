# Implementation Plan

## Summary

Port the option-file parsing portion currently embedded in `src/main.c` into Rust, covering the behavior associated with `optfile_lookup` and `parseopt_from_rc`. The Rust implementation should preserve existing control flow and lookup semantics while moving from C-style pointer/string handling to owned and borrowed Rust string types and explicit result-based error propagation.

The technical approach is to migrate the logic into a small Rust module focused only on:
- locating the relevant option file input source,
- reading and parsing option entries from that source,
- returning lookup/parsing outcomes through typed Rust APIs.

The implementation should stay close to the current C organization rather than introducing broader abstractions. Memory ownership should be expressed with `String`, `PathBuf`, slices, and iterators from the standard library, and failure cases should be converted from implicit C conventions into `Result`/`Option` return values.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behaviorally similar throughput for option-file lookup and parsing relative to the C implementation.
  - Avoid unnecessary allocation beyond line buffering and string ownership required for safe parsing.
  - Keep file reading single-pass where practical.
  - Preserve predictable startup-time behavior since this logic is part of option initialization.

## Module Mapping

### C to Rust File Mapping

- `src/main.c`
  - migrate `optfile_lookup`
  - migrate `parseopt_from_rc`

### Rust Module Placement

Use restrained Rust project structure with the migrated logic placed in a dedicated module under the main binary crate:

- `src/main.rs`
  - retains program entry and existing top-level orchestration
  - calls into migrated parse-option module

- `src/parseopt_file.rs`
  - Rust port of `optfile_lookup`
  - Rust port of `parseopt_from_rc`
  - any minimal helper functions required only to preserve existing file-lookup and parsing flow

This keeps the extracted functionality scoped to the original responsibility without creating unrelated subsystems.

## Data Model

The C analysis only exposes repeated anonymous data structures, so the Rust data model should be introduced conservatively and only where the migrated functions require explicit structure.

### C Anonymous Structures -> Rust Types

Because the source structures are unnamed in the analysis, map them by role as encountered during migration:

- **anonymous config/option record used during file parsing**
  - Rust: `struct RcOptionEntry`
  - Purpose: hold the parsed option key and its associated raw or normalized value
  - Suggested fields:
    - `key: String`
    - `value: Option<String>`

- **anonymous lookup/input context used by `optfile_lookup`**
  - Rust: `struct OptFileCandidate`
  - Purpose: represent one candidate option-file path during lookup
    - `path: std::path::PathBuf`
    - `is_readable: bool`

- **anonymous parse outcome / state holder**
  - Rust: `struct ParseOptState`
  - Purpose: track current parsing progress only if the C flow requires mutable state across lines
    - minimal migrated fields only, derived from actual C locals/state

- **anonymous status or category flags**
  - Rust: `enum` or `bool` fields as appropriate
  - Purpose: replace integer sentinel values and flag fields with typed states when directly inferable from the C code

### String and Buffer Mapping

- C `char *` / string buffers -> `String`, `&str`, or `PathBuf`
- C mutable line buffers -> `String`
- C pointer-return conventions for optional values -> `Option<T>`
- C integer error/success returns -> `Result<T, ParseOptError>` or `Option<T>` where absence is not an error

### Error Model

Introduce a minimal module-local error type:

```rust
enum ParseOptError {
    Io(std::io::Error),
    InvalidFormat,
}
```

Refine only as required by actual C branches. Do not expand error taxonomy beyond what is needed to preserve control flow and testability.

### Memory Management Notes

- Replace manual C buffer lifetime management with owned `String` and `PathBuf`.
- Avoid storing references into temporary line buffers unless lifetime is clearly bounded; prefer copying into owned fields where needed.
- Keep parsing line-oriented to avoid whole-file retention unless the original logic requires accumulation.

## Implementation Phases

### Phase 1: Extract and Map Existing Behavior

- Inspect `src/main.c` and isolate the full call graph and local dependencies of:
  - `optfile_lookup`
  - `parseopt_from_rc`
- Identify:
  - path resolution inputs,
  - expected file search order,
  - line parsing rules,
  - success/failure return conventions,
  - any anonymous structures or flags actually touched by these functions.
- Create `src/parseopt_file.rs` with function signatures that mirror the C responsibilities closely.
- Define only the minimal Rust structs/enums needed to represent the migrated state and return values.

### Phase 2: Port Option-File Lookup Logic

- Implement the Rust equivalent of `optfile_lookup`.
- Map C path and file-access checks to:
  - `std::path::Path`
  - `std::path::PathBuf`
  - `std::fs::metadata` or `std::fs::File::open`
- Preserve original lookup order and stopping conditions.
- Replace C null-pointer or integer-status outputs with explicit `Option<PathBuf>` or `Result<PathBuf, ParseOptError>` based on the original semantics.
- Add focused unit tests for:
  - found candidate path,
  - missing file,
  - unreadable/invalid candidate handling if represented in C behavior.

### Phase 3: Port RC Parsing Logic

- Implement the Rust equivalent of `parseopt_from_rc`.
- Read the file using standard-library buffered I/O (`BufRead`).
- Port line parsing rules directly:
  - whitespace trimming,
  - comment skipping,
  - key/value extraction,
  - empty-line behavior,
  - malformed-line handling,
  - option application or entry collection as the C flow dictates.
- Keep helper functions private and minimal; do not generalize beyond the original parsing routine.
- Convert C mutation/error paths into idiomatic Rust returns while preserving externally visible behavior.
- Add unit tests for representative parsing cases derived from the C logic.

### Phase 4: Integrate and Validate Against Main Flow

- Wire `src/main.rs` to call the migrated Rust module in place of the original C logic’s role.
- Ensure the returned values and side effects match the expectations of surrounding startup/option-processing code.
- Add integration-oriented tests where feasible within the Rust crate to validate:
  - lookup followed by parsing,
  - no-file path,
  - malformed config handling according to current behavior.
- Remove or avoid duplicate temporary logic so the Rust module is the single implementation path for this functionality.