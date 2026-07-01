# Implementation Plan

## Summary

Port `gnu/printf-args.c` into an equivalently scoped Rust module that preserves the existing argument-fetch behavior of `PRINTF_FETCHARGS` without widening the module’s responsibilities. The Rust implementation should translate the original varargs-driven argument extraction logic into a Rust-owned internal representation that is explicit about argument kinds and ownership. The approach should prioritize direct control-flow migration, localized unsafe usage only where unavoidable, and replacement of C memory handling with standard Rust containers and lifetimes.

The implementation should remain narrowly focused on the current file and function boundary: one Rust module for the migrated logic, one internal argument model for fetched printf arguments, and tests that validate parity for supported fetch cases and edge conditions.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear argument-fetch behavior proportional to the parsed argument count.
  - Avoid unnecessary heap reallocations by pre-sizing Rust vectors when argument count is known.
  - Keep copied data limited to what is required to replace C pointer-managed storage safely.
  - Maintain behavior close to the C implementation’s cost model for argument classification and storage.

## Module Mapping

| C File | Rust Module/File | Notes |
|---|---|---|
| `gnu/printf-args.c` | `src/gnu/printf_args.rs` | Direct migration target for `PRINTF_FETCHARGS`. |
| `gnu/printf-args.c` (`PRINTF_FETCHARGS`) | `gnu::printf_args::printf_fetchargs` | Rust function implementing the same fetch workflow with explicit typed storage. |

Suggested crate-local module layout:

| Rust Path | Purpose |
|---|---|
| `src/gnu/mod.rs` | Declares the migrated `printf_args` module if the project already groups GNU-derived ports there. |
| `src/gnu/printf_args.rs` | Contains the translated fetch logic and internal argument value definitions. |

## Data Model

The input analysis does not list named C structs for this module, so the Rust data model should be introduced only as needed to represent state that was implicit in C arrays, unions, or pointer-managed buffers.

### Data-structure Mapping

| C Representation | Rust Representation | Notes |
|---|---|---|
| C varargs-derived argument value storage | `enum PrintfArgValue` | Replaces C-style untyped or union-style fetched argument storage with explicit variants. |
| C argument type tags / classification codes | `enum PrintfArgKind` | Encodes the expected fetched type for each argument slot. |
| C dynamically managed argument arrays | `Vec<PrintfArgValue>` / `Vec<PrintfArgKind>` | Replaces manual allocation and resizing. |
| C pointer/null error signaling | `Result<_, PrintfArgError>` or existing crate error style | Prefer explicit error propagation over sentinel returns where compatible with the surrounding port. |

### Proposed Rust Types

```rust
enum PrintfArgKind {
    Int,
    UInt,
    LongInt,
    ULongInt,
    LongLongInt,
    ULongLongInt,
    Double,
    LongDoubleLike,
    Pointer,
    Str,
    WideStrLike,
    Char,
    WideCharLike,
    CountPtr,
}
```

```rust
enum PrintfArgValue {
    Int(i32),
    UInt(u32),
    LongInt(i64),
    ULongInt(u64),
    LongLongInt(i64),
    ULongLongInt(u64),
    Double(f64),
    LongDoubleLike(/* mapped type based on surrounding project constraints */),
    Pointer(*const core::ffi::c_void),
    Str(*const core::ffi::c_char),
    WideStrLike(/* project-compatible wide pointer type */),
    Char(i32),
    WideCharLike(u32),
    CountPtr(*mut i32),
}
```

```rust
enum PrintfArgError {
    InvalidTypeTag,
    ArgumentCountMismatch,
    UnsupportedArgumentKind,
}
```

### Memory Management Notes

- Replace all manual allocation and release logic with `Vec`.
- Preserve borrowed/raw-pointer semantics only where the original C function stores external pointers rather than owned data.
- Keep raw pointers opaque; do not introduce ownership transfer where the C code did not own memory.
- Isolate any required `unsafe` to the smallest possible fetch boundary and document the assumption being preserved from the C code.

### Error Handling Notes

- If the surrounding Rust port already uses C-style integer status returns, keep the public signature aligned and use internal `Result` helpers.
- If no compatibility constraint exists, expose a `Result` from the Rust implementation.
- Invalid or unsupported argument classifications should be surfaced explicitly rather than silently producing placeholder values.

## Implementation Phases

## Phase 1: Establish Module Skeleton and Type Mapping

- Create `src/gnu/printf_args.rs`.
- Add only the module declarations needed to compile the migrated file in its expected project location.
- Define the minimal Rust enums required to represent fetched argument kinds and fetched argument values.
- Map each C-side argument classification used by `PRINTF_FETCHARGS` to a Rust enum variant.
- Decide the public function signature based on the surrounding port boundary, keeping it as close as possible to the existing call pattern.

### Deliverables
- Compiling Rust module skeleton.
- `PrintfArgKind` and `PrintfArgValue` definitions.
- Placeholder `printf_fetchargs` entry point with documented migration assumptions.

## Phase 2: Port `PRINTF_FETCHARGS` Control Flow

- Translate the body of `PRINTF_FETCHARGS` into Rust in the same processing order as the C implementation.
- Replace C loops, switch statements, and array writes with idiomatic but direct Rust equivalents.
- Use `Vec` for result storage, pre-sizing when the source logic knows argument counts.
- Keep raw-pointer handling and any varargs-adjacent logic narrowly scoped.
- Preserve distinction among integer, floating-point, pointer, string, and count-target argument categories exactly as required by the original function.

### Deliverables
- Functional Rust implementation of `printf_fetchargs`.
- Localized unsafe blocks only where required by the original low-level behavior.
- Removal of C-style manual memory management in favor of standard Rust ownership.

## Phase 3: Integrate Error Paths and Edge Conditions

- Port all meaningful failure paths from the C implementation.
- Convert null-like or invalid type conditions into explicit Rust error handling or equivalent status returns.
- Verify behavior for mismatched counts, invalid tags, and unsupported fetch classes if they exist in the original control flow.
- Ensure partial state does not leak resources; rely on Rust drop behavior for vectors and temporary state.

### Deliverables
- Completed error-path handling.
- Stable function signature and return behavior aligned with the project’s Rust conventions.
- Code comments documenting preserved invariants around raw pointers and non-owned argument memory.

## Phase 4: Add Migration-Parity Tests

- Add unit tests covering the translated classification-to-value mapping logic.
- Validate representative cases for:
  - signed and unsigned integer fetches,
  - floating-point fetches,
  - pointer and string argument storage,
  - error handling for invalid or unsupported kinds.
- Prefer table-driven tests for argument-kind mapping and output slot expectations.
- Keep tests confined to this module’s behavior; do not introduce broader infrastructure.

### Deliverables
- `cargo test` coverage for the migrated function’s core paths.
- Regression tests for edge cases identified during the port.
- Final cleanup to remove dead migration scaffolding.