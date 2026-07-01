# Implementation Plan

## Summary
This module ports the C file `xalloc-die.c` into a Rust implementation that preserves the existing single-purpose behavior: terminate execution on unrecoverable allocation failure with the same process-level semantics expected by the surrounding `cat` program.

The Rust approach should stay minimal and close to the original module boundary. The implementation should expose a dedicated Rust function corresponding to `xalloc_die`, centralize the failure path in one file, and rely on the Rust standard library for process termination and stderr output. No additional abstraction layers or generalized allocation framework should be introduced.

## Technical Context

### Language/Version
- Rust stable
- Recommended minimum version: **Rust 1.74+**

### Primary Dependencies
- Rust standard library only:
  - `std::io` for diagnostic output
  - `std::process` for exiting the process

No third-party crates are recommended because the provided input does not show any need beyond standard process termination and error reporting.

### Testing
- `cargo test`

Testing should focus on compilation, module integration, and any testable helper behavior if the exit path is factored into an internal routine. The direct terminating function itself should remain simple and should not be over-engineered for testability.

### Performance Goals
- Negligible runtime overhead compared with the C version
- Constant-time failure handling path
- No heap allocation introduced in the termination routine unless unavoidable for formatting
- Preserve straightforward control flow for the out-of-memory fatal path

## Module Mapping

### C to Rust File Mapping
- `xalloc-die.c` → `src/xalloc_die.rs`

### Function Mapping
- `xalloc_die` → `pub fn xalloc_die() -> !`

### Integration Mapping
- Declare the Rust module from the existing crate root or the current binary root:
  - `mod xalloc_die;`
- Update existing call sites that relied on the C symbol so they invoke the Rust function directly within the crate module structure.

The migration should remain limited to replacing this module and wiring its direct callers, without creating extra utility modules.

## Data Model

This module has no module-specific data structures in the provided analysis.

### Data-structure Mapping
- C structs: none
- Rust structs/enums: none required

### Behavioral Mapping
- C fatal no-return function → Rust diverging function:
  - `void xalloc_die(void)` → `fn xalloc_die() -> !`

### Memory Management Notes
- The original C module likely exists to handle memory exhaustion centrally. In Rust, the port should avoid any design that depends on further heap allocation during the fatal path.
- Prefer static string diagnostics or minimal formatting to reduce risk of allocation while reporting the error.
- Process termination should be explicit and immediate after best-effort diagnostic emission.

### Error Handling Notes
- This function represents a terminal error path, not a recoverable error result.
- Rust should model this with a diverging function (`-> !`) and process exit behavior rather than `Result`.

## Implementation Phases

### Phase 1: Create the Rust Module Skeleton
- Add `src/xalloc_die.rs`
- Implement the Rust equivalent of `xalloc_die` as the only public item required by this module
- Use standard library facilities for:
  - writing a fixed diagnostic to stderr
  - terminating the process with the intended non-zero exit status
- Keep the implementation self-contained and avoid introducing unrelated helpers

### Phase 2: Integrate With Existing Crate Structure
- Register the module in the appropriate crate root file (`main.rs` or existing module tree)
- Replace references to the former C implementation with Rust module calls
- Verify the function signature matches caller expectations for a non-returning fatal path
- Ensure no legacy declarations or duplicate symbols remain after the migration

### Phase 3: Validate Termination Semantics
- Confirm the Rust implementation preserves:
  - fatal behavior
  - stderr diagnostic behavior
  - non-returning control flow
- Add targeted tests only where practical, such as testing any internal non-exit formatting helper if one is introduced
- Run `cargo test` and fix any integration issues caused by diverging control flow typing

### Phase 4: Final Cleanup
- Remove obsolete build references tied specifically to `xalloc-die.c`
- Check that the Rust file naming and module naming remain consistent with project conventions
- Confirm the migrated module stays narrow in scope and does not accumulate unrelated allocation or error-policy code