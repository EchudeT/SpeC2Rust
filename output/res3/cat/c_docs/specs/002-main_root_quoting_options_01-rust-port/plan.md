# Implementation Plan: main_root_quoting_options_01

## Summary

Port the `quotearg.c` functionality used by `cat` into a Rust module that preserves the existing quoting-options behavior and call patterns needed by the listed functions. The Rust implementation should focus on migrating the current option/state handling and argument-quoting entry points without adding new quoting features or broader abstractions.

Technically, the Rust port should:
- translate the C quoting configuration structures into compact Rust structs/enums,
- implement the listed option mutation/query functions as direct Rust equivalents,
- preserve the buffer-producing and allocation-returning quoting APIs with Rust-owned memory (`Vec<u8>`/`String`) while keeping behavior aligned with the C routines,
- centralize the actual quoting logic in one internal formatter used by the public wrapper functions.

The implementation should remain narrowly scoped to `quotearg.c` and the functions listed for this module, with migration order driven by data types first, then option setters/getters, then quoting output functions.

## Technical Context

### Language/Version
- Rust 1.81 or stable compatible toolchain for the branch
- Edition: Rust 2021

### Primary Dependencies
- Rust standard library only
- No third-party crates are recommended from the provided evidence

### Testing
- `cargo test`

### Performance Goals
- Match the existing C module’s practical runtime characteristics for short command-line argument quoting paths
- Avoid unnecessary intermediate allocations where a caller-visible buffer API is expected
- Reuse shared internal formatting logic so wrapper functions do not duplicate work
- Keep cloning and option mutation costs constant-time or near-constant-time relative to option structure size

## Module Mapping

### C to Rust File Mapping
- `quotearg.c` → `src/main_cluster/main_root_quoting_options_01.rs`

If the project already uses a different module layout for the `cat` port, keep the Rust implementation in the existing `main_cluster` area and map this C file to a single Rust source file rather than splitting it further.

### Function Mapping
- `clone_quoting_options` → `pub fn clone_quoting_options(...) -> ...`
- `get_quoting_style` → `pub fn get_quoting_style(...) -> ...`
- `set_quoting_style` → `pub fn set_quoting_style(...) -> ...`
- `set_char_quoting` → `pub fn set_char_quoting(...) -> ...`
- `set_quoting_flags` → `pub fn set_quoting_flags(...) -> ...`
- `set_custom_quoting` → `pub fn set_custom_quoting(...) -> ...`
- `quoting_options_from_style` → `pub fn quoting_options_from_style(...) -> ...`
- `quotearg_buffer` → `pub fn quotearg_buffer(...) -> ...`
- `quotearg_alloc` → `pub fn quotearg_alloc(...) -> ...`
- `quotearg_alloc_mem` → `pub fn quotearg_alloc_mem(...) -> ...`
- `quotearg_n_options` → `pub fn quotearg_n_options(...) -> ...`
- `quotearg_n_style` → `pub fn quotearg_n_style(...) -> ...`
- `quotearg_n_style_mem` → `pub fn quotearg_n_style_mem(...) -> ...`
- `quotearg_char_mem` → `pub fn quotearg_char_mem(...) -> ...`
- `quotearg_n_style_colon` → `pub fn quotearg_n_style_colon(...) -> ...`

### Internal Rust Helpers
Keep helper scope local to this file/module:
- one internal quoting-style enum conversion/helper,
- one internal formatting routine that accepts bytes plus options,
- one internal helper for custom delimiters/literal fragments if needed.

Do not create additional public modules beyond what is required to host the migrated file.

## Data Model

The input only reports anonymous C data structures, so the Rust model should be reconstructed from function usage and constants in `quotearg.c`, not from speculative expansion.

### Data Structure Mapping
- C anonymous quoting options struct → `struct QuotingOptions`
- C quoting style constants / enum-like values → `enum QuotingStyle`
- C quoting flags bitfield / integer flags → `struct QuotingFlags` or `type QuotingFlags = u32`
- C per-character quoting table / bitmap → fixed-size byte table such as `[bool; 256]` or packed integer array
- C custom quoting delimiter pair / custom quote strings → `struct CustomQuoting`
- C returned allocated quoted string → `String` when UTF-8-safe behavior is intended, otherwise `Vec<u8>` plus explicit conversion at API boundaries
- C output buffer pointer + length pair → `&mut [u8]` and returned written length, or a Rust function returning the produced length while filling a caller buffer
- C slot-based cached quoted arguments used by `quotearg_n_*` family → module-local cache structure only if required by observed call semantics; otherwise implement direct formatting without introducing persistence not required by call sites

### Proposed Rust Types

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuotingStyle {
    // exact variants derived from quotearg.c during port
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuotingOptions {
    pub style: QuotingStyle,
    pub flags: u32,
    pub quote_these_too: [bool; 256],
    pub custom: Option<CustomQuoting>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustomQuoting {
    pub left_quote: Vec<u8>,
    pub right_quote: Vec<u8>,
}
```

### Ownership and Memory Decisions
- Replace C heap allocation with Rust-owned values.
- Prefer byte-oriented storage internally because quoting APIs often operate on explicit memory ranges rather than guaranteed UTF-8 strings.
- Convert to `String` only for APIs whose semantics clearly require textual return values.
- For `_mem` variants, preserve explicit input length handling and avoid assuming NUL termination.
- For custom quoting strings, own the bytes in the options structure so cloning behavior is explicit and does not depend on external lifetimes.

### Error Handling
- Use infallible setters/getters where the C behavior is infallible.
- Use `Result` only where invalid option combinations or buffer constraints require explicit reporting in Rust.
- If a C function would effectively panic/abort on invalid internal state, prefer a narrow Rust assertion during development rather than adding a new recovery layer.
- For APIs that write into a caller buffer, return the required/written length and avoid unchecked writes.

## Implementation Phases

### Phase 1: Port Core Types and Option State
Goals:
- Create the Rust module file for `quotearg.c`
- Define `QuotingStyle`, `QuotingOptions`, and custom-quote storage
- Implement option cloning and simple style/flag accessors

Functions in scope:
- `clone_quoting_options`
- `get_quoting_style`
- `set_quoting_style`
- `set_char_quoting`
- `set_quoting_flags`
- `set_custom_quoting`
- `quoting_options_from_style`

Technical notes:
- Reconstruct the exact style enum and flag constants from the C source during migration.
- Represent the per-character quoting selection with a fixed-size table indexed by `u8`.
- Preserve setter return behavior from C where prior values are returned.
- Ensure cloning duplicates custom quote data rather than aliasing it.

Exit criteria:
- The options object can be created, cloned, mutated, and inspected entirely in Rust.
- Unit tests verify flag/style updates and per-character quoting changes.

### Phase 2: Port the Core Quoting Engine
Goals:
- Implement the internal formatting path used by all public quoting entry points
- Support byte-slice input and explicit-length behavior first
- Add buffer-writing behavior matching the C buffer-oriented API

Functions in scope:
- `quotearg_buffer`
- internal formatter used by allocation and style wrappers

Technical notes:
- Build around an internal byte-oriented formatter that accepts `&[u8]` and `&QuotingOptions`.
- Keep escaping/quoting decisions centralized so wrapper functions differ only in argument adaptation and result packaging.
- For caller-provided buffers, compute the produced length and copy only the fitting prefix if that matches existing semantics; confirm exact truncation/length behavior from the C implementation during port.
- Avoid introducing Unicode-specific transformations not present in the source behavior.

Exit criteria:
- The module can format input into a destination buffer using the migrated options.
- Tests cover empty input, explicit-length input containing NUL or punctuation, and custom quote delimiters.

### Phase 3: Port Allocating and Style-Based Wrappers
Goals:
- Implement the family of convenience wrappers that allocate or derive options from style arguments
- Keep wrappers thin and routed through Phase 2 logic

Functions in scope:
- `quotearg_alloc`
- `quotearg_alloc_mem`
- `quotearg_n_options`
- `quotearg_n_style`
- `quotearg_n_style_mem`
- `quotearg_char_mem`
- `quotearg_n_style_colon`

Technical notes:
- Determine from current call sites whether `quotearg_n_*` requires persistent slot semantics or only argument-indexed API compatibility; if no persistent storage is required by the Rust port, keep implementation direct and local.
- `_style` wrappers should construct options through `quoting_options_from_style` rather than duplicating style initialization.
- `_char_mem` and `_style_colon` should be implemented as specific option adjustments layered on top of the common formatter.
- Keep memory ownership simple: allocation-returning functions return owned Rust results and avoid emulating manual free patterns.

Exit criteria:
- All listed public functions exist in Rust and share one common formatting engine.
- Tests verify wrapper equivalence against the base options-driven path.

### Phase 4: Integration Cleanup and Behavioral Verification
Goals:
- Integrate the module into the branch’s existing `cat` crate structure
- Remove migration scaffolding and validate behavior consistency at module level

Technical notes:
- Align visibility (`pub(crate)` vs `pub`) with actual crate usage after wiring call sites.
- Normalize signatures to the minimum needed by existing Rust callers while preserving semantics from the C module.
- Add focused regression tests for combinations of style, flags, custom quoting, and explicit memory-length inputs.
- Keep changes confined to this module and only the immediate call-site adjustments needed to compile.

Exit criteria:
- The Rust module replaces the targeted C functionality for this branch.
- `cargo test` passes for the module and dependent code paths.

## Validation Strategy

- Unit tests for:
  - cloning option state,
  - style and flag mutation,
  - per-character quoting toggles,
  - custom quote pair handling,
  - fixed buffer output behavior,
  - allocating wrappers and `_mem` variants.
- Compare wrapper outputs against the core options-driven function to ensure one behavior path.
- Verify byte-preserving handling for non-UTF-8 input in `_mem`-style paths.

## Constraints and Non-Goals

- Do not add new quoting styles or option fields beyond those required by `quotearg.c`.
- Do not split the port into extra support modules solely for abstraction.
- Do not add thread-safety layers, serialization, FFI adapters, benchmarking work, or recovery frameworks.
- Do not broaden API scope beyond the listed functions and their direct supporting types.