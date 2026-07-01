# Implementation Plan: main_root_quote_n_10

## Summary

This module ports the `quote_n_mem` and `quote_n` logic from `quotearg.c` into Rust for the `cat` project branch `011-main_root_quote_n_10-rust-port`.

The Rust implementation should preserve the current quoting-slot behavior and function-level semantics while replacing C-managed memory and pointer arithmetic with owned Rust data structures and slice-based processing. The work should stay narrowly scoped to the existing file and functions: migrate the quoting entry points, represent the required internal state in Rust, and keep allocation and lifetime handling explicit and safe.

The technical approach is:

- move the logic from `quotearg.c` into a single Rust module following standard crate layout,
- translate the per-index quoting storage used by `quote_n_mem` / `quote_n` into Rust-managed buffers,
- use `&[u8]` and `&str`/`String` where appropriate to replace raw pointer-plus-length interfaces,
- keep behavior compatible with the original C code’s indexing, buffer reuse expectations, and returned quoted data model,
- implement error handling with standard Rust result/ownership patterns where internal fallible allocation or indexing must be handled.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve linear-time quoting behavior relative to input size,
  - avoid unnecessary intermediate allocations beyond what is needed for the quoted output,
  - reuse per-slot storage where the C code reused heap buffers for repeated `quote_n*` calls,
  - maintain low overhead for the `quote_n` wrapper over `quote_n_mem`.

## Module Mapping

| C Source File | Rust Target | Notes |
| --- | --- | --- |
| `quotearg.c` | `src/quotearg.rs` | Port only the functionality needed for `quote_n_mem` and `quote_n`, keeping related local helper/state in the same file unless required by existing crate structure. |

| C Function | Rust Function | Mapping Notes |
| --- | --- | --- |
| `quote_n_mem` | `pub(crate) fn quote_n_mem(...)` | Main port target; accept byte slice plus slot index, produce quoted data using Rust-managed slot storage. |
| `quote_n` | `pub(crate) fn quote_n(...)` | Thin wrapper over `quote_n_mem`, using string-byte length derived in Rust. |

## Data Model

The C analysis reports multiple anonymous structures in `quotearg.c`. Since only `quote_n_mem` and `quote_n` are in scope, the Rust plan should map only the state actually touched by these functions and their immediate helpers.

| C Data Shape | Rust Mapping | Notes |
| --- | --- | --- |
| Anonymous quoting option/state structs | Private named `struct`s in `src/quotearg.rs` | Replace anonymous C aggregates with explicit Rust names based on role in the migrated logic. |
| Heap-managed character buffers | `Vec<u8>` and/or `String` | Use `Vec<u8>` for byte-exact quoted output; convert to `String` only if the original API usage guarantees text semantics. |
| Pointer + length input pairs | `&[u8]` | Direct replacement for `quote_n_mem` inputs. |
| NUL-terminated string input | `&str` or `&CStr`-equivalent internal handling | Prefer `&str` for `quote_n` if callers are already Rust-native; preserve byte-oriented logic internally if escaping is byte based. |
| Global/static slot vector in C | `Vec<QuoteSlot>` guarded by module-local mutable state pattern already used by the crate | Replace manual realloc growth with Rust vector growth, while keeping the same slot-index semantics. |
| Per-slot cached result buffer | `QuoteSlot { buf: Vec<u8> }` | Preserve overwrite/reuse behavior across repeated calls for the same slot. |
| Size/count fields | `usize` | Replace C integer sizes/counts. |
| Nullable pointers for optional state | `Option<T>` / `Option<&T>` | Use explicit optionality instead of null checks. |

### Proposed Rust Internal Structures

These names are implementation placeholders and should remain private unless existing crate APIs require exposure.

```rust
struct QuoteSlot {
    buf: Vec<u8>,
}

struct QuoteSlotStore {
    slots: Vec<QuoteSlot>,
}
```

If the migrated functions depend on option/config state from `quotearg.c`, introduce one private Rust struct matching only the fields actually used by `quote_n_mem` / `quote_n`, rather than porting all anonymous C structures wholesale.

## Implementation Phases

### Phase 1: Establish Rust module and state mapping

- Create `src/quotearg.rs` as the Rust destination for the migrated logic from `quotearg.c`.
- Identify the exact helper state and option fields referenced by `quote_n_mem` and `quote_n`.
- Replace anonymous C aggregates with minimally scoped private Rust structs.
- Define the per-slot storage model using `Vec<QuoteSlot>`.
- Decide the exact function signatures based on existing Rust crate call sites:
  - `quote_n_mem` should operate on explicit bytes and index,
  - `quote_n` should delegate with length derived from the input string/bytes.
- Keep the module narrowly focused; do not split out extra submodules.

### Phase 2: Port core quoting-slot behavior

- Translate slot growth logic from C realloc-style code to Rust vector resizing.
- Port the main quoting path for `quote_n_mem`, preserving:
  - slot selection by `n`,
  - output buffer replacement for the selected slot,
  - byte-exact handling of the provided memory region,
  - any required trailing terminator semantics as needed by the Rust-facing API.
- Port `quote_n` as a direct wrapper calling `quote_n_mem`.
- Replace pointer arithmetic with index/slice operations.
- Replace manual allocation/free behavior with owned buffers and scoped borrows.

### Phase 3: Integrate error handling and API-compatible returns

- Resolve how the Rust functions expose the quoted result:
  - borrowed view into slot storage,
  - owned `String`,
  - or owned `Vec<u8>`,
  based on existing crate usage and the need to model reusable slot buffers.
- Ensure invalid slot indexing, conversion assumptions, and allocation-related failures are handled idiomatically.
- Avoid `unsafe` unless strictly necessary to match existing crate-global mutable storage patterns; if unavoidable, isolate it to the smallest internal section and document invariants.
- Confirm repeated calls preserve the expected overwrite behavior for each quote slot.

### Phase 4: Validation and cleanup

- Add unit tests covering:
  - repeated use of the same slot,
  - use of different slot indices,
  - empty input,
  - non-empty byte input through `quote_n_mem`,
  - wrapper equivalence between `quote_n` and `quote_n_mem`.
- Compare behavior against the original C implementation for representative inputs handled by these two functions.
- Remove any unused translated fields or helper fragments not required by the scoped port.
- Run `cargo test` and finalize the module with standard Rust formatting and borrow-checker-clean ownership boundaries.

## Notes and Constraints

- Keep the migration limited to the existing `quotearg.c` functionality needed by `quote_n_mem` and `quote_n`.
- Prefer standard library types and patterns; no external crates are required by the provided inputs.
- Preserve C semantics where they affect observable output or slot reuse, but express them through Rust ownership rather than manual memory management.
- Do not add new abstraction layers, concurrency support, FFI surfaces, or unrelated quoting features beyond what is needed to port the listed functions.