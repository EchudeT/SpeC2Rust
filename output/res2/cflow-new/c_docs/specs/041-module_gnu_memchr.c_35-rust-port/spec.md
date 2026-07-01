# spec.md

## Title

Rust Functional Specification for `module_gnu_memchr.c_35`

## Document Control

- **Project**: `cflow-new`
- **Module**: `module_gnu_memchr.c_35`
- **Category**: `module_cluster`
- **Source basis**: `gnu/memchr.c`
- **Primary function in scope**: `__memchr`
- **Target Rust branch**: `041-module_gnu_memchr.c_35-rust-port`
- **Generation date**: `2026-06-17`

## Overview

This module provides byte-search functionality over a bounded memory region. Its purpose is to inspect up to `n` bytes starting at a caller-provided memory address and locate the first byte equal to a caller-provided target value.

The Rust rewrite must preserve the observable behavior of the source module: given a memory region, a byte value, and a length bound, it returns the location of the first matching byte within that bound, or indicates that no match exists in the inspected range.

This specification covers only the behavior evidenced by `gnu/memchr.c` and `__memchr`. No additional APIs or capabilities are in scope.

## Feature Specification

### Feature: Bounded byte search

The module shall provide functionality equivalent to `__memchr`:

- Accept a starting memory region reference, a target byte value, and a maximum number of bytes to inspect.
- Compare bytes in order from the beginning of the region.
- Stop at the first byte equal to the target value.
- If such a byte is found within the first `n` bytes, return its location relative to the provided region.
- If no such byte exists in the first `n` bytes, indicate no match.

### Behavioral scope

The Rust version must implement the following observable behavior:

- The search is limited strictly to the first `n` bytes.
- The target value is interpreted as a byte for comparison purposes.
- The result identifies the first matching byte, not later matches.
- A zero-length search produces no match.
- The operation is read-only with respect to the searched memory.

## User Scenarios & Testing

### Scenario 1: Match at the beginning

A caller searches a byte region whose first byte equals the requested value.

**Expected result**:
- The module reports a match at the first position.

**Test coverage**:
- Input region begins with the target byte.
- Length `n` is greater than zero.
- Returned location corresponds to offset `0`.

### Scenario 2: Match in the middle of the range

A caller searches a region where the first occurrence of the target byte appears after several non-matching bytes.

**Expected result**:
- The module reports the first matching position within the inspected range.

**Test coverage**:
- Region contains one or more leading non-matching bytes.
- A matching byte appears before `n`.
- Returned location corresponds to the first matching offset.

### Scenario 3: Multiple matches

A caller searches a region containing the target byte more than once.

**Expected result**:
- The module reports only the first occurrence.

**Test coverage**:
- Region contains at least two matching bytes within `n`.
- Returned location corresponds to the earliest one.

### Scenario 4: No match within range

A caller searches a region that does not contain the target byte in the first `n` bytes.

**Expected result**:
- The module indicates no match.

**Test coverage**:
- Region may contain no match at all, or only beyond `n`.
- Result is absence of a match.

### Scenario 5: Match exists beyond the bound

A caller provides a region where the target byte occurs after the first `n` bytes.

**Expected result**:
- The module indicates no match, because the search is bounded.

**Test coverage**:
- A matching byte exists only at an offset `>= n`.
- Result is absence of a match.

### Scenario 6: Zero-length search

A caller requests a search with `n = 0`.

**Expected result**:
- The module indicates no match without any successful comparison result.

**Test coverage**:
- Any input region and any target byte.
- Result is absence of a match.

### Scenario 7: Non-byte-sized integer input for target

A caller provides an integer target value whose low byte matches a byte in the region.

**Expected result**:
- Comparison behavior is based on the byte value used by the source function.

**Test coverage**:
- Use target values whose byte-truncated form is present in the region.
- Verify match behavior aligns with byte comparison.

## Requirements

### Functional Requirements

#### FR-1: Bounded search
The module shall search only within the first `n` bytes of the provided memory region.

**Traceability**:
- `gnu/memchr.c`
- `__memchr`

#### FR-2: Byte equality comparison
The module shall compare each inspected element as a byte against the requested target byte value.

**Traceability**:
- `gnu/memchr.c`
- `__memchr`

#### FR-3: First-match result
If one or more matching bytes exist within the inspected range, the module shall return the location of the earliest matching byte.

**Traceability**:
- `gnu/memchr.c`
- `__memchr`

#### FR-4: No-match result
If no matching byte exists within the inspected range, the module shall return a no-match result.

**Traceability**:
- `gnu/memchr.c`
- `__memchr`

#### FR-5: Zero-length handling
If `n` is zero, the module shall return a no-match result.

**Traceability**:
- `gnu/memchr.c`
- `__memchr`

#### FR-6: Read-only search behavior
The module shall not modify the searched memory as part of performing the search.

**Traceability**:
- `gnu/memchr.c`
- `__memchr`

### Key Entities

#### Entity: Memory region
A contiguous byte-addressable input region that serves as the search domain.

**Relationship**:
- `__memchr` reads from this region.
- The result, if any, identifies a position within this region.

#### Entity: Target byte
The byte value being searched for.

**Relationship**:
- `__memchr` compares each inspected byte in the memory region against this value.

#### Entity: Search bound
The maximum number of bytes to inspect from the start of the memory region.

**Relationship**:
- Constrains the portion of the memory region that `__memchr` may consider.

#### Entity: Match result
The outcome of the search: either a location of the first matching byte within the region and bound, or no match.

**Relationship**:
- Produced by `__memchr` from evaluating the memory region against the target byte under the search bound.

## Success Criteria

### SC-1: Correct first-match identification
For test inputs containing one or more target-byte matches within the first `n` bytes, the Rust implementation returns a result identifying the earliest such position.

**Traceability**:
- `__memchr`

### SC-2: Correct no-match behavior
For test inputs containing no target-byte match within the first `n` bytes, the Rust implementation returns no match.

**Traceability**:
- `__memchr`

### SC-3: Strict bound compliance
For test cases where a matching byte exists only at offsets greater than or equal to `n`, the Rust implementation returns no match.

**Traceability**:
- `__memchr`

### SC-4: Zero-length correctness
For test cases with `n = 0`, the Rust implementation returns no match.

**Traceability**:
- `__memchr`

### SC-5: Byte-value comparison correctness
For test cases using integer target inputs with varying higher-order bits, the Rust implementation behaves according to byte comparison semantics and matches the source module’s observable results.

**Traceability**:
- `__memchr`

### SC-6: Memory preservation
For all supported scenarios, the bytes in the searched region remain unchanged after the operation.

**Traceability**:
- `__memchr`

## Out of Scope

The Rust rewrite specification does not require or imply:

- Additional public search APIs beyond the behavior represented by `__memchr`
- New configuration mechanisms
- Concurrency guarantees
- Serialization or persistence behavior
- Error recovery systems
- Performance commitments beyond preserving functional behavior
- Any behavior not evidenced by `gnu/memchr.c`