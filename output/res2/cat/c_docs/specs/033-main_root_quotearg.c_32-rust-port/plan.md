# Implementation Plan: main_root_quotearg.c_32

## Summary

This module ports the `quotearg.c` functionality into Rust for the `cat` project branch `033-main_root_quotearg.c_32-rust-port`. The Rust implementation should preserve the existing quoting behavior and public call patterns of the C functions while replacing manual buffer management and global cleanup patterns with owned Rust types and scoped memory handling.

The implementation should stay narrowly focused on migrating the existing file and functions:

- `gettext_quote`
- `quotearg_buffer_restyled`
- `quotearg_free`
- `quotearg`
- `quotearg_mem`
- `quotearg_char`
- `quote_mem`
- `quote`

Technical approach:

- Port the logic in `quotearg.c` into a single Rust module with minimal internal reorganization.
- Represent quote styles, option flags, and transient quoting state using Rust enums/structs that mirror the C layout closely enough to preserve behavior.
- Replace raw output buffers with `String` or `Vec<u8>` depending on whether the original logic is byte-oriented.
- Preserve byte-exact behavior where the C code operates on arbitrary memory ranges rather than UTF-8 text.
- Convert cleanup-oriented APIs such as `quotearg_free` into a no-op or narrowly scoped state reset if Rust ownership removes the need for heap tracking.
- Keep error handling explicit and local; avoid introducing broader abstractions or new utility layers not required by this module migration.

## Technical Context

- **Language/Version:** Rust 1.78+
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - Maintain linear-time processing over input length.
  - Avoid unnecessary intermediate allocations beyond what is needed to build the quoted output.
  - Preserve efficient handling of byte slices for `*_mem` functions.
  - Match existing behavior without adding caching or background state.

## Technical Context Details

### Rust Edition and Compiler
Use stable Rust with the project’s standard edition configuration. Target implementation compatibility with Rust 1.78 or newer.

### Dependency Policy
No third-party crates are recommended from the provided input. The module should rely on:

- `std::borrow`
- `std::string`
- `std::vec`
- `std::sync` only if already required by surrounding project code for existing global state patterns

Do not introduce localization, unicode, or escaping crates unless they are already present elsewhere in the project and directly required by the existing interface.

### Testing Strategy
Use `cargo test` with focused unit tests covering:

- quoting of plain strings
- quoting of arbitrary byte buffers
- character-specific quoting behavior
- consistency between wrapper functions and the core quoting routine
- edge cases such as empty input, embedded special bytes, and repeated calls

### Performance and Memory
The Rust port should:

- avoid per-character heap allocations
- pre-size output buffers when the C logic makes an output growth pattern predictable
- prefer `Vec<u8>` internally when operating on raw memory inputs
- convert to `String` only when the function contract is text-oriented and valid encoding is guaranteed or intentionally lossy conversion is acceptable per current project behavior

## Module Mapping

### Source File Mapping
- `quotearg.c` -> `src/quotearg.rs`

### Function Mapping
- `gettext_quote` -> `pub(crate) fn gettext_quote(...) -> ...`
- `quotearg_buffer_restyled` -> `fn quotearg_buffer_restyled(...) -> ...`
- `quotearg_free` -> `pub(crate) fn quotearg_free()`
- `quotearg` -> `pub(crate) fn quotearg(...) -> ...`
- `quotearg_mem` -> `pub(crate) fn quotearg_mem(...) -> ...`
- `quotearg_char` -> `pub(crate) fn quotearg_char(...) -> ...`
- `quote_mem` -> `pub(crate) fn quote_mem(...) -> ...`
- `quote` -> `pub(crate) fn quote(...) -> ...`

### Internal Organization
Keep the Rust implementation in one module mirroring the original C file. Internal helper functions may be added only where required to separate:

- style selection
- buffer writing
- wrapper function adaptation

Do not split the port into additional modules unless the existing Rust crate layout already requires a specific placement.

## Data Model

Because the analysis only reports anonymous C data structures, the Rust plan should map them according to their actual roles in `quotearg.c` during implementation rather than inventing new domain models.

### Planned Structure Mapping

- anonymous quote/options struct(s) -> `struct QuotingOptions`
- anonymous style discriminator(s) -> `enum QuotingStyle`
- anonymous flag fields / mode selectors -> integer fields or small Rust enums inside `QuotingOptions`
- anonymous character-set / quoting mask storage -> fixed-size byte array, bitset-like array, or `Vec<bool>` matching the original semantics
- anonymous transient output/buffer state -> local `Vec<u8>` / `String` builders
- anonymous static slot tracking used by `quotearg`-style wrappers -> module-local owned storage, only if the original behavior requires stable returned values across calls

### C to Rust Type Guidance

- `char *` output buffers -> `Vec<u8>` or `String`
- `char const *` input strings -> `&str` or `&[u8]` depending on function semantics
- `size_t` -> `usize`
- `ptrdiff_t` / signed size-like counters -> `isize` where required, otherwise `usize`
- C enum values -> Rust `enum` with explicit conversion helpers
- mutable global allocation lists -> removed if unnecessary under ownership, or replaced with narrowly scoped module-local state only to preserve API behavior

### Memory Management Decisions

- Functions that conceptually return quoted text should return owned Rust values rather than pointers.
- If surrounding code requires C-like repeated-call semantics for `quotearg`, implement that behavior with minimal internal retained storage.
- `quotearg_free` should only clear retained module state if such state is necessary for compatibility; otherwise it remains a no-op with documented rationale.
- Avoid unsafe code unless required to match an existing crate interface.

### Error Handling Decisions

- The original C code likely signals failures through allocation failure or buffer sizing behavior. In Rust, ordinary owned-string construction should rely on infallible growth under standard panic-on-OOM semantics.
- Do not introduce custom error enums unless a specific function boundary in the current Rust crate requires them.
- For byte-oriented quoting, preserve non-UTF-8 inputs without forcing validation.

## Implementation Phases

## Phase 1: Establish Rust Module Skeleton and Core Data Types

Scope:

- Create `src/quotearg.rs`.
- Define Rust equivalents for the C quote style and option/state structures.
- Add function signatures for all migrated functions.
- Identify which functions are byte-oriented versus text-oriented from current call sites and preserve that distinction in signatures.

Deliverables:

- `QuotingStyle` enum
- `QuotingOptions` struct
- placeholder or initial implementation for `gettext_quote`
- wrapper signatures for:
  - `quotearg`
  - `quotearg_mem`
  - `quotearg_char`
  - `quote_mem`
  - `quote`
  - `quotearg_free`

Notes:

- Keep structure layout and field purposes close to the C source.
- Do not add new public APIs beyond the migrated functions.

## Phase 2: Port Core Quoting Logic

Scope:

- Implement `quotearg_buffer_restyled` as the central logic path.
- Port the original escaping, quoting-style selection, delimiter insertion, and per-byte handling into Rust.
- Use `Vec<u8>` as the primary internal output buffer where arbitrary memory inputs are accepted.
- Add small internal helpers only when needed to express the existing C branching cleanly.

Deliverables:

- working `quotearg_buffer_restyled`
- integrated `gettext_quote` behavior used by the core routine
- correct handling of empty inputs, special characters, and style-dependent quoting

Notes:

- Preserve original branch order where practical to reduce behavioral drift.
- Prefer direct translation of logic over architectural cleanup.

## Phase 3: Port Wrapper Functions and State Cleanup Behavior

Scope:

- Implement:
  - `quotearg`
  - `quotearg_mem`
  - `quotearg_char`
  - `quote_mem`
  - `quote`
- Determine whether compatibility requires retained internal slots for repeated wrapper calls.
- Implement `quotearg_free` to release only the retained state actually used by the Rust port.

Deliverables:

- all wrapper functions calling the shared core logic
- minimal retained state, if required
- final `quotearg_free` behavior aligned with the Rust ownership model

Notes:

- If wrapper semantics can be preserved with purely owned return values, prefer that over emulating C allocation patterns.
- If existing crate interfaces require borrowed/static-like outputs, confine the compatibility mechanism to this module.

## Phase 4: Verification and Behavioral Alignment

Scope:

- Add unit tests for direct and wrapper-based quoting paths.
- Compare wrapper outputs for equivalent inputs to ensure they route consistently through the core implementation.
- Validate memory-oriented cases and cleanup behavior.

Deliverables:

- `cargo test` coverage for:
  - plain text quoting
  - byte-buffer quoting
  - special-character quoting
  - repeated call behavior
  - `quotearg_free` effects if retained state exists

Exit Criteria:

- all migrated functions compile and are wired into the branch
- tests pass with stable output behavior
- no unnecessary modules, crates, or compatibility layers were added