# Implementation Plan

## Summary

This module ports the `quotearg.c` functionality used by `quotearg_style` and `quotearg_style_mem` into Rust for the `cat` project branch `014-main_root_quotearg_style_13-rust-port`.

The Rust implementation should preserve the existing behavior shape of style-driven quoting while limiting scope to the code paths required by these two functions. The migration should focus on translating the current C logic into a Rust module with explicit ownership, slice-based input handling, and string/byte-buffer construction via standard library types. The implementation should avoid introducing new abstraction layers beyond what is needed to represent the existing style selection and output generation flow.

The technical approach is:

- map the style selector used by the C code to a Rust enum;
- implement a byte-oriented quoting routine that accepts `&[u8]` for the `_mem` variant;
- provide a convenience wrapper for style-only quoting that forwards into the byte-oriented implementation;
- preserve behavior around non-UTF-8 input by operating on bytes internally and only converting to `String` at the boundary if the surrounding Rust port already requires string output;
- replace C global/static argument handling patterns with explicit function-local or module-local constants where required by the original logic.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - keep quoting work linear in input length;
  - avoid unnecessary intermediate allocations where possible;
  - use `String` or `Vec<u8>` with predictable growth instead of repeated small concatenations;
  - maintain behavior parity with the C implementation without adding heavyweight formatting or parsing layers.

## Module Mapping

| C File | C Function | Rust Target |
|---|---|---|
| `quotearg.c` | `quotearg_style` | `src/quotearg.rs::quotearg_style` |
| `quotearg.c` | `quotearg_style_mem` | `src/quotearg.rs::quotearg_style_mem` |

### Rust module placement

- Add or update `src/quotearg.rs` as the direct port target for the relevant logic from `quotearg.c`.
- Expose only the functions needed by the current migration scope.
- If the crate already has a central module declaration file, add a single `mod quotearg;` or equivalent export there, without creating extra helper modules unless already required by the existing project layout.

## Data Model

The analysis only identifies anonymous C data structures, so the Rust mapping should be driven by actual usage encountered in `quotearg_style` and `quotearg_style_mem` rather than by creating speculative full-module equivalents.

| C Representation | Rust Mapping | Notes |
|---|---|---|
| anonymous style-related integral constants / enum-like values | `enum QuotingStyle` | Preferred if the C code switches over known style values. Use `#[derive(Clone, Copy, Debug, PartialEq, Eq)]`. |
| anonymous option/config aggregates referenced by these functions | `struct QuotingOptions` | Only introduce if the selected functions depend on grouped quoting parameters. Keep fields limited to what is directly used. |
| anonymous static tables / character maps | `const` arrays or slices | Use fixed-size arrays or `&'static [u8]` / `&'static [char]` depending on actual C usage. |
| raw C string pointer + length pairs | `&[u8]` | Primary input form for `_mem` behavior. |
| C output buffer assembled incrementally | `String` or `Vec<u8>` | Prefer `String` if output is guaranteed text; prefer `Vec<u8>` internally if escaping operates on arbitrary bytes. |
| nullable pointers signaling defaults | `Option<T>` / explicit default constants | Replace null-based control flow with explicit options. |

### Data-structure decisions

- **Quoting style selector**: represent as a Rust enum if the C implementation uses symbolic style cases. If external compatibility requires numeric values internally, use `#[repr(i32)]` only when necessary.
- **Input memory handling**: `quotearg_style_mem` should accept a byte slice to preserve exact input semantics and avoid assuming UTF-8.
- **Output representation**: if the surrounding Rust port expects textual output from these functions, construct with `String` while escaping non-printable bytes explicitly. If exact byte preservation is needed during assembly, build into `Vec<u8>` and convert at the end only when valid and intended.
- **Error handling**: these functions are expected to be deterministic transformations rather than fallible I/O. Prefer infallible APIs returning the quoted value directly. Use internal assertions only for impossible states introduced by the port, not for input validation beyond the C behavior.

## Implementation Phases

## Phase 1: Establish module skeleton and style mapping

- Create or update `src/quotearg.rs`.
- Port the style discriminator required by `quotearg_style` and `quotearg_style_mem` into a Rust enum or equivalent constant set.
- Identify the minimal set of constants, tables, and option values from `quotearg.c` that these two functions read directly.
- Translate any file-scope static configuration used by these functions into Rust `const` items or narrowly scoped statics.
- Wire the module into the crate without adding unrelated exports.

### Deliverables
- Compiling Rust module skeleton.
- Rust representation for style values and any directly used quoting configuration.

## Phase 2: Port core quoting logic for memory-based input

- Implement `quotearg_style_mem` as the primary logic path.
- Translate the C byte-walking and escaping logic directly into Rust iteration over `&[u8]`.
- Preserve branch structure from the C implementation where practical so behavior remains reviewable against the source.
- Replace pointer arithmetic with indexed or iterator-based access.
- Replace manual buffer management with `String::with_capacity` or `Vec::with_capacity` based on the observed output construction pattern.
- Ensure non-UTF-8 input is handled without lossy pre-conversion.

### Deliverables
- Working Rust implementation of `quotearg_style_mem`.
- Unit tests covering:
  - empty input;
  - plain ASCII input;
  - bytes requiring quoting/escaping under each used style;
  - embedded non-UTF-8 bytes if applicable to the original behavior.

## Phase 3: Port style-only wrapper and align interfaces

- Implement `quotearg_style` as a thin wrapper over `quotearg_style_mem`.
- Match the C call relationship and defaults rather than introducing new convenience APIs.
- Normalize any shared setup so both functions use the same Rust core path.
- Confirm return types and ownership are idiomatic for the existing Rust crate while preserving the original call semantics as closely as possible.

### Deliverables
- Working Rust implementation of `quotearg_style`.
- Reduced duplication between wrapper and core implementation.
- Tests verifying wrapper-to-core parity.

## Phase 4: Behavioral verification and cleanup

- Compare the Rust control flow and output cases against the original C implementation for the migrated functions only.
- Remove unused provisional structures or constants introduced during translation.
- Tighten allocation choices if obvious excess copies remain.
- Finalize tests around edge cases directly visible in the C logic instead of adding new functionality.

### Deliverables
- Clean, scoped Rust port of the targeted functions.
- `cargo test` passing for the migrated module.
- Final code review checklist confirming:
  - no raw pointer lifetime emulation remains;
  - no unnecessary module expansion was introduced;
  - memory ownership and output construction are explicit and safe.