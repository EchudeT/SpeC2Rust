# Implementation Plan

## Summary

Port `src/parseopt/wordwrap.c` to a Rust module that preserves the existing word-wrapping behavior and call structure while replacing C varargs, buffer mutation, and character-by-character output handling with safe Rust APIs.

The Rust implementation should stay narrowly scoped to the current module responsibilities:

- emit wrapped text to an output sink
- process paragraph-style wrapping logic
- support formatted output entry points analogous to the C functions
- preserve line-width and spacing semantics from the original implementation

Technical approach:

- Implement a dedicated Rust module for the word-wrapping logic, centered on a small state struct that owns wrapping configuration and current output state.
- Replace raw output callbacks and mutable C buffers with `std::io::Write` or `std::fmt::Write`-style interfaces, selecting the minimal one that matches current project usage.
- Replace `wordwrap_vprintf`/`wordwrap_printf` with Rust formatting entry points based on `std::fmt::Arguments`.
- Keep function boundaries close to the C originals so migration and verification remain straightforward.
- Preserve output ordering and wrapping behavior exactly where observable.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates required by current evidence
- **Testing**:
  - `cargo test`
  - unit tests focused on wrapping width, whitespace handling, paragraph transitions, and formatted output paths
- **Performance Goals**:
  - Match the C module’s asymptotic behavior for sequential text processing
  - Avoid unnecessary intermediate allocations beyond what Rust formatting requires
  - Process text in a single forward pass where practical
  - Keep per-character/per-word state compact and stack-resident when possible

## Module Mapping

### C to Rust File Mapping

- `src/parseopt/wordwrap.c`
  - → `src/parseopt/wordwrap.rs`

### Function Mapping

Keep the migrated API close to the original function set, but adapt signatures to Rust idioms.

- `wordwrap_putc`
  - → `wordwrap_putc` as a private helper or method on the wrapper state
  - Responsibility: consume one character and update wrapping/output state

- `wordwrap_para`
  - → `wordwrap_para`
  - Responsibility: process paragraph text according to width/indent/current-column state

- `wordwrap_vprintf`
  - → `wordwrap_vprintf` equivalent taking `std::fmt::Arguments<'_>`
  - Responsibility: accept prebuilt formatting arguments and feed produced text into wrapping logic

- `wordwrap_printf`
  - → `wordwrap_printf` equivalent built as a thin wrapper around the `Arguments`-based function
  - Responsibility: preserve the outer formatted entry point without C varargs

### Rust Module Placement

Use the existing Rust project layout and place the migrated module alongside related parse-option code:

- `src/parseopt/mod.rs` updated only as needed to expose `wordwrap`
- `src/parseopt/wordwrap.rs` containing:
  - wrapper state struct
  - migrated public functions
  - private helpers for spacing, line breaking, and sink writes
  - unit tests in `#[cfg(test)]`

## Data Model

The analysis reports only anonymous C data structures, so the Rust plan should avoid inventing extra public model types. Introduce only the minimum internal state needed to represent the C module’s operational state.

### Data Structure Mapping

- anonymous C structs used for local/stateful wrapping context
  - → one private Rust struct, tentatively `WordWrapState`
  - Fields derived from actual C usage during migration, likely including:
    - target line width
    - current column
    - indentation or left margin values
    - pending space/newline flags
    - sink reference/adapter
    - any mode flags required by paragraph formatting

- anonymous C temporary grouping of formatting/output data
  - → local Rust variables or small private helper structs only if direct translation shows repeated grouped state
  - Avoid creating multiple abstraction layers unless the C code clearly maintains separate state objects

### C-to-Rust Type Conversions

Use direct, conservative mappings:

- `int` / width / counters
  - → `usize` for sizes and columns when always non-negative
  - → `isize` or `i32` only if negative sentinel behavior exists in the C code

- `char`
  - → `char` when operating on decoded characters
  - → `u8` when behavior is byte-oriented and must mirror C exactly

- `char *` / string input
  - → `&str` if the C logic is textual and UTF-8-safe in project context
  - → `&[u8]` or byte iteration if exact byte semantics are required to preserve wrapping behavior

- output stream or callback-like sink
  - → `&mut dyn std::fmt::Write` if formatting is string-oriented
  - → `&mut dyn std::io::Write` if direct byte emission is closer to existing call sites
  - Choose one after checking neighboring Rust modules; do not support both unless current migration requires it

- variadic argument handling
  - → `std::fmt::Arguments<'_>`

### Ownership and Lifetime Strategy

- State object owns only scalar wrapping state.
- Output sink is borrowed mutably for the duration of a call or stored in the state with an explicit lifetime.
- Avoid heap allocation for state unless required by call structure.
- Keep helper functions borrowing state mutably instead of cloning strings or buffers.

### Error Handling Strategy

Because C output functions may signal write failures differently, map failures explicitly:

- sink write failures
  - → `Result<(), std::fmt::Error>` or `std::io::Result<()>`, depending on sink choice
- invalid internal assumptions discovered during translation
  - → resolve by preserving C control flow rather than adding new recoveries
- no panic-based control flow for normal write/wrap conditions

Use one consistent result type across the module to keep the migration simple.

## Implementation Phases

## Phase 1: Establish Rust module skeleton and state translation

Goals:

- Create `src/parseopt/wordwrap.rs`
- Identify the exact operational state in `wordwrap.c`
- Translate anonymous C state into one minimal private Rust struct
- Select the sink trait (`fmt::Write` or `io::Write`) based on closest existing Rust call sites

Tasks:

- Inspect `wordwrap.c` and enumerate all persistent variables used across helper calls
- Define `WordWrapState` with direct field mapping from C state
- Add placeholder Rust signatures for:
  - `wordwrap_putc`
  - `wordwrap_para`
  - `wordwrap_vprintf`
  - `wordwrap_printf`
- Update `src/parseopt/mod.rs` only as needed to compile the new module
- Choose concrete integer types conservatively to avoid behavior drift

Acceptance criteria:

- Module compiles with stubbed logic
- State fields correspond directly to C usage
- No extra public API beyond the migrated surface

## Phase 2: Port core wrapping logic

Goals:

- Migrate the actual line-break and paragraph behavior
- Preserve character/space handling and line-width transitions

Tasks:

- Port `wordwrap_putc` first, keeping control flow close to the C original
- Port `wordwrap_para` on top of the same state and helper routines
- Reproduce:
  - current-column updates
  - indentation handling
  - pending whitespace behavior
  - newline emission rules
  - end-of-paragraph behavior
- Resolve C pointer/buffer traversal into safe iteration without changing visible semantics
- Keep helper extraction minimal and only where needed to avoid duplicated write logic

Acceptance criteria:

- Static review shows one-to-one correspondence with C control flow for wrap decisions
- No unchecked indexing or raw pointer logic remains
- Output for representative manual cases matches the C behavior

## Phase 3: Port formatted entry points

Goals:

- Replace C varargs-based formatting with Rust formatting while keeping call layering familiar

Tasks:

- Implement `wordwrap_vprintf` equivalent using `std::fmt::Arguments<'_>`
- Implement `wordwrap_printf` as a thin wrapper that forwards formatted arguments
- Bridge formatted text into the paragraph/wrapping path with minimal temporary storage
- If temporary string materialization is necessary, keep it local and documented as the Rust replacement for C varargs expansion

Acceptance criteria:

- Formatted output flows through the same wrapping logic as raw paragraph text
- API layering mirrors the C module closely
- Error propagation from formatting/output is consistent across all entry points

## Phase 4: Validation and cleanup

Goals:

- Verify behavioral parity and finalize the narrow migration

Tasks:

- Add unit tests covering:
  - text shorter than width
  - exact-width boundaries
  - wrap at spaces
  - repeated spaces or leading indentation cases present in C behavior
  - explicit paragraph/newline handling
  - formatted output through the `printf`-style entry point
- Compare Rust outputs against expected outputs derived from the C module for targeted cases
- Remove any migration scaffolding or unused helpers
- Confirm module visibility and signatures are limited to what the project needs

Acceptance criteria:

- `cargo test` passes
- The module has no dead code introduced by over-generalization
- Final Rust code remains focused on the original file’s responsibilities only

## Notes and Constraints

- Do not broaden this port into a generic text-layout library.
- Do not add concurrency primitives, FFI layers, serialization, or benchmark harnesses.
- Prefer byte- or character-processing semantics only as required by the original C behavior.
- Keep the migration local to `wordwrap.c` and its direct Rust module mapping.
- Any ambiguity from anonymous C structures should be resolved by inspecting actual field usage, not by introducing speculative abstractions.