# Implementation Plan: module_doc_wc.c_05

## Summary

This module ports `doc/wc.c` to Rust with a narrow, file-by-file migration scope. The Rust implementation should preserve the existing command-style behavior: scanning input, classifying word boundaries, counting relevant units, and reporting results, while translating C-style I/O and error paths into explicit Rust `Result`-based control flow.

The implementation approach should remain close to the original function decomposition:

- retain small helper functions corresponding to `isword`, `getword`, and reporting/error helpers,
- keep the counting logic centralized in a Rust equivalent of `counter`,
- use standard library buffered input and UTF-8-safe text handling where practical, while being careful not to introduce behavior beyond the original module’s needs,
- replace implicit C global/process error handling patterns with explicit return values and stderr output.

The migration should favor a single Rust module with internal helper functions rather than introducing additional abstraction layers.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the original module’s linear scan behavior over input data.
  - Avoid unnecessary allocations during token scanning and counting.
  - Use buffered reading from `std::io` for file/stdin processing.
  - Keep per-character classification simple and branch-light, consistent with the original C utility style.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `doc/wc.c` | `src/module_doc_wc_c_05.rs` | Direct port target containing the module logic and helper functions. |

### Function Mapping

| C Function | Rust Mapping | Notes |
|---|---|---|
| `error_print` | `fn error_print(...)` | Write formatted error text to stderr using standard library I/O/macros. |
| `errf` | `fn errf<T>(...) -> Result<T, ModuleError>` or thin stderr helper | Convert C-style formatted error path into explicit Rust error propagation. |
| `perrf` | `fn perrf<T>(...) -> Result<T, ModuleError>` | Preserve system/IO-error reporting semantics by attaching `std::io::Error` context. |
| `report` | `fn report(...)` | Print final counts/results in the original output order/format. |
| `isword` | `fn is_word(...) -> bool` | Character/byte classification helper, kept local and simple. |
| `getword` | `fn get_word(...) -> Option<...>` | Token extraction helper; signature adapted to iterator/buffer usage. |
| `counter` | `fn counter(...) -> Result<Counts, ModuleError>` | Main scan/count routine over a reader or input source. |

## Data Model

No explicit C structs were identified in the analyzed module. The Rust port should therefore use only minimal internal types needed to replace implicit C state passing.

### Data Structure Mapping

| C Representation | Rust Representation | Purpose |
|---|---|---|
| Local counters / grouped scalar state | `struct Counts` | Collect count totals passed from `counter` to `report`. |
| Integer/boolean classification flags | `bool`, `usize`, `u64` as appropriate | Replace C scalar state with explicit typed values. |
| C error return conventions | `enum ModuleError` | Distinguish I/O failures from usage/reporting failures without adding extra facilities. |

### Proposed Rust Types

```rust
struct Counts {
    words: usize,
    // add only the counters actually required by the original file
}

enum ModuleError {
    Io(std::io::Error),
    Message(String),
}
```

### Ownership and Memory Notes

- Input processing should borrow from buffered reader state where possible and avoid storing full-file contents unless required by the original logic.
- Temporary token data produced by `get_word` should be short-lived and not retained beyond counting/reporting.
- Error messages should be owned only when needed for propagation; simple stderr printing can remain allocation-light.

## Implementation Phases

## Phase 1: Establish the Rust module skeleton and error path migration

### Goal
Create a direct Rust home for `doc/wc.c` and port the module’s error/reporting surface first so the remaining logic has stable interfaces.

### Tasks
- Create `src/module_doc_wc_c_05.rs`.
- Define the minimal internal error type (`ModuleError`) and `Result` alias if helpful.
- Port `error_print`, `errf`, and `perrf` into Rust-oriented helpers.
- Decide and document the exact function signatures for:
  - reporting helpers,
  - scanning helpers,
  - main counting routine.
- Keep stderr output behavior close to the C module and avoid adding logging frameworks.

### Notes
- Prefer `eprintln!`/`writeln!(std::io::stderr(), ...)` over custom facilities.
- Convert C sentinel/error-code flows into `Result` returns at function boundaries.
- Do not introduce separate utility modules; keep helpers in the same Rust file.

## Phase 2: Port token classification and counting logic

### Goal
Translate the word-detection and scan loop behavior while preserving the original counting semantics.

### Tasks
- Port `isword` to a Rust helper (`is_word`) with behavior matching the C classification logic as closely as possible.
- Port `getword` using either:
  - buffered character iteration, or
  - byte-oriented scanning if the original behavior is byte/classification driven.
- Implement `Counts` to hold only the metrics used by this file.
- Port `counter` as the central routine that reads input, applies `is_word`/`get_word`, and updates counts.
- Ensure EOF and empty-input handling match the original control flow.

### Notes
- Prefer `BufRead`-based processing for efficient streaming input.
- If the original C logic is ASCII-oriented, preserve that behavior explicitly rather than broadening classification rules unintentionally.
- Be careful with Rust `char` vs byte semantics; choose the narrower model that best matches the C implementation.

## Phase 3: Port output formatting and integrate module behavior

### Goal
Complete the Rust equivalent of the module’s visible behavior by wiring counting results into final reporting.

### Tasks
- Port `report` to emit results in the original layout and ordering.
- Connect `counter` output to `report`.
- Finalize function visibility (`pub` only where required by the crate).
- Ensure all former C helper responsibilities are covered within the single Rust module.

### Notes
- Avoid redesigning the CLI or introducing new argument parsing unless directly required by the existing file’s responsibilities.
- Preserve formatting details such as separators, spacing, and newline behavior as closely as possible.

## Phase 4: Verification and cleanup

### Goal
Confirm behavioral parity for normal input, edge cases, and error paths, then simplify any overly C-shaped code that is no longer needed.

### Tasks
- Add unit tests for:
  - word classification edge cases,
  - empty input,
  - simple multi-word input,
  - punctuation/boundary handling consistent with the original logic,
  - I/O error propagation where practical.
- Add focused tests for `report` formatting if output shape is significant.
- Run `cargo test` and address borrow/ownership simplifications without changing behavior.
- Remove unused temporary compatibility code introduced during porting.

### Notes
- Keep tests at module scope; do not create a larger test harness unless required.
- Prioritize behavioral equivalence over stylistic refactoring.