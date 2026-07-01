# Implementation Plan

## Summary

Port `gnu/vasnprintf.c` into an idiomatic Rust module that preserves the existing formatting-buffer growth behavior represented by `MAX_ROOM_NEEDED`. The Rust implementation should stay narrowly scoped to the current module surface, translating the C allocation-sizing logic into safe Rust helpers using standard-library integer and buffer facilities. The technical approach is to replace C preprocessor-style capacity computation with Rust `const`/helper functions, use checked arithmetic to prevent overflow, and represent any failure paths through explicit `Result`-based error handling rather than implicit C memory/error conventions.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve constant-time sizing calculations equivalent to the C macro/function behavior.
  - Avoid unnecessary heap allocations beyond buffer growth required by the original logic.
  - Keep integer overflow checks explicit and low-overhead.
  - Match the original module’s role as a formatting-support utility without introducing extra abstraction layers.

## Module Mapping

- **C source**: `gnu/vasnprintf.c`
- **Rust target**: `src/module_gnu_vasnprintf.rs`

### Function and item mapping

- **C macro/function**: `MAX_ROOM_NEEDED`
  - **Rust mapping**: module-private `const fn` or small helper function for capacity calculation
  - **Migration note**: preserve exact arithmetic intent, but implement with checked or saturating operations where needed to avoid undefined overflow behavior

### File migration scope

- Migrate only logic required from `gnu/vasnprintf.c` for `MAX_ROOM_NEEDED`.
- Do not introduce additional support modules unless required by compilation boundaries in the existing Rust crate.
- Keep all migrated logic in a single Rust source file unless the crate already requires a different placement convention.

## Data Model

## C to Rust structure mapping

- **anonymous**
  - No named C struct is identified for migration in this module analysis.
  - Rust plan: avoid creating new structs unless required to hold intermediate formatting-buffer state already implicit in the C file.
  - If state grouping is necessary during porting, prefer a private Rust struct with owned buffer fields such as `Vec<u8>` or `String`, but only if directly required by the translated code path.

## Memory Management and Error Handling

- Replace manual C buffer sizing assumptions with Rust-owned buffers (`Vec<u8>` or `String`) only where the translated logic requires storage.
- Use `usize` for capacity-related calculations, with explicit checked arithmetic (`checked_add`, `checked_mul`) where overflow is possible.
- Convert overflow or impossible-capacity cases into a small internal error type or existing crate-local error return, depending on surrounding crate conventions.
- Avoid raw pointers unless they are already mandated by adjacent migrated interfaces; for this module’s identified scope, safe standard-library containers should be sufficient.

## Implementation Phases

### Phase 1: Establish module skeleton and sizing translation

- Create `src/module_gnu_vasnprintf.rs`.
- Identify the exact arithmetic and boundary assumptions behind `MAX_ROOM_NEEDED` in `gnu/vasnprintf.c`.
- Translate that logic into a Rust `const fn` or private helper with `usize`-based inputs/outputs.
- Document any places where C integer semantics differ from Rust and choose the narrowest compatible Rust behavior.

### Phase 2: Integrate overflow-safe buffer sizing behavior

- Wire the translated sizing helper into the corresponding Rust call sites for this module branch.
- Replace implicit C growth/allocation expectations with explicit Rust capacity computations.
- Ensure failure paths are surfaced through `Result` or equivalent crate-local error propagation.
- Keep the implementation file-local and avoid introducing unrelated utility layers.

### Phase 3: Validate edge cases and parity

- Add unit tests covering:
  - small input sizes
  - boundary/near-overflow sizes
  - exact expected sizing results from the original arithmetic
- Verify that the Rust helper preserves intended behavior for all reachable capacity calculations.
- Run `cargo test` and adjust only for correctness and parity with the source module.

### Phase 4: Final cleanup and migration review

- Remove any temporary translation scaffolding left from the port.
- Confirm the Rust module naming and visibility are minimal and crate-conventional.
- Recheck that no extra capabilities were introduced beyond the original `gnu/vasnprintf.c` scope.
- Finalize comments to explain only non-obvious arithmetic or safety decisions.