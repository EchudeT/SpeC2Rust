# spec.md

## Title

**Functional Specification: `module_src_collect_data_10` Rust Port**

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_src_collect_data_10`
- **Category**: `module_cluster`
- **Source basis**: `src/symbol.c`
- **Rust branch**: `073-module_src_collect_data_10-rust-port`
- **Generation date**: 2026-06-17

## Overview

This module provides symbol collection from an existing symbol table/list population in `src/symbol.c`. Its evidenced responsibility is to traverse stored symbol entries, apply an optional selection predicate, and produce a collected array of symbol references while preserving a caller-requested number of reserved leading slots.

The Rust rewrite must preserve this behavior as a bounded collection facility over the module’s existing symbol storage domain. The specification covers only functionality evidenced by:

- `collect_list_entry`
- `collect_symbols`
- `collect_data`

No additional capabilities are in scope.

---

## 1. Feature Specification

### 1.1 Purpose

The module gathers symbol entries from the module’s maintained symbol collection into a contiguous output sequence for downstream use.

### 1.2 Functional Scope

The Rust version must implement the following module behavior:

- Traverse symbol entries held by the module’s symbol storage.
- Evaluate each encountered symbol against a caller-supplied selection function when one is provided.
- Include matching symbols in the output collection.
- Support collection with a caller-specified count of reserved leading slots in the output sequence.
- Return the total number of collected symbols, excluding any reserved but unfilled leading slots.
- Provide the collected symbol sequence to the caller through an output parameter/result binding.

### 1.3 Observed Behavioral Model

Based on `collect_list_entry` and `collect_symbols`, the collection flow is:

1. A caller requests symbol collection.
2. The caller may provide:
   - a destination for the resulting symbol array,
   - a selection predicate,
   - a number of reserved slots.
3. The module iterates over symbol table/list entries.
4. For each entry, the module:
   - extracts the symbol reference associated with the entry,
   - applies the selection predicate if present,
   - appends accepted symbols after the reserved leading region.
5. The module returns the number of accepted symbols.

### 1.4 Out of Scope

The Rust port specification does not require or imply:

- creation, parsing, or mutation of symbols beyond collection behavior,
- new query semantics beyond optional predicate filtering,
- ordering guarantees not evidenced by the source,
- public APIs unrelated to symbol collection,
- concurrency behavior,
- persistence or serialization.

---

## 2. User Scenarios & Testing

### 2.1 Scenario: Collect all available symbols

**Description**
A caller needs a flat list of all symbols currently present in the module’s maintained symbol set.

**Expected behavior**
- The caller invokes collection with no restrictive selector.
- The module returns all available symbols.
- The returned count equals the number of symbols included.
- The output sequence begins at index `reserved_slots`.

**Test focus**
- Verify every stored symbol is returned when no selector excludes entries.
- Verify the count matches the number of returned symbols.

### 2.2 Scenario: Collect only symbols accepted by a predicate

**Description**
A caller needs only a subset of symbols, determined by a predicate supplied at collection time.

**Expected behavior**
- The module invokes the predicate on each candidate symbol.
- Only symbols for which the predicate accepts are placed into the output sequence.
- The returned count equals the number of accepted symbols.

**Test focus**
- Use a predicate that accepts a known subset.
- Verify rejected symbols do not appear in the output.
- Verify accepted symbols appear exactly once in the collected output.

### 2.3 Scenario: Reserve leading slots for caller-managed data

**Description**
A caller needs to prepend other entries before the collected symbols and therefore requests reserved leading capacity.

**Expected behavior**
- The module leaves the first `reserved_slots` positions unfilled by collected symbols.
- Collected symbols begin immediately after the reserved region.
- The returned count reports collected symbols only, not reserved slots.

**Test focus**
- Request one or more reserved slots.
- Verify collected symbols are written after the reserved region.
- Verify the reported count excludes the reserved region.

### 2.4 Scenario: No symbols satisfy the selector

**Description**
A caller requests collection with a predicate that rejects every symbol.

**Expected behavior**
- The module completes traversal successfully.
- The output contains no collected symbols after the reserved region.
- The returned count is zero.

**Test focus**
- Use a predicate that always rejects.
- Verify zero count and no collected entries.

### 2.5 Scenario: Empty symbol population

**Description**
A caller invokes collection when the underlying symbol storage contains no entries.

**Expected behavior**
- The module returns an empty result.
- The returned count is zero.
- No collected entries are produced.

**Test focus**
- Execute collection against an empty storage state.
- Verify zero count and empty collected output.

---

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Symbol collection entry point
The Rust module shall provide collection behavior equivalent to `collect_symbols`, producing a caller-observable sequence of symbol references from the module’s current symbol population.

**Traceability**: `collect_symbols` (`src/symbol.c:343-361`)

#### FR-2: Per-entry evaluation during traversal
The Rust module shall evaluate symbol entries one at a time during collection using behavior equivalent to `collect_list_entry`.

**Traceability**: `collect_list_entry` (`src/symbol.c:329-341`)

#### FR-3: Optional selection predicate
The Rust module shall support a caller-supplied selection predicate that determines whether an encountered symbol is included in the output.

**Traceability**: `collect_symbols` (`src/symbol.c:343-361`), `collect_list_entry` (`src/symbol.c:329-341`), `collect_data` (`src/symbol.c:305-310`)

#### FR-4: Inclusion of accepted symbols only
The Rust module shall include in the collected output only those symbols that satisfy the selection predicate, or all traversed symbols when no effective filtering is applied.

**Traceability**: `collect_list_entry` (`src/symbol.c:329-341`), `collect_symbols` (`src/symbol.c:343-361`)

#### FR-5: Reserved leading slots
The Rust module shall support a reserved leading region in the output collection, with collected symbols placed after `reserved_slots`.

**Traceability**: `collect_symbols` (`src/symbol.c:343-361`), `collect_data` (`src/symbol.c:305-310`)

#### FR-6: Collected count reporting
The Rust module shall report the number of symbols collected as the result of the operation.

**Traceability**: `collect_symbols` (`src/symbol.c:343-361`)

#### FR-7: Output binding
The Rust module shall expose the collected symbol sequence through the collection call’s output/result mechanism corresponding to `return_sym`.

**Traceability**: `collect_symbols` (`src/symbol.c:343-361`)

#### FR-8: Compatibility with existing symbol storage domain
The Rust module shall collect from the symbol entries maintained by the surrounding symbol table/list structures in `src/symbol.c`, rather than from an external source.

**Traceability**: `src/symbol.c`, `collect_list_entry` (`src/symbol.c:329-341`), linked-list and table-entry structures referenced in the module

### 3.2 Key Entities

#### Symbol
The primary collected entity. A symbol is the item evaluated by the selection predicate and the item returned in the collected output sequence.

**Traceability**: `collect_symbols`, `collect_list_entry`

#### Collect Data
A transient collection-state structure used to carry the destination sequence, current insertion position/count, selection predicate, and reserved-slot context across traversal.

**Traceability**: `collect_data` (`src/symbol.c:305-310`, and references at lines 316, 333, 347)

#### Table Entry
A storage record in the module’s symbol table domain from which a symbol is obtained during traversal.

**Traceability**: multiple `struct table_entry` declarations/references in `src/symbol.c`, including usage associated with collection logic

#### Linked List / Linked List Entry
Traversal-supporting storage structures used by the module to enumerate symbol-containing entries.

**Traceability**: multiple `struct linked_list` and `struct linked_list_entry` declarations/references in `src/symbol.c`

### 3.3 Entity Relationships

- The module’s linked-list/table-entry storage contains or links to symbol-bearing entries.
- Collection traverses those entries.
- `collect_data` carries traversal-time state.
- Accepted symbols are appended into the output sequence after any reserved leading slots.

---

## 4. Success Criteria

### 4.1 Behavioral Equivalence Criteria

1. **All-symbol collection works**
   Given a non-empty symbol population and a selector that does not exclude entries, the Rust module returns a collected sequence containing all eligible stored symbols and reports a count equal to the number returned.
   **Traceability**: `collect_symbols`, `collect_list_entry`

2. **Predicate filtering works**
   Given a selector that accepts only a known subset, the Rust module returns exactly that subset and reports the matching count.
   **Traceability**: `collect_symbols`, `collect_list_entry`, `collect_data`

3. **Reserved-slot behavior is preserved**
   Given `reserved_slots > 0`, collected symbols begin after the reserved leading region, and the reported count excludes reserved slots.
   **Traceability**: `collect_symbols`, `collect_data`

4. **Zero-match behavior is correct**
   When no symbol satisfies the selector, the Rust module reports zero collected symbols and produces no collected entries after the reserved region.
   **Traceability**: `collect_symbols`, `collect_list_entry`

5. **Empty-input behavior is correct**
   When the underlying symbol storage is empty, the Rust module reports zero collected symbols and returns an empty collected result.
   **Traceability**: `collect_symbols`

### 4.2 Port Completion Criteria

6. **Source-traceable scope only**
   The Rust implementation covers the collection functionality evidenced by `collect_symbols`, `collect_list_entry`, and `collect_data`, without introducing unrelated module responsibilities.
   **Traceability**: specified source functions and structure

7. **Storage-domain integration**
   The Rust implementation collects from the Rust port of the module’s existing symbol storage domain represented by table/list entities in `src/symbol.c`.
   **Traceability**: `src/symbol.c` storage structures and collection functions