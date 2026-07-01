# Implementation Plan: module_gnu_stdio-read.c_48

## Summary

Port `gnu/stdio-read.c` into a Rust module that preserves the existing read-oriented stdio surface represented by:

- `scanf`
- `fscanf`
- `vscanf`
- `vfscanf`
- `getchar`
- `fgetc`
- `fgets`
- `fread`

The Rust implementation should focus on migrating the existing file and function responsibilities only, without adding new abstraction layers beyond what is needed to express the C behavior safely. The main technical approach is:

- use `std::io::Read`, `BufRead`, and standard input handles for byte- and line-oriented reads,
- model C `FILE *` usage with a narrow Rust wrapper over reader types needed by the migrated functions,
- preserve C-like return conventions where required by the surrounding port, while internally using `Result` and checked buffer handling,
- implement formatted input entry points in a restrained way by sharing a single internal scanning path rather than duplicating logic across `scanf`/`fscanf`/`vscanf`/`vfscanf`.

The migration should keep behavior centered on input consumption, EOF/error distinction, and bounded buffer writes.

## Technical Context

- **Language/Version:** Rust 1.78+
- **Primary Dependencies:** Rust standard library only (`std::io`, `std::fmt`, `std::str`, `std::mem` as needed)
- **Testing:** `cargo test`
- **Performance Goals:**
  - no unnecessary heap allocation for single-byte and fixed-buffer reads,
  - reuse buffered reading for line and block input paths,
  - keep `fread` throughput close to direct `Read::read` behavior,
  - avoid duplicate parsing passes for the formatted input family.

## Module Mapping

### Source Mapping

- **C source:** `gnu/stdio-read.c`
- **Rust target:** `src/module_gnu_stdio_read_c_48.rs`

If the project already uses a module tree matching source clusters, this file should be exposed directly from the existing crate root or parent module without introducing extra submodules.

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `scanf` | `pub fn scanf(...) -> i32` | Delegates to shared formatted-input logic using `stdin`. |
| `fscanf` | `pub fn fscanf(reader: &mut CFileLike, ...) -> i32` | Same scanning path, explicit input source. |
| `vscanf` | `pub fn vscanf(...) -> i32` | Implemented through the same internal scanner entry point as `scanf`. |
| `vfscanf` | `pub fn vfscanf(reader: &mut CFileLike, ...) -> i32` | Core formatted-input implementation target for the family. |
| `getchar` | `pub fn getchar() -> i32` | Reads one byte from stdin; returns C-style int/EOF sentinel as required by the port. |
| `fgetc` | `pub fn fgetc(reader: &mut CFileLike) -> i32` | Single-byte read from file-like source. |
| `fgets` | `pub fn fgets(buf: &mut [u8], reader: &mut CFileLike) -> Option<usize>` or project-required C-style signature | Reads at most `buf.len() - 1`, preserves terminating NUL behavior if the port retains C buffers. |
| `fread` | `pub fn fread(buf: &mut [u8], size: usize, nmemb: usize, reader: &mut CFileLike) -> usize` | Returns completed elements count, not raw bytes, matching C semantics. |

### Internal Helper Mapping

To avoid duplicating logic while staying minimal, the following internal helpers are appropriate within the same Rust file:

- byte read helper for `getchar`/`fgetc`
- bounded line read helper for `fgets`
- block read helper for `fread`
- shared formatted scanning helper used by the four scanf variants

These helpers should remain file-local unless needed elsewhere by the existing port.

## Data Model

No explicit C structs were identified in the input for this module. The main data-model work is mapping C stdio concepts into Rust types.

### Data Structure Mapping

| C Concept | Rust Representation | Notes |
|---|---|---|
| `FILE *` | `CFileLike` wrapper or existing project file abstraction | Should be the narrowest type already used by the port for readable streams. |
| input buffer `char *` | `&mut [u8]` | Preferred for direct C buffer migration and explicit bounds handling. |
| C string output for `fgets` | `&mut [u8]` with explicit trailing `0` write | Necessary if surrounding code expects NUL-terminated buffers. |
| byte / character result as `int` | `i32` | Supports unsigned-byte-or-EOF convention. |
| variadic argument list for `vscanf` / `vfscanf` | project-local scan argument representation | Must match the broader port’s variadic strategy; do not invent a new general-purpose varargs system in this module. |

### Memory Management

- Replace raw pointer writes with slice-based writes wherever the surrounding port permits.
- For any unavoidable pointer-based boundary, isolate `unsafe` to minimal conversion points and perform explicit length checks first.
- `fgets` must never write past the provided buffer and must reserve room for terminating NUL when using C-style buffers.
- `fread` should compute `size * nmemb` using checked arithmetic before reading.

### Error Handling

- Internal implementation should use `std::io::Result` and convert at public boundaries to C-compatible return values.
- Distinguish:
  - EOF with no bytes read,
  - partial read,
  - I/O error.
- For `fgetc`/`getchar`, return EOF sentinel on end-of-input or error as required by the surrounding compatibility layer.
- For `fread`, convert read byte count into fully completed element count, including partial-final-element truncation in the return value.

## Implementation Phases

## Phase 1: Module Skeleton and Unformatted Read Paths

Create `src/module_gnu_stdio_read_c_48.rs` and migrate the non-format parsing functions first:

- `getchar`
- `fgetc`
- `fgets`
- `fread`

Work items:

- establish the readable stream type used by the port (`stdin` plus file-like input),
- implement a shared one-byte read routine,
- implement bounded line reading for `fgets`,
- implement block reading for `fread` with checked multiplication and correct element-count return.

Key checks:

- EOF handling for empty input,
- no buffer overrun in `fgets`,
- correct return value for partial `fread` completion.

## Phase 2: Shared Formatted-Input Core

Implement the internal scanning core that can serve:

- `scanf`
- `fscanf`
- `vscanf`
- `vfscanf`

Work items:

- define the minimal internal representation needed for already-existing variadic argument handling in the port,
- centralize source reading so stdin- and file-based variants differ only by the input source,
- keep parsing and assignment counting in one code path,
- preserve C-like failure signaling and assignment counts.

Key checks:

- whitespace consumption behavior,
- conversion stop conditions,
- input failure before and after first successful assignment.

## Phase 3: Public Scan Entry Points and Signature Reconciliation

Expose the four formatted-input functions with project-compatible signatures and wire them to the shared core.

Work items:

- map each public function to the correct source (`stdin` or passed reader),
- route `scanf`/`fscanf` through the same implementation shape as `vscanf`/`vfscanf`,
- reconcile any required compatibility with existing crate-wide C API conventions.

Key checks:

- identical behavior across paired functions,
- no duplicated parser logic,
- minimal and well-contained `unsafe` if required for variadic compatibility.

## Phase 4: Tests and Behavioral Fixups

Add focused unit tests in the module’s test section or the project’s standard test location.

Coverage should include:

- `fgetc` and `getchar` byte reads and EOF,
- `fgets` with short lines, exact-fit lines, empty buffer edge cases, and newline retention,
- `fread` with full read, short read, and partial element completion,
- formatted-input family smoke tests across stdin/file-source entry points if the project test harness supports them.

Finalize by tightening:

- return-value compatibility,
- boundary checks,
- conversion of internal `Result` values to public C-style results.