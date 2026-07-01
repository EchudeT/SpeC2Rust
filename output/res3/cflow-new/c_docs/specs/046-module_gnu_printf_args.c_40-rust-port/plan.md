# Implementation Plan

## Summary
Port `gnu/printf-args.c` into a focused Rust module that preserves the existing argument-fetch behavior of `PRINTF_FETCHARGS` without expanding scope. The Rust implementation should mirror the current control flow for collecting and storing printf argument values, using explicit Rust enums/structs to represent the C-side argument categories and owned storage.

The technical approach is to migrate the single C source file into one Rust module with a narrow API, keeping parsing/fetch sequencing logic local to that module. Memory ownership should move from manual C allocation patterns to Rust-owned containers such as `Vec`, with index-based access replacing pointer arithmetic where applicable. Error paths that are implicit in C should become explicit `Result` returns where the surrounding Rust code requires failure reporting.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the current asymptotic behavior of argument collection.
  - Avoid unnecessary heap allocations beyond the argument storage already implied by the C implementation.
  - Keep argument fetch and storage operations near C-equivalent cost by using contiguous storage and minimal conversion layers.

## Module Mapping

- **C source**
  - `gnu/printf-args.c`
- **Rust target**
  - `src/gnu/printf_args.rs`

### Function Mapping
- `PRINTF_FETCHARGS`
  - Migrate to a Rust function in `src/gnu/printf_args.rs`
  - Keep it as the main entry point for this module’s fetch/store behavior
  - Prefer a snake_case Rust name such as `printf_fetchargs`, with visibility limited to the current crate unless broader exposure is already required by existing Rust-side call sites

## Data Model

No concrete C struct definitions were provided in the analysis input, so the Rust data model should be derived strictly from the fields and value categories actually used by `PRINTF_FETCHARGS`.

### Expected Mapping Strategy
- **C primitive argument categories**
  - Map integer-like values to Rust integer primitives matching width/signedness used by the original logic
  - Map floating-point values to `f32`/`f64` as required by the C source
  - Map pointer-like values to raw pointers or references only if required by surrounding ported code; otherwise prefer opaque pointer representations that do not imply ownership

- **C tagged argument storage**
  - If the C code stores heterogenous fetched arguments in a tagged union, represent this in Rust as:
    - an `enum` for the stored value variants
    - a companion struct if metadata and value storage are separated in the original implementation

### Recommended Rust Shapes
```rust
enum PrintfArgValue {
    // Filled only with variants actually used by the C implementation
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    Float(f64),
    Pointer(*const core::ffi::c_void),
    // additional variants only if directly required by the source file
}

struct PrintfArg {
    value: PrintfArgValue,
    // include metadata fields only if the C function reads/writes them
}
```

### Memory Management Notes
- Replace manual allocation/reallocation with `Vec<...>`.
- Preserve stable indexing if the original code relies on positional argument lookup.
- Avoid introducing shared ownership types unless the original logic truly requires aliasing.
- Keep raw pointers non-owning; ownership should remain with the caller or surrounding module as in the C implementation.

### Error Handling Notes
- Convert allocation or invalid-state failures into `Result`.
- If the original function reports success/failure by integer status, define a narrow Rust error type and translate status codes at the module boundary.
- Do not add recovery logic beyond direct propagation of existing failure cases.

## Implementation Phases

### Phase 1: Module Skeleton and Signature Port
- Create `src/gnu/printf_args.rs`.
- Port the `PRINTF_FETCHARGS` interface into an idiomatic but scope-preserving Rust function.
- Identify all external types and constants referenced by `PRINTF_FETCHARGS` and declare the minimum Rust equivalents needed for compilation.
- Establish the module’s return type and error boundary based on the C function’s current status signaling.

### Phase 2: Data Representation Migration
- Translate the C-side argument storage model into Rust enums/structs.
- Replace manual memory management with `Vec`-backed storage.
- Port any tag/value branching used by `PRINTF_FETCHARGS` so that fetched arguments are stored in Rust with explicit variant handling.
- Ensure width-sensitive C types are mapped carefully to Rust primitives to preserve behavior.

### Phase 3: Fetch Logic Port
- Migrate the body of `PRINTF_FETCHARGS` in execution-order form, keeping the original branching structure as close as practical.
- Replace pointer arithmetic and mutable out-parameters with indexed access and mutable Rust references.
- Preserve positional argument handling exactly as implemented in the C file.
- Make failure cases explicit through `Result` or equivalent crate-local status handling.

### Phase 4: Validation and Cleanup
- Add unit tests covering the migrated fetch/store paths that can be exercised from the available Rust interfaces.
- Validate edge cases implied by the original code, such as empty argument sets, mixed argument categories, and invalid tag/state combinations if present.
- Remove any temporary compatibility scaffolding not required by the final module shape.
- Confirm the module builds and passes `cargo test`.