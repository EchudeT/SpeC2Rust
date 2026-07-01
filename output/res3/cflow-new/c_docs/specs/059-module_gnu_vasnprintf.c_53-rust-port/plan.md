# Implementation Plan

## Summary
Port `gnu/vasnprintf.c` floating-point helper logic into a Rust module that preserves existing behavior and call structure for:

- `decode_long_double`
- `decode_double`
- `floorlog10l`
- `floorlog10`
- `is_borderline`

The Rust implementation should stay narrowly scoped to these routines and the anonymous internal data they manipulate. The technical approach is a direct translation of the C algorithms into safe Rust where possible, using standard primitive floating-point types and explicit bit-level decoding only where required by the original logic. The plan should favor function-by-function migration from the existing C file into one Rust source file, keeping helper relationships intact and avoiding broader formatting subsystem redesign.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C helper routines closely in algorithmic complexity.
  - Avoid heap allocation in these helper functions unless the original logic requires intermediate owned buffers.
  - Preserve constant-time bit extraction and bounded arithmetic paths where applicable.
  - Keep overhead minimal versus the C implementation by using primitive numeric operations and compact internal representations.

## Module Mapping

### C to Rust File Mapping
- `gnu/vasnprintf.c` -> `src/module_gnu_vasnprintf_c_53.rs`

### Function Mapping
- `decode_long_double` -> `decode_long_double`
- `decode_double` -> `decode_double`
- `floorlog10l` -> `floorlog10l`
- `floorlog10` -> `floorlog10`
- `is_borderline` -> `is_borderline`

### Rust Module Scope
Create a single Rust module containing only the migrated functions and their immediately required internal helpers/constants. Do not split into additional submodules unless forced by compilation constraints.

## Data Model

The source analysis identifies only an anonymous internal data structure. The Rust port should introduce a named internal type only if needed to represent decoded floating-point components.

### Data-structure Mapping
- Anonymous C internal structure used for floating-point decomposition -> private Rust `struct` with explicit named fields

Suggested constrained shape:
```rust
struct DecodedFloat {
    sign: bool,
    exponent: i32,
    mantissa: u128,
    is_zero: bool,
    is_infinite: bool,
    is_nan: bool,
}
```

Notes:
- Use `u64` instead of `u128` if the migrated logic only needs double-precision mantissa width.
- Separate decoded representations may be used for `double` and `long double` if the original C layout handling differs enough to make a single type misleading.
- Keep all such types private to the module unless another already-existing Rust file must call through them.

## Implementation Phases

## Phase 1: Establish module skeleton and migrate logarithm helpers
- Create `src/module_gnu_vasnprintf_c_53.rs`.
- Add Rust equivalents for:
  - `floorlog10`
  - `floorlog10l`
- Translate the original arithmetic flow directly, preserving boundary handling and rounding assumptions from C.
- Define internal constants needed by these functions in the same module.
- Add focused unit tests for:
  - exact powers of ten
  - values just below and above powers of ten
  - zero/subnormal-adjacent behavior only if the C routine supports such inputs

## Phase 2: Migrate floating-point decode routines
- Implement `decode_double` first, using Rust primitive operations and bit extraction via `to_bits()`.
- Introduce the private decoded representation required by the C logic.
- Port `decode_long_double` next.
  - If the original C code assumes platform-specific `long double` layout not represented by Rust primitives, encode only the layout actually needed by the current project target assumptions.
  - Keep the implementation local and explicit rather than introducing portability abstraction layers.
- Preserve distinctions among zero, normal, subnormal, infinity, and NaN where the C routines rely on them.
- Add unit tests covering representative bit patterns and special values.

## Phase 3: Migrate borderline classification logic
- Implement `is_borderline` against the Rust decoded representation and previously ported logarithm helpers.
- Preserve original comparison ordering and any sensitivity to mantissa/exponent edge cases.
- Verify that no hidden C integer-promotion behavior is lost; use explicit casts and checked reasoning around signed/unsigned operations.
- Add tests derived from threshold-adjacent inputs that exercise true/false transitions.

## Phase 4: Integration verification and cleanup
- Review all migrated functions together to remove translation artifacts while preserving behavior.
- Ensure there are no unnecessary allocations, panics, or unchecked indexing paths.
- Confirm error handling remains internal and data-driven; these helpers should return plain values rather than introducing new error abstractions unless the original call contract requires fallibility.
- Run `cargo test` and finalize documentation comments limited to implementation constraints and C mapping notes.