# Implementation Plan

## Summary
Port the C quoting logic in `quotearg.c` into a single Rust module that preserves the existing function-level behavior and call patterns used by the `pwd` project branch `001-main_root_quoting_options_01-rust-port`.

The Rust implementation should center on:
- a direct Rust representation of quoting configuration state,
- faithful migration of the listed option-manipulation functions,
- string/byte-buffer oriented quoting entry points matching the existing C function split,
- minimal, explicit ownership for allocated outputs instead of manual heap management.

The implementation should avoid introducing new abstraction layers beyond what is needed to replace the C file. The preferred shape is one Rust source module containing:
- quoting style and flags definitions,
- quoting options state,
- helper routines for quoting into buffers and allocated outputs,
- public functions corresponding closely to the C API names and responsibilities.

The technical approach is to map C’s mutable option structs and buffer-writing routines into Rust structs and functions over `&[u8]`, `String`, and `Vec<u8]`, while preserving behavior around style selection, per-character quoting, custom quotes, and caller-visible output sizing.

## Technical Context

### Language/Version
- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.74+`

### Primary Dependencies
- Rust standard library only:
  - `std::borrow`
  - `std::ffi` only if existing project interfaces require byte/OS-string boundaries
  - `std::fmt`
  - `std::mem`
  - `std::ops`
  - `std::string`
  - `std::vec`

No third-party crates are recommended from the provided input, since the module scope is a direct migration of existing C functionality and does not require external parsing or escaping libraries.

### Testing
- `cargo test`

Testing should cover:
- option cloning and mutation behavior,
- quoting style selection,
- custom quote delimiter validation,
- buffer-size and returned-length behavior,
- byte-oriented quoting for inputs containing ASCII punctuation, colon, quotes, and embedded non-printable bytes where relevant,
- parity-focused regression cases for the listed public functions.

### Performance Goals
- Match the C module’s practical performance characteristics for short path/string inputs typical of `pwd`.
- Avoid unnecessary intermediate allocations in buffer-oriented paths.
- Ensure allocated-output wrappers perform at most one final owned allocation for the returned result where practical.
- Keep per-byte quoting checks O(n) with constant-time option lookups.

## Module Mapping

### C to Rust File Mapping
- `quotearg.c` -> `src/quotearg.rs`

If the current crate already keeps root-level functionality in `src/main.rs` or `src/lib.rs`, expose only the migrated functions needed by existing callers:
- `mod quotearg;`
- re-export only if the current project layout already expects direct top-level access.

### Function Mapping
Each listed C function should be migrated as a Rust function with the same conceptual role, keeping names as close as practical to aid review:

- `clone_quoting_options` -> `pub fn clone_quoting_options(...) -> QuotingOptions`
- `get_quoting_style` -> `pub fn get_quoting_style(...) -> QuotingStyle`
- `set_quoting_style` -> `pub fn set_quoting_style(...)`
- `set_char_quoting` -> `pub fn set_char_quoting(...) -> bool` or previous-state indicator
- `set_quoting_flags` -> `pub fn set_quoting_flags(...) -> QuotingFlags` or previous flags value
- `set_custom_quoting` -> `pub fn set_custom_quoting(...)`
- `quoting_options_from_style` -> `pub fn quoting_options_from_style(...) -> QuotingOptions`
- `quotearg_buffer` -> `pub fn quotearg_buffer(...) -> usize`
- `quotearg_alloc` -> `pub fn quotearg_alloc(...) -> String` or `Vec<u8>` depending on caller usage
- `quotearg_alloc_mem` -> `pub fn quotearg_alloc_mem(...) -> Vec<u8>`
- `quotearg_n_options` -> `pub fn quotearg_n_options(...) -> String` or owned bytes wrapper
- `quotearg_n_style` -> `pub fn quotearg_n_style(...) -> String`
- `quotearg_n_style_mem` -> `pub fn quotearg_n_style_mem(...) -> Vec<u8>`
- `quotearg_char_mem` -> `pub fn quotearg_char_mem(...) -> Vec<u8>` or string wrapper when valid UTF-8 is guaranteed by caller
- `quotearg_n_style_colon` -> `pub fn quotearg_n_style_colon(...) -> String`

Where the C API distinguishes string and memory variants, the Rust API should preserve that distinction rather than collapsing everything into UTF-8 strings. This keeps behavior closer to the source and avoids accidental invalid UTF-8 assumptions.

## Data Model

Because the input names only anonymous C data structures, the Rust plan should define explicit internal types based on observed responsibilities rather than attempting a one-to-one anonymous-struct naming scheme.

### Core Type Mapping

| C concept | Rust type | Notes |
|---|---|---|
| quoting style enum/int | `enum QuotingStyle` | Explicit enum variants for the styles used by `quotearg.c`. |
| quoting flags bitfield/int | `struct QuotingFlags(u32)` or type alias | Use bitmask constants or a small newtype; no external bitflags crate needed. |
| quoting options struct | `struct QuotingOptions` | Holds style, flags, per-character quoting map, and optional custom delimiters. |
| custom left/right quote pointers | `Option<Box<[u8]>>` or `Option<Vec<u8>>` | Owned storage avoids borrowed-lifetime complexity during migration. |
| per-character quoting table/bitmap | `[bool; 256]` or `[u8; 32]` bitmap | Prefer `[bool; 256]` first for clarity unless exact bit behavior is easier to preserve with a bitmap. |
| output buffer and size arguments | `&mut [u8]` plus returned `usize` | Return required/written length separately as in C-style buffer APIs. |
| allocated quoted string | `String` or `Vec<u8>` | Use `Vec<u8>` for `_mem` forms and convert to `String` only for string-oriented wrappers. |
| slot-based `quotearg_n_*` state | module-local reusable storage, if required by existing callers | Keep restrained; only implement if the project currently depends on slot semantics. Otherwise preserve behavior through owned return values. |

### Proposed Rust Structures

#### `QuotingStyle`
An enum representing the supported styles from the source C implementation. Variants should be limited to the styles actually referenced by `quotearg.c` and current callers.

#### `QuotingOptions`
Fields:
- `style: QuotingStyle`
- `flags: u32` or `QuotingFlags`
- `quote_these_too: [bool; 256]`
- `left_quote: Option<Vec<u8>>`
- `right_quote: Option<Vec<u8>>`

This structure replaces the mutable C options object and should derive `Clone` so `clone_quoting_options` is trivial and exact.

#### `QuotingFlags`
If the C module uses a simple integer mask, use:
- `type QuotingFlags = u32`
or
- `struct QuotingFlags(u32)`

Use named constants for each migrated flag. Avoid introducing richer abstractions not required by the current file.

### Memory Management Decisions
- Replace C heap allocation with Rust-owned `Vec<u8>` and `String`.
- Avoid shared mutable global allocation state unless `quotearg_n_*` semantics strictly require retained slots.
- For custom quotes, copy inputs into owned vectors inside `QuotingOptions` to avoid dangling references.
- Buffer-writing functions should never write past `&mut [u8]`; return total required length so wrappers can allocate correctly.

### Error Handling Decisions
- Preserve infallible setters where the C code assumes valid inputs.
- Use `assert!` only for invariants that are programmer errors and mirror impossible C states.
- If custom quoting requires non-empty delimiters, return a simple deterministic outcome:
  - either panic only if the original C code aborts on invalid input,
  - or use `Option`/`Result` internally and keep public wrappers aligned with expected project behavior.
- Do not introduce broad error enums unless the source behavior requires observable failure reporting.

## Implementation Phases

## Phase 1: Establish Rust Types and Option Mutation Functions
Scope:
- Create `src/quotearg.rs`.
- Define `QuotingStyle`.
- Define flag constants/newtype.
- Define `QuotingOptions` with cloneable owned fields.
- Implement:
  - `clone_quoting_options`
  - `get_quoting_style`
  - `set_quoting_style`
  - `set_char_quoting`
  - `set_quoting_flags`
  - `set_custom_quoting`
  - `quoting_options_from_style`

Technical notes:
- Keep style/flag values and defaults aligned with the C file.
- Implement per-character quoting state with a fixed 256-entry table for direct byte indexing.
- Store custom delimiters as owned bytes to avoid lifetime propagation through all APIs.
- Add focused unit tests for mutation semantics and cloning parity.

Exit criteria:
- Option state can be constructed, cloned, and modified with deterministic behavior.
- Style-derived defaults are reproducible in tests.

## Phase 2: Implement Core Buffer Quoting Logic
Scope:
- Implement the internal quoting engine operating on byte slices and `QuotingOptions`.
- Implement `quotearg_buffer`.

Technical notes:
- Build around a single internal function that:
  - scans input bytes,
  - decides whether to quote/escape based on style, flags, and per-character table,
  - writes into an optional destination buffer,
  - always computes total output length.
- Use a two-mode pattern:
  - measurement mode: no writes, only count output bytes,
  - write mode: fill `&mut [u8]` up to capacity according to the C behavior.
- Keep byte-oriented processing to preserve non-UTF-8 behavior.
- Ensure colon-specific and custom-delimiter behavior can be layered without duplicating the engine.

Testing:
- add cases for empty input, plain ASCII, forced quoted characters, custom left/right quotes, and truncation-safe buffer writes.

Exit criteria:
- `quotearg_buffer` returns correct lengths and produces stable output for representative cases.
- No unsafe code is needed unless exact C buffer semantics force it; if used, isolate it and document the invariant.

## Phase 3: Add Owned-Output Wrappers
Scope:
- Implement:
  - `quotearg_alloc`
  - `quotearg_alloc_mem`
  - `quotearg_n_options`
  - `quotearg_n_style`
  - `quotearg_n_style_mem`
  - `quotearg_char_mem`
  - `quotearg_n_style_colon`

Technical notes:
- These functions should be thin wrappers over the core buffer engine.
- For `_mem` variants, return `Vec<u8>`.
- For string variants, convert only when the produced quoting form is guaranteed or expected to be UTF-8-compatible for current callers; otherwise keep internal byte paths and use lossy conversion only if the existing Rust project interface already accepts that behavior.
- For `quotearg_char_mem`, construct a temporary `QuotingOptions`, mark the requested byte in `quote_these_too`, and delegate.
- For `quotearg_n_style_colon`, construct style-based options and additionally force quoting of `b':'`.

About slot-indexed `n` variants:
- Implement only the visible API shape required by the port.
- If the C code relies on persistent slot storage, use a restrained module-local vector of owned results.
- If the Rust call sites only need owned returns, keep `n` accepted but do not expand hidden caching beyond compatibility needs.

Testing:
- wrapper parity tests against the core engine,
- explicit tests for colon forcing and character forcing,
- allocation-size correctness for `_alloc` paths.

Exit criteria:
- All listed public functions are present and routed through shared logic.
- Owned-return APIs do not leak C allocation assumptions into Rust code.

## Phase 4: Integrate with Callers and Finalize Migration
Scope:
- Wire the new module into the existing `pwd` Rust branch at the call sites that previously depended on `quotearg.c`.
- Remove or isolate any temporary compatibility scaffolding introduced during the port.
- Normalize signatures to the smallest public surface needed by current project usage.

Technical notes:
- Keep migration local to the existing file/function boundaries.
- Prefer preserving call order and naming correspondence to simplify review against the C source.
- Validate that any remaining C-style assumptions about null termination or mutable global state are eliminated or explicitly adapted at the boundary.

Testing:
- run `cargo test`
- add regression tests for the specific caller paths in this branch that exercise root/main quoting options behavior.

Exit criteria:
- The module replaces the C functionality used by this branch.
- Tests pass without introducing additional support modules or nonessential abstractions.