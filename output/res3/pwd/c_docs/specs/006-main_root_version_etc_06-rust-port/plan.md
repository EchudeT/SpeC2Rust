# Implementation Plan

## Summary

Port the functionality from `version-etc.c` into a single Rust module that preserves the existing behavior and call shape of the C implementation as closely as is practical in idiomatic Rust. The scope of this module is limited to formatting and emitting version and author information through the four existing entry points:

- `version_etc_arn`
- `version_etc_ar`
- `version_etc_va`
- `version_etc`

The Rust implementation should focus on migrating the existing formatting and argument-handling logic without introducing new abstractions beyond what is needed to represent the current behavior safely. The preferred technical approach is to implement these functions in a Rust source file under the main crate, using standard library formatting facilities and borrowed string slices where possible. Variadic behavior from C should be migrated by consolidating internal logic around slice-based author lists, with thin Rust-facing wrappers that reflect the existing function family.

Special attention should be given to:
- preserving output ordering and formatting,
- handling optional or empty author lists correctly,
- avoiding unnecessary allocations when writing formatted output,
- replacing C varargs and pointer-based string handling with safe Rust references,
- expressing write failures using Rust `Result` values internally, while matching the surrounding crate’s error-reporting conventions at integration points.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only (`std::io`, `std::fmt`)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s practical runtime characteristics for short formatted output.
  - Avoid repeated heap allocation where direct streaming to a writer is sufficient.
  - Keep author-list processing linear in the number of authors.
  - Do not introduce extra buffering beyond what is needed by the chosen output API.

## Module Mapping

### C to Rust File Mapping

- `version-etc.c` → `src/version_etc.rs`

### Function Mapping

The Rust module should retain a close correspondence to the C entry points, while using Rust-safe argument forms:

- `version_etc_arn`
  → `pub(crate) fn version_etc_arn<W: std::io::Write>(...) -> std::io::Result<()>`
- `version_etc_ar`
  → `pub(crate) fn version_etc_ar<W: std::io::Write>(...) -> std::io::Result<()>`
- `version_etc_va`
  → `pub(crate) fn version_etc_va<W: std::io::Write>(...) -> std::io::Result<()>`
- `version_etc`
  → `pub(crate) fn version_etc<W: std::io::Write>(...) -> std::io::Result<()>`

### Internal Mapping Strategy

Because C variadic entry points do not map directly to safe Rust:
- migrate the shared formatting logic into one internal helper that accepts:
  - target writer,
  - command/package identifiers,
  - version string,
  - author list as a slice,
  - any count information required by the C behavior.
- implement the public module functions as thin adapters over that helper.
- if the surrounding crate expects direct emission to stdout/stderr rather than generic writers, keep the generic writer helper internal and provide file-handle-specific wrappers at the module boundary.

This preserves the original function family while ensuring the actual implementation is centralized and testable.

## Data Model

No explicit C structs are listed for this module. The migration is therefore primarily a mapping of C string and argument conventions into Rust types.

### C Type to Rust Type Mapping

- `const char *`
  → `&str` for validated Rust text passed from internal callers
  → possibly `&std::ffi::CStr` only if required by existing adjacent ported code, otherwise avoid
- `const char **` / author array
  → `&[&str]`
- `size_t` / author count
  → `usize`
- `FILE *` destination
  → generic `W: std::io::Write` or concrete standard output/error handle, depending on existing crate integration
- C varargs (`...`, `va_list`)
  → slice-based parameters in the core helper; wrapper functions should normalize inputs into slices rather than emulate unsafe varargs

### Rust Data Structures

A dedicated struct is not required unless needed to reduce repetition during implementation. Prefer direct parameter passing. If grouping becomes necessary during migration, keep it minimal:

```rust
pub(crate) struct VersionEtcArgs<'a> {
    pub command_name: &'a str,
    pub package_name: &'a str,
    pub version: &'a str,
    pub authors: &'a [&'a str],
}
```

This should only be introduced if it simplifies shared formatting logic across the four migrated functions; otherwise, avoid adding it.

## Implementation Phases

### Phase 1: Create module skeleton and migrate shared formatting path

- Add `src/version_etc.rs`.
- Identify the exact output responsibilities currently implemented in `version-etc.c`.
- Implement a single internal helper that performs:
  - header/version line emission,
  - copyright/license-related text emission if present in the source,
  - author-list formatting,
  - newline placement and spacing exactly as needed.
- Use `std::io::Write` and return `std::io::Result<()>` from the helper.
- Keep allocations minimal by writing directly to the target stream.

### Phase 2: Port the four function entry points

- Implement Rust equivalents for:
  - `version_etc_arn`
  - `version_etc_ar`
  - `version_etc_va`
  - `version_etc`
- Normalize all paths to the shared helper.
- Replace C pointer/count handling with:
  - `&[&str]` for array-based inputs,
  - explicit `usize` where count semantics must be preserved.
- For the C variadic forms, do not reproduce unsafe C-style varargs; instead, expose crate-internal Rust functions with explicit slice arguments and preserve the original role separation among the four functions.
- Ensure empty, single-author, and multi-author cases are handled consistently with the C logic.

### Phase 3: Integrate with the main crate and align error handling

- Export the module only as needed by the existing main-cluster code.
- Update the relevant `mod` declarations and call sites to use the new Rust functions.
- Ensure write failures are either propagated as `Result` or converted at the call boundary to match the crate’s existing error handling approach.
- Confirm there are no lifetime or ownership issues from borrowed string parameters.
- Remove any temporary duplication once integration is complete.

### Phase 4: Add focused tests for formatting and edge cases

- Add unit tests in `src/version_etc.rs` or adjacent test modules using in-memory writers.
- Cover:
  - no authors,
  - one author,
  - two authors,
  - several authors,
  - exact line breaks and separators,
  - count-limited author display if `version_etc_arn` semantics require it.
- Validate that wrapper functions all converge on identical output for equivalent inputs.
- Run `cargo test` and fix any formatting mismatches against expected output captured from the C behavior.