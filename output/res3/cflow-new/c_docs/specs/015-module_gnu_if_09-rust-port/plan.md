# Implementation Plan: module_gnu_if_09

## Summary

This module ports the GNU-side formatted output parsing and variable-sized string formatting support from `gnu/printf-parse.c` and `gnu/vasnprintf.c` into Rust, preserving existing behavior and file-level responsibility rather than redesigning the subsystem.

The Rust implementation should focus on:
- migrating the existing parsing and formatting control flow into Rust modules with equivalent boundaries,
- replacing C pointer arithmetic and manual buffer growth with slice-based parsing and `Vec<u8>`/`String` buffer management,
- representing parse state and formatting metadata with explicit Rust structs/enums,
- converting C error signaling and allocation failure paths into `Result`-based returns,
- maintaining compatibility with the surrounding project’s current expectations without adding new formatting capabilities.

The technical approach is a direct translation of the current module logic into idiomatic but restrained Rust. Parsing should operate over byte slices to stay close to the C implementation. Dynamic output assembly should use standard-library owned buffers with controlled resizing to replace manual allocation logic from `vasnprintf.c`.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Keep formatted output generation within the same asymptotic behavior as the C implementation.
  - Avoid unnecessary UTF-8 conversions during parse and intermediate formatting steps by using byte-oriented processing where appropriate.
  - Minimize reallocations by preserving explicit buffer growth logic when migrating `vasnprintf` behavior to `Vec<u8>`.
  - Preserve single-pass or near-single-pass parsing structure from the original module where feasible.

## Module Mapping

### Source File Mapping

- `gnu/printf-parse.c`
  - **Rust target**: `src/module_gnu_if_09/printf_parse.rs`
  - **Responsibility**:
    - format string scanning,
    - extraction of conversion/specifier metadata,
    - argument-position and flag parsing,
    - storage of parsed directives in Rust data structures.

- `gnu/vasnprintf.c`
  - **Rust target**: `src/module_gnu_if_09/vasnprintf.rs`
    - output buffer construction,
    - application of parsed directives to emitted bytes/strings,
    - dynamic capacity growth,
    - error propagation for invalid parse/use combinations and allocation-sensitive paths.

### Module Entry Mapping

- **Rust module root**: `src/module_gnu_if_09/mod.rs`
- **Contents**:
  - declares `printf_parse` and `vasnprintf`,
  - re-exports only the minimum surface needed by the rest of the project,
  - keeps internal helper types private unless shared across the two translated files.

### Function Mapping

The function list only identifies anonymous `if` nodes from analysis, so function planning should be done by preserving the actual C file-local routines and top-level formatting/parsing entry points found in:
- `gnu/printf-parse.c`
- `gnu/vasnprintf.c`

During migration:
- each non-trivial C function should first be mirrored as a Rust function with close responsibility,
- helper functions should remain file-local unless both translated Rust files require the same type or routine,
- no additional public API should be introduced beyond what is required to replace the C module behavior.

## Data Model

The analysis only reports an anonymous structure set, so the Rust data model should be derived from the concrete parse and formatting records present in the C files.

### C-to-Rust Structure Mapping

- **Anonymous parse/specification structs**
  - **Rust target**: named `struct`s in `src/module_gnu_if_09/printf_parse.rs`
  - **Likely role**:
    - directive span/range,
    - flags,
    - width/precision source,
    - argument index information,
    - conversion kind/length modifier.
  - **Rust representation**:
    - `struct` with explicit typed fields,
    - offsets as `usize`,
    - optional width/precision argument positions as `Option<usize>`,
    - boolean flags as `bool`,
    - conversion and length classes as `enum`.

- **Anonymous output/buffer state structs**
  - **Rust target**: named `struct`s in `src/module_gnu_if_09/vasnprintf.rs`
    - destination buffer,
    - current length/capacity tracking,
    - formatting state shared across emission steps.
    - `Vec<u8>` for mutable byte buffer,
    - `usize` for indexes and capacities,
    - helper methods for append/grow operations.

### Scalar and Memory Mapping

- `char *` / `unsigned char *`
  - `&[u8]`, `&mut [u8]`, or `Vec<u8>` depending on ownership.

- C string segments with explicit length
  - `&[u8]` during parsing/emission,
  - `String` only when UTF-8 text is required by surrounding Rust APIs.

- integer status/error returns
  - `Result<T, ModuleError>` for fallible operations,
  - `Option<T>` only for expected absence, not error cases.

- manual allocation/reallocation
  - `Vec<u8>` growth through `reserve`/`try_reserve` where failure handling needs to remain explicit.

### Error Model

Define a small module-local error enum, for example:
- invalid format description,
- unsupported internal state encountered during translation,
- buffer growth/allocation failure,
- argument-index inconsistency.

This error type should stay narrowly scoped to replacing existing C failure paths and should not become a general project-wide abstraction unless already required elsewhere.

## Implementation Phases

### Phase 1: Establish Rust module skeleton and parse-state types

- Create:
  - `src/module_gnu_if_09/mod.rs`
  - `src/module_gnu_if_09/printf_parse.rs`
  - `src/module_gnu_if_09/vasnprintf.rs`
- Define the minimum shared data structures required by both translated files:
  - parsed directive record(s),
  - conversion kind enum(s),
  - width/precision source representation,
  - module-local error type.
- Translate C macros/constants from both files into Rust `const`s or small helper functions.
- Keep naming close enough to the C source to make migration review straightforward.

**Exit criteria**:
- module compiles with placeholder logic,
- all core C parse/output state holders have Rust equivalents,
- no unsafe code is introduced unless a specific C dependency makes it unavoidable.

### Phase 2: Port `printf-parse.c` into byte-oriented Rust parsing

- Implement format-string scanning in `printf_parse.rs`.
- Translate directive parsing step by step:
  - literal text detection,
  - flag parsing,
  - width and precision parsing,
  - positional argument handling,
  - length modifier handling,
  - final conversion classification.
- Replace pointer movement with indexed traversal over `&[u8]`.
- Preserve parse-order semantics and validation behavior from C.
- Return parsed directive collections and any metadata needed by `vasnprintf`.

**Memory and error handling focus**:
- validate all index movement before access,
- avoid lossy conversion to `String`,
- use `Result` for malformed format descriptions and inconsistent argument references.

**Exit criteria**:
- parser passes focused tests for literals, directives, invalid directives, and mixed positional/non-positional patterns if present in C behavior,
- parsed output can drive the formatting stage without C-style mutable global state.

### Phase 3: Port `vasnprintf.c` buffer management and formatting flow

- Implement output assembly in `vasnprintf.rs` using `Vec<u8>`.
- Translate the buffer growth strategy from C into explicit reserve/append helpers.
- Integrate parsed directives from Phase 2 into the emission flow.
- Mirror current formatting control paths without introducing additional formatting features or alternate formatting engines.
- Preserve separation between:
  - copying literal segments,
  - formatting directive output,
  - appending into the destination buffer.

**Memory and error handling focus**:
- replace raw writes with bounded `Vec<u8>` extension,
- use checked size arithmetic where the C code computed new lengths/capacities,
- convert allocation-sensitive branches to `Result` returns.

**Exit criteria**:
- end-to-end formatting path compiles and runs through representative tests,
- no manual memory management remains,
- output construction behavior matches current C module expectations for covered cases.

### Phase 4: Conformance cleanup and module integration

- Align public visibility with actual project use; keep helpers private by default.
- Add regression tests derived from currently observed C behavior in these two files:
  - simple literals,
  - multiple directives,
  - width/precision combinations,
  - malformed format handling,
  - buffer expansion scenarios.
- Review for:
  - off-by-one differences from pointer-based C parsing,
  - accidental UTF-8 assumptions,
  - unnecessary allocations,
  - mismatches in error propagation versus original failure paths.
- Finalize file/module wiring on branch `015-module_gnu_if_09-rust-port`.

**Exit criteria**:
- `cargo test` passes,
- migrated Rust module replaces the targeted C file responsibilities,
- implementation remains limited to the existing module scope and behavior.