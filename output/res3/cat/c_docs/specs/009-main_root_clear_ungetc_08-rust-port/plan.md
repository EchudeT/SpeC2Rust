# Implementation Plan

## Summary
This module ports the C logic from `fflush.c` that clears any pushed-back input state created by `ungetc` while preserving the underlying stream position semantics expected by the original code. The Rust implementation should keep the scope narrow: migrate the two functions `clear_ungetc_buffer_preserving_position` and `clear_ungetc_buffer` into a small Rust module within the main executable path, preserving call behavior and error propagation as closely as practical.

The technical approach should favor Rust standard library I/O abstractions first, but because direct `FILE *` and `ungetc` buffer manipulation do not have a direct safe Rust equivalent, the port should model the required behavior at the boundary where input state is managed. The implementation should translate the C behavior into explicit Rust helper functions operating on the project’s stream/input abstraction already used by the `cat` port. If the surrounding port still uses low-level file descriptor or stream wrappers, these functions should be implemented against that existing wrapper rather than introducing a new abstraction layer.

## Technical Context
- **Language/Version**: Rust 1.78 or newer
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended from the provided input
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Preserve constant-space behavior
  - Avoid extra buffering beyond what is required to emulate the C behavior
  - Keep stream-position adjustments bounded to the same operational steps as the original C logic
  - Do not introduce unnecessary allocations or duplicated reads

## Module Mapping
- **C source file**: `fflush.c`
- **Rust target module**: `src/main_root_clear_ungetc_08.rs` or the nearest existing main-cluster module file used by this branch’s port layout
- **Function mapping**:
  - `clear_ungetc_buffer_preserving_position` -> `fn clear_ungetc_buffer_preserving_position(...) -> io::Result<()>` (or project-local result type if already established)
  - `clear_ungetc_buffer` -> `fn clear_ungetc_buffer(...) -> io::Result<()>`

The Rust placement should follow the existing executable/module structure already present in the `cat` port. If this branch already groups main-cluster helpers under a shared module, place these two functions there instead of creating extra submodules.

## Data Model
No standalone C structs are listed for this module. The main mapping concern is the C stream handle and its implicit pushed-back state.

### C to Rust mapping
- `FILE *`
  - Map to the project’s existing Rust stream/input handle abstraction
  - If no wrapper exists yet in this branch, use the narrowest standard-library-compatible type already used by adjacent ported code, such as:
    - `std::fs::File` for seekable file-backed streams
    - a mutable generic over `Read`/`Seek` only if the surrounding code already uses generics
- C integer status / error return
  - Map to `std::io::Result<()>`
  - Preserve failure paths explicitly rather than using sentinel integers
- Implicit `ungetc` buffer state
  - Map to explicit state handled by the project’s input wrapper, if one exists
  - If the surrounding port tracks one-byte or multi-byte pushback manually, these functions should clear that state directly
  - Do not introduce a new general-purpose buffering type unless required by already migrated calling code

## Implementation Phases

### Phase 1: Inspect and anchor the port boundary
- Identify how the current Rust branch represents input streams in the main executable path.
- Locate all current or planned callers of the two functions.
- Determine whether pushed-back input is represented:
  - by a custom wrapper,
  - by a seekable file abstraction,
  - or by untranslated libc-style code still pending migration.
- Define the exact Rust function signatures to match the surrounding module conventions.
- Keep the implementation in the same module area as the existing main-cluster file mappings; do not introduce unrelated helpers.

### Phase 2: Port the clearing logic with minimal abstraction
- Implement `clear_ungetc_buffer_preserving_position` first.
  - Reproduce the original behavior for clearing pushed-back input without changing the caller-visible current position.
  - Handle only the state transitions required by the C code.
  - Use explicit `Result`-based error propagation for seek/read/reset operations where applicable.
- Implement `clear_ungetc_buffer` next.
  - Reuse the preserving-position helper where the C implementation does so logically.
  - Keep control flow direct and close to the C source structure.
- Ensure memory safety by avoiding raw-pointer-style state manipulation; use mutable borrowing of the existing stream wrapper/state instead.
- Keep ownership local and avoid storing long-lived references beyond the operation.

### Phase 3: Integrate with the surrounding main path
- Replace or wire up any translated call sites that previously depended on the C implementation.
- Confirm that the Rust functions are used only where the original module was responsible for stream-state cleanup.
- Maintain existing module visibility at the narrowest level practical (`pub(crate)` if only internal).
- Avoid broad refactors of neighboring I/O code; adapt only what is necessary for these two functions to compile and behave correctly.

### Phase 4: Validate behavior with targeted tests
- Add `cargo test` coverage for:
  - clearing pushed-back state on a seekable input abstraction,
  - preserving apparent stream position after the preserving-position variant,
  - no-op behavior when no pushed-back state exists,
  - proper error propagation when underlying seek/reset operations fail, if such failure can be simulated in the current test setup.
- Prefer unit tests in the same module or adjacent module test blocks.
- Keep tests focused on migrated behavior only; do not add broader integration scenarios unrelated to these functions.