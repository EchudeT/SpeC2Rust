# spec.md

## Title
Rust Functional Specification for `module_gnu_getdtablesize.c_29`

## Metadata
- Project: `cflow-new`
- Module: `module_gnu_getdtablesize.c_29`
- Category: `module_cluster`
- Source file: `gnu/getdtablesize.c`
- Rust branch: `035-module_gnu_getdtablesize.c_29-rust-port`
- Generation date: `2026-06-17`

## Overview
This module provides the project’s behavior for obtaining the process file descriptor table size limit in a GNU compatibility context.

The Rust rewrite must preserve the observable behavior of the C module: it must provide the module-level capability to determine the descriptor-table size limit used by callers, using the platform limit information available to the process and preserving failure-tolerant behavior where the original module uses non-throwing limit adjustment support.

This specification is limited to behavior evidenced by:
- `gnu/getdtablesize.c`
- `_setmaxstdio_nothrow`
- use of `struct rlimit`

## Feature Specification

### Purpose
The module supplies a file-descriptor table size value for the current process environment.

### Functional Scope
The Rust version must implement behavior equivalent to the C module’s supported scope:

1. Determine a descriptor-table size limit for the running process.
2. Use operating-system-provided limit information where available.
3. Preserve compatibility-oriented behavior for environments where the effective descriptor count may depend on a standard-I/O or process open-file limit.
4. Avoid surfacing internal adjustment failures as hard module failures when the original behavior is explicitly non-throwing.

### Out of Scope
The Rust version must not introduce unevidenced capabilities, including:
- new public configuration APIs
- persistence or serialization
- thread-safety guarantees beyond normal Rust safety
- recovery or retry policies not present in the source behavior
- benchmarking or tuning interfaces

## User Scenarios & Testing

### Scenario 1: Query effective descriptor table size
A caller needs the module-provided descriptor-table size limit for the current process.

**Expected behavior**
- The module returns a non-negative size representing the effective descriptor-table capacity used by the process context.
- The value reflects process/system limit information rather than an invented constant when such information is available.

**Test guidance**
- Run in an environment with a known open-file limit.
- Verify the Rust implementation returns a value consistent with the active process limit behavior supported by the platform.

### Scenario 2: Process limit available through resource limits
A caller executes on a platform where descriptor limits are exposed through resource-limit facilities.

**Expected behavior**
- The module derives its result from the available resource-limit data.
- The returned value is compatible with the limit semantics represented by `struct rlimit`.

**Test guidance**
- In a controlled test process, set or observe the open-file resource limit.
- Verify the returned value tracks the relevant resource-limit value within the same process.

### Scenario 3: Non-throwing adjustment path
The module operates in an environment where an internal maximum-stdio adjustment attempt is relevant.

**Expected behavior**
- If the adjustment path fails, the module does not convert that failure into an unexpected hard error solely because the helper is defined as non-throwing.
- The module still produces behavior consistent with the C module’s compatibility intent.

**Test guidance**
- Exercise the code path corresponding to non-throwing max-stdio adjustment behavior.
- Verify failure in that path is handled without panicking or introducing a new externally visible error mode.

### Scenario 4: Large process limit values
A caller runs in a process environment with a large open-file limit.

**Expected behavior**
- The module returns a value that preserves the effective supported limit within the target return type constraints used by the original module behavior.
- The Rust version does not silently invent a higher value than the source behavior would support.

**Test guidance**
- Execute under a raised file descriptor limit.
- Verify returned values remain valid, non-negative, and bounded consistently with source-module semantics.

## Requirements

### Functional Requirements

#### FR-1: Descriptor-table size reporting
The module shall provide the functional behavior of reporting the process descriptor-table size limit represented by the source module in `gnu/getdtablesize.c`.

**Traceability**
- Source file: `gnu/getdtablesize.c`

#### FR-2: Resource-limit based behavior
When process open-file or descriptor-limit information is available through resource-limit facilities, the module shall use that limit information to determine the reported descriptor-table size.

**Traceability**
- Source file: `gnu/getdtablesize.c`
- Data structure: `struct rlimit`

#### FR-3: Compatibility with non-throwing stdio-max adjustment
Where the source module uses `_setmaxstdio_nothrow`, the Rust rewrite shall preserve the same functional boundary: any internal attempt corresponding to maximum standard-I/O adjustment must be failure-tolerant and must not introduce a new throwing/panicking external behavior.

**Traceability**
- Function: `_setmaxstdio_nothrow`
- Source file: `gnu/getdtablesize.c`

#### FR-4: Valid integer result behavior
The module shall produce a valid integer descriptor-table size result consistent with the original module’s return-domain behavior.

**Traceability**
- Source file: `gnu/getdtablesize.c`
- Function: `_setmaxstdio_nothrow` (evidences integer-based limit handling)

#### FR-5: No expansion beyond module purpose
The Rust rewrite shall remain limited to descriptor-table size limit behavior and shall not add unrelated module responsibilities.

**Traceability**
- Source file: `gnu/getdtablesize.c`

### Key Entities

#### `struct rlimit`
Represents process resource-limit data used by the module to obtain limit values relevant to descriptor-table sizing.

**Relationship to module behavior**
- Supplies the resource-limit information from which the module derives descriptor/open-file capacity behavior.

#### Internal non-throwing max-stdio adjustment behavior
Represented in the source by `_setmaxstdio_nothrow(int newmax)`.

**Relationship to module behavior**
- Supports compatibility behavior in environments where the effective descriptor-related limit may require or reflect maximum stdio adjustment.
- Its non-throwing character constrains acceptable error behavior in the Rust rewrite.

## Success Criteria

### SC-1: Correct limit retrieval
For process environments where the open-file resource limit is observable, the Rust module returns a descriptor-table size result consistent with that process limit behavior.

**Traceability**
- `gnu/getdtablesize.c`
- `struct rlimit`

### SC-2: Non-throwing compatibility preserved
When exercising behavior corresponding to `_setmaxstdio_nothrow`, the Rust module does not panic and does not expose a new hard-failure mode solely due to internal adjustment failure.

**Traceability**
- `_setmaxstdio_nothrow`

### SC-3: Result validity
Across supported scenarios, the Rust module returns a non-negative integer result within the range supported by the original module behavior.

**Traceability**
- `gnu/getdtablesize.c`

### SC-4: Scope fidelity
The Rust rewrite implements descriptor-table size limit behavior only, with no additional externally visible functionality beyond what is evidenced by the source module.

**Traceability**
- `gnu/getdtablesize.c`

## Acceptance Notes
- Acceptance should compare Rust behavior against the C module’s observable results under the same process limit conditions.
- Tests should focus on descriptor/open-file limit reporting behavior and non-throwing compatibility handling only.
- Any Rust-internal design is acceptable if these functional outcomes are preserved.