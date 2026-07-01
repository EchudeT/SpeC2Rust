# Implementation Plan

## Summary

Port `gnu/vasnprintf.c` numeric floating-decoding helpers into a Rust module that preserves the existing behavior boundaries needed by formatted output code, with no expansion of scope beyond the listed functions. The Rust implementation should translate the C helper logic directly into internal, non-public functions where possible, keeping call structure and numerical decision points close to the original.

The implementation approach is:

- migrate the floating-point helper functions in place into a single Rust source module;
- represent the C anonymous helper data with private Rust structs or tuples only where required by the translated logic;
- prefer `f64`/`f32`/platform `c_longdouble`-adjacent behavior emulation through Rust numeric types and explicit helper routines rather than introducing broader formatting abstractions;
- preserve deterministic handling of edge conditions in logarithm/exponent boundary checks and borderline classification;
- keep allocation needs minimal and local, relying on ownership and stack values rather than C-style temporary memory management.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain comparable asymptotic cost to the C helpers.
  - Avoid heap allocation in numeric decoding paths unless unavoidable from translated data assembly.
  - Keep helper functions inlineable and private to reduce overhead in formatting hot paths.
  - Preserve predictable floating-point branch behavior for boundary cases.

## Module Mapping

- **C source**: `gnu/vasnprintf.c`
- **Rust target**: `src/module_gnu_vasnprintf_c_53.rs`

Function migration mapping:

- `decode_long_double` -> `decode_long_double`
- `decode_double` -> `decode_double`
- `floorlog10l` -> `floorlog10l`
- `floorlog10` -> `floorlog10`
- `is_borderline` -> `is_borderline`

Module organization constraints:

- Keep all migrated functions in one Rust module file matching this port unit.
- Expose only the minimal visibility required by the rest of the Rust port.
- Do not split numeric helpers into extra utility modules unless required by existing crate integration.

## Data Model

The C analysis reports only an **anonymous** data structure. The Rust mapping should therefore be derived strictly from actual usage in the migrated functions.

Recommended mapping strategy:

- **Anonymous C helper struct** -> **private Rust `struct`** if fields are named and reused across multiple helper calls.
- **Anonymous one-shot aggregate** -> **tuple or local bindings** if the values do not escape a single function.
- **Bit/flag-style integer fields** -> Rust integer primitives (`i32`, `u32`, `u64`, `usize`) chosen to match arithmetic and indexing behavior.
- **Floating members** -> `f64` for `double`-based logic; for `long double` logic, use the narrowest Rust representation that still matches the project’s chosen portability strategy for this port unit.

Memory and ownership decisions:

- Replace pointer-based output parameter patterns with:
  - returned structs/tuples when a helper computes multiple values; or
  - `&mut` references only when that best preserves existing call ordering without widening scope.
- Eliminate manual lifetime management; rely on stack allocation and ownership transfer.
- Avoid `unsafe` unless exact bit-level translation proves necessary for correctness and cannot be expressed safely.

## Implementation Phases

### Phase 1: Establish module skeleton and numeric type strategy

- Create `src/module_gnu_vasnprintf_c_53.rs`.
- Add direct Rust signatures for:
  - `decode_long_double`
  - `decode_double`
  - `floorlog10l`
  - `floorlog10`
  - `is_borderline`
- Inspect the original C helper signatures and determine:
  - which functions can remain private;
  - the exact Rust return style for decoded components;
  - whether `long double` logic will be represented through `f64`-compatible behavior or isolated behind a dedicated private representation.
- Define any required private Rust struct for the anonymous C data shape only after confirming repeated multi-field use in the translated code.

### Phase 2: Port logarithm and borderline helpers first

- Implement `floorlog10` and `floorlog10l` before the decode routines, preserving integer rounding semantics and boundary handling.
- Implement `is_borderline` using the translated comparison rules from the C source, with explicit handling for floating-point edge cases rather than relying on implicit C conversion behavior.
- Add focused unit tests for:
  - exact powers of ten;
  - values just below and above powers of ten;
  - zero/subnormal-adjacent inputs if the original logic accepts them;
  - representative borderline classifications used by the decode helpers.

### Phase 3: Port decode routines with minimal structural change

- Implement `decode_double` using the Phase 2 helpers and translate any output-parameter logic into a small private result struct or tuple.
- Implement `decode_long_double` next, keeping its flow as close as practical to the C version and reusing shared helper logic only where the original behavior is clearly identical.
- Preserve the original ordering of normalization, exponent extraction, and digit/borderline checks to reduce behavioral drift.
- Ensure no unnecessary heap allocation is introduced during decode result construction.

### Phase 4: Verification and integration cleanup

- Add module tests that compare `decode_double` and `decode_long_double` behavior across representative numeric classes:
  - ordinary finite values;
  - exact decimal boundaries;
  - very small and very large magnitudes within supported range.
- Reconcile integer type widths and casts so that all arithmetic is explicit and warning-free.
- Tighten visibility to the smallest practical surface and remove any temporary translation scaffolding not needed by callers.
- Confirm the module builds and passes `cargo test` on the target branch with no added capability outside the migrated helper set.