# Implementation Plan: `module_src_parseopt_wordwrap.c_14`

## Summary

This module ports the C implementation in `src/parseopt/wordwrap.c` to Rust with a narrow, behavior-preserving scope. The target functionality is formatted text emission with word-wrapping support, centered around character output, paragraph wrapping, and printf-style formatting entry points.

The Rust implementation should keep the same operational decomposition as the C file:

- a low-level character emission routine corresponding to `wordwrap_putc`
- paragraph processing and wrap control corresponding to `wordwrap_para`
- a formatting core corresponding to `wordwrap_vprintf`
- a convenience wrapper corresponding to `wordwrap_printf`

The preferred technical approach is to migrate this file into a single Rust module that preserves existing control flow and state transitions while replacing C-specific mechanisms with safe Rust equivalents:

- replace raw output callbacks / mutable C state with explicit mutable Rust state structs
- replace variadic formatting internals with `std::fmt`-based APIs where possible
- replace pointer/null-based optional state with `Option`
- replace manual buffer and lifetime handling with owned or borrowed Rust slices/strings

No additional formatting features or generalized text-layout facilities should be introduced. The port should remain focused on reproducing the existing module’s behavior in a standard Rust module layout.

## Technical Context

### Language/Version

- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.76` or newer

### Primary Dependencies

Use the Rust standard library by default.

Recommended dependencies:

- No third-party crate required for the initial port
- Use `std::fmt` for formatted output adaptation
- Use `std::io` only if the surrounding project already models output through `Write`; otherwise prefer an internal sink abstraction local to this module

### Testing

- `cargo test`

Testing focus:

- direct unit tests for wrapping boundaries
- formatting-path tests for `wordwrap_printf` and `wordwrap_vprintf` equivalents
- regression tests for paragraph splitting, spacing, and line-width behavior
- edge-case tests for empty input, single long words, embedded newlines, and width limits

### Performance Goals

- Preserve linear-time text scanning behavior relative to input size
- Avoid unnecessary string copies during wrapping where borrowing or incremental emission is sufficient
- Keep allocation bounded to formatting and temporary paragraph handling already implied by the C logic
- Match C behavior closely enough that no avoidable asymptotic regressions are introduced

## Module Mapping

### Source Mapping

C source:

- `src/parseopt/wordwrap.c`

Rust target:

- `src/parseopt/wordwrap.rs`

If the Rust project already exposes a parseopt module tree, wire it through the existing module declarations only, without creating extra abstraction layers.

### Function Mapping

| C Function | Rust Target | Migration Notes |
|---|---|---|
| `wordwrap_putc` | `fn wordwrap_putc(...)` or method on a local state struct | Preserve low-level emission semantics; convert callback/state mutation into safe mutable references |
| `wordwrap_para` | `fn wordwrap_para(...)` | Implement paragraph scanning and wrap decisions directly from C control flow |
| `wordwrap_vprintf` | `fn wordwrap_vprintf(...)` or internal formatting helper | Replace C variadic handling with Rust `fmt::Arguments`-based entry point |
| `wordwrap_printf` | `fn wordwrap_printf(...)` macro-style or function wrapper | Thin wrapper over `wordwrap_vprintf` equivalent using `format_args!` |

### API Shape

The Rust API should follow the existing project’s internal visibility needs:

- use `pub(crate)` unless broader exposure is already required by adjacent migrated code
- prefer free functions if the C module is function-oriented and state is transient
- introduce a small state struct only when needed to carry wrapping position, indentation, width, or sink state previously threaded through C arguments

## Data Model

The analysis lists only anonymous C data structures, which is typical for internal callback/context structs or locally defined state containers. The Rust port should avoid inventing a large type hierarchy and instead map each actually used data shape to a minimal named Rust struct.

### Data-Structure Mapping Strategy

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous internal output/context struct | `struct WordWrapState` | Primary mutable state for width, current column, indent, and target sink |
| anonymous callback payload struct | `struct OutputTarget<'a>` or embedded fields in `WordWrapState` | Use borrowed mutable references instead of raw pointers |
| anonymous formatting-related helper struct | `struct FormatState` only if required | Introduce only if the C file has separate formatting bookkeeping |
| anonymous token/scan locals | local variables, not standalone structs | Keep as locals when the C usage is purely procedural |
| anonymous flag groups | `bool` fields or a small `enum` | Use `enum` only when state is mutually exclusive and explicit in C control flow |

### Expected Core Rust Structures

These names are intentionally restrained and should be adjusted to match the actual C state once implementation begins.

```rust
pub(crate) struct WordWrapState<'a, W> {
    pub width: usize,
    pub column: usize,
    pub indent: usize,
    pub target: &'a mut W,
    pub pending_space: bool,
}
```

If output is not naturally modeled with `std::io::Write`, use a project-local sink shape instead:

```rust
pub(crate) struct WordWrapState<'a> {
    pub width: usize,
    pub column: usize,
    pub indent: usize,
    pub out: &'a mut String,
    pub pending_space: bool,
}
```

### C-to-Rust Type Mapping

- `char *` / `const char *` → `&str` when valid UTF-8 is already guaranteed by project assumptions; otherwise `&[u8]`
- mutable context pointers → `&mut T`
- nullable pointers → `Option<&mut T>` or `Option<T>` depending on ownership
- integer width/column fields → `usize` unless exact signed semantics are required
- C flags → `bool`
- callback return codes / status integers → `Result<(), E>` when failure is meaningful; otherwise plain `()`

### Memory Management

- Eliminate manual lifetime management by making wrapping state explicitly borrowed
- Avoid retaining references beyond the formatting/wrapping call scope
- Use stack-local scanning state wherever the C version uses transient locals
- Allocate only for formatted strings or when exact `fmt::Arguments` integration requires temporary materialization

### Error Handling

- If the C implementation assumes infallible writing into an internal buffer, keep Rust functions infallible
- If output can fail because it targets an I/O writer, return `std::io::Result<()>` or a narrow module-local `Result`
- Do not add recovery paths absent from the original logic
- Keep error propagation direct from low-level output through paragraph and formatting helpers

## Implementation Phases

## Phase 1: Establish module skeleton and state mapping

Goals:

- create `src/parseopt/wordwrap.rs`
- identify the exact mutable state carried through `wordwrap.c`
- define the minimal Rust state struct(s) required to replace anonymous C structs
- wire the module into the existing Rust crate layout

Tasks:

- inspect `wordwrap.c` for all file-local structs, callback signatures, and state fields
- rename each used anonymous data shape into a minimal Rust struct only where persistent state is required
- map C integer and pointer fields to Rust scalar and reference types
- decide whether the output target is best represented as `String`, a project-local sink, or `std::fmt`/`std::io` writer based strictly on existing call sites
- stub the four migrated functions with matching internal call order

Acceptance criteria:

- module compiles with placeholder bodies
- all required state fields from the C file have an identified Rust home
- no extra helper modules or generalized formatting framework introduced

## Phase 2: Port low-level output and paragraph wrapping

Goals:

- implement the behavior corresponding to `wordwrap_putc`
- implement paragraph scanning and wrap logic corresponding to `wordwrap_para`

Tasks:

- translate character emission semantics, including column updates and line-break handling
- preserve indentation and spacing rules from the C implementation
- port paragraph traversal in the same order as the original code, minimizing logic reshaping
- replace pointer arithmetic with slice/string index traversal that preserves boundary safety
- handle empty paragraphs, explicit newlines, and overlong words according to the C behavior

Acceptance criteria:

- unit tests cover basic wrapping cases and line-state transitions
- output for representative paragraph inputs matches the C implementation’s expected behavior
- no unsafe code required unless the surrounding project already mandates byte-level interoperability

## Phase 3: Port formatting entry points

Goals:

- implement the Rust equivalents of `wordwrap_vprintf` and `wordwrap_printf`
- connect formatted text generation to the already-ported paragraph/output path

Tasks:

- model `wordwrap_vprintf` as the internal formatting entry point using `std::fmt::Arguments`
- model `wordwrap_printf` as a thin wrapper that forwards `format_args!`
- if exact streaming semantics are not practical, format into a temporary `String` and feed it through `wordwrap_para`
- preserve call ordering and width-state behavior across formatted and non-formatted emissions
- map any C status codes to direct Rust result propagation if failures exist

Acceptance criteria:

- formatted output paths compile and behave consistently with direct paragraph calls
- tests cover mixed text and formatted substitutions
- no C-style variadic emulation or unnecessary macro machinery beyond a simple wrapper

## Phase 4: Verification and cleanup

Goals:

- verify behavioral equivalence for the migrated file
- reduce implementation-only scaffolding without changing behavior

Tasks:

- add regression tests based on observed C behavior for edge cases
- review all field types for unnecessary signedness or owned allocations
- remove dead placeholders from phase 1
- ensure function visibility matches actual crate usage
- run `cargo test` and address discrepancies in line wrapping, spacing, and newline handling

Acceptance criteria:

- all tests pass under `cargo test`
- the Rust module is limited to the scope of `wordwrap.c`
- implementation remains idiomatic but structurally close to the C source for maintainability during migration