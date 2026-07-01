# Implementation Plan

## Summary

Port the `main_root_stat_04` C module logic from `pwd.c` and `root-dev-ino.c` into a small Rust implementation that preserves current behavior and file-level responsibilities without adding new capabilities.

The Rust work should center on migrating:

- `logical_getcwd`
- `get_root_dev_ino`

Technical approach:

- Keep the implementation close to the existing C control flow.
- Prefer the Rust standard library for path handling, ownership, and error propagation.
- Use minimal OS-specific interop only where device/inode metadata is required and not exposed portably by `std`.
- Replace manual buffer and lifetime management with owned Rust types such as `PathBuf`, `OsString`, and plain structs.
- Convert integer/error-code style flows into `Result`-based APIs while preserving observable command behavior at the binary boundary.

## Technical Context

- **Language/Version**: Rust 1.77 or newer
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates required by the provided module evidence
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behaviorally equivalent filesystem traversal and metadata lookup cost
  - Avoid unnecessary path cloning and string conversion
  - Keep allocations bounded to path construction needs already inherent in the C implementation
  - Preserve direct metadata-based root detection rather than introducing extra filesystem passes

## Module Mapping

### C to Rust File Mapping

- `pwd.c`
  - migrate `logical_getcwd`
  - target Rust location: `src/main.rs` if still binary-local, or `src/pwd.rs` if the project already separates binary logic into modules
- `root-dev-ino.c`
  - migrate `get_root_dev_ino`
  - target Rust location: `src/root_dev_ino.rs`

### Function Mapping

- `logical_getcwd`
  - C role: derive current working directory through logical/path-aware handling
  - Rust target: `fn logical_getcwd(...) -> Result<PathBuf, io::Error>` or a binary-local equivalent matching existing call sites
- `get_root_dev_ino`
  - C role: obtain root filesystem device/inode identity
  - Rust target: `fn get_root_dev_ino() -> Result<RootDevIno, io::Error>`

### Boundary Decisions

- Keep the module boundary narrow:
  - one Rust module for root device/inode lookup
  - existing main/binary module for current-directory logic unless current code organization already requires a helper module
- Do not introduce additional abstraction layers beyond what is needed to replace the two C functions.

## Data Model

The analysis only identifies anonymous C data structures, with no named public struct surface. The Rust plan should therefore use only the minimum explicit data model needed by migrated functions.

### C Struct to Rust Mapping

- anonymous structs used only as temporary/local state
  - map to Rust local variables, tuples, or small private structs only when needed for clarity
- root device/inode holder from `root-dev-ino.c`
  - map to a private Rust struct:

```rust
struct RootDevIno {
    dev: u64,
    ino: u64,
}
```

### Type Mapping Notes

- C path buffers / `char *`
  - Rust: `PathBuf`, `OsString`, `&Path`
- C integer status returns
  - Rust: `Result<T, std::io::Error>` where OS calls can fail
- C device/inode integral fields
  - Rust: `u64` for stable internal comparison after extraction from platform metadata
- C manually managed temporary buffers
  - Rust: owned stack/local values and standard collections only where required

### Platform Metadata Access

For `get_root_dev_ino` and any metadata comparisons used by `logical_getcwd`:

- use `std::fs::metadata`
- on Unix, extract device/inode via `std::os::unix::fs::MetadataExt`
- keep this Unix-specific usage localized to the relevant module/function rather than spreading conditional logic through unrelated code

## Implementation Phases

### Phase 1: Establish Rust module skeleton and root metadata port

- Create the Rust destination for `root-dev-ino.c` as `src/root_dev_ino.rs` unless the existing project layout already embeds this logic elsewhere.
- Implement `RootDevIno` as a private or crate-visible struct only if needed by callers.
- Port `get_root_dev_ino` using:
  - root path metadata lookup
  - Unix metadata extraction for device and inode values
  - `io::Result` error propagation
- Add focused unit tests for:
  - successful retrieval on Unix
  - returned values being non-panicking and comparable
- Keep function names close to the source naming to ease review against the C code.

### Phase 2: Port `logical_getcwd` from `pwd.c`

- Translate the existing logic into Rust in the binary module or a single helper module.
- Preserve current algorithm choice and path assembly order rather than replacing it with a higher-level alternative unless the C logic is only wrapping standard current-directory retrieval.
- Use:
  - `env::current_dir` only where it matches the existing C behavior
  - `PathBuf`/component joins for path construction
  - metadata comparison where the C code relies on root device/inode checks
- Replace mutable C buffers with owned Rust path values.
- Convert all fallible steps to `Result`, letting the top-level caller decide final diagnostics/output behavior.

### Phase 3: Integrate call paths and align error handling

- Wire the migrated functions into the existing Rust binary flow.
- Ensure error conversions preserve the existing command exit behavior and user-visible failure conditions.
- Remove or retire the corresponding C-driven logic from the active path on this branch, keeping migration scope limited to these files/functions.
- Confirm there is no duplicated implementation of root metadata lookup or cwd resolution.

### Phase 4: Verification and cleanup

- Add `cargo test` coverage for:
  - root metadata helper behavior
  - logical cwd success path
  - failure propagation for invalid/unavailable filesystem state where practical
- Run formatting and standard lint checks used by the project.
- Review for:
  - no unnecessary heap copies beyond path ownership needs
  - no unsafe code unless unavoidable for a proven API gap
  - no extra modules or helper facilities beyond the migrated scope

## Notes on Memory Management and Error Handling

- Eliminate manual allocation/free patterns from the C implementation.
- Prefer borrowed path references internally and return owned `PathBuf` only at function boundaries that need ownership.
- Keep device/inode data as plain copyable values.
- Use `io::Error` directly for filesystem failures instead of introducing custom error enums unless an existing project type already requires adaptation.
- Avoid UTF-8 assumptions; keep filesystem paths as `OsStr`/`OsString`/`Path` types throughout.