# Implementation Plan

## Summary

Port the option-help context logic from `src/parseopt/help.c` into a Rust module that preserves existing behavior for option ordering and grouped help rendering. The Rust implementation should focus narrowly on migrating the functionality represented by `sort_options` and `print_option_group`, keeping the same operational flow: collect or view option descriptors, order them according to the original comparison rules, and emit formatted help output for each group.

The technical approach is to translate the C data flow into borrowed Rust data structures where possible, using slices and `Vec` for ordered collections and explicit enums/structs for formerly anonymous C records. Formatting should be implemented with `std::fmt` and string building or direct writer-based output, depending on the surrounding Rust port structure. Memory ownership should be made explicit so that temporary sorted views do not require unsafe code. Error handling should use `Result` only where output can fail; pure ordering logic should remain infallible.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the effective complexity of the C implementation for option sorting and group printing.
  - Avoid unnecessary cloning of option metadata during sorting; prefer sorting references or lightweight derived records.
  - Keep allocation limited to what is required for sorted ordering and formatted output assembly.
  - Match current module-scale performance expectations; no additional optimization work beyond faithful migration.

## Module Mapping

### C to Rust File Mapping

- `src/parseopt/help.c` -> `src/parseopt/help.rs`

### Function Mapping

- `sort_options` -> `sort_options`
  - Implement as an internal Rust function operating on slices or vectors of option/group descriptors.
  - Prefer sorting indices or references if the surrounding data is borrowed from a larger parser/help context.

- `print_option_group` -> `print_option_group`
  - Implement as an internal Rust function responsible for rendering one option group into a formatter, string buffer, or generic writer consistent with the rest of the Rust port.

### Module Placement

Follow the existing project path structure in Rust form:

- `src/parseopt/mod.rs` if already present in the port
- `src/parseopt/help.rs` for this migrated logic

Do not introduce extra helper modules unless they are required to replace file-local C state with Rust-local types.

## Data Model

The C analysis lists only anonymous structures, so the Rust port should introduce local named types only as needed to model the data actually consumed by `sort_options` and `print_option_group`. The goal is not to redesign the parser, but to give stable names to the records already implicit in the C implementation.

### Data-Structure Mapping Strategy

- **C anonymous structs used for option metadata** -> `struct OptionHelpEntry`
  - Holds the option attributes needed for sorting and display.
  - Expected fields should be limited to migrated needs, such as:
    - short option form
    - long option form
    - argument/metavariable text
    - help description text
    - grouping key / section reference
    - sort-related flags or priority values

- **C anonymous structs used for option groups** -> `struct OptionGroup`
  - Represents one logical help section and its option members.
  - Should contain:
    - group heading or label
    - optional descriptive text
    - collection of option entries or references

- **C anonymous structs used for display context** -> `struct HelpRenderContext`
  - Contains formatting inputs required by `print_option_group`.
  - Should be limited to values already present in the C flow, such as width, indentation, spacing, or destination writer abstraction.

- **C anonymous ad hoc sort/view records** -> `struct SortKey` or tuple-based temporary values
  - Use only if the original C function derives intermediate ordering fields.
  - If unnecessary, sort directly with `sort_by` on references.

### Ownership and Borrowing

- Use borrowed string data (`&str`) when the wider parser context owns the original text.
- Use owned `String` only when the Rust port already normalizes or constructs help text during parsing.
- For sorting:
  - Prefer `Vec<&OptionHelpEntry>` when reordering should not mutate the owning storage.
  - Prefer `&mut [OptionHelpEntry]` when the original C code sorts in place and ownership naturally belongs to this module.

### Error Handling

- `sort_options`:
  - Should be infallible unless the surrounding port introduces validated invariants.
  - Invalid or incomplete option data should be handled by preserving original ordering where possible rather than adding new behaviors.

- `print_option_group`:
  - If writing to an `io::Write` sink, return `std::io::Result<()>`.
  - If writing to a formatter, return `std::fmt::Result`.
  - Keep formatting failures propagated directly; do not add recovery paths.

### Memory Management Notes

- Replace C pointer traversal and manual lifetime management with:
  - slices for contiguous collections
  - `Vec` for temporary sorted views
  - `Option<T>` for nullable pointers
- Avoid `unsafe` unless forced by already-existing project interfaces; none should be necessary for this module migration.
- Do not mirror C anonymous layout mechanically; define only the fields read by these two functions.

## Implementation Phases

## Phase 1: Establish Rust Module Skeleton and Named Local Types

- Create `src/parseopt/help.rs`.
- Identify the exact inputs and outputs currently required by `sort_options` and `print_option_group` from the surrounding ported code.
- Introduce minimal Rust named types to replace the anonymous C records involved in:
  - option metadata access
  - group membership
  - formatting context
- Decide ownership boundaries:
  - borrowed entries if parser-owned
  - owned entries only if already transformed earlier in the Rust port
- Add unit-test scaffolding for:
  - option ordering cases
  - empty group rendering
  - single-option group rendering

## Phase 2: Port Option Ordering Logic

- Implement `sort_options` with Rust slice/vector sorting.
- Translate the original comparison behavior directly:
  - preserve primary and secondary ordering keys
  - preserve any original stable ordering expectations where equal keys occur
- If the C implementation sorts pointers to option records, mirror this with sorted references rather than cloning records.
- Validate edge cases through tests:
  - no options
  - mixed short/long options
  - equal sort keys
  - options with missing optional display fields
- Keep the function internal unless required elsewhere by the existing Rust module graph.

## Phase 3: Port Group Help Rendering Logic

- Implement `print_option_group` using the Rust output style already adopted in the port:
  - `fmt::Write`
  - `io::Write`
  - or string accumulation if the caller expects returned text
- Recreate the original formatting structure:
  - group heading emission
  - per-option line formatting
  - spacing/indentation behavior
  - handling of optional help text and argument notation
- Integrate `sort_options` into the rendering flow only where the C module does so.
- Add tests for:
  - group heading output
  - aligned rendering for multiple options
  - omission/presence of optional fields
  - deterministic output ordering

## Phase 4: Integrate and Validate Against Existing Behavior

- Wire `src/parseopt/help.rs` into the existing Rust parseopt module layout.
- Remove or avoid placeholder logic for these two functions in the branch.
- Run `cargo test` and fix any mismatches in formatting or ordering semantics.
- Perform a final pass for:
  - eliminating unnecessary allocations
  - ensuring no unsafe memory handling
  - confirming function visibility is no broader than needed
  - confirming the data model includes only fields used by this migrated module