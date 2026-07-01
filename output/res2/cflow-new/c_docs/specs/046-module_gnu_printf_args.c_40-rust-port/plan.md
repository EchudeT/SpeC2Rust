# Implementation Plan

## Summary

Port `gnu/printf-args.c` into a focused Rust module that preserves the existing argument-fetch behavior of `PRINTF_FETCHARGS` without broadening the API surface. The Rust implementation should mirror the original control flow for collecting and materializing printf argument values based on parsed format requirements, while replacing raw C memory handling with owned Rust containers and explicit result-based error propagation.

The implementation should stay narrowly scoped to the existing file and function. The main technical approach is:

- translate the fetch-arguments logic into a single Rust module;
- represent argument slots and fetched values with Rust enums/structs instead of untyped storage;
- replace manual allocation/reallocation patterns with `Vec`;
- convert C failure paths into `Result<_, _>`;
- preserve call ordering and indexing semantics expected by the surrounding printf machinery.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain linear processing relative to the number of printf arguments.
  - Avoid unnecessary reallocations by pre-sizing vectors from known argument counts where available.
  - Preserve low-overhead indexed access for argument metadata and fetched values.
  - Keep value movement predictable and stack/heap usage comparable to the original C implementation.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `gnu/printf-args.c` | `src/gnu/printf_args.rs` | Direct port of the argument-fetch routine and its supporting internal types. |

| C Function | Rust Item | Notes |
|---|---|---|
| `PRINTF_FETCHARGS` | `pub(crate) fn printf_fetchargs(...) -> Result<..., PrintfArgsError>` | Keep function scope crate-internal unless broader visibility is already required by the surrounding port. Signature should be shaped by existing Rust ports of adjacent printf parsing/data modules. |

## Data Model

No concrete C structs are listed in the input, so the plan should introduce only the minimum Rust representations needed to replace C-side typed storage used by `PRINTF_FETCHARGS`.

| C Representation | Rust Representation | Notes |
|---|---|---|
| Heterogeneous fetched printf argument storage | `enum PrintfArgValue` | Encodes the supported runtime argument kinds instead of using unions or raw memory. Variants should be limited to the types actually consumed by the existing format pipeline. |
| Argument type/category tags used while fetching | `enum PrintfArgKind` | Represents the requested fetch type for each positional argument. |
| Parallel arrays for argument descriptors/results | `struct PrintfArgs { kinds: Vec<PrintfArgKind>, values: Vec<PrintfArgValue> }` or direct vectors in function scope | Use the smallest shape that matches surrounding module interfaces. Do not introduce additional abstraction if adjacent modules already define descriptor containers. |
| C integer status / allocation failure signaling | `Result<T, PrintfArgsError>` | Replaces sentinel returns and centralizes conversion of invalid type requests or allocation problems. |
| Raw pointer / `va_list`-style traversal state | Rust-side traversal input defined by surrounding port | If a true `va_list` analogue already exists in the Rust project, consume it directly; otherwise keep the fetch logic aligned to the existing parser/output pipeline rather than inventing a new general abstraction. |

### Error Type

Introduce a narrow internal error enum only if required by the ported logic:

```rust
pub(crate) enum PrintfArgsError {
    InvalidArgumentKind,
    AllocationFailed,
    ArgumentSourceExhausted,
}
```

This should remain minimal and be adjusted to match real failure modes encountered during translation. Do not add recovery-oriented variants beyond the original behavior.

## Implementation Phases

### Phase 1: Establish Rust module and function skeleton

- Create `src/gnu/printf_args.rs`.
- Define the crate-internal entry point corresponding to `PRINTF_FETCHARGS`.
- Identify the exact inputs/outputs required from the surrounding printf parser/data modules already being ported.
- Introduce minimal Rust enums/structs for argument kinds and fetched values where no equivalent Rust definitions already exist.
- Preserve the original indexing model for positional arguments and argument count handling.

### Phase 2: Port core fetch logic and storage management

- Translate the main fetch loop from `PRINTF_FETCHARGS` into idiomatic but structure-preserving Rust.
- Replace manual memory allocation and resizing with `Vec`, using reserved capacity based on known argument counts.
- Convert type-dispatch logic into `match` over `PrintfArgKind`.
- Store fetched values in typed Rust representations rather than raw buffers/unions.
- Map original failure exits to `Result` returns, keeping error handling local and explicit.

### Phase 3: Integrate with adjacent printf module types

- Align the Rust function signature with the existing or planned Rust equivalents of parsed format argument descriptors.
- Ensure fetched argument ordering, positional lookup, and repeated-reference behavior match the C implementation.
- Reconcile any C-specific assumptions about integer widths, signedness, and pointer-like values with Rust primitive types such as `i32`, `u32`, `isize`, `usize`, and raw pointers/references as appropriate.
- Keep ownership boundaries clear so fetched values outlive only the formatting operation that consumes them.

### Phase 4: Verification and cleanup

- Add focused unit tests covering:
  - sequential argument fetching;
  - positional argument fetching;
  - mixed argument kinds;
  - invalid or unsupported argument kind handling;
  - empty/no-argument cases if supported by the original logic.
- Run `cargo test` and fix semantic mismatches against the C behavior.
- Remove any temporary translation scaffolding that is not required by the final module boundary.