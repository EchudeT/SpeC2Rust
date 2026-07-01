# Implementation Plan: main_root_quoting_options_01

## Summary

This module ports the quoting-options and argument-quoting portion of `quotearg.c` into Rust for the `pwd` project branch `001-main_root_quoting_options_01-rust-port`.

The Rust implementation should preserve the existing module boundary and behavior of the listed functions, with a direct migration of quoting configuration state, quoting-style selection, per-character quoting overrides, and the buffer/allocation entry points used to produce quoted argument strings. The implementation should stay narrowly scoped to the functions named in this module analysis and avoid introducing new abstraction layers beyond what is needed to represent the original C state safely.

The technical approach is:

- map the C quoting option structures and style/flag values into Rust structs and enums;
- replace raw memory mutation with owned Rust buffers (`String`/`Vec<u8>`) while preserving the observable output of the C APIs;
- implement the lower-level buffer-writing path first, then layer allocation and convenience wrappers on top of it;
- model C APIs that conceptually “clone” or “return modified old state” using explicit owned values and straightforward return types;
- keep error handling minimal and local, using standard Rust results only where internal conversions require it, while preserving C-like total behavior for public module functions where practical.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve linear-time quoting over the input byte sequence;
  - avoid unnecessary intermediate allocations in the core buffer path;
  - ensure wrapper functions reuse the core quoting routine instead of duplicating logic;
  - keep per-character quoting lookups constant-time.

## Module Mapping

### Source Mapping

- **C source file**: `quotearg.c`
- **Rust target file**: `src/quotearg.rs`

### Function Mapping

| C Function | Rust Target |
|---|---|
| `clone_quoting_options` | `pub fn clone_quoting_options(...) -> QuotingOptions` |
| `get_quoting_style` | `pub fn get_quoting_style(...) -> QuotingStyle` |
| `set_quoting_style` | `pub fn set_quoting_style(...) -> QuotingStyle` |
| `set_char_quoting` | `pub fn set_char_quoting(...) -> bool` or prior-state equivalent |
| `set_quoting_flags` | `pub fn set_quoting_flags(...) -> QuotingFlags` or prior-flags equivalent |
| `set_custom_quoting` | `pub fn set_custom_quoting(...)` |
| `quoting_options_from_style` | `pub fn quoting_options_from_style(...) -> QuotingOptions` |
| `quotearg_buffer` | `pub fn quotearg_buffer(...) -> usize` |
| `quotearg_alloc` | `pub fn quotearg_alloc(...) -> String` or byte buffer equivalent |
| `quotearg_alloc_mem` | `pub fn quotearg_alloc_mem(...) -> Vec<u8>` |
| `quotearg_n_options` | `pub fn quotearg_n_options(...) -> String` |
| `quotearg_n_style` | `pub fn quotearg_n_style(...) -> String` |
| `quotearg_n_style_mem` | `pub fn quotearg_n_style_mem(...) -> Vec<u8>` |
| `quotearg_char_mem` | `pub fn quotearg_char_mem(...) -> Vec<u8>` or `String` depending on current call sites |
| `quotearg_n_style_colon` | `pub fn quotearg_n_style_colon(...) -> String` |

### Integration Mapping

- If the current Rust port already has a `main.rs` or `lib.rs`, add:
  - `mod quotearg;`
- Import only where existing `pwd` main-cluster code requires these quoting helpers.
- Do not split this migration into additional helper modules unless required by existing project file layout.

## Data Model

The analysis only exposes anonymous C data structures, so the Rust data model should be inferred strictly from the listed functions and limited to the data needed by them.

### Data Structure Mapping

| C Representation | Rust Representation | Notes |
|---|---|---|
| quoting options struct | `#[derive(Clone, Debug, PartialEq, Eq)] pub struct QuotingOptions` | Holds style, flags, per-character quoting map, and optional custom quote delimiters. |
| quoting style enum/int constants | `#[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum QuotingStyle` | Replace integer style codes with a closed enum. Include only variants required by migrated functions and current `quotearg.c` behavior. |
| quoting flags bitfield | `#[derive(Copy, Clone, Debug, PartialEq, Eq)] pub struct QuotingFlags(u32)` | Bitmask newtype for direct flag translation without expanding semantics. |
| per-character quoting table/bitmap | `[bool; 256]` or `[u8; 256]` inside `QuotingOptions` | Fixed-size table gives direct byte-indexed lookup equivalent to C arrays/bitsets. |
| custom left/right quote pointers | `Option<CustomQuoting>` | Encapsulates custom delimiters without raw pointers. |
| custom quote strings | `pub struct CustomQuoting { left: Vec<u8>, right: Vec<u8> }` | Use owned bytes to preserve byte-oriented behavior. |
| output buffer and length pair | `&mut [u8]` plus returned `usize` | Replaces pointer+size mutation in `quotearg_buffer`. |
| allocated quoted string buffer | `String` or `Vec<u8>` | Prefer `String` only for text-safe wrappers; use `Vec<u8>` for `_mem` variants. |
| slot-based cached quoting state used by `quotearg_n_*` | module-local `Vec<Vec<u8>>`/`Vec<String>` if required by compatibility | Add only if current semantics depend on numbered reusable slots. Keep private to `src/quotearg.rs`. |

### Ownership and Memory Decisions

- `clone_quoting_options` becomes a normal Rust clone of owned state.
- `set_custom_quoting` must copy delimiter data into owned storage rather than borrowing external buffers, to avoid lifetime coupling that does not exist in C’s pointer-based API contract.
- `_mem` functions should operate on raw bytes and must not assume UTF-8.
- Non-`_mem` wrappers may return `String` only if the quoting output is guaranteed to be valid UTF-8 under the implemented styles; otherwise use `Vec<u8>` internally and convert at the outermost boundary only where existing Rust call sites require text.
- For APIs corresponding to “set and return previous value”, return the prior enum/flag/boolean state directly instead of mutating through pointers.

### Error Handling

- Keep public behavior close to the C implementation: quoting operations themselves should be total for any input bytes.
- Reserve `Result` for internal validation where custom quoting configuration is invalid in a way the C code would guard against explicitly.
- Avoid `unsafe` unless exact slot-cache compatibility forces low-level buffer handling; the preferred implementation is entirely safe Rust.

## Implementation Phases

## Phase 1: Establish core option types and setters

### Goals
Port the configuration state and mutation APIs first so the rest of the module can build on a stable Rust representation.

### Tasks
- Create `src/quotearg.rs`.
- Define:
  - `QuotingStyle`
  - `QuotingFlags`
  - `CustomQuoting`
  - `QuotingOptions`
- Implement:
  - `clone_quoting_options`
  - `get_quoting_style`
  - `set_quoting_style`
  - `set_char_quoting`
  - `set_quoting_flags`
  - `set_custom_quoting`
  - `quoting_options_from_style`
- Add default construction matching the C module’s default quoting behavior.
- Represent per-character quoting overrides with a fixed 256-entry table indexed by byte.

### Deliverables
- Compiling Rust data model and setter/getter API surface.
- Unit tests covering:
  - cloning preserves all fields;
  - style changes return/update the expected values;
  - char quoting updates only the targeted byte entry;
  - custom quoting replaces prior delimiters correctly.

## Phase 2: Port the core quoting engine and direct buffer API

### Goals
Implement the single quoting path that all allocation and convenience wrappers will use.

### Tasks
- Implement the internal quoting routine in byte-oriented form.
- Implement `quotearg_buffer` against a caller-provided mutable output slice and return the produced or required length according to the original C behavior.
- Encode style selection, flags, custom delimiters, and forced per-character quoting in this single path.
- Ensure truncation/length semantics are explicitly tested if the C API permits writing into bounded output memory.

### Deliverables
- Core quoting logic in `src/quotearg.rs`.
- Unit tests covering:
  - empty input;
  - ASCII input with no escaping;
  - bytes requiring quoting under style rules;
  - forced quoting for a selected character;
  - custom left/right delimiter output;
  - `_buffer` length behavior with exact-fit and too-small buffers.

## Phase 3: Add allocation wrappers and style-specific entry points

### Goals
Layer the remaining exported functions on top of the core engine without duplicating quoting logic.

### Tasks
- Implement:
  - `quotearg_alloc`
  - `quotearg_alloc_mem`
  - `quotearg_n_options`
  - `quotearg_n_style`
  - `quotearg_n_style_mem`
  - `quotearg_char_mem`
  - `quotearg_n_style_colon`
- For numbered (`quotearg_n_*`) functions, add only the minimum module-local slot storage required to preserve existing behavior; avoid broad caching infrastructure if current Rust call sites do not observe it.
- Route colon-specific behavior through normal style/options configuration rather than a separate quoting engine.

### Deliverables
- Complete exported function set for this module.
- Wrapper-focused unit tests verifying:
  - allocating APIs match buffer API output;
  - `_mem` variants preserve non-UTF-8 bytes correctly;
  - numbered wrappers produce stable output per slot if slot semantics are required;
  - colon variant applies the intended additional quoting rule.

## Phase 4: Integration cleanup and regression validation

### Goals
Finalize module integration into the branch and confirm the migration stays behaviorally aligned with the original C code paths used by `pwd`.

### Tasks
- Wire `src/quotearg.rs` into the existing crate module tree.
- Replace any temporary stubs or duplicate quoting logic in the branch with calls into this migrated module.
- Add regression tests derived from current `pwd` usage paths rather than new generalized scenarios.
- Review for:
  - unnecessary allocations in wrappers;
  - accidental UTF-8 assumptions in byte-oriented functions;
  - any remaining raw-pointer-style logic that can be simplified into safe ownership.

### Deliverables
- Integrated Rust module used by the main cluster.
- Passing `cargo test`.
- Final code limited to the migrated `quotearg.c` scope, without extra support facilities beyond what these functions require.