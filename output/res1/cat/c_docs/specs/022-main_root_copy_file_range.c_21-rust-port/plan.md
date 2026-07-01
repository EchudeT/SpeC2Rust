# Implementation Plan

## Summary
Port `copy-file-range.c` into a Rust module that preserves the existing file-range copy behavior and boundary handling of `copy_file_range` without adding new features. The Rust implementation should stay close to the current C control flow: accept file descriptors or file-like handles at the module boundary used by the surrounding `cat` port, perform range-based copying using Rust’s standard-library Unix facilities where available, and translate OS errors into explicit `Result` values.

The implementation should prioritize:
- preserving existing semantics for partial copies, offsets, and end-of-file behavior,
- avoiding unnecessary buffering or extra allocations,
- using safe Rust by default, with narrowly scoped `unsafe` only if required for syscall-level interoperability on Unix.

## Technical Context
- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - `std::os::unix::io` for Unix file descriptor integration
  - No third-party crates recommended based on current evidence
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain syscall-level file-range copy performance comparable to the C implementation
  - Avoid introducing redundant userspace copy loops when the OS-native range copy path is available
  - Preserve efficient handling of large files and partial-transfer loops
  - Keep allocation overhead at zero or near zero for the core copy path

## Module Mapping
- **C source**: `copy-file-range.c`
- **Rust target**: `src/copy_file_range.rs`

### Function mapping
- `copy_file_range` -> `pub(crate) fn copy_file_range(...) -> std::io::Result<u64>`

### Integration approach
- Keep the Rust module focused on the direct port of the existing function.
- Expose only the function needed by the surrounding translated main-cluster code.
- Place any platform-specific imports inside this module rather than creating extra utility layers.

## Data Model
The C analysis reports only an anonymous data structure, so the Rust port should avoid inventing standalone types unless a local helper is required by the translated control flow.

### Data structure mapping
- **anonymous C data structure** -> inline Rust locals / tuple values / small private helper enum only if needed to represent branch state

### Type mapping guidance
- C file descriptors -> `std::os::unix::io::RawFd` or borrowed file descriptor types if the surrounding migrated code already uses them
- C size/count values -> `usize` for syscall-sized inputs, `u64` for accumulated byte counts where overflow-safe accumulation matters
- C return/error conventions -> `std::io::Result<T>`
- C offset values -> signed or unsigned integer types matching syscall expectations, likely `libc`-compatible integer widths only if required by the final call boundary; otherwise use standard Rust integer types and convert carefully

### Memory and error handling
- No manual heap management should be introduced.
- Convert errno-based failure paths into `std::io::Error`.
- Preserve partial-progress behavior: if a loop copies some bytes before an error condition that the C code treats specially, reflect that in the translated control flow rather than collapsing everything into one fallible call.
- Keep ownership external; the function should borrow or receive raw descriptors and should not close them.

## Implementation Phases

### Phase 1: Establish module skeleton and signature
- Create `src/copy_file_range.rs`.
- Add the Rust function corresponding to `copy_file_range` with a conservative signature aligned to the surrounding ported call sites.
- Map all C inputs, outputs, and return conventions into Rust types.
- Identify whether the surrounding code passes raw file descriptors, `File`, or borrowed descriptors, and keep this module compatible with that existing migration direction.
- Add module declarations/imports using only standard library Unix APIs unless syscall binding gaps force a minimal `unsafe` path.

### Phase 2: Port core copy logic
- Translate the main copy loop and range/offset handling from `copy-file-range.c` directly into Rust.
- Preserve the C branching structure for:
  - zero-byte transfer cases,
  - partial transfer continuation,
  - EOF detection,
  - OS error propagation,
  - any retry/termination decisions present in the original function.
- Use safe Rust where possible; if a direct syscall interface is necessary, isolate `unsafe` to the smallest call boundary and document its invariants.
- Ensure no descriptor ownership transfer or accidental closing occurs.

### Phase 3: Integrate error semantics and edge-case handling
- Verify integer conversions for counts and offsets to prevent truncation or sign errors.
- Match the original module’s behavior for invalid ranges, unsupported operations, and short-copy conditions.
- Confirm that the Rust function reports errors at the same decision points as the C version.
- Keep platform assumptions local to this module and avoid introducing compatibility abstractions not required by the source.

### Phase 4: Add focused tests and finalize wiring
- Add unit tests for the Rust module covering:
  - successful copy of a non-empty range,
  - zero-length copy behavior,
  - partial copy / EOF behavior,
  - invalid descriptor or unsupported-operation error propagation if reproducible in test conditions.
- Add integration-level coverage only where needed to confirm the function works with the surrounding `cat` main-cluster path.
- Run `cargo test` and fix any behavioral mismatches between the Rust translation and the original C implementation.