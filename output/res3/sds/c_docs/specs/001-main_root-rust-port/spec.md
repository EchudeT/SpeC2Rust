# spec.md

## Title

Rust Functional Specification for `main_root` in `sds`

## Metadata

- Project: `sds`
- Module: `main_root`
- Category: `main`
- Source basis: `sds.c`
- Rust branch: `001-main_root-rust-port`
- Generation date: `2026-06-09`

## Overview

This module provides the project’s core dynamic string behavior. It creates, owns, resizes, updates, and transforms mutable string values that track both current length and available capacity while remaining compatible with null-terminated byte-string use.

The Rust rewrite must preserve the functional behavior evidenced by `sds.c`, including:

- creation of empty and initialized dynamic strings
- duplication and destruction of string values
- explicit length maintenance
- capacity growth and capacity trimming
- append, copy, and zero-growth operations
- integer-to-string creation helpers
- formatted append operations
- trimming, slicing-by-range, and ASCII case conversion
- allocation-size and allocation-pointer related introspection behavior where represented in the Rust port

This specification defines required behavior only. It does not require preserving the original C memory layout as a public Rust API, except where behavior depends on the hidden header concept.

## Scope

Included in scope:

- Functional behavior represented by the exported module functions in `sds.c`
- Hidden metadata behavior associated with length, free space, and header sizing
- Mutation semantics for byte strings with trailing null termination

Out of scope:

- New APIs not evidenced in the source
- Thread-safety guarantees
- Serialization formats
- FFI contracts
- Performance targets beyond preserving functional growth/shrink behavior
- Any module behavior not traceable to the listed functions and header structures

## Feature Specification

### Dynamic string creation and ownership

The module must support creation of dynamic strings from:

- an explicit byte buffer and explicit length
- an empty value
- a C-style null-terminated string
- another existing dynamic string
- a signed integer converted to text

Created strings must behave as mutable owned string values. A null terminator must be maintained after the logical content so the byte buffer remains usable as a null-terminated string representation.

### Length-aware mutable string behavior

Each string must track:

- current used length
- total allocated capacity available for content
- remaining free space derivable from allocation and used length

The module must support clearing content without discarding the allocation, updating stored length from the current null-terminated content, and incrementing or decrementing logical length within valid bounds.

### Capacity management

The module must support:

- making room for additional appended bytes
- removing unused free space while preserving content
- reporting total allocation size
- exposing allocation-base identity/position behavior if the Rust design includes an equivalent for `sdsAllocPtr`

Growth behavior must preserve existing content and maintain valid null termination. Shrink-to-fit behavior must preserve content while reducing free space.

### Content extension and replacement

The module must support:

- appending arbitrary byte sequences of explicit length
- appending null-terminated strings
- appending another dynamic string
- replacing content with a specified byte sequence and explicit length
- replacing content with a null-terminated string
- growing to a requested length with newly added bytes zero-filled

These operations must correctly update stored length and preserve a trailing null terminator.

### Numeric formatting helpers

The module must support conversion of signed and unsigned integer values to their decimal textual form. It must also support creating a dynamic string from a signed integer.

### Formatted append

The module must support two formatted-append styles:

- printf-style formatting based on a format string and arguments
- a restricted fast-format append mode represented by `sdscatfmt`

The Rust version must preserve the source-evidenced outcome of appending formatted text to an existing dynamic string.

### In-place content transformation

The module must support:

- trimming from both ends using a set of bytes/characters
- restricting content to a specified range, including negative indexes relative to the end as evidenced by `sdsrange`
- converting content to lowercase in place
- converting content to uppercase in place

These operations must update logical length and maintain null termination.

## User Scenarios & Testing

### Scenario 1: Create and dispose of dynamic strings

A caller creates an empty string, creates another string from initial bytes, duplicates a string, and then frees the owned values.

The Rust version must support tests that verify:

- empty creation yields length 0
- creation from bytes preserves exact content length, including when input contains zero bytes
- duplication yields equal content but independent ownership
- freeing/dropping releases ownership without requiring caller-managed header logic

### Scenario 2: Use strings as mutable append buffers

A caller creates a string, reserves additional room, appends multiple fragments, and verifies the final content and length.

The Rust version must support tests that verify:

- reserving capacity does not alter existing content
- append-by-length accepts arbitrary bytes
- append-by-string and append-by-dynamic-string produce equivalent concatenation results
- the string remains null-terminated after mutation

### Scenario 3: Reuse an allocation after clearing

A caller clears a string and writes new content into the same logical object.

The Rust version must support tests that verify:

- clearing resets logical length to zero
- existing allocation can still be reused for subsequent writes
- content after rewriting matches the new value only

### Scenario 4: Grow with zero-filled extension

A caller requests that a shorter string be grown to a larger length.

The Rust version must support tests that verify:

- original prefix remains unchanged
- newly exposed bytes are zero-filled
- resulting logical length equals the requested length
- a null terminator is present after the logical end

### Scenario 5: Copy/overwrite existing content

A caller overwrites an existing string with shorter and longer replacement content.

The Rust version must support tests that verify:

- copy-by-length replaces prior content completely
- copy-by-C-string replaces prior content completely
- longer replacements trigger growth when required
- shorter replacements leave a correct logical length and terminator

### Scenario 6: Update length after direct content change model

The C module supports recomputing stored length from null-terminated content and explicit length adjustment. The Rust port must preserve this functional capability in whatever internal-safe form is chosen.

The Rust version must support tests that verify:

- recalculating length from current null-terminated content updates metadata correctly
- explicit length increase and decrease respect valid bounds
- after length adjustment, null termination and content prefix rules remain correct

### Scenario 7: Convert integers to strings

A caller converts signed and unsigned integers to decimal text, and creates a dynamic string from a signed integer.

The Rust version must support tests that verify:

- positive, zero, and negative signed values format correctly
- unsigned values format correctly
- resulting lengths match decimal representation length
- `from long long` creation yields exact decimal content

### Scenario 8: Append formatted output

A caller appends formatted text to an existing string using both formatting paths.

The Rust version must support tests that verify:

- formatted content is appended, not replaced
- repeated formatted appends preserve previous content
- integer and string substitutions produce correct output for source-evidenced formatting behavior

### Scenario 9: Trim and slice content in place

A caller trims unwanted boundary characters and then selects a range from the remaining content.

The Rust version must support tests that verify:

- trimming removes only leading and trailing bytes found in the trim set
- interior bytes are preserved
- range extraction works for positive indexes
- negative indexes behave relative to string end as in the source
- out-of-range selections result in the same effective content as the C behavior

### Scenario 10: Change ASCII case in place

A caller converts content to lowercase or uppercase.

The Rust version must support tests that verify:

- alphabetic ASCII bytes change case as expected
- non-alphabetic bytes remain unchanged
- logical length is unchanged

## Requirements

### Functional Requirements

#### FR-1: Dynamic string creation
The module shall create owned dynamic strings from explicit input bytes and length, from empty input, from null-terminated strings, from existing dynamic strings, and from signed integer values.
Traceability: `sdsnewlen`, `sdsempty`, `sdsnew`, `sdsdup`, `sdsfromlonglong`

#### FR-2: Null-terminated content model
The module shall maintain a trailing null byte after the logical content for all created and mutated strings.
Traceability: `sdsnewlen`, `sdsclear`, `sdsMakeRoomFor`, `sdsRemoveFreeSpace`, `sdsIncrLen`, `sdsgrowzero`, `sdscatlen`, `sdscpylen`, `sdstrim`, `sdsrange`

#### FR-3: Stored length management
The module shall maintain logical string length independently of total allocation and provide behavior to recalculate stored length from current null-terminated content, clear length to zero, and increment or decrement length within valid limits.
Traceability: `sdsupdatelen`, `sdsclear`, `sdsIncrLen`

#### FR-4: Capacity growth
The module shall support reserving additional capacity for future writes without losing existing content.
Traceability: `sdsMakeRoomFor`

#### FR-5: Capacity reduction
The module shall support removing unused free space while preserving current content and valid termination.
Traceability: `sdsRemoveFreeSpace`

#### FR-6: Allocation introspection
The module shall provide behavior equivalent to reporting total allocation size for a string. If the Rust port represents allocation-base access, it shall preserve the source-evidenced relation between string data and allocation pointer identity.
Traceability: `sdsAllocSize`, `sdsAllocPtr`

#### FR-7: Zero-growth behavior
The module shall support extending a string to a requested length and initialize any newly added content bytes to zero.
Traceability: `sdsgrowzero`

#### FR-8: Append operations
The module shall support appending explicit-length byte sequences, null-terminated strings, and other dynamic strings to existing content.
Traceability: `sdscatlen`, `sdscat`, `sdscatsds`

#### FR-9: Copy/replace operations
The module shall support replacing existing content from explicit-length input and from null-terminated input, growing allocation if needed.
Traceability: `sdscpylen`, `sdscpy`

#### FR-10: Integer text conversion
The module shall provide decimal text conversion behavior for signed and unsigned integer values and use signed conversion to create dynamic strings from integers.
Traceability: `sdsll2str`, `sdsull2str`, `sdsfromlonglong`

#### FR-11: Formatted append
The module shall support appending formatted text using a printf-style formatting path and a restricted format path corresponding to `sdscatfmt`.
Traceability: `sdscatvprintf`, `sdscatprintf`, `sdscatfmt`

#### FR-12: Boundary trimming
The module shall remove leading and trailing bytes that belong to a caller-provided trim set, without removing matching bytes from the interior.
Traceability: `sdstrim`

#### FR-13: Range selection
The module shall support in-place selection of a substring range using start and end indexes, including negative indexes relative to the current end, with source-matching behavior for empty and out-of-range results.
Traceability: `sdsrange`

#### FR-14: ASCII case conversion
The module shall support in-place lowercase and uppercase conversion of string content.
Traceability: `sdstolower`, `sdstoupper`

#### FR-15: Header class selection behavior
The module shall preserve the functional consequence of selecting different hidden header classes based on required string size and of deriving header size from that class. This behavior may remain internal in Rust but must support the same externally visible string operations.
Traceability: `sdsHdrSize`, `sdsReqType`, `sdshdr5`, `sdshdr8`, `sdshdr16`, `sdshdr32`, `sdshdr64`

### Key Entities

#### Dynamic string value
The primary entity is an owned mutable string object corresponding to `sds`, containing:

- logical content bytes
- current logical length
- allocation/capacity information sufficient to support free-space-aware operations
- a trailing null terminator after logical content

Relationship:
- All creation, append, copy, trim, range, and case-conversion operations read or mutate this entity.

Traceability: all exported string-manipulation functions in `sds.c`

#### Hidden header class
The source defines multiple hidden header classes representing different storage widths for metadata:

- `sdshdr5`
- `sdshdr8`
- `sdshdr16`
- `sdshdr32`
- `sdshdr64`

Relationship:
- Header class selection depends on required string size.
- Header size influences allocation layout and capacity bookkeeping.
- This is an internal representation concern whose externally visible effect is support for strings of different sizes with correct length/allocation behavior.

Traceability: `sdsHdrSize`, `sdsReqType`, header struct definitions

#### Numeric textual buffer behavior
The module includes helper behavior for converting integer values into decimal textual bytes before storing or appending them to dynamic strings.

Relationship:
- Supports standalone integer formatting functions and dynamic string creation from integer values.

Traceability: `sdsll2str`, `sdsull2str`, `sdsfromlonglong`, `sdscatfmt`

## Success Criteria

1. The Rust module can create, duplicate, clear, mutate, and destroy dynamic strings with content and logical lengths matching the C module for the covered operations.
   Traceability: `sdsnewlen`, `sdsempty`, `sdsnew`, `sdsdup`, `sdsfree`, `sdsclear`

2. For all supported mutation operations, the Rust module preserves a trailing null terminator immediately after the logical content.
   Traceability: `sdsnewlen`, `sdsclear`, `sdsMakeRoomFor`, `sdsRemoveFreeSpace`, `sdsIncrLen`, `sdsgrowzero`, `sdscatlen`, `sdscpylen`, `sdstrim`, `sdsrange`

3. Capacity growth and capacity trimming produce content-preserving results equivalent to the C behavior.
   Traceability: `sdsMakeRoomFor`, `sdsRemoveFreeSpace`

4. Append and copy operations produce byte-for-byte results equivalent to the C module for explicit-length input, C-string input, and dynamic-string input.
   Traceability: `sdscatlen`, `sdscat`, `sdscatsds`, `sdscpylen`, `sdscpy`

5. Length update operations behave equivalently to the C module for recalculation from current null-terminated content and explicit increment/decrement within valid bounds.
   Traceability: `sdsupdatelen`, `sdsIncrLen`

6. Integer formatting outputs match decimal text results of the C module for representative negative, zero, and positive signed values and for unsigned values.
   Traceability: `sdsll2str`, `sdsull2str`, `sdsfromlonglong`

7. Formatted append operations produce appended text equivalent to the C module for supported formatting behavior evidenced by `sdscatvprintf`, `sdscatprintf`, and `sdscatfmt`.
   Traceability: `sdscatvprintf`, `sdscatprintf`, `sdscatfmt`

8. Trim, range, lowercase, and uppercase transformations produce in-place results equivalent to the C module for normal and edge-case inputs.
   Traceability: `sdstrim`, `sdsrange`, `sdstolower`, `sdstoupper`

9. Allocation-size reporting in the Rust port is consistent with the port’s internal representation and preserves the source-evidenced meaning of total allocated storage for a string object.
   Traceability: `sdsAllocSize`

10. Any Rust-internal replacement for header-class logic preserves externally visible correctness across string sizes without requiring public exposure of the C header structures.
    Traceability: `sdsHdrSize`, `sdsReqType`, `sdshdr5`, `sdshdr8`, `sdshdr16`, `sdshdr32`, `sdshdr64`