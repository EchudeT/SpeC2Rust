# Implementation Plan

## Summary
Port `copy-file-range.c` into a focused Rust module that preserves the existing `copy_file_range` behavior and call shape as closely as practical within Rust conventions. The Rust implementation should center on a small, file-local function that performs range-based file copying using standard library file descriptors and OS-backed seek/read/write operations where available, without introducing broader abstractions or unrelated helpers.

The implementation should prioritize:
- matching existing control flow and return semantics,
- explicit handling of partial transfers and EOF,
- careful translation of system-call error paths into `std::io::Error`,
- avoiding unsafe code unless a syscall binding is strictly required by the source behavior.

## Technical Context
- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library (`std::fs`, `std::io`, `std::os::unix` as needed)
  - No third-party crates recommended from the provided evidence
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the module’s role as a low-overhead file-range copy path
  - Avoid unnecessary buffering or heap allocation beyond what is required for fallback transfer loops
  - Correctly handle partial reads/writes so large transfers remain efficient and accurate
  - Keep syscall count close to the C implementation’s behavior where possible

## Module Mapping
| C Source File | Rust Target File | Notes |
| --- | --- | --- |
| `copy-file-range.c` | `src/copy_file_range.rs` | Direct migration target for the file-range copy logic |
| `copy_file_range` | `copy_file_range` | Keep function purpose and narrow scope aligned with the C original |

## Data Model
The input identifies only an anonymous data structure.

| C Data Structure | Rust Mapping | Notes |
| --- | --- | --- |
| anonymous | Inline local state / private local variables | Prefer local variables or a private tuple/struct only if needed to mirror transient transfer state |
| file offsets / byte counts | `u64`, `usize`, or `i64` as appropriate | Choose signedness based on syscall/interface requirements; perform checked conversions |
| error returns | `std::io::Result<usize>` or `std::io::Result<u64>` | Final return type should reflect transferred byte count and propagate OS errors cleanly |

## Implementation Phases

### Phase 1: Module Skeleton and Signature Port
- Create `src/copy_file_range.rs` as the direct Rust destination for `copy-file-range.c`.
- Identify the exact C function signature inputs and map them to Rust file descriptor or file handle types with minimal interface expansion.
- Define the Rust `copy_file_range` function with a return type based on byte-count success and `std::io::Error` failure.
- Translate constants, counters, and offset-related locals into Rust scalar types with checked numeric conversion at boundaries.
- Keep the implementation self-contained rather than introducing extra modules.

### Phase 2: Core Transfer Logic Migration
- Port the main copy loop and any range/offset update behavior from C into Rust.
- Preserve handling for:
  - partial transfer results,
  - zero-byte results indicating completion,
  - interrupted operations and retryable errors where present in the C logic,
  - offset advancement for source and destination state.
- Use standard library Unix file-descriptor access where sufficient.
- If direct syscall behavior is required to preserve semantics and cannot be expressed with stable standard library APIs alone, isolate the minimal unsafe syscall invocation within this module and document its assumptions.
- Ensure ownership and borrowing rules prevent aliasing mistakes that were manually managed in C.

### Phase 3: Error Handling and Boundary Conditions
- Map C error paths to `std::io::Error` without broad custom error layers.
- Validate all count and offset conversions to avoid truncation or sign errors.
- Preserve end-of-file and short-copy semantics.
- Review any C assumptions about mutable offsets, null pointers, or optional parameters and represent them explicitly in Rust with `Option`, references, or mutable locals as needed.
- Confirm no memory-unsafe temporary buffer handling remains in the Rust version.

### Phase 4: Focused Tests and Integration Check
- Add unit or integration tests covering:
  - copying a full range between files,
  - partial range copy with explicit offsets,
  - zero-length copy behavior,
  - EOF-shortened copy,
  - propagation of invalid descriptor or permission errors.
- Use temporary files in tests via the standard library.
- Verify byte counts and file contents after each transfer.
- Run `cargo test` and confirm the new module integrates without requiring unrelated project restructuring.