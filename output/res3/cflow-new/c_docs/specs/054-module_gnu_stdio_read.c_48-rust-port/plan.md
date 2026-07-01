# Implementation Plan: module_gnu_stdio-read.c_48

## Summary

This module ports the input-oriented behavior from `gnu/stdio-read.c` into Rust, covering the existing function surface: `scanf`, `fscanf`, `vscanf`, `vfscanf`, `getchar`, `fgetc`, `fgets`, and `fread`.

The Rust implementation should stay narrowly aligned with the current C module scope and migrate behavior file-by-file and function-by-function rather than introducing new abstractions. The technical approach is to map C stdio-style reading operations onto Rust’s standard I/O traits and types, primarily `std::io::Read`, `std::io::BufRead`, and `std::fs::File` where applicable. Where the original API depends on variadic formatted input (`scanf` family), the Rust port should isolate parsing logic into internal helper functions driven by explicit format-string processing, avoiding expansion into broader formatting frameworks.

The implementation should favor:
- standard-library I/O primitives,
- explicit buffer-boundary handling,
- clear translation of C EOF and error conditions into Rust `Result`-based behavior internally,
- minimal compatibility wrappers where C semantics require distinction between successful reads, EOF, and parse failure.

## Technical Context

### Language / Version
- Rust stable
- Recommended minimum version: **Rust 1.75+**

### Primary Dependencies
- **Rust standard library** only:
  - `std::io::{self, Read, BufRead, Stdin, StdinLock}`
  - `std::fs::File`
  - `std::str`
  - `std::fmt` only if needed for internal formatting-related helpers

No third-party crates are recommended because the input does not justify external parsing or I/O dependencies.

### Testing
- `cargo test`

Testing should cover:
- byte-wise reads,
- line-oriented reads,
- bounded buffer reads,
- EOF handling,
- partial-read behavior,
- formatted scanning behavior for supported conversions and whitespace handling.

### Performance Goals
- Preserve streaming behavior without unnecessary full-input buffering.
- Avoid extra allocations for `fgetc`, `getchar`, and `fread` paths.
- Keep `fgets`-style logic bounded to the caller-provided limit.
- For formatted input, parse directly from buffered input where feasible rather than copying entire streams.
- Match C-like partial-read semantics for bulk reads so successful partial progress is not converted into hard failure.

## Module Mapping

### Source Mapping
- C source: `gnu/stdio-read.c`
- Rust target: `src/module_gnu_stdio_read_c_48.rs`

If the project already uses per-module layout, keep this module as a single Rust source file mirroring the original C file rather than splitting into submodules.

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `scanf` | `pub fn scanf(...)` or internal equivalent exposed at module boundary | Implement as stdin-based formatted reader delegating to shared scanning logic. |
| `fscanf` | `pub fn fscanf<R: BufRead>(reader: &mut R, ...)` or concrete file-reader equivalent | Main formatted-input entry point over a supplied reader. |
| `vscanf` | `fn vscanf_internal(...)` | Replace C varargs handling with internal parsed-argument dispatch appropriate to Rust call sites. |
| `vfscanf` | `fn vfscanf_internal(...)` | Core scanning engine shared by `scanf`/`fscanf`. |
| `getchar` | `pub fn getchar(...)` | Read one byte/character from stdin with EOF distinction. |
| `fgetc` | `pub fn fgetc<R: Read>(reader: &mut R)` | Single-byte read wrapper over `Read`. |
| `fgets` | `pub fn fgets<R: BufRead>(reader: &mut R, buf: &mut [u8])` | Line/bounded read preserving C-style stop conditions. |
| `fread` | `pub fn fread<R: Read>(reader: &mut R, buf: &mut [u8], size: usize, count: usize)` | Bulk read with item-count semantics and partial-read handling. |

### Behavioral Mapping Notes
- C integer return codes such as EOF or item counts should be represented internally with Rust enums or `Result` plus explicit count values, then adapted at the module API boundary.
- Variadic C interfaces cannot be ported directly; preserve functionality by moving the scanning engine into typed/internal helpers used by whatever Rust-side call pattern the project already expects.
- `FILE *`-based behavior maps to borrowed reader types; ownership should remain with the caller.

## Data Model

This module has no declared C structs in the input.

### Data Structure Mapping

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| `FILE *` usage | `&mut impl Read` / `&mut impl BufRead` / `&mut File` | Use trait-based borrowing instead of raw handles. |
| C input buffer pointers | `&mut [u8]` / `&mut String` only where required | Prefer byte slices for close behavioral alignment. |
| C EOF / error integer conventions | Internal `Result<_, io::Error>` plus explicit status/count types | Preserve EOF-vs-error distinction at API boundary. |

### Recommended Internal Types
Use only small internal helper types if needed to preserve C semantics cleanly:

- `enum ReadStatus { Eof, Bytes(usize) }`
  - For helpers that must distinguish no-data EOF from successful reads.

- `enum ScanStatus`
  - Variants for matched item count, input failure, or format failure.
  - Keeps `vfscanf`-style behavior explicit without relying on ad hoc integer signaling.

These should remain module-private unless the surrounding project API requires exposure.

## Implementation Phases

### Phase 1: Establish raw read operations
Implement the direct, non-formatted input functions first in the target Rust module:
- `fgetc`
- `getchar`
- `fgets`
- `fread`

Key work:
- Map single-byte reads using `Read::read` into a one-byte buffer.
- Implement bounded line reading for `fgets` with correct newline stop behavior and null-termination equivalent strategy if the surrounding API uses byte buffers.
- Implement `fread` with correct item-size and item-count semantics, especially partial completion and EOF.
- Define internal status/error translation helpers so C-style return expectations are not duplicated per function.

Exit criteria:
- Unit tests validate EOF, empty input, newline retention for `fgets`, and partial reads for `fread`.

### Phase 2: Build shared formatted-input engine
Implement the core scanning logic corresponding to `vfscanf` and `vscanf` as internal helpers.

Key work:
- Parse format strings incrementally.
- Support whitespace consumption behavior matching C scanning rules.
- Implement only the conversion handling required by the existing C module behavior discovered during migration; do not generalize beyond that scope.
- Route all reader interaction through `BufRead` so token extraction and lookahead remain controlled.
- Represent match count, input failure, and conversion failure explicitly.

Memory and error handling focus:
- Avoid unchecked indexing while walking format strings and input buffers.
- Keep parsing borrowed from buffered input where possible.
- Distinguish “nothing matched because of EOF/input exhaustion” from “format token mismatch after prior matches.”

Exit criteria:
- Internal tests cover whitespace skipping, token boundaries, matching counts, and EOF during conversion.

### Phase 3: Expose `scanf`/`fscanf` frontends
Wire public-facing scanning functions onto the shared engine:
- `fscanf`
- `scanf`
- `vscanf`
- `vfscanf` as Rust-internal compatibility layer equivalents

Key work:
- Connect `scanf` to `stdin().lock()`.
- Connect `fscanf` to caller-provided readers.
- Replace C varargs entry points with Rust-callable wrappers consistent with the surrounding codebase’s expectations.
- Keep the frontend layer thin so behavior remains centralized in the shared scanner.

Exit criteria:
- Tests demonstrate equivalent behavior between stdin-backed and reader-backed scanning paths.
- Frontends do not duplicate parsing or error logic.

### Phase 4: Integration cleanup and conformance pass
Finalize migration alignment with the original C file.

Key work:
- Review each migrated function against original return-value semantics.
- Normalize module visibility and naming to the project’s Rust conventions without expanding scope.
- Remove temporary duplication introduced during earlier phases.
- Add focused regression tests for edge cases found during direct comparison with the C implementation.

Exit criteria:
- All listed functions are migrated in the single target module.
- `cargo test` passes.
- The Rust module remains limited to the original file’s responsibilities.