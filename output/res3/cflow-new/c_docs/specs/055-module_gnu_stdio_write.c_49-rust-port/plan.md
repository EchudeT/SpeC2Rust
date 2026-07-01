# Implementation Plan: module_gnu_stdio-write.c_49

## Summary

Port `gnu/stdio-write.c` to a Rust module that covers the same write-oriented stdio surface: `printf`, `fprintf`, `vprintf`, `vfprintf`, `putchar`, `fputc`, `fputs`, `puts`, and `fwrite`.

The Rust implementation should stay narrowly aligned with the existing C file’s responsibilities. The technical approach is to map C stdio output behavior onto Rust’s standard I/O types and traits, primarily `std::io::Write`, `std::io::stdout`, and `std::io::stderr` where applicable. Since several listed C functions are variadic or format-driven, the Rust port should represent formatting through Rust’s standard formatting machinery and explicit writer-based helper functions rather than attempting to reproduce C varargs directly.

The implementation should prioritize:
- direct migration of output logic from the existing file,
- minimal abstraction beyond what Rust requires,
- explicit propagation of I/O failures,
- careful treatment of byte counts and newline behavior to preserve C-level semantics as closely as practical.

## Technical Context

- **Language/Version**: Rust 1.77+
- **Primary Dependencies**: Rust standard library only (`std::io`, `std::fmt`)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain buffered write behavior where the standard library already provides it.
  - Avoid unnecessary intermediate allocations for simple character, string, and byte-slice output paths.
  - Preserve linear-time write behavior for `fwrite`-style bulk output.
  - Keep wrapper overhead negligible relative to underlying stream writes.

## Module Mapping

### Source File Mapping
- `gnu/stdio-write.c` → `src/module_gnu_stdio_write.rs`

### Function Mapping
The Rust module should provide internal or crate-visible functions that correspond to the migrated C behavior:

- `printf` → Rust helper writing formatted output to standard output
- `fprintf` → Rust helper writing formatted output to a provided writer/stream abstraction
- `vprintf` → Rust helper delegating formatted output to standard output from prebuilt format arguments where feasible
- `vfprintf` → Rust helper delegating formatted output to a provided writer from prebuilt format arguments where feasible
- `putchar` → Rust helper writing a single byte/character to standard output
- `fputc` → Rust helper writing a single byte/character to a provided writer
- `fputs` → Rust helper writing a string to a provided writer without implicit newline
- `puts` → Rust helper writing a string followed by newline to standard output
- `fwrite` → Rust helper writing a byte slice to a provided writer and returning element-count-oriented results if that behavior exists in the C source

### Rust API Shape
Because Rust does not support C-style variadic functions in safe ordinary Rust:
- shared implementation should be centered on writer-based helpers,
- formatting entry points should use `std::fmt::Arguments` where formatted behavior is required,
- byte and string output functions should take `&mut impl Write`, `&str`, `&[u8]`, or character/byte parameters as appropriate.

## Data Model

This module does not define standalone C data structures in the provided analysis. The relevant type mappings are operational rather than structural.

### Type Mapping
- `FILE *` → `&mut dyn std::io::Write` or generic `W: std::io::Write`
- `char *` / `const char *` → `&str` for text paths, `&[u8]` for raw byte paths when C semantics are byte-oriented
- `void *` buffer input for `fwrite` → `&[u8]`
- `int` character/result values → `i32` or `Result<usize, std::io::Error>` internally, with conversion only where the migrated interface requires C-like status codes
- `size_t` → `usize`
- variadic argument state for `vprintf` / `vfprintf` → `std::fmt::Arguments<'_>` in Rust-facing implementation

### Memory Management Notes
- No manual memory ownership model is required for this module if output paths operate on borrowed strings and byte slices.
- Avoid heap allocation unless formatting paths inherently require it; prefer direct `write!`, `write_all`, and `write_fmt`.
- Ensure no borrowed data outlives the write call that consumes it.

### Error Handling Notes
- C stdio commonly signals failure through negative return values or short counts; Rust implementation should use `std::io::Result`.
- Where compatibility layers require C-like return conventions, perform localized translation from `Result` into status codes or written-count values.
- `fwrite`-style behavior needs special attention: if the original function reports completed elements instead of bytes, Rust code should compute that count explicitly after write attempts.

## Implementation Phases

## Phase 1: Establish module skeleton and raw write primitives
- Create `src/module_gnu_stdio_write.rs`.
- Port the simplest non-formatting write operations first:
  - `putchar`
  - `fputc`
  - `fputs`
  - `puts`
  - `fwrite`
- Implement these on top of `std::io::Write`.
- Decide and document per-function return conventions in Rust:
  - direct `io::Result<()>` / `io::Result<usize>` internally,
  - compatibility conversions only if required by surrounding project code.
- Verify newline handling for `puts` and non-newline behavior for `fputs`.
- Verify `fwrite` element-count calculation if the C logic uses `(ptr, size, nitems)` semantics.

## Phase 2: Port formatted output paths
- Add shared formatting helpers based on `std::fmt::Arguments<'_>`.
- Implement writer-targeted formatted output corresponding to:
  - `fprintf`
  - `vfprintf`
- Implement stdout-targeted formatted output corresponding to:
  - `printf`
  - `vprintf`
- Keep the implementation centralized so stdout variants delegate to writer-based variants.
- Preserve output error propagation rather than suppressing formatting or write failures.
- Keep scope limited to the formatting behavior present in the source file; do not introduce a custom printf parser unless the original file itself contains logic that requires it.

## Phase 3: Align C semantics and edge-case behavior
- Review all return values against the original C code:
  - success status,
  - byte count vs element count,
  - error signaling,
  - newline inclusion,
  - single-character conversion behavior.
- Resolve text vs raw-byte handling boundaries:
  - string-oriented functions use `&str` where valid,
  - raw output remains `&[u8]`.
- Ensure output functions do not retain references or buffers beyond call boundaries.
- Keep any necessary compatibility shims local to this module rather than adding extra support layers elsewhere.

## Phase 4: Testing and final integration
- Add unit tests covering:
  - single-character writes,
  - string writes with and without newline,
  - raw byte-slice writes,
  - formatted output routing to stdout-like and writer-backed sinks,
  - error propagation via failing mock writers,
  - `fwrite` count behavior on full and partial writes if partial-write semantics are exposed.
- Use in-memory writers such as `Vec<u8>` and minimal custom failing writers for deterministic tests.
- Run `cargo test` and confirm the module builds cleanly on branch `055-module_gnu_stdio_write.c_49-rust-port`.
- Perform a final pass to remove any abstraction not required by the migrated C file.