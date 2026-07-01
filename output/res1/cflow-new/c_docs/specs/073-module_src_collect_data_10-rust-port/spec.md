# Functional Specification: `module_src_collect_data_10`

- **Project**: `cflow-new`
- **Module**: `module_src_collect_data_10`
- **Category**: `module_cluster`
- **Source basis**: `src/symbol.c`
- **Rust target branch**: `073-module_src_collect_data_10-rust-port`
- **Generation date**: `2026-06-11`

## 1. Feature Specification

### 1.1 Purpose

This module provides symbol collection from an existing symbol table/list owned elsewhere in `src/symbol.c`. Its primary responsibility is to traverse stored symbol entries, apply a caller-supplied selection predicate, and return a contiguous array of pointers to the symbols that match.

The Rust rewrite must preserve this behavior:

- iterate over existing symbol entries in the module’s maintained collection,
- optionally reserve leading slots in the returned array,
- include only symbols accepted by a caller-provided selector,
- report the number of collected symbols,
- expose the collected result as an ordered pointer/reference array suitable for downstream processing.

### 1.2 In-Scope Functionality

Based on `collect_list_entry` and `collect_symbols`, the Rust version must implement:

- collection of symbol references from module-managed symbol storage,
- per-symbol filtering through a callback/predicate,
- accumulation into a caller-visible array/vector-like result,
- support for a configurable number of reserved leading result slots,
- count reporting for the number of selected symbols.

### 1.3 Out of Scope

The following are not evidenced by this module slice and therefore are not required by this specification:

- creation or destruction of symbols,
- mutation of symbol properties during collection,
- sorting or deduplication beyond what naturally results from traversing the existing source collection,
- persistence, serialization, or external I/O,
- concurrency guarantees,
- recovery behavior beyond ordinary allocation/collection success expected by the original function contract.

## 2. User Scenarios & Testing

### 2.1 Scenario: Collect all eligible symbols

A caller needs a flat array of symbol references representing symbols currently stored by the symbol subsystem.

**Expected behavior**

- The caller provides a selector that accepts desired symbols.
- The module traverses the backing collection.
- Each accepted symbol is appended to the result array after any reserved leading slots.
- The module returns the number of accepted symbols.

**Testing focus**

- Given a backing collection with known symbols and a selector that accepts all of them, the returned count equals the number of symbols traversed.
- The result array contains all accepted symbols in traversal order.

### 2.2 Scenario: Filter symbols by caller policy

A caller wants only a subset of symbols, such as symbols matching a specific property represented in `Symbol`.

**Expected behavior**

- The selector is invoked for each traversed symbol.
- Only symbols for which the selector returns acceptance are included.
- Rejected symbols are absent from the returned array.
- The returned count reflects only accepted symbols.

**Testing focus**

- With a mixed input population and deterministic selector, the returned array contains exactly the accepted subset.
- The count matches the number of accepted entries.

### 2.3 Scenario: Reserve prefix space in the returned array

A caller needs the result array to leave some initial slots unused or preallocated for separate upstream logic.

**Expected behavior**

- The caller specifies `reserved_slots`.
- Collection begins writing accepted symbols after that reserved prefix.
- The count of collected symbols reflects only appended symbol entries, not the reserved prefix itself.

**Testing focus**

- When `reserved_slots` is nonzero, the returned storage has capacity for the reserved prefix plus collected symbols.
- Collected symbols start at the first slot after the reserved prefix.
- The reported count excludes reserved slots.

### 2.4 Scenario: No symbols match

A caller invokes symbol collection with a selector that rejects everything, or the backing collection is empty.

**Expected behavior**

- The module returns a collected count of zero.
- The output array/result remains valid for the caller’s use according to the original contract, with no accepted symbol entries populated beyond any reserved prefix.

**Testing focus**

- Empty backing collection yields zero count.
- Always-rejecting selector yields zero count even when symbols exist.

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Traverse stored symbol entries
The module shall traverse the module-managed symbol entry collection used by `collect_symbols`, sourcing symbols from the table/list structures defined in `src/symbol.c`.

**Traceability**: `collect_symbols`; linked-list and table-entry structures in `src/symbol.c`.

#### FR-2: Apply a caller-provided selection predicate
The module shall evaluate a caller-supplied selector for each candidate `Symbol` and shall collect only those symbols for which the selector indicates acceptance.

**Traceability**: `collect_symbols(Symbol ***return_sym, int (*sel)(Symbol *p), size_t reserved_slots)`; `collect_list_entry`.

#### FR-3: Accumulate accepted symbols into contiguous caller-visible storage
The module shall place accepted symbol references into contiguous result storage returned to the caller.

**Traceability**: `collect_symbols`; `collect_data` structure used during accumulation.

#### FR-4: Support reserved leading slots
The module shall honor the `reserved_slots` input by leaving that many leading positions available before storing collected symbol references.

**Traceability**: `collect_symbols(..., size_t reserved_slots)`; `collect_data` structure.

#### FR-5: Report collected symbol count
The module shall return the number of symbols accepted by the selector and stored in the result region.

**Traceability**: return type and behavior of `collect_symbols`.

#### FR-6: Perform per-entry collection through callback-style list traversal
The module shall support collection through a traversal callback that processes each list/table entry and updates shared collection state.

**Traceability**: `collect_list_entry(void *item, void *proc_data)`; `collect_data`.

### 3.2 Key Entities

#### Symbol
The domain object being collected. The selector operates on `Symbol`, and the result array contains references/pointers to `Symbol` instances.

**Relationship**:
- sourced from entries in the module’s symbol storage,
- filtered by selector,
- emitted into the returned collection.

#### Collect Data
A transient accumulation state object used during traversal to track result storage and collection progress.

**Relationship**:
- passed into per-entry processing,
- updated as accepted symbols are appended,
- carries state needed to respect reserved slots and build the final result.

#### Table Entry
An entry in the module’s symbol table structure that associates storage-layer entries with symbols.

**Relationship**:
- traversed or reached during collection,
- provides access path from storage structure to `Symbol`.

#### Linked List / Linked List Entry
The traversal structure used by the symbol subsystem for stored entries.

**Relationship**:
- defines the iteration order used during collection,
- supplies items processed by the per-entry callback.

## 4. Success Criteria

### 4.1 Behavioral Equivalence

- For a given symbol population and selector, the Rust module returns the same collected count as the C module.
- For a given symbol population, selector, and `reserved_slots`, the Rust module yields the same ordered set of collected symbol references after the reserved prefix as the C module.

### 4.2 Filtering Correctness

- Every returned symbol satisfies the caller-provided selector.
- No symbol rejected by the selector appears in the collected result.

### 4.3 Reserved Slot Correctness

- When `reserved_slots = N`, collected symbols begin at offset `N` in the returned storage.
- The reported collected count excludes the reserved prefix.

### 4.4 Empty and Zero-Match Handling

- If the source collection is empty, the returned collected count is `0`.
- If no traversed symbol satisfies the selector, the returned collected count is `0`.

### 4.5 Traceable Test Coverage

The Rust rewrite shall include tests demonstrating:

- all-match collection,
- selective filtering,
- nonzero reserved-slot handling,
- zero-match handling,
- empty-source handling.