# Implementation Plan: `module_src_parseopt_wordwrap.c_14`

## Summary

This module is a focused port of `src/parseopt/wordwrap.c` from C to Rust, preserving the existing formatting and line-wrapping behavior while replacing C varargs, buffer mutation, and stream-oriented character emission with Rust-safe APIs.

The Rust implementation should keep the module narrowly scoped around the four existing functions:

- `wordwrap_putc`
- `wordwrap_para`
- `wordwrap_vprintf`
- `wordwrap_printf`

Technical approach:

- Port the logic into a single Rust module that owns wrap-state explicitly.
- Replace implicit C state mutation with a small internal Rust state struct.
- Use `std::fmt` formatting machinery for printf-like output paths rather than recreating C varargs behavior directly.
- Keep output sink handling simple and standard-library-based, using `std::io::Write` where the C code writes to a stream-like destination.
- Preserve observable formatting behavior, especially wrapping boundaries, paragraph handling, indentation, and newline behavior.
- Keep allocation and copying limited to what is required for formatting and line assembly.

## Technical Context

### Language/Version

- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.76+`

### Primary Dependencies

Use the Rust standard library by default:

- `std::io::Write` for output targets
- `std::fmt` for formatted output
- `String` / `&str` for text buffering and slicing
- `Option`, `Result`, and small internal structs for explicit state management

Recommended third-party crates:

- None

### Testing

- `cargo test`

Testing focus:

- Wrap width boundary behavior
- Paragraph splitting behavior
- Indentation and leading-space handling
- Interaction between direct character output and formatted output
- Newline handling and line state resets
- Equivalence-oriented tests against representative inputs from the C behavior

### Performance Goals

- Match the practical performance profile of the C implementation for command-line help and option text formatting workloads.
- Avoid per-character heap allocation.
- Keep line assembly linear in input size.
- Minimize intermediate formatting buffers where possible, while accepting small temporary allocations for `printf`-style formatting replacement.
- Preserve predictable memory usage for short and medium-sized wrapped text blocks.

## Module Mapping

### Source File Mapping

| C File | Rust File |
|---|---|
| `src/parseopt/wordwrap.c` | `src/parseopt/wordwrap.rs` |

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `wordwrap_putc` | `pub(crate) fn wordwrap_putc(...) -> io::Result<()>` or internal method on wrap state | Port as the primitive character-emission and state-update operation. |
| `wordwrap_para` | `pub(crate) fn wordwrap_para(...) -> io::Result<()>` | Port paragraph wrapping logic directly; keep whitespace and line-break decisions aligned with C behavior. |
| `wordwrap_vprintf` | `pub(crate) fn wordwrap_vprintf(...) -> io::Result<()>` | Replace C `va_list` path with Rust `fmt::Arguments`-based function. |
| `wordwrap_printf` | `macro_rules!` helper or thin function wrapper around `format_args!` + `wordwrap_vprintf` | Mirror the C convenience entry point without introducing broader formatting infrastructure. |

### Suggested Rust Module Placement

If `parseopt` already exists as a Rust module:

- `src/parseopt/mod.rs`
- `src/parseopt/wordwrap.rs`

If this file is being migrated within an existing larger port, keep the exported visibility restricted to the current crate unless existing call sites require broader access.

## Data Model

The analysis lists only anonymous C data structures, which indicates this file likely uses local or embedded struct types rather than a stable public data model. The Rust port should therefore avoid inventing extra public types and introduce only the minimum internal state needed to represent the C runtime state safely.

### Data Structure Mapping

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous internal wrap-related struct(s) | `struct WordWrapState` | Consolidate mutable wrap state used across putc/paragraph/printf logic. |
| anonymous output target/reference data | `&mut dyn std::io::Write` or generic `W: Write` | Use standard writer abstraction in place of C stream pointers. |
| anonymous formatting state | `std::fmt::Arguments<'_>` plus temporary `String` | Replaces `va_list`-driven formatting. |
| anonymous flags/counters | integer fields such as `usize`, `bool` | Map line width, indent, current column, and paragraph state to explicit Rust fields. |
| anonymous text slices/pointers | `&str`, `String`, or byte iteration over `str` | Prefer `&str` for borrowed input and `String` only when formatted assembly is required. |

### Proposed Internal Rust State

A restrained internal state shape is appropriate:

```rust
struct WordWrapState<'a, W: std::io::Write> {
    out: &'a mut W,
    width: usize,
    indent: usize,
    column: usize,
    at_line_start: bool,
    // additional booleans/counters only if required by the C logic
}
```

This is not a commitment to add new behavior; it is a safe representation of the mutable state that the C implementation likely carries implicitly through parameters and local structs.

### Memory Management Decisions

- Replace raw pointers with borrowed references.
- Keep ownership external for the output sink; the word-wrap state only borrows it.
- Use stack-resident state for counters and flags.
- Use temporary `String` only for formatted output assembly in the `vprintf`/`printf` path.
- Avoid unsafe code unless a specific byte-level behavior from the C implementation absolutely requires it; the default plan is safe Rust only.

### Error Handling Decisions

- Replace C write-status conventions with `std::io::Result<()>`.
- Propagate write failures directly with `?`.
- Do not introduce custom error enums unless the C logic clearly distinguishes non-I/O failure modes.
- Treat formatting preparation as infallible when using Rust formatting macros, with I/O remaining the only operational error path.

## Implementation Phases

### Phase 1: Module Skeleton and State Port

- Create `src/parseopt/wordwrap.rs`.
- Identify the exact C function signatures and all call sites that depend on them.
- Define the minimal internal Rust state struct required to hold wrap width, indentation, current column, and line-start state.
- Decide the narrowest viable Rust signatures for the four functions based on existing caller expectations.
- Wire the module into the existing Rust crate module tree without adding unrelated abstractions.

Deliverables:

- Rust module file present and compiled into the crate
- Internal state representation defined
- Function stubs with final intended signatures

### Phase 2: Core Wrapping Logic Migration

- Port `wordwrap_putc` first as the primitive operation that updates output and column state.
- Port `wordwrap_para` next, keeping the C control flow and wrapping decisions as close as practical.
- Preserve exact newline and indentation transitions from the original implementation.
- Validate whitespace handling carefully, especially repeated spaces, paragraph boundaries, and line resets.

Deliverables:

- Functional character emission path
- Functional paragraph wrapping path
- Unit tests for core wrapping behavior

### Phase 3: Formatted Output Path Migration

- Port `wordwrap_vprintf` using `std::fmt::Arguments<'_>` as the Rust equivalent of the C variadic formatting payload.
- Implement `wordwrap_printf` as the thin convenience layer that forwards `format_args!` to the underlying formatted-output function.
- Keep formatting assembly localized to this module; do not build shared formatting infrastructure.
- Route formatted text through the same paragraph/wrapping logic to preserve behavior consistency.

Deliverables:

- Functional formatted output path
- Tests covering mixed formatting and wrapping cases
- No duplicated wrap logic between plain-text and formatted-text paths

### Phase 4: Behavioral Verification and Caller Alignment

- Compare Rust output against the C implementation for representative help-text and option-description inputs.
- Adjust edge-case behavior only where tests show mismatch in wrapping semantics.
- Confirm all existing internal callers compile against the new Rust API with no expansion of module scope.
- Perform final cleanup to reduce unnecessary allocations and simplify signatures without changing behavior.

Deliverables:

- Passing `cargo test`
- Caller integration completed
- Output behavior aligned with the C module for covered scenarios

## Notes and Constraints

- Keep the port limited to `src/parseopt/wordwrap.c` behavior; do not generalize it into a reusable text-layout subsystem.
- Prefer direct migration of control flow over stylistic rewrites that risk changing wrap semantics.
- Keep visibility minimal.
- Use safe Rust ownership and borrowing to replace C pointer mutation and lifetime ambiguity.
- Avoid introducing additional modules, traits, or helper layers unless they are strictly necessary to preserve the current file’s behavior.