# Implementation Plan: `module_src_output.c_27`

## Summary

Port `src/output.c` to Rust as a focused output-formatting and output-driver selection module, preserving the current behavior and call structure rather than redesigning the subsystem.

The Rust implementation should migrate the existing responsibilities in place:

- output driver registration and selection
- output lifecycle entry points
- formatted token/text emission
- symbol/type-oriented output helpers
- xref-oriented output dispatch

The preferred approach is to translate the C file into a single Rust module with closely corresponding functions and compact internal data types. C global-state patterns should be converted into explicit module-owned state passed by mutable reference where feasible; if the existing surrounding code requires process-wide state, keep it narrowly scoped and internal to the module.

Memory ownership should be expressed with standard Rust references, `String`, and `Vec`, replacing pointer-based C storage. Error handling should replace sentinel/int-style failure paths with `Result` where initialization or driver lookup can fail; pure formatting helpers can remain infallible when the C logic is infallible.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve output behavior with no material regression in normal formatting and driver-selection paths
  - Avoid unnecessary string cloning during text emission
  - Keep lookup and comparison logic at least equivalent to the C implementation
  - Maintain predictable stack/heap ownership without manual allocation management

## Module Mapping

### C to Rust File Mapping

- `src/output.c` -> `src/output.rs`

### Function Mapping

Keep function names close to the C source to reduce migration risk, using Rust naming conventions where practical.

- `print_level` -> `print_level`
- `register_output` -> `register_output`
- `select_output_driver` -> `select_output_driver`
- `output_init` -> `output_init`
- `newline` -> `newline`
- `begin` -> `begin`
- `end` -> `end`
- `separator` -> `separator`
- `print_text` -> `print_text`
- `compare` -> `compare`
- `is_var` -> `is_var`
- `symbol_is_function` -> `symbol_is_function`
- `clear_active` -> `clear_active`
- `print_type` -> `print_type`
- `xref_output` -> `xref_output`

### Rust Module Shape

Use one Rust source module corresponding directly to the C file:

- `src/output.rs`
  - driver/state definitions
  - translated helper functions
  - translated public entry points

If the crate root currently exposes C-module-like files, export this module without introducing extra layering beyond what compilation requires.

## Data Model

The analysis only reports anonymous C data structures. The Rust port should introduce named, minimal replacements only where required to model existing state.

### Data Structure Mapping

| C Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous output driver record | `struct OutputDriver` | Holds driver identity and function hooks/state needed by `begin`, `end`, `newline`, `separator`, and text/type/xref emission. |
| anonymous registered-driver collection element | `struct RegisteredOutput` or `OutputDriver` entry in `Vec<OutputDriver>` | Use `Vec` for ordered registration if C behavior depends on registration order. |
| anonymous module-global output state | `struct OutputState` | Tracks selected driver, indentation/level, and any active flags. |
| anonymous symbol/type-related temporary record | `enum` or `struct` only if needed by direct translation | Reuse existing project symbol/type types if they already exist elsewhere; do not duplicate them here. |
| anonymous active/flag storage | primitive fields inside `OutputState` | Prefer `bool`, `usize`, and `Option<usize>`/`Option<String>` instead of separate wrapper structs. |

### Expected Rust Types

Use standard Rust replacements for common C patterns:

- C strings / `char *` -> `&str`, `String`
- nullable pointers -> `Option<T>` / `Option<&T>`
- arrays/lists -> `Vec<T>`
- function pointers -> plain function items or trait-object-free callbacks if static dispatch is enough
- integer flags -> `bool` or small enums
- comparison return codes -> `std::cmp::Ordering` internally, converted only if caller compatibility requires it

### Ownership and Lifetime Decisions

- Registered output drivers should be owned by the module state via `Vec<OutputDriver>`.
- The selected driver should be referenced by index or name key instead of storing self-referential borrows.
- Output text should be accepted as `&str` where borrowed input is sufficient.
- Any generated text in `print_type` or `xref_output` should be appended into caller-visible output through mutable writers or owned buffers, avoiding raw pointer mutation patterns.

### Error Handling Mapping

Replace C-style failure signaling with narrow Rust error types:

- driver not found in `select_output_driver` -> `Result<(), OutputError>`
- invalid initialization state in `output_init` -> `Result<(), OutputError>`
- duplicate registration in `register_output` if the C logic rejects it -> `Result<(), OutputError>`

Use a small local error enum in `src/output.rs` unless a crate-wide error type already exists.

## Implementation Phases

## Phase 1: Skeleton Port and State Definition

Goals:

- Create `src/output.rs`
- Define the Rust equivalents for module state and driver records
- Establish function signatures for all translated functions
- Wire the module into the crate build

Tasks:

- Add `OutputState`
- Add `OutputDriver`
- Add `OutputError` if needed
- Translate global/static C storage into module-owned Rust state
- Define placeholders for:
  - `register_output`
  - `select_output_driver`
  - `output_init`
  - `newline`
  - `begin`
  - `end`
  - `separator`
  - `print_text`
  - `print_level`
  - `compare`
  - `is_var`
  - `symbol_is_function`
  - `clear_active`
  - `print_type`
  - `xref_output`

Deliverable:

- Module compiles with all core types and function signatures in place, even if logic is still partial.

## Phase 2: Driver Registration and Basic Output Flow

Goals:

- Port the driver registration/selection logic
- Port the common output lifecycle and text emission helpers
- Preserve C ordering and state transitions

Tasks:

- Implement `register_output` using `Vec<OutputDriver>`
- Implement `select_output_driver` with deterministic lookup matching C behavior
- Implement `output_init`
- Implement `begin`, `end`, `newline`, `separator`, `print_text`, and `print_level`
- Preserve any active/current-driver semantics using explicit `OutputState`
- Convert any raw output stream operations into `std::fmt::Write` or `std::io::Write` only if the surrounding call pattern clearly requires one; otherwise keep the existing output target model localized

Validation:

- Add unit tests for:
  - registering one or more drivers
  - selecting a valid driver
  - failure on unknown driver
  - stable behavior of newline/separator/text functions

## Phase 3: Comparison, Classification, and Type Output Helpers

Goals:

- Port the symbol/type helper logic without widening responsibilities
- Replace pointer/null checks with Rust enums and options

Tasks:

- Implement `compare`
- Implement `is_var`
- Implement `symbol_is_function`
- Implement `clear_active`
- Implement `print_type`
- Reuse existing Rust symbol/type definitions from the project if present
- Keep helper visibility minimal (`pub(crate)` or private unless external callers require `pub`)

Validation:

- Add unit tests for comparison edge cases
- Add unit tests for variable/function classification based on representative symbol fixtures
- Add tests for type formatting output shape matching current behavior

## Phase 4: Xref Output Integration and Behavior Verification

Goals:

- Complete `xref_output`
- Verify the full module behavior against the C module’s observable output paths

Tasks:

- Implement `xref_output` using the translated helpers and selected-driver state
- Ensure ordering among `begin`, body emission, separators/newlines, and `end` matches the original control flow
- Remove any temporary compatibility shims introduced during earlier phases
- Review for unnecessary cloning, unreachable states, and overexposed APIs

Validation:

- Add end-to-end tests covering representative xref output sequences
- Confirm all translated functions are exercised by `cargo test`
- Confirm no unsafe code is required unless a surrounding API absolutely forces it; if so, isolate and document the smallest possible unsafe boundary

## Notes on Migration Constraints

- Keep the Rust port aligned to `src/output.c` instead of splitting functionality across new architectural layers.
- Prefer explicit state over mutable globals, but do not invent new subsystem boundaries.
- Do not introduce concurrency-oriented wrappers or generalized plugin abstractions beyond what the C file already implies.
- Keep naming and control flow recognizable to ease side-by-side validation during the port.