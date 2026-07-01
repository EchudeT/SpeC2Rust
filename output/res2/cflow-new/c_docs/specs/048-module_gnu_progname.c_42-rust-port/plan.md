# Implementation Plan: module_gnu_progname.c_42

## Summary

Port `gnu/progname.c` into a focused Rust module that provides the `set_program_name` behavior used by the original C code. The Rust implementation should preserve the existing responsibility of deriving and storing the process program name from an invocation path, while replacing C global string handling with safe Rust ownership and borrowing patterns.

The implementation should stay narrow: migrate the logic in `set_program_name`, map the single C source file into one Rust source module, and use standard-library path and string handling to avoid manual memory management. Any state exposed by this port should be represented with minimal Rust globals appropriate to the original module’s role, without adding new facilities beyond what is required for the migrated function.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time or near-constant-time global state update apart from input normalization
  - Single-pass extraction of basename/program name from the provided path
  - No unnecessary heap duplication beyond storing the final program-name value
  - Behavior suitable for startup-time invocation, with no special optimization work beyond parity with the C logic

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `gnu/progname.c` | `src/module_gnu_progname.rs` | Direct port of `set_program_name` logic into one Rust module |

| C Function | Rust Function | Notes |
|---|---|---|
| `set_program_name` | `pub fn set_program_name(argv0: &str)` | Accept string input from caller; derive program name using Rust string/path operations |

If the surrounding port already uses a crate module layout tied to source names, expose this file from `src/lib.rs` or the existing module registry without introducing extra abstraction layers.

## Data Model

This module does not define custom C structs in the provided input, so the Rust port should avoid inventing new data types unless required for state storage.

| C Representation | Rust Representation | Notes |
|---|---|---|
| Global program-name pointers/character storage | `static` global storing owned string data | Use a safe global primitive from the standard library such as `OnceLock` or `Mutex<String>` only if mutation semantics require it |
| `char *` / C string input | `&str` | Caller-facing API should accept valid UTF-8 Rust strings |
| Derived basename slice | `&str` during parsing, promoted to `String` for stored state | Borrow while computing; allocate only for retained global value |

### Memory Management

- Replace raw pointer assignment with owned `String` storage.
- Avoid leaked allocations and aliasing concerns present in C global pointer patterns.
- Keep borrowed intermediate slices scoped to parsing only.
- If global state must be overwritten to match C behavior, use a standard-library synchronization/container type that permits safe replacement without unsafe raw globals.

### Error Handling

- Prefer a total function if the original C logic assumes valid process-name input.
- Handle empty input conservatively:
  - either store an empty program name if this matches surrounding expectations, or
  - define a minimal fallback behavior documented in tests.
- Do not add rich error enums unless the existing migration context requires them.

## Implementation Phases

### Phase 1: Module Skeleton and State Mapping

- Create `src/module_gnu_progname.rs`.
- Add the Rust equivalent of module-level program-name state required by `set_program_name`.
- Wire the module into the crate through the existing root module file.
- Decide the narrowest safe standard-library global mechanism needed for this state based on whether the value is set once or may be updated.

### Phase 2: Port `set_program_name`

- Translate the C function’s path/program-name extraction logic into Rust.
- Use standard-library string operations, and only use `std::path::Path` if it matches the original behavior closely enough for basename extraction.
- Preserve edge-case handling from the C implementation, especially:
  - invocation paths containing directory separators
  - plain executable names without separators
  - empty or unusual inputs if present in the original logic
- Store the derived result in the module’s global state without exposing unsafe memory patterns.

### Phase 3: Validation Tests

- Add unit tests covering:
  - simple executable name input
  - path-qualified input
  - separator edge cases relevant to the original implementation
  - repeated invocation behavior if the state can be reset/reassigned
- Confirm the implementation compiles cleanly and passes `cargo test`.

### Phase 4: Integration Cleanup

- Align naming and visibility with the rest of the Rust port branch.
- Remove any unnecessary temporary helpers introduced during translation.
- Verify that the final module remains a direct migration of `gnu/progname.c` only, with no added capabilities.