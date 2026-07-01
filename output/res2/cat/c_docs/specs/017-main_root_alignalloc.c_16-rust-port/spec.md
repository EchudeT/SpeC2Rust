# Functional Specification: `main_root_alignalloc.c_16`

## Document Control

- **Project**: `cat`
- **Module**: `main_root_alignalloc.c_16`
- **Category**: `main_cluster`
- **Source file**: `alignalloc.c`
- **Rust branch target**: `017-main_root_alignalloc.c_16-rust-port`
- **Generation date**: 2026-06-07

## 1. Feature Specification

### 1.1 Purpose

This module provides aligned dynamic memory allocation paired with a matching deallocation routine.

Its functional role is:

- allocate a memory region of a requested size,
- ensure the returned pointer is aligned to a requested alignment boundary,
- retain enough internal bookkeeping so the original allocation can later be released correctly,
- free memory previously returned by this module.

### 1.2 In-Scope Functionality

The Rust rewrite must implement the same functional behavior evidenced by `alignalloc.c`:

- align an address downward to a specified alignment boundary,
- derive the storage location used to remember the original allocated pointer associated with an aligned returned pointer,
- allocate memory and return an aligned pointer for caller use,
- free memory using the aligned pointer previously returned by the allocator.

### 1.3 Behavioral Summary

The module behaves as a small allocation utility with paired operations:

- `alignalloc(alignment, size)` returns a usable pointer aligned to the requested boundary.
- `alignfree(ptr)` accepts a pointer previously returned by `alignalloc` and releases the underlying allocation.

The allocation behavior includes hidden metadata sufficient to recover the original malloc-returned address from the aligned address supplied to the caller.

### 1.4 Rust Port Scope

The Rust version must preserve the module’s functional boundary:

- provide aligned allocation behavior equivalent to the C module,
- preserve the requirement that deallocation uses the original underlying allocation rather than the aligned interior address,
- maintain the pairing contract between allocation and free.

No additional capabilities are required by this specification.

## 2. User Scenarios & Testing

### 2.1 Scenario: Request aligned storage for general use

A caller needs a dynamically allocated memory block whose starting address satisfies a requested alignment.

**Expected behavior**
- The caller requests `size` bytes with a given `alignment`.
- The module returns a pointer suitable for use as the base of that allocation.
- The returned pointer value is aligned to the requested boundary.

**Test focus**
- Verify returned address modulo `alignment` is zero.
- Verify the allocation can be written to for the requested byte range.

### 2.2 Scenario: Free an aligned allocation through the module’s paired free

A caller has finished using memory obtained from this module and must release it.

**Expected behavior**
- The caller passes the exact pointer previously returned by the aligned allocation routine.
- The module releases the correct underlying allocation.

**Test focus**
- Allocate with the module, then free with the paired free function without error.
- Repeat across several sizes and alignments supported by the module behavior.

### 2.3 Scenario: Internal recovery of original allocation address

The module returns an aligned pointer that may differ from the original allocator-returned address.

**Expected behavior**
- The module stores enough information near the aligned result to recover the original allocation address later.
- Deallocation uses that recovered address rather than assuming the aligned pointer itself is the original allocation.

**Test focus**
- Exercise allocations where alignment adjustment necessarily moves the returned pointer away from the original allocation base.
- Confirm paired deallocation still succeeds.

### 2.4 Scenario: Address alignment helper behavior

The module contains internal address adjustment behavior used to compute aligned addresses.

**Expected behavior**
- Given an input address and alignment, the helper computes the greatest aligned address not greater than the input address.

**Test focus**
- Unit-test helper behavior with representative synthetic addresses and alignments.
- Verify downward alignment semantics.

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1 Aligned allocation
The module shall provide an allocation operation that accepts an alignment value and a size value and returns a pointer for caller use.
**Traceability**: `alignalloc` in `alignalloc.c:74-102`

#### FR-2 Alignment guarantee on returned pointer
The allocation operation shall return a pointer whose address satisfies the requested alignment boundary.
**Traceability**: `alignalloc` in `alignalloc.c:74-102`; `align_down` in `alignalloc.c:44-49`

#### FR-3 Preservation of deallocation provenance
The allocation operation shall preserve access to the original underlying allocated pointer so that later deallocation can target the correct allocation base.
**Traceability**: `alignalloc` in `alignalloc.c:74-102`; `address_of_pointer_to_malloced` in `alignalloc.c:55-66`

#### FR-4 Paired deallocation
The module shall provide a free operation that accepts a pointer previously returned by the allocation operation and releases the corresponding underlying allocation.
**Traceability**: `alignfree` in `alignalloc.c:106-116`

#### FR-5 Metadata lookup relative to aligned pointer
The module shall support recovering the stored original allocation pointer from the aligned pointer returned to the caller.
**Traceability**: `address_of_pointer_to_malloced` in `alignalloc.c:55-66`; `alignfree` in `alignalloc.c:106-116`

#### FR-6 Downward alignment computation
The module shall support computing an address aligned downward from a candidate address according to a supplied alignment.
**Traceability**: `align_down` in `alignalloc.c:44-49`

### 3.2 Key Entities

#### KE-1 Aligned user pointer
A caller-visible pointer returned by the allocation operation. This is the pointer the caller uses and later supplies to the free operation.
**Relationships**
- Produced by the allocation operation.
- Must satisfy the requested alignment.
- Is associated with hidden metadata that identifies the original allocation.

**Traceability**: `alignalloc`, `alignfree`

#### KE-2 Original allocated pointer
The underlying allocation base returned by the backing allocator and required for correct release.
**Relationships**
- Stored in metadata associated with the aligned user pointer.
- Recovered during deallocation.
- Freed by the paired free operation.

**Traceability**: `alignalloc`, `address_of_pointer_to_malloced`, `alignfree`

#### KE-3 Pointer-storage location
An internal location derivable from the aligned user pointer where the original allocated pointer is stored.
**Relationships**
- Computed relative to the aligned pointer.
- Written during allocation.
- Read during deallocation.

**Traceability**: `address_of_pointer_to_malloced`

#### KE-4 Alignment value
An integer alignment parameter governing how the returned pointer address is adjusted.
**Relationships**
- Consumed by alignment computation.
- Constrains the returned user pointer.

**Traceability**: `alignalloc`, `align_down`

## 4. Success Criteria

### 4.1 Functional Correctness

- For representative supported alignment values and allocation sizes, the Rust allocation function returns pointers whose addresses satisfy the requested alignment.
  **Traceability**: `alignalloc`, `align_down`

- A pointer returned by the Rust allocation function can be passed to the Rust free function and the underlying allocation is released through the original allocation base, not by treating the aligned pointer as the base.
  **Traceability**: `alignalloc`, `address_of_pointer_to_malloced`, `alignfree`

- The Rust implementation preserves the pairing contract: memory allocated by the module is releasable by the module’s free routine using only the aligned returned pointer.
  **Traceability**: `alignalloc`, `alignfree`

### 4.2 Testability

- Unit tests verify downward alignment behavior for representative addresses and alignment values.
  **Traceability**: `align_down`

- Unit or integration tests verify metadata recovery from the aligned pointer is sufficient for deallocation.
  **Traceability**: `address_of_pointer_to_malloced`, `alignfree`

- Integration tests cover multiple allocation sizes and multiple alignment requests and confirm successful allocation followed by successful deallocation.
  **Traceability**: `alignalloc`, `alignfree`

### 4.3 Behavioral Equivalence

- The Rust module exposes no broader functional scope than aligned allocation and its paired free as evidenced by the C source module.
  **Traceability**: `alignalloc.c`

- The Rust rewrite maintains the same core semantic model: aligned returned pointer plus hidden recoverable original allocation pointer used during free.
  **Traceability**: `alignalloc`, `address_of_pointer_to_malloced`, `alignfree`