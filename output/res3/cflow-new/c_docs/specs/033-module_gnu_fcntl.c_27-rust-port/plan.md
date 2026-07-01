# Implementation Plan: module_gnu_fcntl.c_27

## Summary

Port `gnu/fcntl.c` into a focused Rust module that preserves the current module scope: implementation of `dupfd`. The Rust version should mirror the existing low-level file-descriptor behavior closely, using minimal abstractions and keeping the logic centered on descriptor duplication and associated error propagation.

The implementation approach should favor the Rust standard library where possible, but because file-descriptor duplication is a low-level OS operation not fully covered by safe std APIs, the port should use direct Unix syscall bindings through Rust’s ecosystem with a narrow surface area. The Rust module should expose a function corresponding to `dupfd`, return explicit `Result`-based errors, and avoid introducing new capabilities beyond the migrated function.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**:
  - `std`
  - `libc` for `fcntl`/descriptor-related constants and syscall access on Unix
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the constant-time, syscall-dominated behavior of descriptor duplication.
  - Avoid heap allocation in the migrated path.
  - Keep wrapper overhead negligible relative to the underlying `fcntl` call.
  - Maintain equivalent OS-level semantics for success and failure cases.

## Module Mapping

- **C source**: `gnu/fcntl.c`
- **Rust target**: `src/module_gnu_fcntl.rs`

### Function Mapping

- `dupfd` -> `pub(crate) fn dupfd(...) -> Result<..., std::io::Error>`

### Scope Notes

- Only migrate logic required for `dupfd`.
- Do not split the implementation into extra helper modules unless required by Rust visibility or test organization.
- Keep Unix-specific behavior local to this module.

## Data Model

The analysis identifies only an **anonymous** data structure and no named persistent struct API.

### Data-structure Mapping

- Anonymous C data structure -> Prefer local Rust bindings, tuples, or block-scoped variables
- If the C implementation uses unnamed temporary storage for syscall arguments or intermediate results, map these directly to local variables with primitive Rust types such as:
  - `i32` for file descriptors and command values
  - `std::os::fd::RawFd` for descriptor parameters and returns where appropriate

### Ownership and Memory Management

- File descriptors remain non-owning integer handles unless the original function clearly transfers ownership.
- The migrated `dupfd` implementation should return a raw duplicated descriptor rather than introducing ownership wrappers unless the surrounding Rust code already expects owned descriptors.
- No heap-managed state is expected for this module.

### Error Handling

- Translate syscall failure into `std::io::Error::last_os_error()`.
- Preserve invalid-parameter and OS error behavior as closely as practical.
- Avoid panics in the operational path.

## Implementation Phases

### Phase 1: Module Skeleton and Signature Mapping

- Create `src/module_gnu_fcntl.rs`.
- Add the Rust function corresponding to `dupfd`.
- Define the Rust signature using Unix descriptor types (`RawFd`/`i32`) and `Result`.
- Establish any required imports from `std::os::fd` and `libc`.
- Keep visibility limited to the project’s actual call sites (`pub(crate)` by default).

### Phase 2: Core Logic Port

- Port the `dupfd` logic directly from `gnu/fcntl.c`.
- Map the descriptor duplication path to the appropriate `libc::fcntl` call and command constants.
- Reproduce C control flow with minimal restructuring so behavior remains easy to compare against the source.
- Handle all return-value checks explicitly:
  - successful duplicated descriptor
  - syscall failure with propagated OS error
- Keep unsafe usage tightly scoped around the syscall boundary and document why it is required.

### Phase 3: Test Migration and Edge Validation

- Add unit tests in the same module or the standard Rust test module layout.
- Cover:
  - successful duplication of a valid descriptor
  - failure on invalid source descriptor
  - behavior when the target minimum descriptor argument is invalid, if applicable to the source logic
- Use standard temporary file setup from `std` where sufficient.
- Confirm `cargo test` passes on supported Unix environments.

### Phase 4: Integration Cleanup

- Wire the new Rust module into the crate module tree.
- Remove or isolate any remaining references expecting the C implementation for `dupfd`.
- Verify type compatibility at call sites and adjust only as needed for `Result`-based error propagation.
- Keep the final module limited to the migrated functionality with no additional abstractions or utilities.