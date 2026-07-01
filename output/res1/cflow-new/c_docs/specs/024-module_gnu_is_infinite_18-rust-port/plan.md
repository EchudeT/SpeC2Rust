# Implementation Plan: module_gnu_is_infinite_18

## Summary

This module ports the floating-point classification helpers currently embedded in `gnu/vasnprintf.c` into Rust, preserving existing behavior and keeping the scope limited to the two identified functions:

- `is_infinite_or_zero`
- `is_infinite_or_zerol`

The Rust implementation should use the standard library’s floating-point classification facilities instead of reproducing C-level bit inspection or platform-specific macros. The main technical approach is to translate the C helper logic into small, private or crate-visible Rust functions that operate on Rust floating-point primitives, with naming and placement aligned to the existing module migration path. Since both functions are classification helpers, the port should emphasize semantic equivalence, minimal allocation, and no expansion beyond the current file/function set.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time floating-point classification
  - Zero allocation
  - Behavior equivalent to the C helpers for zero and infinity detection
  - No unnecessary wrapper layers or conversions beyond what is required for type mapping

## Module Mapping

| C Source File | Rust Target File | Notes |
|---|---|---|
| `gnu/vasnprintf.c` | `src/gnu/vasnprintf.rs` | Migrate only the helper logic related to floating-point classification used by this module scope. |
| `is_infinite_or_zero` | `src/gnu/vasnprintf.rs::is_infinite_or_zero` | Port as a direct Rust function using primitive float classification. |
| `is_infinite_or_zerol` | `src/gnu/vasnprintf.rs::is_infinite_or_zerol` | Port with the closest Rust type mapping available for C `long double`. |

### Rust module layout

The implementation should remain within the Rust counterpart of `gnu/vasnprintf.c`. Do not split these helpers into additional utility modules unless the existing project layout already requires it. The goal is a direct file-level migration path.

## Data Model

The analysis identifies only an anonymous data structure and no named struct dependency for these functions. No dedicated Rust struct or enum is required for this module port.

### C to Rust type mapping

| C Type / Concept | Rust Type | Notes |
|---|---|---|
| `double` | `f64` | Direct mapping. |
| `long double` | `f64` | Rust has no portable native equivalent of C `long double`; use `f64` unless the surrounding migrated code already establishes a different internal convention. Keep this choice local and explicit. |
| Floating-point zero/infinity classification | `f64` intrinsic methods | Use `is_infinite()` and equality-to-zero checks as appropriate. |

### Memory management

These functions are pure classification helpers and should not allocate or own resources. Rust’s ownership model imposes no special management burden here.

### Error handling

No explicit error channel is needed. The original C helpers appear to be total functions over floating-point inputs; the Rust versions should preserve that property by returning plain boolean results.

## Implementation Phases

### Phase 1: Establish file-local Rust mappings

- Create or update the Rust counterpart for `gnu/vasnprintf.c` at `src/gnu/vasnprintf.rs`.
- Identify the exact call sites or expected visibility for:
  - `is_infinite_or_zero`
  - `is_infinite_or_zerol`
- Map C floating-point parameter types to Rust primitives:
  - `double` → `f64`
  - `long double` → `f64` for this port
- Keep function signatures narrow and aligned with current usage; avoid introducing generic abstractions.

### Phase 2: Port helper logic directly

- Implement `is_infinite_or_zero` using Rust standard library floating-point methods.
- Implement `is_infinite_or_zerol` with the same classification semantics and the chosen `long double` mapping.
- Preserve edge-case behavior for:
  - positive zero
  - negative zero
  - positive infinity
  - negative infinity
  - finite nonzero values
  - NaN values
- Keep the implementation free of unsafe code unless the surrounding migrated file already requires it for unrelated reasons.

### Phase 3: Validate behavior with unit tests

- Add focused unit tests in the corresponding Rust module or test module covering:
  - `0.0`
  - `-0.0`
  - `f64::INFINITY`
  - `f64::NEG_INFINITY`
  - representative finite values
  - `f64::NAN`
- Ensure tests verify only the migrated helper semantics and do not introduce broader formatting behavior coverage not evidenced by the input.
- Run `cargo test` and fix any mismatches caused by C-to-Rust floating-point type differences.

### Phase 4: Integrate and finalize module migration

- Replace any in-file C-style helper assumptions with Rust function calls in the migrated `vasnprintf` implementation.
- Confirm there is no duplicated classification logic left in the Rust file.
- Keep naming, visibility, and placement consistent with the rest of the ported module cluster.
- Perform a final review for:
  - no added capabilities
  - no unnecessary dependencies
  - no allocation or error-handling scaffolding beyond what the original helpers require