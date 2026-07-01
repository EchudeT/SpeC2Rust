# Implementation Plan

## Summary

Port the `quotearg.c` functionality into a single Rust module that preserves the existing quoting-option behavior and call patterns used by the `pwd` main cluster. The Rust implementation should focus on translating the current option/state model and the buffer-producing APIs without adding new quoting features or restructuring beyond what is needed for the migration.

The implementation approach is:

- migrate the quoting configuration state into Rust-owned data types;
- represent quoting style and flags with Rust enums/bitfields;
- implement the core quoting path once, then layer the public helper functions on top of it;
- replace C allocation/buffer ownership patterns with `String`/`Vec<u8>` and explicit return values;
- keep function grouping close to the source file so the migration remains easy to audit against `quotearg.c`.

## Technical Context

- **Language/Version**: Rust 1.74+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve linear-time processing over input bytes;
  - avoid unnecessary intermediate allocations where a caller-provided buffer variant exists;
  - keep cloning of quoting options explicit and small;
  - match the C module’s practical behavior closely enough for command-line use in `pwd` without introducing extra abstraction overhead.

## Module Mapping

- **C source**
  - `quotearg.c`

- **Rust target**
  - `src/quotearg.rs`

- **Migration scope by function**
  - `clone_quoting_options` -> `src/quotearg.rs`
  - `get_quoting_style` -> `src/quotearg.rs`
  - `set_quoting_style` -> `src/quotearg.rs`
  - `set_char_quoting` -> `src/quotearg.rs`
  - `set_quoting_flags` -> `src/quotearg.rs`
  - `set_custom_quoting` -> `src/quotearg.rs`
  - `quoting_options_from_style` -> `src/quotearg.rs`
  - `quotearg_buffer` -> `src/quotearg.rs`
  - `quotearg_alloc` -> `src/quotearg.rs`
  - `quotearg_alloc_mem` -> `src/quotearg.rs`
  - `quotearg_n_options` -> `src/quotearg.rs`
  - `quotearg_n_style` -> `src/quotearg.rs`
  - `quotearg_n_style_mem` -> `src/quotearg.rs`
  - `quotearg_char_mem` -> `src/quotearg.rs`
  - `quotearg_n_style_colon` -> `src/quotearg.rs`

- **Integration note**
  - expose only the module items needed by the Rust `pwd` main path;
  - keep the implementation in one file unless existing branch structure already requires a different standard Rust file split.

## Data Model

The C analysis lists anonymous data structures; for the Rust port, map them into named Rust types that match the observed responsibilities in `quotearg.c`.

| C construct | Rust mapping | Notes |
|---|---|---|
| anonymous quoting-options struct | `struct QuotingOptions` | Owns style, flags, per-character quoting table, and optional custom delimiters. |
| anonymous quoting-style constants | `enum QuotingStyle` | Rust enum for the style selector used by getters/setters and helper constructors. |
| anonymous quoting flags field | `struct QuotingFlags(u32)` or `type QuotingFlags = u32` | Prefer a small newtype if flags are manipulated in several places; otherwise keep as plain integer. No external crate required. |
| anonymous per-character quoting table | `[bool; 256]` or `[u8; 256]` | Use fixed-size array indexed by byte value to mirror C table semantics. `bool` is preferred unless exact bitwise carry-through is required. |
| anonymous custom quoting delimiter storage | `Option<CustomQuoting>` | Present only when custom left/right quotes are configured. |
| anonymous custom quoting data | `struct CustomQuoting { left: Vec<u8>, right: Vec<u8> }` | Store raw bytes to avoid premature UTF-8 assumptions. |
| anonymous output buffer usage | `Vec<u8>` / `String` | Use `Vec<u8>` in core quoting path, convert to `String` only where valid and intended by API shape. |
| anonymous slot-based temporary storage used by `quotearg_n_*` family | `Vec<Option<Vec<u8>>>` or small internal slot store | Keep module-local mutable storage only if required to preserve API behavior; otherwise collapse helpers onto owned-return functions. |

### Rust type decisions

- **Input representation**
  - accept `&[u8]` for byte-oriented functions corresponding to `_mem` variants;
  - add narrow string-facing wrappers only where needed by call sites.

- **Output representation**
  - for buffer-writing behavior, write into `&mut Vec<u8>` or return a `Vec<u8>`;
  - for allocation-style functions, return owned buffers directly instead of manual heap ownership.

- **Error handling**
  - avoid fallible custom error hierarchies unless an actual invalid configuration must be surfaced;
  - use `Result` only for cases such as invalid custom quoting setup if the C code distinguishes that path;
  - otherwise preserve C-style total functions with deterministic outputs.

- **Memory management**
  - cloning options becomes `Clone` on `QuotingOptions`;
  - no manual free logic;
  - custom quote strings are owned by Rust containers and dropped automatically.

## Implementation Phases

## Phase 1: Establish Rust types and option-state migration

### Goal
Translate the option/configuration layer from `quotearg.c` into Rust so all state-manipulation functions exist before the formatting path is ported.

### Work items
- Create `src/quotearg.rs`.
- Define:
  - `QuotingStyle`
  - `QuotingFlags`
  - `CustomQuoting`
  - `QuotingOptions`
- Implement `Default` and `Clone` where they match C behavior.
- Port these functions first:
  - `clone_quoting_options`
  - `get_quoting_style`
  - `set_quoting_style`
  - `set_char_quoting`
  - `set_quoting_flags`
  - `set_custom_quoting`
  - `quoting_options_from_style`

### Technical decisions
- Keep setter semantics close to C, including returning prior values where the original API does so.
- Represent the per-byte quoting map as a fixed 256-entry array for direct index parity with C.
- Store custom quote delimiters as owned byte vectors to support arbitrary byte content.

### Exit criteria
- All option/state functions compile and are covered by unit tests validating:
  - style round-tripping;
  - flag replacement behavior;
  - per-character quote toggling;
  - cloning independence;
  - custom delimiter installation.

## Phase 2: Port the core quoting engine and direct buffer/allocation APIs

### Goal
Implement the byte-processing logic once and use it to support the core public entry points.

### Work items
- Add one internal quoting routine that takes:
  - input bytes,
  - quoting options,
  - destination buffer.
- Port:
  - `quotearg_buffer`
  - `quotearg_alloc`
  - `quotearg_alloc_mem`

### Technical decisions
- Make the internal routine byte-oriented to match C behavior and avoid accidental UTF-8 dependence.
- Use `Vec<u8>` as the internal destination buffer.
- If a public function needs string output, convert only after quoting is complete and only if output is guaranteed valid for the selected quoting rules; otherwise keep byte-vector return types internal and adapt at the outer API according to actual branch usage.
- Preserve visible length semantics from C where the API expects output size alongside allocated content.

### Memory and correctness focus
- Avoid aliasing issues by separating source input from destination storage.
- Ensure embedded NUL and non-UTF-8 input are handled in `_mem` code paths without truncation.
- Keep escaping/quoting decisions centralized so helper variants do not diverge.

### Exit criteria
- Unit tests cover:
  - empty input;
  - ASCII input;
  - inputs requiring quoting/escaping;
  - embedded delimiter characters;
  - non-UTF-8 byte input;
  - consistency between buffer and alloc variants.

## Phase 3: Port the helper wrappers in migration order used by main-cluster callers

### Goal
Add the thin convenience functions that select styles/options around the core engine, keeping storage behavior minimal and aligned with current use.

### Work items
- Port:
  - `quotearg_n_options`
  - `quotearg_n_style`
  - `quotearg_n_style_mem`
  - `quotearg_char_mem`
  - `quotearg_n_style_colon`

### Technical decisions
- Implement these as wrappers over the Phase 2 core path rather than duplicating quoting logic.
- Only introduce module-local slot storage if existing translated callers require the exact indexed-slot behavior of `quotearg_n_options`.
- If slot retention is required, keep it private to `src/quotearg.rs` and limited to reproducing current behavior; do not generalize it into a broader cache or service.

### Error and state handling
- Keep any internal mutable slot storage narrowly scoped and deterministic.
- Preserve behavior when style-specific helper functions override one aspect of options, such as quoting a colon or selecting a named style.

### Exit criteria
- Wrapper functions compile and produce results consistent with direct core API use.
- Tests confirm:
  - style wrapper equivalence;
  - `_mem` wrapper handling of explicit lengths;
  - character-specific quoting behavior for colon-oriented helpers;
  - repeated calls do not corrupt prior results.

## Phase 4: Integrate with `pwd` main cluster and complete parity checks

### Goal
Wire the Rust module into the branch’s main-cluster code and verify the migration is complete for current consumers.

### Work items
- Update module declarations and imports for the `pwd` Rust port.
- Replace current call sites targeting the C behavior with the Rust `quotearg` module functions.
- Remove any temporary stubs introduced during migration.
- Add focused regression tests driven by `pwd`-relevant call patterns.

### Technical decisions
- Keep public visibility as narrow as possible outside the current main-cluster needs.
- Do not split the module further unless required by existing Rust project structure.
- Prefer exact behavioral checks at the function boundary over introducing new abstractions.

### Exit criteria
- `cargo test` passes.
- The Rust branch builds with the `quotearg` path fully sourced from `src/quotearg.rs`.
- All listed functions are migrated and no longer depend on the original C implementation.