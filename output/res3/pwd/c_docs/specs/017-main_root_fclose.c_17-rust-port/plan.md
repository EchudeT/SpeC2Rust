# Implementation Plan

## Summary
Port `fclose.c` into a Rust module that preserves the existing close-path behavior represented by `fclose_nothrow` and `rpl_fclose`. The Rust implementation should stay narrowly scoped to file-close handling and associated error propagation, mirroring the current module boundaries rather than introducing broader I/O abstractions.

The technical approach is to map the C logic into a small Rust module under the main executable/library crate, using standard-library file and descriptor ownership types where possible. The implementation should make close behavior explicit, avoid panic-based control flow, and represent close failures with `std::io::Result`. Any C behavior that distinguishes a “non-throwing” close helper from a replacement `fclose` entrypoint should be preserved as separate Rust functions, with ownership and resource finalization made explicit through Rust’s drop semantics and manual error-return paths where needed.

## Technical Context

- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended from the provided module evidence
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Maintain equivalent close-path cost to the C implementation
  - Avoid extra allocations in the close helpers
  - Keep wrapper overhead negligible relative to OS/file close operations
  - Preserve deterministic resource release behavior

## Module Mapping

### C to Rust File Mapping
- `fclose.c` -> `src/main_root_fclose.rs` or `src/fclose.rs`

Preferred choice:
- Use `src/fclose.rs` if this project already places migrated utility-style modules at crate root.
- Re-export or call from the existing main-cluster entry module as needed, without creating additional layers.

### Function Mapping
- `fclose_nothrow` -> `pub(crate) fn fclose_nothrow(...) -> std::io::Result<()>`
- `rpl_fclose` -> `pub(crate) fn rpl_fclose(...) -> std::io::Result<()>`

### Mapping Notes
- Keep both functions distinct even if one becomes a thin wrapper over the other.
- Preserve migration order from helper to public/internal replacement function:
  1. `fclose_nothrow`
  2. `rpl_fclose`
- If the original C code operates on `FILE *`, represent the Rust side in terms of the narrowest ownership-bearing standard type already used by the surrounding ported code:
  - `std::fs::File` when full file objects are available
  - `std::os::fd::OwnedFd` on Unix when only descriptor ownership matters
- Do not add compatibility layers beyond what is required to migrate the existing call sites.

## Data Model

No named C structs are listed for this module. The main data-model work is ownership and error-state mapping for file handles.

### C to Rust Type Mapping
- `FILE *` -> `std::fs::File` or `std::os::fd::OwnedFd` depending on surrounding call-site usage
- C integer status return (`0`, `EOF`, or error sentinel) -> `std::io::Result<()>`
- `errno`-based failure reporting -> `std::io::Error`

### Error Handling Mapping
- C close failure with `errno` inspection -> `Err(std::io::Error)`
- Successful close -> `Ok(())`

### Memory/Resource Management
- C manual resource release via `fclose` -> Rust ownership consumption and explicit close-equivalent logic
- Ensure the close function consumes the owned handle where practical, so the resource cannot be reused after closure
- Avoid relying solely on implicit `Drop` when the C behavior requires observing close errors; implement an explicit path that returns `Result`

## Implementation Phases

### Phase 1: Module Skeleton and Type Decisions
- Create the Rust module file for `fclose.c`.
- Determine the exact Rust handle type from existing migrated call sites:
  - use `File` if the module is still operating at stream/file-object level
  - use `OwnedFd` only if the surrounding port has already reduced usage to raw descriptor ownership
- Define Rust function signatures for:
  - `fclose_nothrow`
  - `rpl_fclose`
- Document any unavoidable semantic differences between C `fclose` and Rust close/drop behavior directly in module comments, limited to migration-relevant notes.

### Phase 2: Port Core Close Logic
- Port `fclose_nothrow` first as the low-level helper.
- Implement explicit resource finalization with returned `std::io::Result<()>`.
- Preserve any special-case handling present in the C logic around close errors, suppressed errors, or errno-sensitive behavior.
- Port `rpl_fclose` second, reusing the helper where the C structure indicates that relationship.
- Keep logic local to this module; do not extract utility modules.

### Phase 3: Integrate Call Sites and Error Semantics
- Update the immediate callers in the main cluster to use the Rust replacements.
- Replace integer-status checks with `Result` handling while preserving the existing control-flow intent.
- Verify ownership transfer at each call site so that no handle remains accessible after successful or attempted close.
- Ensure no double-close paths remain after migration.

### Phase 4: Testing and Behavior Verification
- Add unit tests covering:
  - successful close
  - close on already-invalid/failed path if representable in the chosen Rust type model
  - propagation of close errors where the platform and test setup can trigger them
  - wrapper relationship between `rpl_fclose` and `fclose_nothrow`
- Run `cargo test`.
- Confirm the migrated module compiles cleanly on the target branch without adding unevidenced platform-specific support code.