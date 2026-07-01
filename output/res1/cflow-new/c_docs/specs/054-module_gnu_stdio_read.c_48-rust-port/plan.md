# Implementation Plan

## Summary

Port `gnu/stdio-read.c` into a single Rust module that preserves the existing read-oriented stdio surface represented by the analyzed functions: `scanf`, `fscanf`, `vscanf`, `vfscanf`, `getchar`, `fgetc`, `fgets`, and `fread`.

The Rust implementation should focus on migrating current behavior into idiomatic but restrained Rust code, using `std::io` primitives as the base for stream reads and buffering. Since the source module groups formatted input and byte/line-oriented input, the Rust port should keep these concerns together in one module rather than splitting them into additional abstractions. The implementation approach is:

- map character, line, and block reads onto `Read`, `BufRead`, and standard input handles from `std::io`;
- represent C-style return conventions explicitly with Rust result and count types, then adapt them to the module’s exported API;
- isolate any variadic/formatted scanning compatibility logic behind narrow internal helpers, keeping migration aligned to the original function set only;
- preserve EOF/error distinctions through explicit handling instead of implicit panics;
- avoid ownership ambiguity by using borrowed readers and mutable output buffers where the C code operated on caller-provided storage.

## Technical Context

### Language / Version

- Rust 1.78 or newer
- Edition: 2021

### Primary Dependencies

Use the Rust standard library by default:

- `std::io::{self, Read, BufRead, Stdin, StdinLock}`
- `std::fs::File` where file-backed stream tests are needed
- `std::string::String`
- `std::vec::Vec`

Third-party crates are not required by the analyzed input and should not be introduced unless a later porting constraint proves standard library support insufficient.

### Testing

- `cargo test`

Test scope should cover:
- single-character reads
- line reads with newline retention/truncation behavior as required by the migrated API
- block reads and partial reads
- EOF handling
- formatted scan behavior that can be directly supported by the Rust implementation plan for this module
- error propagation from invalid streams or closed inputs where representable in tests

### Performance Goals

- Match C module intent for sequential input workloads without unnecessary allocation beyond caller-visible buffers.
- Use buffered reading paths for repeated character and line reads.
- Avoid copying input data more than once for `fread`-style operations.
- Keep formatted input parsing limited to the current function set and avoid generalized parser frameworks.

## Module Mapping

### Source to Target

- C source: `gnu/stdio-read.c`
- Rust target: `src/module_gnu_stdio_read_c_48.rs`

If the project uses a central module registry, expose it from:
- `src/lib.rs` with `pub mod module_gnu_stdio_read_c_48;`

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `scanf` | `pub fn scanf(...) -> ...` | Implement as stdin-based formatted read entry point using internal shared scan logic. |
| `fscanf` | `pub fn fscanf<R: BufRead>(reader: &mut R, ...) -> ...` | File/stream-based formatted read over borrowed reader. |
| `vscanf` | `fn vscanf_internal(...) -> ...` or equivalent internal helper | Since C variadics do not map directly, implement only as an internal Rust helper backing `scanf`. |
| `vfscanf` | `fn vfscanf_internal<R: BufRead>(...) -> ...` | Internal shared formatted scan routine; centralizes parsing. |
| `getchar` | `pub fn getchar() -> io::Result<Option<u8>>` or project-equivalent return type | Read one byte from stdin, distinguish EOF. |
| `fgetc` | `pub fn fgetc<R: Read>(reader: &mut R) -> io::Result<Option<u8>>` | Single-byte read from stream. |
| `fgets` | `pub fn fgets<R: BufRead>(reader: &mut R, buf: &mut String, limit: usize) -> io::Result<Option<usize>>` | Preserve bounded line-read semantics as closely as practical. |
| `fread` | `pub fn fread<R: Read>(reader: &mut R, buf: &mut [u8]) -> io::Result<usize>` | Return number of bytes/items read according to selected API convention. |

### Internal Helper Mapping

Keep helpers local to the module and limited to direct migration needs:

- single-byte read helper for `getchar`/`fgetc`
- bounded line read helper for `fgets`
- shared formatted scan parser/helper for `scanf`/`fscanf` and their internal `v*` equivalents

Do not create extra submodules or generic parsing frameworks beyond these helpers.

## Data Model

No explicit C structs were identified in the analyzed module. The port therefore centers on function and stream-type mapping rather than structure migration.

### Type Mapping

| C Concept | Rust Mapping | Notes |
|---|---|---|
| `FILE *` | `&mut dyn Read` / `&mut dyn BufRead` / generic `R: Read + BufRead` where needed | Use borrowed stream references; choose `BufRead` for line/formatted operations. |
| `char *` output buffer | `&mut [u8]` or `&mut String` | Use byte slices for raw reads, `String` for text line reads when UTF-8 semantics are acceptable; if byte-exact behavior is needed, prefer `&mut [u8]`. |
| return `int` for char read | `io::Result<Option<u8>>` | `None` represents EOF, `Err` represents read failure. |
| read count `size_t` | `usize` | Native Rust count type. |
| variadic argument list | internal typed helper parameters | No direct Rust variadic equivalent for this migration. |

### Memory Management

- Use caller-owned mutable buffers for `fread` and bounded `fgets` behavior.
- Avoid heap allocation in `fgetc`/`getchar`.
- For line reads, reuse the provided `String` or byte buffer rather than constructing additional owned storage.
- Borrow reader handles mutably for the duration of each operation; no retained references beyond the call.

### Error Handling

- Replace C sentinel/error returns with `io::Result<_>` internally.
- Where the exported API must mimic C-style EOF/count semantics, convert from `Result` at the outer boundary in a small adapter layer.
- Distinguish:
  - successful read with data,
  - EOF with no data,
  - I/O error.
- Do not use `unwrap`/`expect` in implementation code.

## Implementation Phases

## Phase 1: Establish module skeleton and byte/character read migration

### Goals

Create the Rust module file and migrate the simplest unformatted read operations first so stream handling and EOF/error conventions are settled early.

### Tasks

- Create `src/module_gnu_stdio_read_c_48.rs`.
- Export the module from `src/lib.rs` if not already wired.
- Implement shared single-byte read helper using `Read::read`.
- Implement `fgetc` on top of the helper.
- Implement `getchar` using locked stdin and the same helper.
- Implement `fread` using direct reads into caller-provided byte slices.
- Define the module’s concrete return conventions for EOF and counts.

### Validation

- Unit tests for:
  - reading one byte from a cursor-backed stream,
  - EOF on empty input,
  - partial `fread`,
  - repeated `getchar`/`fgetc` style reads.

## Phase 2: Migrate bounded line input behavior

### Goals

Implement the line-oriented path corresponding to `fgets`, keeping size-limited behavior and EOF distinctions compatible with the original usage pattern.

### Tasks

- Add a bounded line-read helper using `BufRead`.
- Implement `fgets` with explicit limit handling.
- Preserve newline inclusion behavior where present in input and within limit.
- Ensure no out-of-bounds writes or implicit buffer growth beyond the requested bound semantics.
- Choose either `String`-based or byte-buffer-based API according to the closest required behavior; if exact C byte semantics are needed, prefer byte-oriented buffering.

### Validation

- Unit tests for:
  - normal line read ending in newline,
  - line longer than limit,
  - final line without newline,
  - EOF before any data,
  - consecutive `fgets` calls over the same buffered reader.

## Phase 3: Migrate shared formatted input core

### Goals

Implement the internal formatted scanning routine that will back the four scan entry points without introducing unsupported generalized parsing infrastructure.

### Tasks

- Analyze the existing C formatting cases actually required by `stdio-read.c` and mirror only that supported subset in Rust.
- Implement a restrained internal parser for format-string-driven token extraction.
- Add internal shared helper corresponding to `vfscanf`.
- Add stdin wrapper helper corresponding to `vscanf`.
- Keep parsing state local and borrowed; do not build persistent parser objects unless needed by the current file.

### Validation

- Unit tests for supported format tokens and token separation behavior.
- Tests for EOF before assignment and partial assignment cases.
- Tests that invalid format/input combinations surface the expected failure/count behavior.

## Phase 4: Expose `scanf`/`fscanf` entry points and finalize compatibility

### Goals

Complete the public API surface and align exported behavior with the C module’s expected call and return conventions.

### Tasks

- Implement `fscanf` as a thin wrapper over the shared formatted helper.
- Implement `scanf` as a thin wrapper over stdin plus the same helper.
- Add small outer adapters if project-level API requires C-like integer return values rather than direct `Result` exposure.
- Review all functions for consistent EOF/error/count handling.
- Remove duplicated logic so all read paths go through the minimal helper set established in earlier phases.

### Validation

- End-to-end tests covering each exported function.
- Cross-check return values for success, EOF, and error paths.
- Run `cargo test` and fix any API inconsistencies within the single module scope only.