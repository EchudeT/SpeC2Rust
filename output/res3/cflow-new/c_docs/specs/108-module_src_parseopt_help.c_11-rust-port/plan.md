# Implementation Plan

## Summary

Port `src/parseopt/help.c` into an idiomatic Rust module that preserves the existing help/option parsing behavior needed by this unit, with special attention to the `min` function and any file-local helper logic embedded around it. The Rust implementation should stay narrowly scoped to the current C file’s responsibilities and migrate behavior into a single Rust source module without introducing new subsystems.

The technical approach is a direct C-to-Rust translation:
- move file-local logic into one Rust module under the parse option/help area,
- replace C macros and anonymous aggregate usage with private Rust items,
- represent nullable pointers and conditional state with `Option`,
- use slice/string APIs where the C code currently relies on pointer arithmetic or raw buffers,
- preserve output and branching behavior while making ownership explicit and eliminating manual memory management.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behavior-equivalent runtime characteristics for small utility-style help formatting/parsing paths.
  - Avoid unnecessary heap allocation beyond what is required to represent strings and collections safely in Rust.
  - Keep helper functions inline/simple where they correspond to small C utility routines such as `min`.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `src/parseopt/help.c` | `src/parseopt/help.rs` | Direct port of the file’s logic into a single Rust module. |
| file-local `min` function | private Rust function in `src/parseopt/help.rs` | Preserve narrow visibility; use generic/std comparison only if it matches exact call usage. |

If the Rust crate already exposes a `parseopt` module tree, wire this file through the existing `mod.rs` or equivalent with no extra restructuring beyond what is needed to include `help.rs`.

## Data Model

The analysis only exposes repeated anonymous C data structures, so the Rust plan should derive concrete mappings from actual usage during migration rather than inventing new abstractions.

| C Construct | Rust Mapping | Migration Rule |
|---|---|---|
| anonymous struct used only locally | private `struct` in `help.rs` | Name by role from field usage in the C file. |
| anonymous union-like state patterns | private `enum` or `struct` with `Option` fields | Use only when the C code actually switches among variants. |
| C string pointer (`char *`, `const char *`) | `String`, `&str`, or `Option<String>` / `Option<&str>` | Choose borrowed vs owned form based on lifetime and mutation needs. |
| pointer to arrays / counted sequences | `&[T]`, `Vec<T>`, or iterator-based traversal | Prefer slices for borrowed input; `Vec` only for owned transformed data. |
| integer flags / boolean-like fields | `bool` or integer type matching range | Keep integer type only if values are semantically numeric. |
| nullable pointer fields | `Option<T>` / `Option<&T>` | Replace null checks directly with pattern matching. |

### Data-structure naming guidance

Because the source analysis reports only `anonymous` entries, each C anonymous aggregate should be renamed in Rust according to its actual role in `help.c`, for example:
- help entry/state record,
- option description row,
- formatting context,
- temporary traversal state.

Do not create public types unless the original C file participates in a broader public interface requiring them.

## Implementation Phases

### Phase 1: File Translation Skeleton

- Create `src/parseopt/help.rs`.
- Establish the module entry points corresponding to the C file’s externally visible functions, if any.
- Port the `min` helper first as a private Rust function or replace it with `core::cmp::min` only if all call sites are type-compatible and semantics are unchanged.
- Identify all anonymous C aggregates in `help.c` and introduce minimal private Rust `struct`/`enum` definitions required to compile the translated logic.
- Keep function boundaries close to the C source to reduce migration risk.

### Phase 2: Logic Port and Type Tightening

- Translate the main control flow from `help.c` into safe Rust.
- Replace raw-pointer traversal with:
  - slices for indexed collections,
  - `Option` for nullable values,
  - `match`/`if let` for branch clarity.
- Convert C string handling into `&str`/`String` as dictated by ownership.
- Preserve formatting and comparison behavior exactly where it affects help text layout or option presentation.
- Resolve any C integer-width assumptions with explicit Rust integer types where needed.

### Phase 3: Error Handling and Boundary Cleanup

- Remove any remaining implicit C failure patterns such as sentinel/null returns where Rust can express the same contract more clearly.
- Use `Result` only for functions that can actually fail in the Rust port; keep pure helpers like `min` infallible.
- Ensure there are no hidden lifetime issues from borrowed string/slice views.
- Minimize cloning/allocation after the initial safe translation is working.

### Phase 4: Verification and Integration

- Add unit tests in the Rust module or adjacent test module focused on:
  - `min` behavior across relevant edge cases,
  - help/formatting logic covered by the original file,
  - null/empty input equivalents from the C implementation.
- Run `cargo test`.
- Compare translated behavior against the C source expectations for representative inputs and outputs.
- Finalize module visibility and remove unused transitional items introduced during porting.

## Migration Notes

- Keep the migration limited to `src/parseopt/help.c`; do not generalize shared formatting or option infrastructure unless the existing Rust tree already requires a direct signature match.
- Prefer private items and narrow interfaces.
- Treat all anonymous C structures as implementation details unless the original file’s API proves otherwise.
- Preserve semantics first, then simplify only where the Rust standard library provides an exact replacement.