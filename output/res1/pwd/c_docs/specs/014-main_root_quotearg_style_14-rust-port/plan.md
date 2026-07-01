# Implementation Plan

## Summary

Port the `quotearg.c` portion used by `main_root_quotearg_style_14` into a Rust module that preserves the existing quoting-style entry points required by the `pwd` project: `quotearg_style` and `quotearg_style_mem`.

The Rust implementation should focus on a direct migration of the current logic rather than redesign. The main technical approach is:

- translate the relevant quoting-style selection and memory-based quoting flow into idiomatic Rust functions;
- represent C-style option/state records with Rust structs and enums only where required by these two functions;
- use owned `String` or `Vec<u8>` results internally to replace C buffer management;
- keep behavior-compatible handling for byte slices passed to the `_mem` variant;
- expose a minimal module surface matching the needs of the existing caller path in `main_cluster`.

This plan intentionally limits scope to the existing file and functions named in the analysis and does not introduce broader quoting infrastructure beyond what is necessary to support this migration.

## Technical Context

- **Language/Version**: Rust 1.75 or newer
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates are required based on the available module analysis
- **Testing**:
  - `cargo test`
  - unit tests for style-driven quoting and byte-slice handling
  - targeted compatibility tests for edge cases migrated from the C behavior
- **Performance Goals**:
  - preserve linear-time processing over the input byte sequence
  - avoid unnecessary intermediate allocations beyond the final quoted output
  - keep per-call overhead comparable to the C implementation for typical `pwd` path-sized inputs

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `quotearg.c` | `src/quotearg.rs` | Direct migration target for `quotearg_style` and `quotearg_style_mem` |
| caller usage from main cluster | existing caller module(s) updated to import `crate::quotearg` | Limit changes to call-site adaptation only as needed for Rust signatures |

### Function Mapping

| C Function | Rust Function | Return/Behavior Notes |
|---|---|---|
| `quotearg_style` | `pub fn quotearg_style(style: QuotingStyle, arg: &str) -> String` | Rust string-based wrapper for style-selected quoting |
| `quotearg_style_mem` | `pub fn quotearg_style_mem(style: QuotingStyle, arg: &[u8]) -> String` | Byte-slice entry point preserving explicit-length semantics |

If the surrounding port requires closer compatibility with non-UTF-8 data at call sites, `quotearg_style` can internally delegate to the `_mem` form using `arg.as_bytes()`.

## Data Model

The analysis only exposes anonymous C data structures, so the Rust data model should be introduced minimally and only for values actually required by these two functions.

| C Data | Rust Mapping | Purpose |
|---|---|---|
| anonymous quoting-style discriminator | `enum QuotingStyle` | Represents the style selector consumed by both functions |
| anonymous options/state record(s) used by quoting logic | `struct QuotingOptions` | Holds only the subset of fields needed by migrated logic |
| anonymous tables / constant records | `const` items or static arrays | Replaces C static lookup tables where present |
| anonymous temporary buffer usage | `String` / `Vec<u8>` | Replaces manual allocation and resizing |

### Proposed Rust Types

```rust
pub enum QuotingStyle {
    // Variants added strictly to match styles used by the migrated C logic
}

pub struct QuotingOptions {
    pub style: QuotingStyle,
    // Add only fields referenced by quotearg_style/_mem migration
}
```

### Memory Management Decisions

- Replace C-owned output buffers with returned `String`.
- Use `Vec<u8>` internally if the C logic operates on raw bytes before text assembly.
- Avoid global mutable storage patterns commonly used in C quotearg helpers unless strictly required by the current caller path.
- For invalid UTF-8 inputs in `quotearg_style_mem`, construct output from bytes in a controlled way rather than assuming UTF-8 input.

### Error Handling Decisions

- Prefer infallible public APIs returning `String` if the original C functions are operational formatting helpers rather than recoverable-error producers.
- If internal conversion from bytes to text needs escaping for non-UTF-8 content, implement that directly in the quoting logic instead of surfacing conversion errors.
- Reserve `Result` only for unavoidable migration points introduced by surrounding Rust integration; do not add it preemptively.

## Implementation Phases

## Phase 1: Establish module skeleton and type mappings

- Create `src/quotearg.rs`.
- Introduce the minimal `QuotingStyle` enum required by the migrated call path.
- Introduce a minimal `QuotingOptions` struct only if the C logic for these functions depends on option bundling.
- Identify the exact helper constants, static tables, and internal helper routines in `quotearg.c` that are transitively required by `quotearg_style` and `quotearg_style_mem`.
- Wire the Rust module into the crate with the smallest possible public surface.

### Deliverables

- Compiling module skeleton
- Rust type definitions for style/options
- Placeholder function signatures for:
  - `quotearg_style`
  - `quotearg_style_mem`

## Phase 2: Port core quoting logic for explicit-length input

- Migrate the logic underlying `quotearg_style_mem` first, since it preserves the C function’s explicit memory-length behavior.
- Translate loops and conditional escaping rules directly from C into Rust over `&[u8]`.
- Replace pointer arithmetic with indexed iteration or iterator-based traversal while preserving output order and escaping behavior.
- Recreate any required static character-class or quoting tables with Rust constants.
- Use a single growable output buffer (`String` or `Vec<u8>`) to replace C manual memory management.

### Deliverables

- Working `quotearg_style_mem`
- Internal helper functions/constants required by the `_mem` path
- Unit tests covering:
  - empty input
  - embedded non-printable bytes
  - representative quoting styles
  - explicit-length behavior independent of NUL termination

## Phase 3: Add string wrapper and integrate callers

- Implement `quotearg_style` as the string-oriented wrapper over the migrated memory-based function.
- Update the relevant `main_cluster` caller path to use the Rust module and adjusted signatures.
- Keep call-site changes minimal and local to compatibility needs such as `&str` versus pointer/length passing.
- Confirm that output formatting matches expected behavior in the `pwd` execution path.

### Deliverables

- Working `quotearg_style`
- Updated imports and call sites
- Integration tests for the main-path usage that depends on this module

## Phase 4: Compatibility cleanup and validation

- Review the migrated code for any remaining C-specific patterns that can be safely reduced without changing behavior.
- Remove unused translated helpers or fields not required by these two exported functions.
- Add focused regression tests from observed C edge behavior relevant to the caller path.
- Verify ownership, allocation, and non-UTF-8 handling remain contained and deterministic.

### Deliverables

- Finalized minimal Rust port of the module scope
- Passing `cargo test`
- Reduced implementation containing only the logic necessary for `main_root_quotearg_style_14`