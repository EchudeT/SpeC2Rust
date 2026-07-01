# Implementation Plan: module_doc_wc.c_05

## Summary

This module ports `doc/wc.c` into a single Rust module that preserves the existing file-oriented word-count behavior and helper routine structure from the C implementation. The Rust version should keep the scope narrow: migrate the counting flow, input scanning, reporting, and error-print helpers corresponding to `error_print`, `errf`, `perrf`, `report`, `isword`, `getword`, and `counter`.

The implementation approach should favor the Rust standard library, using buffered input for stream processing and idiomatic `Result`-based error propagation internally while preserving the C module’s externally visible behavior as closely as practical. Since the C file appears function-driven and does not define dedicated data structures, the Rust port should remain similarly lightweight, using plain functions and small local state values rather than introducing new abstraction layers.

## Technical Context

- **Language/Version**: Rust 1.78 or current stable compatible with the project toolchain
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended by default
- **Testing**:
  - `cargo test`
  - Unit tests for word classification and token extraction behavior
  - Integration-style tests for file/stream counting and report formatting
- **Performance Goals**:
  - Match the C module’s streaming behavior without loading whole files into memory
  - Use buffered reads to keep I/O overhead low
  - Preserve linear-time scanning over input bytes/text
  - Avoid unnecessary allocation during word detection and counting

## Module Mapping

- **C source file**
  - `doc/wc.c`
- **Rust target module**
  - `src/module_doc_wc.rs` or `src/doc/wc.rs`

Recommended mapping by function:

| C Function | Rust Mapping | Notes |
| --- | --- | --- |
| `error_print` | `fn error_print(...)` | Keep as a small stderr-print helper; prefer `eprintln!` or `writeln!(io::stderr(), ...)` |
| `errf` | `fn errf(...)` | Implement as a formatting helper for non-OS errors; likely thin wrapper over `error_print` |
| `perrf` | `fn perrf(...)` | Implement as OS/I/O error reporting helper taking `std::io::Error` or `&dyn std::error::Error` |
| `report` | `fn report(...)` | Emit formatted count output; preserve field order and spacing required by existing behavior |
| `isword` | `fn isword(...) -> bool` | Port classification logic directly; stay close to original character rules |
| `getword` | `fn getword(...) -> Option<...>` or equivalent scanner helper | Implement as local scanning routine over buffered input; exact signature should fit Rust ownership and iterator use |
| `counter` | `fn counter(...) -> io::Result<...>` | Main counting routine for a file or stream; return counts or report directly depending on current C flow |

If this module is invoked by a broader binary, keep public visibility minimal and expose only the functions required by the existing call path.

## Data Model

No explicit C structs were identified in the source analysis, so the Rust port should avoid inventing persistent object models unless required by the migrated logic.

Recommended lightweight Rust data representation:

| C Concept | Rust Representation | Notes |
| --- | --- | --- |
| Running line/word/byte counters | Local `usize` or `u64` values | Prefer `u64` if the C code may count large files and uses wide integer semantics |
| Current input token / word buffer | `String` or byte slice window | Choose the least invasive representation that matches original scanning rules |
| File/stream handle | `std::fs::File` / generic `impl std::io::Read` or `BufRead` | Favor `BufRead` where line or buffered scanning is useful |
| Error state via `errno`/stdio failure | `std::io::Error` and `Result<T, io::Error>` | Convert to stderr output only at reporting boundaries |

If repeated count values are passed together across functions, a minimal internal struct may be introduced strictly for migration clarity, for example:

```rust
struct Counts {
    lines: u64,
    words: u64,
    bytes: u64,
}
```

This should only be added if it directly replaces grouped scalar passing already implied by the C implementation.

## Implementation Phases

## Phase 1: Establish module skeleton and migrate helper/report functions

- Create the Rust module file corresponding to `doc/wc.c`.
- Port `error_print`, `errf`, `perrf`, and `report` first.
- Keep signatures simple and close to the original control flow.
- Decide early whether reporting helpers print directly or format into writers; if testability is needed, use `impl Write` internally and thin public wrappers.
- Preserve stderr/stdout separation explicitly.
- Add focused tests for:
  - report line formatting
  - plain error message formatting
  - I/O error rendering behavior

## Phase 2: Port word classification and scanning logic

- Port `isword` directly, preserving the original classification semantics rather than replacing them with broader Unicode-aware behavior.
- Implement `getword` as a scanner helper over buffered input or a byte iterator.
- Keep scanning logic close to the C implementation order so the resulting counts remain compatible.
- Avoid introducing parser frameworks or regex dependencies.
- Add unit tests covering:
  - delimiter handling
  - word boundary detection
  - empty input
  - punctuation and whitespace edge cases implied by the original helper behavior

## Phase 3: Port the main counting routine

- Implement `counter` using `std::io::BufRead` or buffered `Read`.
- Preserve the original counting semantics for lines, words, and byte/character totals as determined by the C logic.
- Ensure the routine processes input incrementally and does not retain unnecessary data.
- Use `Result` for internal error propagation and route final user-facing messages through the migrated error helpers.
- If the C routine supports multiple files versus stdin-like behavior, mirror that flow without adding new interfaces.
- Add tests for:
  - single input sample
  - multi-line input
  - no trailing newline
  - empty file
  - error path for unreadable file if applicable in the Rust call path

## Phase 4: Integrate and align behavior with the original module

- Connect the migrated functions in the same order and responsibility split as the C source.
- Remove any temporary scaffolding introduced during the port.
- Verify output formatting and error paths against expected behavior from the original module.
- Keep public API surface minimal and module-local where possible.
- Run `cargo test` and address any mismatches in counting semantics, especially around word detection and byte totals.