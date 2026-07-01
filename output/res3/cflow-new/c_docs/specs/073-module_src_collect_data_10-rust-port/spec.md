# spec.md

## Title
Rust Functional Specification for `module_src_collect_data_10`

## Document Metadata
- Project: `cflow-new`
- Module: `module_src_collect_data_10`
- Category: `module_cluster`
- Source basis: `src/symbol.c`
- Rust branch: `073-module_src_collect_data_10-rust-port`
- Generation date: 2026-06-17

## Overview
This module provides symbol collection from an existing symbol table/list population. Its evidenced responsibility is to gather `Symbol` pointers that satisfy a caller-provided selection predicate into a contiguous result array, while reserving a caller-specified number of leading slots in that array.

The Rust rewrite must preserve this behavior: iterate over the module’s symbol entries, apply a selection function to each `Symbol`, collect matching symbols into an output collection, and honor reserved leading capacity/slots in the returned result structure.

## Scope
Included in scope:
- Collecting symbols from the module’s maintained symbol entries.
- Applying a caller-provided selection callback to decide inclusion.
- Returning the number of collected symbols.
- Producing an ordered result container of collected `Symbol` references/pointers with reserved leading slots.

Out of scope:
- Defining new symbol semantics beyond what is required for collection.
- Adding new filtering modes beyond the supplied selector callback.
- Adding persistence, serialization, concurrency guarantees, or foreign-function interfaces.

## Source Evidence
Primary functional evidence for this specification:
- `collect_list_entry(void *item, void *proc_data)` in `src/symbol.c:329-341`
- `collect_symbols(Symbol ***return_sym, int (*sel)(Symbol *p), size_t reserved_slots)` in `src/symbol.c:343-361`
- Supporting collection state via `struct collect_data` in `src/symbol.c:305-310`

## Feature Specification

### Feature: Filtered symbol collection
The module must provide functionality to traverse the module’s current symbol-entry population and collect only those `Symbol` items accepted by a caller-supplied selector.

Behavior required by the Rust version:
- Accept a selector function equivalent in role to `int (*sel)(Symbol *p)`.
- Visit symbol-bearing entries from the module’s maintained collection.
- For each visited symbol, invoke the selector.
- Include only symbols for which the selector indicates acceptance.
- Exclude symbols for which the selector indicates rejection.

### Feature: Result array construction with reserved leading slots
The module must construct a contiguous output symbol array/reference list for accepted symbols while leaving a caller-requested number of slots reserved at the beginning.

Behavior required by the Rust version:
- Support a `reserved_slots` input.
- Ensure collected symbols are placed after the reserved prefix region.
- Return the collected symbol count separately from the reserved slot count.
- Provide the caller access to the produced symbol list/array equivalent to the C `return_sym` out-parameter behavior.

### Feature: Collection count reporting
The module must report how many symbols were collected.

Behavior required by the Rust version:
- Return the number of accepted symbols actually placed into the result.
- Ensure the count reflects selector-filtered items only, not reserved slots.

## User Scenarios & Testing

### Scenario 1: Collect all symbols accepted by a predicate
A caller needs a list of symbols matching a known rule. It invokes the collection function with:
- an output location for the result array/list,
- a selector callback,
- zero reserved slots.

Expected behavior:
- Every symbol entry is evaluated once by the selector.
- The output contains exactly the accepted symbols.
- The returned count equals the number of accepted symbols.

Test expectations:
- Given a known symbol population and selector, the result count matches the number of selector-accepted symbols.
- The result contains no rejected symbols.

### Scenario 2: Reserve leading slots for caller-side later use
A caller intends to prepend additional symbols or metadata-managed entries externally and therefore requests reserved leading slots.

Expected behavior:
- The output collection is created with room for the reserved prefix.
- Collected symbols begin after that prefix.
- Returned collected count still reflects only accepted symbols.

Test expectations:
- With `reserved_slots = N`, accepted symbols occupy positions starting at index `N`.
- Positions before `N` remain reserved/unassigned according to the Rust API design, without being counted as collected results.

### Scenario 3: Selector rejects all symbols
A caller supplies a selector that rejects every symbol.

Expected behavior:
- The collection process completes successfully.
- The returned count is zero.
- The result collection contains no collected symbols beyond any reserved prefix.

Test expectations:
- Count is `0`.
- No accepted symbol entries appear in the result region.

### Scenario 4: Selector accepts all symbols
A caller supplies a selector that accepts every symbol.

Expected behavior:
- Every traversed symbol is included.
- The returned count equals the total number of traversed symbols.
- Output ordering follows the source traversal order used by the original module.

Test expectations:
- Count equals source symbol-entry count.
- Result sequence matches traversal order.

## Requirements

### Functional Requirements

#### FR-1: Symbol traversal
The Rust module shall traverse the module’s maintained symbol-entry collection as the basis for collection behavior.

Traceability:
- `collect_symbols` (`src/symbol.c:343-361`)
- `collect_list_entry` (`src/symbol.c:329-341`)

#### FR-2: Predicate-based inclusion
The Rust module shall apply a caller-provided selector to each traversed `Symbol` and include only those symbols accepted by that selector.

Traceability:
- `collect_symbols` (`src/symbol.c:343-361`)
- `collect_list_entry` (`src/symbol.c:329-341`)

#### FR-3: Output collection production
The Rust module shall provide the caller with a contiguous result collection equivalent in purpose to the C `Symbol ***return_sym` output parameter.

Traceability:
- `collect_symbols` (`src/symbol.c:343-361`)

#### FR-4: Reserved leading slots
The Rust module shall support a caller-specified number of reserved leading slots before the first collected symbol in the returned collection.

Traceability:
- `collect_symbols` (`src/symbol.c:343-361`)
- `struct collect_data` (`src/symbol.c:305-310`)

#### FR-5: Accurate collected count
The Rust module shall return the number of selector-accepted symbols collected, excluding any reserved leading slots.

Traceability:
- `collect_symbols` (`src/symbol.c:343-361`)

#### FR-6: Per-entry collection processing
The Rust module shall preserve the two-stage behavior evidenced by the source: per-entry processing during traversal and aggregate collection result formation.

Traceability:
- `collect_list_entry` (`src/symbol.c:329-341`)
- `collect_symbols` (`src/symbol.c:343-361`)
- `struct collect_data` (`src/symbol.c:305-310`)

### Key Entities

#### Symbol
The primary collected item. The module does not define new symbol meaning here; it operates on existing `Symbol` instances and returns references/pointers to selected ones.

Traceability:
- `collect_symbols` signature
- `collect_list_entry` selector use

#### Collection state (`collect_data`)
A transient state object used during collection to track selector, destination storage, current insertion position/count, and reserved-slot-aware output assembly.

Traceability:
- `struct collect_data` (`src/symbol.c:305-310`, references at 316, 333, 347)

#### Symbol-entry population
The underlying maintained list/table entries from which `Symbol` instances are traversed and evaluated.

Traceability:
- `collect_list_entry` item processing
- `collect_symbols` aggregate traversal initiation
- table/list entry references in `src/symbol.c`

#### Result symbol array/list
The produced contiguous output structure containing reserved prefix space plus collected `Symbol` items in traversal order.

Traceability:
- `collect_symbols` output parameter and `reserved_slots`

## Success Criteria

### SC-1: Correct filtering
For a fixed source symbol population and deterministic selector, the Rust module returns exactly the symbols accepted by the selector and no others.

Traceability:
- FR-2
- `collect_list_entry`
- `collect_symbols`

### SC-2: Correct count
For every invocation, the returned collected count equals the number of accepted symbols placed into the result, excluding reserved slots.

Traceability:
- FR-5
- `collect_symbols`

### SC-3: Reserved-slot preservation
When invoked with `reserved_slots = N`, the Rust result structure preserves `N` leading positions before the first collected symbol.

Traceability:
- FR-4
- `collect_symbols`
- `struct collect_data`

### SC-4: Order preservation
When multiple symbols are accepted, their order in the Rust result matches the traversal order used by the original module’s collection path.

Traceability:
- FR-1
- FR-6
- `collect_list_entry`
- `collect_symbols`

### SC-5: Empty-result handling
If no symbols are accepted, the Rust module completes the collection call successfully and reports a collected count of zero.

Traceability:
- FR-2
- FR-5
- `collect_symbols`