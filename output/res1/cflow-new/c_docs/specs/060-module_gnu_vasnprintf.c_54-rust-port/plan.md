# Implementation Plan

## Summary

Port `gnu/vasnprintf.c` into an idiomatic Rust module that preserves the original allocation and size-calculation behavior needed for formatted string construction, with special attention to the `MAX_ROOM_NEEDED` logic. The Rust implementation should stay narrowly scoped to the existing module surface, translating C size and buffer-management rules into safe Rust using `usize`, checked arithmetic, and explicit error propagation where allocation or size growth would overflow practical limits.

The implementation should favor the Rust standard library. Since the analyzed surface only identifies `MAX_ROOM_NEEDED` and an anonymous C data shape, the Rust port should focus on reproducing the corresponding sizing computation and any immediate local helpers required to keep the translated file coherent, without introducing broader formatting infrastructure beyond what is necessary to migrate the existing file.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain linear-time buffer growth behavior equivalent to the C implementation.
  - Avoid unnecessary intermediate allocations during size computation and output assembly.
  - Use checked arithmetic to prevent integer overflow while keeping overhead minimal in non-error paths.
  - Preserve predictable memory growth characteristics for formatted output construction.

## Module Mapping

| C Source File | Rust Target File | Notes |
|---|---|---|
| `gnu/vasnprintf.c` | `src/module_gnu_vasnprintf_c_54.rs` | Direct migration target for the C module logic. |
| `MAX_ROOM_NEEDED` | `const fn` or private helper in `src/module_gnu_vasnprintf_c_54.rs` | Translate macro-style size computation into a typed Rust constant/helper using `usize` and checked math as needed. |

## Data Model

The analysis only identifies an anonymous data structure. The Rust port should therefore avoid inventing public data types unless they are required by the translated file’s internal logic.

| C Data Shape | Rust Mapping | Notes |
|---|---|---|
| anonymous local struct/container | private `struct` with named fields, if required | Introduce only when necessary to represent state currently held in C locals or anonymous aggregates. Keep visibility private to the module. |
| raw character buffer usage | `Vec<u8>` or `String` | Use `Vec<u8>` for byte-oriented construction when exact buffer growth control is needed; convert to `String` only when UTF-8 validity is guaranteed by the migrated logic. |
| C size types (`size_t`) | `usize` | Use checked arithmetic for growth and capacity calculations. |
| C integer status/error conventions | `Result<T, E>` | Replace sentinel returns with explicit Rust errors for overflow/allocation/state failures. Keep the error type module-local unless already standardized elsewhere in the crate. |

## Implementation Phases

### Phase 1: Establish file-level Rust translation scaffold

- Create `src/module_gnu_vasnprintf_c_54.rs` as the sole migration target for `gnu/vasnprintf.c`.
- Identify the exact C logic surrounding `MAX_ROOM_NEEDED` and place the corresponding Rust constant/helper in the same module.
- Translate C includes, macros, and local constants into Rust imports, `const`, and private helper functions.
- Keep the port file-centered: do not split logic into extra modules unless required by the existing crate layout.

### Phase 2: Port sizing and buffer-management logic

- Rewrite `MAX_ROOM_NEEDED` as a Rust helper with explicit integer types.
- Replace C pointer/length arithmetic with slice indexing, `Vec<u8>` capacity checks, and `usize`-based calculations.
- Use checked operations (`checked_add`, `checked_mul`) anywhere the original C code may rely on bounded `size_t` behavior.
- Preserve the original growth and reservation order so memory behavior remains close to the source implementation.
- If the C module uses anonymous temporary state holders, convert them into small private Rust structs with clearly typed fields.

### Phase 3: Integrate error handling and ownership rules

- Convert C failure paths that depend on null pointers or negative status returns into `Result`-based Rust flow.
- Ensure all temporary buffers are owned values with no aliasing assumptions from the C implementation.
- Audit all translated allocation points to guarantee that overflow and impossible-capacity states return an error instead of panicking.
- Keep APIs private unless the crate already requires a public entry point for this migrated module.

### Phase 4: Validate behavior with focused tests

- Add unit tests in the module or crate test layout that cover:
  - `MAX_ROOM_NEEDED` boundary behavior.
  - Small and large size-growth paths.
  - Overflow-protection cases.
  - Empty/minimal formatting-buffer scenarios relevant to the translated code.
- Use `cargo test` only; keep tests targeted to the migrated file behavior and avoid adding benchmark or integration infrastructure not evidenced by the source module.
- Confirm that the Rust implementation matches the C module’s expected size and allocation semantics for representative inputs.