# Implementation Plan: main_root_version_etc_06

## Summary

This module ports the `version-etc.c` functionality into Rust for the `pwd` project branch `006-main_root_version_etc_06-rust-port`. The implementation should preserve the existing responsibility of formatting and emitting version and author information through a small set of entry-point functions corresponding to the C API shape.

The Rust approach should stay minimal and migration-focused:

- translate the existing formatting/output logic into a single Rust module;
- preserve the function layering represented by:
  - `version_etc_arn`
  - `version_etc_ar`
  - `version_etc_va`
  - `version_etc`
- use safe Rust string handling in place of C variadic and null-terminated string mechanics;
- route output through standard I/O writers where practical, while keeping call structure close to the original file;
- replace C memory and sentinel-based argument handling with borrowed slices and explicit iteration.

No extra capability should be introduced beyond what is required to migrate this file and its functions.

## Technical Context

### Language/Version

- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.74` or newer

### Primary Dependencies

Use the Rust standard library by default.

Recommended dependencies:

- None required for this module

Standard library components expected:

- `std::io::{self, Write}`
- `std::fmt`
- `std::slice`
- `std::option::Option`

### Testing

- `cargo test`

Test coverage should focus on:

- output content for each public function variant;
- handling of author lists with different lengths;
- equivalence of wrapper functions to the core formatting path;
- absence of panics for empty or minimal inputs allowed by the migrated interface.

### Performance Goals

This module is not performance-critical. Goals should remain narrow and practical:

- avoid unnecessary heap allocation where simple streaming or bounded formatting is sufficient;
- keep author-name traversal linear in the number of authors;
- preserve low overhead comparable to the original C implementation for short version/help text output.

## Module Mapping

### C to Rust File Mapping

- `version-etc.c` → `src/version_etc.rs`

If the crate already centralizes command-facing helpers in `src/main.rs` or `src/lib.rs`, expose only the migrated module from the existing root without creating additional abstraction layers.

### Function Mapping

The C functions should map to Rust functions with the same logical layering, while adapting signatures to Rust ownership and argument conventions.

- `version_etc_arn`
  - Rust: core function taking an output target, command/package/version strings, and an author collection as a slice
  - role: lowest-level shared implementation for formatted version/author emission

- `version_etc_ar`
  - Rust: thin wrapper over the core function using a direct author slice/reference form
  - role: preserve the intermediate API shape from C

- `version_etc_va`
  - Rust: wrapper replacing C variadic handling with an explicit slice-based interface in Rust
  - role: consolidate all variable-length author input into safe iteration

- `version_etc`
  - Rust: convenience wrapper forwarding to the shared implementation
  - role: preserve the top-level entry point expected by the translated call sites

### Expected Rust Visibility

Keep visibility restrained:

- mark only the functions needed by existing migrated callers as `pub` or `pub(crate)`;
- keep formatting helpers private to `src/version_etc.rs`.

## Data Model

This module does not define explicit C structs in the provided analysis, so the migration should remain function-oriented.

### C Data to Rust Mapping

- C `const char *`
  - Rust: `&str` for trusted internal string inputs
  - if call-site migration requires optional values, use `Option<&str>` only where the original C behavior depends on nullability

- C author list represented via arrays / variadic arguments / sentinel termination
  - Rust: `&[&str]`
  - rationale: removes null-sentinel scanning and makes bounds explicit

- C output stream usage
  - Rust: `&mut dyn std::io::Write` or a generic `W: Write`
  - choose one style and keep it consistent across the module, preferring a generic writer if it does not complicate call sites

### Memory Management Notes

- borrowed string slices replace raw pointers and eliminate manual lifetime tracking;
- no direct heap ownership model is needed unless an existing caller already constructs owned strings;
- formatting should avoid storing temporary duplicated author strings.

### Error Handling Notes

C output functions often encode failures through stream state or integer results. In Rust:

- internal writing operations should return `io::Result<()>` where possible;
- wrapper functions should either:
  - propagate `io::Result<()>`, if surrounding migrated code already accepts fallible output; or
  - remain narrow adapters that write to standard output/error and handle the result consistently with current project conventions.

The plan should follow the existing crate error style rather than introducing a new error type for this module alone.

## Implementation Phases

## Phase 1: Create the Rust module skeleton and core signatures

- Add `src/version_etc.rs`.
- Define Rust function signatures for:
  - `version_etc_arn`
  - `version_etc_ar`
  - `version_etc_va`
  - `version_etc`
- Select the single shared argument model for author lists (`&[&str]`).
- Select the writer strategy (`W: Write` or `&mut dyn Write`) based on the surrounding crate style.
- Export the module from the existing crate root only as needed by current migration targets.

### Deliverables

- compilable module with stubbed or initial implementations;
- final function signatures aligned to migrated caller needs;
- no additional support modules.

## Phase 2: Port formatting and output behavior from `version-etc.c`

- Translate the text emission logic into the Rust core function.
- Move all common formatting into `version_etc_arn`.
- Implement wrapper forwarding:
  - `version_etc_ar` → core
  - `version_etc_va` → core
  - `version_etc` → core or wrapper chain matching the original call flow
- Replace C-style null/sentinel handling with explicit slice iteration.
- Keep output ordering and newline behavior aligned with the source module.

### Deliverables

- complete Rust behavior for all four functions;
- safe handling of author lists without raw pointer traversal;
- explicit `io::Result<()>` or project-consistent equivalent for write failures.

## Phase 3: Integrate with existing call sites and remove C-specific assumptions

- Update migrated callers on this branch to use the Rust signatures.
- Replace any assumptions about variadic or null-terminated author input with slice construction at call sites.
- Ensure no C-style ownership or lifetime assumptions remain around strings passed into this module.
- Confirm module inclusion and visibility are sufficient but not broader than necessary.

### Deliverables

- working crate integration on branch `006-main_root_version_etc_06-rust-port`;
- no remaining dependency on C variadic behavior for this functionality.

## Phase 4: Add focused tests and finalize parity checks

- Add unit tests for representative output cases:
  - single author;
  - multiple authors;
  - empty or minimal author list if permitted by migrated interface;
  - wrapper equivalence to the core implementation.
- Use in-memory buffers (`Vec<u8>`) to verify exact emitted text.
- Confirm stable behavior for line breaks and author formatting.
- Run `cargo test` and resolve any output mismatches against the C behavior being ported.

### Deliverables

- module-level tests covering the four function paths;
- verified output parity for the migrated scenarios;
- final cleanup of private helpers and imports.