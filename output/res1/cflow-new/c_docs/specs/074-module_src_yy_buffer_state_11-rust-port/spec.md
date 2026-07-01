# spec.md

## Title

Rust Functional Specification for `module_src_yy_buffer_state_11`

## Metadata

- Project: `cflow-new`
- Module: `module_src_yy_buffer_state_11`
- Category: `module_cluster`
- Source file: `src/c.c`
- Rust branch: `074-module_src_yy_buffer_state_11-rust-port`
- Generation date: `2026-06-11`

## Overview

This module manages lexer buffer objects and the internal buffer stack needed to support scanning input from files or caller-provided memory. The analyzed functionality is limited to:

- creating a new buffer associated with a file (`yy_create_buffer`)
- ensuring the internal buffer-state stack exists and can grow (`yyensure_buffer_stack`)
- wrapping an existing caller-provided memory region as a scan buffer (`yy_scan_buffer`)

The Rust rewrite must preserve the observable behavior of these responsibilities, including valid-buffer construction, stack availability/growth, and acceptance or rejection of external memory buffers based on required sentinel layout.

## Scope

### In Scope

- Representation of lexer buffer state corresponding to `yy_buffer_state`
- Creation of a buffer object for file-backed scanning
- Validation and creation of a buffer object over caller-supplied memory
- Allocation and growth of the internal stack that stores buffer-state references
- Preservation of behavior implied by these entry points and their related data structures

### Out of Scope

- Lexer tokenization behavior beyond what is required to create or register buffers
- Features not evidenced by the analyzed functions or referenced buffer-state types
- New public APIs or capabilities not present in the analyzed module slice

## Feature Specification

### Feature: File-backed buffer creation

The module shall provide functionality equivalent to `yy_create_buffer(file, size)`.

Behavior to preserve:

- Accept a file handle/reference and a requested buffer size.
- Create a new lexer buffer-state object associated with that file.
- Prepare storage sized for scanning content with the extra end-of-buffer sentinel space required by the lexer buffer model.
- Return a buffer-state handle/reference on success.
- Preserve failure signaling if allocation or buffer construction cannot be completed.

This feature exists to let the lexer establish a managed input buffer around a file source.

### Feature: Internal buffer-stack availability and growth

The module shall provide functionality equivalent to `yyensure_buffer_stack()`.

Behavior to preserve:

- Ensure that the internal stack used to hold buffer-state references exists before use.
- If the stack does not yet exist, allocate it and initialize its bookkeeping so buffer entries can be stored.
- If the stack exists but lacks capacity for additional entries, grow it while preserving existing stored buffer-state references.
- Leave the module in a state where subsequent buffer-stack use can proceed through the maintained stack structure.

This feature exists to support modules that switch between or register multiple lexer buffers.

### Feature: Scanning over caller-provided memory

The module shall provide functionality equivalent to `yy_scan_buffer(base, size)`.

Behavior to preserve:

- Accept a pointer/reference to an existing memory region and its total size.
- Validate that the supplied memory satisfies the lexer’s required end-of-buffer sentinel contract.
- Reject invalid memory regions that do not meet the sentinel requirement.
- For valid memory, create a buffer-state object that uses the supplied memory as its backing buffer rather than allocating replacement scan contents.
- Return a buffer-state handle/reference on success and failure signaling on invalid input or construction failure.

This feature exists to allow scanning over an externally managed byte buffer when it already has the required terminator layout.

## User Scenarios & Testing

### Scenario 1: Create a buffer for file input

A caller preparing the lexer to read from a file requests a new buffer with a given logical size.

Expected support:

- A buffer-state object is produced and associated with the provided file source.
- The returned object is usable as a managed lexer buffer.
- Failure is reported if the buffer cannot be created.

Testing implications:

- Verify successful creation with a valid file source and positive size.
- Verify that the resulting buffer object records file association and buffer sizing state.
- Verify failure behavior when construction resources are unavailable or invalid creation preconditions are not met.

### Scenario 2: Initialize buffer-stack state on first use

A caller or higher-level lexer path needs to store buffer references before any stack has been allocated.

Expected support:

- The internal stack is created on demand.
- Initial capacity and bookkeeping permit later insertion/use of buffer references.

Testing implications:

- Start from an uninitialized module state.
- Invoke stack assurance logic.
- Verify that stack storage exists afterward and can hold buffer-state references.

### Scenario 3: Grow the buffer stack when capacity is insufficient

The lexer has already stored buffer references and needs additional stack capacity.

Expected support:

- Stack growth occurs without discarding existing entries.
- Post-growth state remains valid for continued buffer management.

Testing implications:

- Populate the stack to its current capacity.
- Trigger stack assurance for additional growth.
- Verify that prior entries remain present and ordered after growth.

### Scenario 4: Scan a valid external memory buffer

A caller already owns a memory buffer whose trailing bytes satisfy the lexer buffer terminator requirement and wants the lexer to scan that memory directly.

Expected support:

- The module accepts the memory region.
- A buffer-state object is created that refers to the provided memory.
- No rejection occurs when sentinel bytes are correct.

Testing implications:

- Supply a buffer with the required trailing sentinel layout.
- Verify successful creation of a scan buffer object.
- Verify that the buffer-state object points to or represents the caller-provided memory region.

### Scenario 5: Reject an invalid external memory buffer

A caller provides memory that does not satisfy the required trailing sentinel contract.

Expected support:

- The module rejects the input.
- No valid scan buffer object is returned.

Testing implications:

- Supply a memory region missing the required sentinel bytes.
- Verify failure signaling and absence of a usable buffer-state result.

## Requirements

### Functional Requirements

#### FR-1: Buffer-state object creation for file input

The Rust module shall create a lexer buffer-state object from a file source and requested size, matching the role of `yy_create_buffer` in `src/c.c`.

Traceability:
- Function: `yy_create_buffer` (`src/c.c:2169-2194`)
- Type: `yy_buffer_state`

#### FR-2: Buffer creation shall account for lexer end-of-buffer space

When constructing a managed scan buffer for file input, the Rust module shall preserve the source module’s buffer model in which the created buffer includes the required extra space for lexer end-of-buffer sentinels.

Traceability:
- Function: `yy_create_buffer` (`src/c.c:2169-2194`)
- Type: `yy_buffer_state`

#### FR-3: Failure signaling for unsuccessful buffer creation

If a file-backed buffer cannot be created, the Rust module shall return failure in the Rust-appropriate representation rather than a usable buffer-state object.

Traceability:
- Function: `yy_create_buffer` (`src/c.c:2169-2194`)
- Function: `yy_scan_buffer` (`src/c.c:2417-2444`)

#### FR-4: On-demand initialization of buffer stack

The Rust module shall ensure that storage for buffer-state references is initialized before stack use, matching the role of `yyensure_buffer_stack`.

Traceability:
- Function: `yyensure_buffer_stack` (`src/c.c:2363-2407`)
- Type: `yy_buffer_state`

#### FR-5: Buffer stack growth with retention of existing entries

If existing buffer-stack storage is insufficient, the Rust module shall grow the stack while preserving previously stored buffer-state references.

Traceability:
- Function: `yyensure_buffer_stack` (`src/c.c:2363-2407`)
- Type: `yy_buffer_state`

#### FR-6: Construction of scan buffers over external memory

The Rust module shall support creation of a buffer-state object that wraps caller-provided memory, matching the role of `yy_scan_buffer`.

Traceability:
- Function: `yy_scan_buffer` (`src/c.c:2417-2444`)
- Type: `yy_buffer_state`

#### FR-7: Validation of external-memory sentinel layout

Before accepting caller-provided memory as a scan buffer, the Rust module shall validate that the memory region satisfies the lexer-required terminating sentinel layout; invalid regions shall be rejected.

Traceability:
- Function: `yy_scan_buffer` (`src/c.c:2417-2444`)
- Type: `yy_buffer_state`

### Key Entities

#### `yy_buffer_state`

Core lexer buffer record used by the analyzed functions.

Required role in the Rust rewrite:

- represent a scan buffer associated with either a file source or caller-provided memory
- carry the state needed for the lexer buffer model
- be storable in the internal buffer stack
- distinguish valid constructed buffer objects from failed construction outcomes

Traceability:
- Type references: `struct yy_buffer_state` in `src/c.c`
- Functions: `yy_create_buffer`, `yyensure_buffer_stack`, `yy_scan_buffer`

#### Internal buffer stack

Internal collection holding references/handles to `yy_buffer_state` objects.

Required role in the Rust rewrite:

- exist lazily or on demand
- track current stored buffer references and growth capacity
- support preservation of existing entries across expansion

Traceability:
- Function: `yyensure_buffer_stack` (`src/c.c:2363-2407`)
- Type usage: `struct yy_buffer_state` references in stack-related code

#### External scan memory region

Caller-supplied memory used as backing storage for a scan buffer when its size and trailing sentinel bytes are valid.

Required role in the Rust rewrite:

- serve as the backing store for scan-buffer creation
- be validated before acceptance
- remain conceptually distinct from module-allocated file-backed buffer storage

Traceability:
- Function: `yy_scan_buffer` (`src/c.c:2417-2444`)

## Success Criteria

1. A Rust test corresponding to file-backed buffer creation succeeds when given a valid file source and requested size, producing a valid buffer-state result.
   - Traceability: `yy_create_buffer`

2. A Rust test confirms that file-backed buffer creation reserves or models the required end-of-buffer sentinel space rather than only the requested logical size.

3. A Rust test confirms that unsuccessful file-backed or external-memory buffer construction returns failure rather than a usable buffer-state object.
   - Traceability: `yy_create_buffer`, `yy_scan_buffer`

4. A Rust test starting from an uninitialized stack state shows that invoking stack assurance creates usable stack storage for buffer references.
   - Traceability: `yyensure_buffer_stack`

5. A Rust test that forces stack growth verifies that previously stored buffer-state references are preserved after growth.

6. A Rust test using a caller-provided memory region with the required trailing sentinel bytes succeeds in creating a scan buffer.
   - Traceability: `yy_scan_buffer`

7. A Rust test using a caller-provided memory region without the required trailing sentinel bytes fails to create a scan buffer.

8. All supported scenarios in this document are covered by Rust tests and pass on the target branch.
   - Traceability: all analyzed functions and `yy_buffer_state`