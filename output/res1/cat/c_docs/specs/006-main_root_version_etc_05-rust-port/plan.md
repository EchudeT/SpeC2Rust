# Implementation Plan: main_root_version_etc_05

## Summary

This module ports the logic from `version-etc.c` into Rust with a narrow scope: preserve the existing version-reporting behavior and function boundaries as closely as Rust permits, without introducing new formatting features or restructuring unrelated code.

The Rust implementation should concentrate on:
- migrating the `version_etc`, `version_etc_va`, `version_etc_ar`, and `version_etc_arn` function family,
- preserving output order and formatting semantics,
- translating C varargs- and array-based entry points into safe Rust interfaces centered on slices and generic writer targets,
- using standard-library I/O and formatting facilities for output generation.

The preferred technical approach is to implement one internal formatting/output routine and keep the public Rust-facing functions as thin adapters corresponding to the C entry points. This reduces duplication while staying close to the original file and function layout.

## Technical Context

### Language/Version

- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.74+`

### Primary Dependencies

- Rust standard library only:
  - `std::io::{self, Write}` for output targets
  - `std::fmt` for message construction where needed
  - `std::slice` and `Option` for argument handling

No third-party crates are recommended because the input shows a small formatting/output migration with no explicit external dependency need.

### Testing

- `cargo test`

Testing should cover:
- output content for each migrated function path,
- empty, single-author, and multiple-author cases,
- termination behavior equivalent to the original array-based conventions after adaptation to Rust slices,
- consistency between the thin adapter functions and the shared internal implementation.

### Performance Goals

- Match the C module’s practical behavior for short version-reporting output.
- Avoid unnecessary intermediate allocations where straightforward use of `Write` permits direct emission.
- Keep per-call overhead minimal and linear in the number of author strings.
- Prioritize correctness and formatting compatibility over micro-optimization.

## Module Mapping

### C to Rust File Mapping

- `version-etc.c` -> `src/version_etc.rs`

If the existing Rust crate already centralizes module declarations elsewhere, expose only this migrated file through the current crate root without creating extra abstraction layers.

### Function Mapping

- `version_etc_arn` -> `version_etc_arn`
- `version_etc_ar` -> `version_etc_ar`
- `version_etc_va` -> `version_etc_va`
- `version_etc` -> `version_etc`

### Rust Function Shape Guidance

Because C varargs do not map directly into safe Rust, preserve naming while adapting signatures to idiomatic Rust:

- Array-oriented C functions should map to slice-based Rust functions.
- The varargs-oriented entry points should become Rust adapters over:
  - slices of author names, or
  - a small macro/front-end only if required by surrounding code already being ported.

Preferred implementation shape:
- one internal helper that accepts:
  - a mutable writer target,
  - command/package/version text,
  - a slice of author names,
- thin wrapper functions preserving the original family names as closely as feasible inside Rust.

### Output Target Mapping

C likely writes through `FILE *`; Rust should use:
- `&mut dyn std::io::Write` or a generic `W: Write`

This keeps memory ownership explicit and avoids unsafe pointer-based stream handling.

## Data Model

This module does not define persistent C structs in the provided input.

### Data-Structure Mapping

- C string inputs (`const char *`) -> Rust borrowed string references, preferably `&str`
- C string arrays (`const char **`) -> Rust slices such as `&[&str]`
- C stream handle (`FILE *`) -> Rust writer reference `&mut W` where `W: Write`
- C varargs (`...`, `va_list`) -> Rust slice-based adapters or internal helper parameters

### Ownership and Lifetime Notes

- Input text values should be borrowed where possible; this module only formats and emits output and should not take ownership unless required by existing call sites.
- Author lists should be passed as borrowed slices.
- The output writer should be borrowed mutably for the duration of the call only.
- Errors from output operations should be surfaced as `io::Result<()>` internally; if crate conventions require infallible outer functions, wrappers may translate in a minimal, localized manner.

## Implementation Phases

## Phase 1: Establish Rust module skeleton and shared output path

### Goals
- Create the Rust file corresponding to `version-etc.c`.
- Identify the exact formatting responsibilities of the original functions.
- Implement a single shared internal routine for version text emission.

### Tasks
- Add `src/version_etc.rs`.
- Port the core formatting logic into one internal helper function.
- Model all string inputs as borrowed `&str`.
- Model the author list as `&[&str]`.
- Use `std::io::Write` as the output abstraction.
- Return `io::Result<()>` from the internal helper to reflect write failures explicitly.

### Notes
- Keep the helper private.
- Do not introduce additional modules, traits, or utility layers beyond what is required to mirror `version-etc.c`.

## Phase 2: Port the function family with restrained Rust signature adaptation

### Goals
- Migrate the four named entry points from the C file.
- Keep naming aligned with the original functions.
- Minimize logic duplication by routing all entry points through the shared helper.

### Tasks
- Implement `version_etc_arn` as the direct slice-based path.
- Implement `version_etc_ar` as a thin wrapper over the same shared logic, reflecting its original role.
- Implement `version_etc_va` by replacing C `va_list` behavior with a Rust-compatible adapter based on pre-collected author slices.
- Implement `version_etc` as the highest-level wrapper expected by the surrounding crate code.

### Error Handling Decisions
- Prefer `io::Result<()>` for functions that perform writes.
- If surrounding migrated code requires signatures closer to C-style void functions, contain any required `.expect`/panic-free handling at the outermost compatibility edge only after project-level confirmation.
- Avoid unsafe code for varargs emulation.

## Phase 3: Validate formatting compatibility and edge cases

### Goals
- Confirm that Rust output matches the C behavior for representative inputs.
- Verify wrapper consistency across all four migrated functions.

### Tasks
- Add unit tests for:
  - zero authors,
  - one author,
  - two authors,
  - several authors,
  - stable punctuation and line breaks,
  - identical output across wrappers when given equivalent inputs.
- Use in-memory buffers such as `Vec<u8>` in tests to inspect emitted text.
- Compare exact strings wherever formatting is deterministic.

### Notes
- Keep tests focused on migrated behavior from `version-etc.c`.
- Do not add snapshot tooling or external test dependencies unless already present elsewhere in the project.

## Phase 4: Integrate with crate module declarations and complete migration cleanup

### Goals
- Wire the new Rust file into the existing crate structure.
- Remove any temporary porting scaffolding left from earlier phases.

### Tasks
- Export the module through the current crate root or existing parent module structure.
- Ensure call sites in this branch use the Rust entry points corresponding to the original C functions.
- Recheck that no unnecessary allocations, cloned strings, or redundant wrappers remain.
- Run `cargo test` and resolve any signature mismatches caused by surrounding module expectations.

### Completion Criteria
- `version-etc.c` functionality is represented in `src/version_etc.rs`.
- All four named functions are migrated and callable in Rust form.
- Tests pass with `cargo test`.
- The implementation remains limited to the original module scope without added facilities.