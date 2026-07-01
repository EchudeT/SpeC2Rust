# Implementation Plan

## Summary
Port the C `safe-read` module into a small Rust module that preserves the existing low-level safe read/write retry behavior of `safe_rw` without adding new capabilities. The Rust implementation should keep the same operational scope: perform a single buffered I/O operation against an already-open file descriptor, retry on interrupt-related transient failures, and return the transferred byte count or an error in a form suitable for the surrounding `cat` port.

The technical approach should stay close to the C code’s structure:
- implement one Rust function corresponding to `safe_rw`,
- operate on Unix file descriptors,
- use standard-library byte slices for buffers,
- represent partial progress and syscall errors explicitly,
- avoid introducing broader abstractions beyond what is needed to migrate the existing file and function.

## Technical Context
- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - `libc` crate for direct `read`/`write` syscall access on Unix file descriptors, if the surrounding port already uses raw descriptor-based Unix I/O
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve syscall-oriented behavior with no unnecessary buffer allocation or copying,
  - match C-style retry behavior for interrupted operations,
  - keep per-call overhead minimal and suitable for repeated use in the main `cat` execution path.

## Module Mapping
| C File | Rust File | Notes |
|---|---|---|
| `safe-read.c` | `src/main_root_safe_rw_15.rs` or integrated into the existing main-cluster module file | Port the implementation of `safe_rw` directly. |
| `include/safe-read.c` | no separate Rust header file | C include-level sharing becomes normal Rust module visibility via `pub(crate)` as needed. |

| C Function | Rust Function | Notes |
|---|---|---|
| `safe_rw` | `pub(crate) fn safe_rw(...) -> Result<usize, std::io::Error>` or equivalent internal result type | Preserve retry semantics and partial-transfer handling. |
| `safe_rw` | single Rust definition | Duplicate listing in analysis appears to reflect declaration/definition, not separate behaviors. |

## Data Model
No dedicated structs are identified in this module.

C-to-Rust data mapping for function-level types:
| C Concept | Rust Mapping | Notes |
|---|---|---|
| file descriptor (`int`) | `std::os::unix::io::RawFd` | Keeps the same low-level Unix interface. |
| buffer pointer (`void *` / `char *`) | `&mut [u8]` for read, `&[u8]` for write, or a carefully scoped raw pointer internally | Prefer slices at the public function boundary. |
| byte count (`size_t`) | `usize` | Natural Rust size mapping. |
| result count (`ssize_t`) | `isize` internally, converted to `usize` on success | Needed only around the syscall boundary. |
| `errno`-based failure | `std::io::Error` | Preserve exact OS error codes where possible. |

If the original C function abstracts both read and write through a function pointer or operation selector, map that selector to a small internal Rust `enum`:
```rust
enum RwOp {
    Read,
    Write,
}
```
This enum should remain private to the module unless required elsewhere.

## Implementation Phases

### Phase 1: Inspect and shape the Rust module boundary
- Create the Rust module file for this migration in the main cluster using the existing crate layout.
- Identify the exact C `safe_rw` signature and whether it is used for reads, writes, or both via a shared helper pattern.
- Define the narrow Rust function signature to match current call sites rather than introducing a new generalized API.
- Decide module visibility (`pub(crate)` vs private) based only on existing usage in the ported code.

### Phase 2: Port syscall and retry logic
- Translate the `safe_rw` control flow directly from C to Rust.
- Use raw file descriptor operations to preserve C behavior.
- Handle interrupted syscalls (`EINTR`) by retrying in place.
- Preserve partial-transfer semantics exactly as in C:
  - return successful byte count when data was transferred,
  - propagate OS errors when no successful transfer occurred and the C logic does so.
- Keep pointer use confined to the syscall boundary; expose slice-based interfaces whenever practical.
- Avoid heap allocation and avoid wrapping this logic in broader reader/writer traits unless required by existing migrated call sites.

### Phase 3: Integrate with surrounding main-cluster code
- Replace references to the C module with imports from the Rust module.
- Update dependent call sites to pass Rust slices and raw file descriptors correctly.
- Normalize error propagation into the project’s existing Rust-side main-path handling, without changing behavior beyond what is needed for compilation and parity.
- Remove any now-obsolete duplicated declarations that existed only for the C include pattern.

### Phase 4: Validate parity with focused tests
- Add unit tests for the Rust `safe_rw` behavior using temporary files or pipes where appropriate.
- Cover:
  - successful full read/write,
  - partial transfer behavior where observable,
  - interrupted-call retry behavior as far as can be reasonably simulated,
  - error return propagation for invalid descriptors or closed pipe ends.
- Run `cargo test` and fix any platform-specific Unix issues in a minimal way consistent with the existing `cat` port.