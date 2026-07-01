# spec.md

## Title

Rust Functional Specification for `main_root` (`sds`)

## Metadata

- Project: `sds`
- Module: `main_root`
- Category: `main`
- Source files analyzed: `sds.c`
- Rust branch target: `001-main_root-rust-port`
- Generation date: `2026-06-07`

## Overview

This module provides the core dynamic string behavior for the project. It creates, frees, resizes, updates, appends, copies, formats, trims, slices, and case-converts mutable strings while preserving a null-terminated byte buffer view for C-style string use.

The Rust rewrite must preserve the functional behavior evidenced by `sds.c`, including:

- creation of empty and initialized dynamic strings
- maintenance of logical string length and spare capacity
- explicit growth and shrink operations
- mutation by append, copy, clear, trim, range selection, and case conversion
- integer-to-string conversion helpers
- formatted append operations
- reporting of allocation-related size information

This specification covers only behavior evidenced by the analyzed module.

## Scope

### In Scope

The Rust module must implement the dynamic string functionality evidenced by these functions in `sds.c`:

- size/type selection helpers for internal string representation
- string creation, duplication, and destruction
- length synchronization and clearing
- capacity growth and removal of unused space
- allocation size and allocation-pointer reporting
- explicit logical length adjustment
- zero-growth expansion
- append and copy operations
- signed and unsigned integer formatting helpers
- dynamic string creation from integer values
- formatted append operations using printf-style formatting behavior
- trim, range, lowercase, and uppercase mutation

### Out of Scope

The Rust rewrite specification does not require any functionality not evidenced in the analyzed module, including but not limited to:

- new public APIs beyond the analyzed behavior
- concurrency guarantees
- persistence or serialization
- recovery or journaling mechanisms
- FFI surface design
- performance targets beyond preserving observed functional behavior

## Source Traceability

Primary source of evidence:

- `sds.c`

Primary functional evidence:

- `sdsHdrSize`
- `sdsReqType`
- `sdsnewlen`
- `sdsempty`
- `sdsnew`
- `sdsdup`
- `sdsfree`
- `sdsupdatelen`
- `sdsclear`
- `sdsMakeRoomFor`
- `sdsRemoveFreeSpace`
- `sdsAllocSize`
- `sdsAllocPtr`
- `sdsIncrLen`
- `sdsgrowzero`
- `sdscatlen`
- `sdscat`
- `sdscatsds`
- `sdscpylen`
- `sdscpy`
- `sdsll2str`
- `sdsull2str`
- `sdsfromlonglong`
- `sdscatvprintf`
- `sdscatprintf`
- `sdscatfmt`
- `sdstrim`
- `sdsrange`
- `sdstolower`
- `sdstoupper`

Primary data structure evidence:

- `sdshdr5`
- `sdshdr8`
- `sdshdr16`
- `sdshdr32`
- `sdshdr64`

## Feature Specification

### 1. Dynamic String Lifecycle

The module must support creation of dynamic strings from:

- arbitrary byte sequences with explicit length
- empty content
- null-terminated input strings
- copies of existing dynamic strings
- signed integer values converted to decimal text

The module must also support releasing strings that were created through this module's allocation path.

Behavioral expectations evidenced by the source:

- created strings expose content as a null-terminated byte sequence
- logical length is tracked independently from reserved capacity
- duplication produces an independent string with the same content and length
- empty-string creation returns a valid mutable dynamic string
- freeing a null string input is tolerated by `sdsfree`

### 2. Length and Capacity Management

The module must manage both used length and remaining free space.

The Rust rewrite must support:

- querying or deriving the allocation-backed size associated with a string
- obtaining the allocation base pointer equivalent for the underlying storage concept
- increasing reserved capacity to allow future writes
- removing unused capacity so allocation matches current content needs more closely
- updating stored length from the current null-terminated content
- clearing content without invalidating the string object
- explicit logical length adjustment by positive or negative increment within valid bounds
- growth to a target length with zero-filling of newly exposed bytes

Behavioral expectations evidenced by the source:

- capacity growth preserves existing content
- capacity removal preserves existing content and current logical length
- clear resets logical length to zero and leaves a valid empty null-terminated string
- updating length recalculates logical length from content up to the terminating null byte
- explicit length increment/decrement updates logical length and terminator placement
- zero-growth fills the newly added region with zero bytes and leaves the string null-terminated

### 3. Content Mutation by Append and Copy

The module must support extending and replacing string content.

The Rust rewrite must support:

- appending raw bytes of a specified length
- appending null-terminated string content
- appending another dynamic string's content
- copying raw bytes of a specified length into an existing string
- copying null-terminated string content into an existing string

Behavioral expectations evidenced by the source:

- append operations increase logical length and preserve prior content prefix
- copy operations replace logical content with the new content
- operations ensure the resulting content remains null-terminated
- operations grow storage as needed to accommodate the requested content

### 4. Numeric Conversion Helpers

The module must support conversion of integer values to decimal textual form.

The Rust rewrite must implement behavior corresponding to:

- converting signed long long values into decimal ASCII text in a caller-provided buffer
- converting unsigned long long values into decimal ASCII text in a caller-provided buffer
- creating a new dynamic string from a signed long long decimal representation

Behavioral expectations evidenced by the source:

- output is textual decimal representation
- helper functions report the resulting character count
- signed conversion handles negative values
- unsigned conversion does not emit a sign

### 5. Formatted Append

The module must support appending formatted textual output to an existing dynamic string.

The Rust rewrite must preserve the functional role of:

- append using a `va_list`-style formatting source
- append using variadic printf-style formatting
- append using the module's custom fast-format specifier handling

Behavioral expectations evidenced by the source:

- formatted output is appended, not replacing existing content
- resulting strings remain valid and null-terminated
- storage expands as needed to hold formatted results

This specification does not require introducing a new formatting language; it requires preserving the formatting behavior evidenced by the original module's supported formatted append paths.

### 6. Substring and Character-Set Mutation

The module must support in-place content reduction and transformation.

The Rust rewrite must support:

- trimming from both ends using a caller-supplied set of removable characters
- restricting a string to a specified inclusive range
- interpreting negative range indexes relative to the end of the string
- converting all characters in the string to lowercase
- converting all characters in the string to uppercase

Behavioral expectations evidenced by the source:

- trimming removes leading and trailing bytes present in the trim set, preserving interior bytes
- range selection mutates the string to the selected substring
- out-of-range or empty selections produce an empty valid string where applicable
- case conversion mutates bytes in place across the current logical content

## User Scenarios & Testing

### Scenario 1: Create and dispose of dynamic strings

A caller creates:

- an empty dynamic string
- a string from a null-terminated literal
- a string from a byte buffer with explicit length
- a duplicate of an existing dynamic string

The caller then frees each string.

The Rust version must support this workflow and preserve content, length, and valid empty/null-terminated state throughout.

### Scenario 2: Grow capacity before writing

A caller has an existing string and requests additional room for future append operations. The caller then appends bytes and later removes free space.

The Rust version must support:

- content preservation across growth
- successful append after growth
- preservation of content after free-space removal
- valid final null-terminated content

### Scenario 3: Synchronize length after direct content change

A caller modifies the visible byte content of a string and then requests length synchronization from the terminating null byte.

The Rust version must support updating the stored logical length to match the current text content.

### Scenario 4: Clear and reuse a string

A caller clears a populated string and then appends new content.

The Rust version must support clearing without invalidating the string object and must allow reuse for subsequent mutation.

### Scenario 5: Adjust logical length explicitly

A caller reserves room, writes into the available region, and then increases the logical length. A caller may also reduce the logical length.

The Rust version must support valid positive and negative length adjustments, with correct terminator placement and preserved prefix content.

### Scenario 6: Zero-grow a string

A caller expands a string to a target length larger than its current length and expects the new bytes between old and new length to be zero-filled.

The Rust version must support this behavior exactly for the newly added region.

### Scenario 7: Append and copy content in multiple forms

A caller:

- appends a fixed-length byte slice
- appends a null-terminated string
- appends another dynamic string
- copies fixed-length content into an existing string
- copies null-terminated content into an existing string

The Rust version must support each path and produce the expected final content and length.

### Scenario 8: Convert integers to strings

A caller converts signed and unsigned integer values to decimal text, and also creates a dynamic string directly from a signed integer.

The Rust version must support decimal conversion results matching the original module behavior for representative positive, zero, and negative values.

### Scenario 9: Append formatted text

A caller starts with an existing string and appends formatted content using the module's formatting entry points.

The Rust version must support formatted append semantics, including preserving the original prefix and appending the produced text.

### Scenario 10: Trim, slice, and case-convert content

A caller:

- trims selected edge characters
- selects a substring using positive indexes
- selects a substring using negative indexes from the end
- converts the result to lowercase
- converts the result to uppercase

The Rust version must support these in-place mutations with correct handling of empty and boundary cases.

## Requirements

### Functional Requirements

#### FR-1: Dynamic string creation
The module shall create a dynamic string from a byte sequence and explicit length, including zero-length input.
Traceability: `sdsnewlen`

#### FR-2: Empty string creation
The module shall create a valid empty dynamic string.
Traceability: `sdsempty`

#### FR-3: C-string creation
The module shall create a dynamic string from a null-terminated input string by using the input string length up to the first null byte.
Traceability: `sdsnew`

#### FR-4: String duplication
The module shall duplicate an existing dynamic string into independently allocated storage with identical content and logical length.
Traceability: `sdsdup`

#### FR-5: String release
The module shall release storage associated with a dynamic string and shall tolerate a null string input.
Traceability: `sdsfree`

#### FR-6: Internal representation selection
The module shall support internal header/representation selection according to required string size.
Traceability: `sdsHdrSize`, `sdsReqType`, `sdshdr5`, `sdshdr8`, `sdshdr16`, `sdshdr32`, `sdshdr64`

#### FR-7: Length synchronization
The module shall recompute and store the logical string length from the current null-terminated content.
Traceability: `sdsupdatelen`

#### FR-8: Clear operation
The module shall reset a string to empty content while keeping it valid for further use.
Traceability: `sdsclear`

#### FR-9: Capacity growth
The module shall enlarge available capacity for additional content while preserving existing content and logical length.
Traceability: `sdsMakeRoomFor`

#### FR-10: Free-space removal
The module shall reduce unused reserved space while preserving existing content and logical length.
Traceability: `sdsRemoveFreeSpace`

#### FR-11: Allocation size reporting
The module shall report the total allocation-backed size associated with a string object.
Traceability: `sdsAllocSize`

#### FR-12: Allocation base reporting
The module shall provide access to the allocation base associated with a string object.
Traceability: `sdsAllocPtr`

#### FR-13: Explicit length adjustment
The module shall adjust logical length by a signed increment within valid bounds and update the terminating null byte accordingly.
Traceability: `sdsIncrLen`

#### FR-14: Zero-filled growth
The module shall grow a string to a requested length, zero-filling any newly added region, and preserve prior content.
Traceability: `sdsgrowzero`

#### FR-15: Byte append
The module shall append a caller-provided byte sequence of specified length to an existing string.
Traceability: `sdscatlen`

#### FR-16: Null-terminated string append
The module shall append a null-terminated input string to an existing string.
Traceability: `sdscat`

#### FR-17: Dynamic string append
The module shall append the content of one dynamic string to another.
Traceability: `sdscatsds`

#### FR-18: Fixed-length copy
The module shall replace a string's content with a caller-provided byte sequence of specified length.
Traceability: `sdscpylen`

#### FR-19: Null-terminated string copy
The module shall replace a string's content with a null-terminated input string.
Traceability: `sdscpy`

#### FR-20: Signed integer formatting helper
The module shall convert a signed long long value into decimal text in a caller-provided buffer and return the produced text length.
Traceability: `sdsll2str`

#### FR-21: Unsigned integer formatting helper
The module shall convert an unsigned long long value into decimal text in a caller-provided buffer and return the produced text length.
Traceability: `sdsull2str`

#### FR-22: String creation from signed integer
The module shall create a new dynamic string containing the decimal text form of a signed long long value.
Traceability: `sdsfromlonglong`

#### FR-23: Formatted append via variadic-list source
The module shall append formatted text produced from a format string and variadic-list source to an existing string.
Traceability: `sdscatvprintf`

#### FR-24: Formatted append via variadic arguments
The module shall append formatted text produced from a format string and variadic arguments to an existing string.
Traceability: `sdscatprintf`

#### FR-25: Custom formatted append
The module shall append formatted text using the module's custom formatting routine.
Traceability: `sdscatfmt`

#### FR-26: Edge trimming
The module shall remove leading and trailing bytes that belong to a caller-provided trim set.
Traceability: `sdstrim`

#### FR-27: Range selection
The module shall mutate a string to retain only the specified inclusive range, supporting negative indexes relative to the end of the string.
Traceability: `sdsrange`

#### FR-28: Lowercase conversion
The module shall convert current string content to lowercase in place.
Traceability: `sdstolower`

#### FR-29: Uppercase conversion
The module shall convert current string content to uppercase in place.
Traceability: `sdstoupper`

### Key Entities

#### Dynamic string value
The main entity is a mutable dynamic string value whose content is represented as a null-terminated byte sequence while also carrying stored metadata for logical length and allocation characteristics.

Relationship to functions:
- created by `sdsnewlen`, `sdsempty`, `sdsnew`, `sdsdup`, `sdsfromlonglong`
- released by `sdsfree`
- mutated by the remaining string-manipulation functions

#### Header representations
The module defines multiple header representations:

- `sdshdr5`
- `sdshdr8`
- `sdshdr16`
- `sdshdr32`
- `sdshdr64`

These represent alternative metadata layouts for dynamic strings based on needed size range. The Rust rewrite must preserve the functional consequence of this design: string behavior must support different size classes as selected by required string size.

#### Logical length
Each dynamic string has a logical content length, distinct from any spare capacity. This length is used by append, copy, trim, range, and case-conversion operations.

Relationship to functions:
- set during creation and copy/append operations
- recomputed by `sdsupdatelen`
- reset by `sdsclear`
- adjusted by `sdsIncrLen`
- expanded by `sdsgrowzero`

#### Spare capacity / allocation-backed storage
Each dynamic string has allocation-backed storage that may exceed the current logical length. This enables growth without reallocation on every write.

Relationship to functions:
- increased by `sdsMakeRoomFor`
- reduced by `sdsRemoveFreeSpace`
- described by `sdsAllocSize`
- referenced by `sdsAllocPtr`

## Success Criteria

### Functional Correctness

1. The Rust module can create, duplicate, clear, and free dynamic strings with behavior matching the source module for empty, non-empty, and explicit-length inputs.
   Traceability: `sdsnewlen`, `sdsempty`, `sdsnew`, `sdsdup`, `sdsfree`

2. For all supported mutation operations, resulting content remains valid as a null-terminated byte sequence and logical length matches the visible content length intended by the operation.
   Traceability: `sdsnewlen`, `sdsclear`, `sdsIncrLen`, `sdsgrowzero`, `sdscatlen`, `sdscpylen`, `sdstrim`, `sdsrange`

3. Capacity growth and free-space removal preserve pre-existing content bytes up to the prior logical length.
   Traceability: `sdsMakeRoomFor`, `sdsRemoveFreeSpace`

4. Length synchronization and explicit length adjustment produce the same logical length outcomes as the source behavior for valid inputs.
   Traceability: `sdsupdatelen`, `sdsIncrLen`

5. Zero-growth sets each newly added byte between old length and target length to zero.
   Traceability: `sdsgrowzero`

6. Append and copy operations produce correct final content and length for raw bytes, null-terminated strings, and dynamic-string inputs.
   Traceability: `sdscatlen`, `sdscat`, `sdscatsds`, `sdscpylen`, `sdscpy`

7. Signed and unsigned integer conversion helpers return decimal text and the correct produced length for representative boundary-oriented cases including zero, positive values, and negative signed values.
   Traceability: `sdsll2str`, `sdsull2str`, `sdsfromlonglong`

8. Formatted append operations preserve the original prefix and append produced formatted text correctly.
   Traceability: `sdscatvprintf`, `sdscatprintf`, `sdscatfmt`

9. Trim, range, lowercase, and uppercase operations produce the expected in-place result for normal and edge cases, including empty results and negative range indexing.
   Traceability: `sdstrim`, `sdsrange`, `sdstolower`, `sdstoupper`

### Testability Expectations

10. Each functional requirement in this specification is covered by automated tests in the Rust rewrite.
    Traceability: FR-1 through FR-29

11. Scenario-based tests exist for all user scenarios listed in this document.
    Traceability: User Scenarios 1 through 10

12. Allocation-related reporting behavior is verified sufficiently to show that reported allocation size/base relationships remain internally consistent for the Rust design used to preserve module functionality.
    Traceability: `sdsAllocSize`, `sdsAllocPtr`