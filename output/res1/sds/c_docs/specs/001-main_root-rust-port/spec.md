# spec.md

## Title

Rust Functional Specification for `main_root` Module

## Metadata

- Project: `sds`
- Module: `main_root`
- Category: `main`
- Source files analyzed: `sds.c`
- Rust branch target: `001-main_root-rust-port`
- Generation date: `2026-06-06`

## Overview

This module provides the core dynamic string behavior for the project. It creates, duplicates, frees, resizes, mutates, formats, and slices heap-allocated strings whose stored length is tracked separately from their NUL terminator. The Rust rewrite must preserve the observable behavior of this string facility, including support for binary-safe content, explicit length changes, append and copy operations, numeric conversion, formatted concatenation, trimming, range extraction, and ASCII case conversion.

The analyzed C module also uses multiple internal header layouts with different size classes to represent string metadata. The Rust rewrite must preserve the functional effect of this capacity-and-length model, but does not need to reproduce the C memory layout verbatim unless required internally by the Rust design.

## Scope

In scope for this module:

- Creating empty and non-empty dynamic strings from byte buffers and C strings.
- Duplicating and freeing dynamic strings.
- Updating, clearing, growing, shrinking, and querying allocation-backed string state.
- Appending, copying, and zero-growing string content.
- Converting signed and unsigned integers to string form.
- Appending formatted content through printf-style and limited custom-format paths.
- Trimming characters, selecting ranges, and converting to lowercase or uppercase.

Out of scope:

- Any behavior not evidenced by the analyzed functions and internal header structures.
- New public APIs not represented by the source module.
- Thread-safety, persistence, serialization, recovery, FFI contracts, or benchmark guarantees.

## Feature Specification

### 1. Dynamic string creation and lifecycle

The module must support creation of dynamic strings from:

- an arbitrary byte buffer and explicit length,
- an empty string,
- a NUL-terminated input string,
- a duplicate of an existing dynamic string,
- a signed integer converted to text.

Created strings must behave as mutable heap-backed strings with tracked logical length and a trailing NUL byte suitable for C-style string use.

The module must support releasing a previously created dynamic string and must tolerate a null-like input to free without requiring special handling by callers, matching the C module’s externally observable behavior.

### 2. Binary-safe length-tracked content

The module must preserve explicit logical length independently of embedded byte values. Operations that accept explicit lengths must not depend on interior NUL bytes. NUL termination must still be maintained after mutations so the string remains terminator-backed.

The module must support recomputing logical length from the first terminating NUL when requested, and resetting a string to empty without releasing its allocation.

### 3. Capacity management

The module must support reserving additional writable space for future appends and reducing excess free space while preserving current content. Allocation growth must maintain string validity and preserve prior content.

The module must expose behavior equivalent to:

- determining total allocation used by a string object,
- obtaining the allocation base pointer associated with the string object,
- increasing or decreasing the logical length within already allocated space.

The Rust rewrite must preserve these functional outcomes even if the internal representation differs.

### 4. Content growth and mutation

The module must support:

- increasing a string to a target length and filling newly exposed bytes with zero,
- appending arbitrary byte sequences,
- appending NUL-terminated text,
- appending another dynamic string,
- copying explicit-length content into an existing string,
- copying NUL-terminated text into an existing string.

These operations must update logical length, preserve valid terminator state, and reuse or expand allocation as needed.

### 5. Integer-to-string conversion

The module must support conversion of:

- signed `long long` values to decimal text,
- unsigned `long long` values to decimal text,
- signed `long long` values directly into a newly allocated dynamic string.

The decimal result must match normal base-10 textual representation, including sign handling for signed values.

### 6. Formatted concatenation

The module must support appending formatted text in two ways:

- via a `printf`/`va_list` style formatting path,
- via a custom compact formatter that appends according to the supported format specifiers implemented by the source module.

The Rust rewrite must preserve the observable append results for format patterns supported by the analyzed C implementation. It must not claim support for additional formatting directives beyond those evidenced by the source behavior.

### 7. Substring-style and character-set transformations

The module must support:

- trimming from both ends any bytes that belong to a caller-provided character set,
- restricting the string to a specified inclusive range with support for negative indexes relative to the end,
- converting ASCII letters to lowercase,
- converting ASCII letters to uppercase.

These operations must mutate the existing string and maintain valid length and terminator state.

## User Scenarios & Testing

### Scenario 1: Create and dispose of a binary-safe string

A caller creates a string from a buffer that may contain embedded NUL bytes, verifies that the stored logical length matches the provided length, uses the content in subsequent operations, and later frees it.

Tests should verify:

- creation from explicit bytes retains all bytes up to the provided length,
- embedded NUL bytes do not truncate logical length,
- a trailing terminator is present after the stored content,
- freeing a created string completes without requiring extra cleanup steps.

### Scenario 2: Start from empty and append incrementally

A caller starts with an empty string, reserves space, appends several byte sequences and text fragments, and ends with a single combined string.

Tests should verify:

- empty creation yields zero logical length,
- reserve operations preserve content and allow later growth,
- append-by-length, append-by-C-string, and append-by-dynamic-string produce the expected final content,
- length and terminator are correct after each append.

### Scenario 3: Reuse an existing allocation

A caller clears a string, copies new content into it, and later shrinks away unused free capacity.

Tests should verify:

- clearing resets logical length to zero without corrupting future reuse,
- explicit-length copy replaces prior content correctly,
- NUL-terminated copy replaces prior content correctly,
- removing free space preserves content and reduces spare capacity functionally.

### Scenario 4: Adjust visible length after direct buffer writes

A caller ensures room for additional bytes, writes directly into the writable tail, then updates the string length using an increment operation or by recomputing length from the terminator.

Tests should verify:

- making room preserves prior content,
- increasing length within available space exposes newly written bytes,
- decreasing length truncates content correctly,
- updating from terminator-derived length reflects manually written content,
- resulting strings remain NUL-terminated.

### Scenario 5: Grow with zero fill

A caller extends a string to a larger target length and expects any newly added region to be zero-filled.

Tests should verify:

- growth to a larger length preserves the original prefix,
- all newly added bytes are zero,
- growth to the current or smaller length does not introduce incorrect content changes,
- final terminator and logical length are correct.

### Scenario 6: Convert integers to text

A caller converts positive, negative, zero, and large integer values into textual form for later string composition.

Tests should verify:

- signed conversion emits correct decimal text for representative values,
- unsigned conversion emits correct decimal text for representative values,
- direct dynamic-string creation from a signed integer matches the standalone conversion result.

### Scenario 7: Append formatted data

A caller builds output text using formatted append operations.

Tests should verify:

- formatted append extends rather than replaces existing content,
- standard formatted append produces expected output for supported cases,
- custom formatter produces expected output for the supported directives implemented in the source module,
- multiple sequential formatting operations preserve order and string validity.

### Scenario 8: Trim and slice content

A caller trims unwanted edge characters and then selects a range using positive or negative indexes.

Tests should verify:

- trimming removes only bytes from the start and end that belong to the provided set,
- interior bytes not at the edges are preserved,
- range selection with positive indexes yields the expected substring,
- range selection with negative indexes counts from the end as implemented by the source module,
- out-of-range selections result in the same effective content as the C behavior.

### Scenario 9: ASCII case conversion

A caller normalizes text to lowercase or uppercase.

Tests should verify:

- lowercase conversion maps ASCII uppercase letters to lowercase,
- uppercase conversion maps ASCII lowercase letters to uppercase,
- non-ASCII or non-alphabetic bytes are preserved unchanged as in the C behavior,
- conversions occur in place and preserve string length.

## Requirements

### Functional Requirements

#### FR-1: Dynamic string creation
The module shall create a mutable dynamic string from an input pointer and explicit byte length, including the case of zero length.
Traceability: `sdsnewlen` in `sds.c`.

#### FR-2: Empty string creation
The module shall provide creation of an empty dynamic string.
Traceability: `sdsempty` in `sds.c`.

#### FR-3: NUL-terminated string creation
The module shall create a dynamic string from a NUL-terminated input string using the input’s current text length.
Traceability: `sdsnew` in `sds.c`.

#### FR-4: String duplication
The module shall create a duplicate dynamic string containing the same bytes and logical length as an existing one.
Traceability: `sdsdup` in `sds.c`.

#### FR-5: String release
The module shall release a dynamic string allocation and support null-like input without requiring caller-side guards.
Traceability: `sdsfree` in `sds.c`.

#### FR-6: Length recomputation and clearing
The module shall support recomputing stored logical length from the current terminating NUL and shall support resetting the string to empty while preserving object validity.
Traceability: `sdsupdatelen`, `sdsclear` in `sds.c`.

#### FR-7: Capacity growth
The module shall provide an operation to ensure space for at least a caller-requested amount of additional content while preserving existing bytes.
Traceability: `sdsMakeRoomFor` in `sds.c`.

#### FR-8: Capacity shrink
The module shall provide an operation to remove unused free space while preserving current content.
Traceability: `sdsRemoveFreeSpace` in `sds.c`.

#### FR-9: Allocation-related queries
The module shall provide access to the total allocation size associated with a dynamic string and to the base allocation pointer associated with the string object.
Traceability: `sdsAllocSize`, `sdsAllocPtr` in `sds.c`.

#### FR-10: Explicit length adjustment
The module shall support increasing or decreasing the logical length of a string by a signed increment, limited to already available storage, while maintaining a correct terminating NUL.
Traceability: `sdsIncrLen` in `sds.c`.

#### FR-11: Zero-filled growth
The module shall support growing a string to a specified logical length and filling any newly added region with zero bytes.
Traceability: `sdsgrowzero` in `sds.c`.

#### FR-12: Append operations
The module shall support appending:
- an explicit-length byte sequence,
- a NUL-terminated string,
- another dynamic string.
Traceability: `sdscatlen`, `sdscat`, `sdscatsds` in `sds.c`.

#### FR-13: Copy/replace operations
The module shall support replacing current content with:
- an explicit-length byte sequence,
- a NUL-terminated string.
Traceability: `sdscpylen`, `sdscpy` in `sds.c`.

#### FR-14: Integer formatting
The module shall convert signed and unsigned integer values to decimal character sequences and shall support creating a dynamic string from a signed integer value.
Traceability: `sdsll2str`, `sdsull2str`, `sdsfromlonglong` in `sds.c`.

#### FR-15: Formatted append
The module shall support appending formatted text through a variadic printf-style path and through the custom compact formatting path implemented by the source module.
Traceability: `sdscatvprintf`, `sdscatprintf`, `sdscatfmt` in `sds.c`.

#### FR-16: Edge trimming
The module shall remove leading and trailing bytes that belong to a caller-supplied character set.
Traceability: `sdstrim` in `sds.c`.

#### FR-17: Range selection
The module shall mutate a string to keep only the requested inclusive range of bytes, including support for negative indexes interpreted relative to the end, following the source behavior.
Traceability: `sdsrange` in `sds.c`.

#### FR-18: ASCII case conversion
The module shall support in-place lowercase and uppercase conversion for ASCII alphabetic bytes.
Traceability: `sdstolower`, `sdstoupper` in `sds.c`.

#### FR-19: Length/capacity class behavior
The module shall preserve the functional behavior of length- and capacity-tracked strings across the internal size classes represented by the analyzed header variants.
Traceability: `sdsHdrSize`, `sdsReqType`; internal structures `sdshdr5`, `sdshdr8`, `sdshdr16`, `sdshdr32`, `sdshdr64` in `sds.c`.

### Key Entities

#### Dynamic string object
The primary entity is the dynamic string returned and consumed by the module API. It represents mutable byte content with:

- a logical content length,
- backing allocation capacity or free space information,
- a trailing NUL terminator,
- content bytes that may include embedded NUL values when created or appended using explicit lengths.

Relationship traceability: all public string operations in `sds.c`.

#### Internal header variants
The source module defines several internal metadata layouts:

- `sdshdr5`
- `sdshdr8`
- `sdshdr16`
- `sdshdr32`
- `sdshdr64`

These variants encode length/capacity bookkeeping in different size classes. Their functional relationship is that each precedes or otherwise governs one dynamic string object and enables operations such as length reporting, capacity growth, shrinkage, and allocation queries.

Traceability: internal structures and helper functions `sdsHdrSize` and `sdsReqType` in `sds.c`.

#### Byte buffer and terminator
Each dynamic string has byte content and an ending NUL byte. Many operations mutate the byte buffer while requiring the terminator and stored logical length to remain synchronized.

Traceability: creation, mutation, append, copy, grow, trim, range, and case-conversion functions in `sds.c`.

## Success Criteria

### SC-1: Behavioral coverage
The Rust module exposes functionality covering all requirements FR-1 through FR-19 with no omitted evidenced capability from `sds.c`.

### SC-2: Binary-safe correctness
For inputs created or appended with explicit lengths, tests confirm that embedded NUL bytes are preserved in logical content and do not truncate stored length.
Traceability: FR-1, FR-12.

### SC-3: Terminator integrity
After every mutating operation covered by this specification, tests confirm the resulting string remains terminated by a trailing NUL byte.
Traceability: FR-1, FR-6 through FR-18.

### SC-4: Length and content correctness
For creation, append, copy, clear, range, trim, and case-conversion operations, tests confirm that both logical length and visible content match the C module’s behavior on representative normal and edge inputs.
Traceability: FR-1 through FR-18.

### SC-5: Capacity-operation preservation
Tests confirm that reserve and free-space-removal operations preserve prior content while functionally changing available spare capacity as intended.
Traceability: FR-7, FR-8, FR-19.

### SC-6: Direct-write adjustment correctness
Tests confirm that explicit length increment/decrement and length recomputation from terminator produce the same final logical content expected from the source behavior when callers write into reserved space.
Traceability: FR-6, FR-10.

### SC-7: Zero-growth correctness
Tests confirm that growth to a larger target length zero-fills only the newly added region and preserves the original prefix.
Traceability: FR-11.

### SC-8: Numeric conversion correctness
Tests confirm that signed and unsigned integer formatting matches decimal textual expectations for zero, positive values, negative signed values, and large boundary-representative values supported by the source types.
Traceability: FR-14.

### SC-9: Formatting-path correctness
Tests confirm that formatted append operations produce expected appended output for supported format cases from both the printf-style path and the custom compact formatter path.
Traceability: FR-15.

### SC-10: Range and trim compatibility
Tests confirm that trimming and inclusive range selection, including negative index handling, match the source module’s observable behavior on representative edge cases.
Traceability: FR-16, FR-17.

### SC-11: ASCII case conversion compatibility
Tests confirm lowercase and uppercase operations affect ASCII alphabetic bytes as expected and preserve string length and non-target bytes.
Traceability: FR-18.