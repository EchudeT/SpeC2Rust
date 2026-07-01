# Specification: main_root_fflush.c_25

- **Project**: cat
- **Module**: `main_root_fflush.c_25`
- **Category**: main_cluster
- **Source basis**: `fflush.c`
- **Rust branch target**: `026-main_root_fflush.c_25-rust-port`
- **Generation date**: 2026-06-06

## 1. Feature Specification

### 1.1 Purpose

This module provides the project's replacement `fflush` behavior for C `FILE*` streams. Its role is to flush buffered output in a way that preserves correct stream-position behavior for seekable streams, including cases where stream state must be adjusted before flushing and restored afterward.

The Rust rewrite must preserve the observable behavior of this replacement flush logic, including:
- handling a specific stream passed by the caller,
- handling the null-stream form of flush,
- preserving or updating stream position state when flushing affects the current file position,
- returning success or failure in the same behavioral cases as the original module.

### 1.2 Functional Scope

The module's functional scope is limited to flush-related stream behavior.

It includes:
- temporarily disabling seek-related optimization on a stream before flush processing when needed,
- restoring the prior optimization state after flush processing,
- updating cached file-position state after a successful flush when the resulting position is known,
- providing the replacement `fflush` entry point used by the project.

It does not define general stream I/O, stream creation, stream ownership, or file descriptor management.

### 1.3 Rust Port Boundary

The Rust version must implement the same functional boundary as this module:
- a module-local mechanism corresponding to temporary seek-optimization disable/restore,
- a module-local mechanism corresponding to file-position cache update for a flushed stream,
- the replacement flush operation corresponding to `rpl_fflush`.

The Rust version must not broaden scope beyond flush behavior evidenced by this module.

## 2. User Scenarios & Testing

### 2.1 Scenario: Flush a writable stream with buffered output

A caller has a writable stream with pending buffered data and invokes the module's replacement flush operation for that stream.

Expected behavior:
- buffered output is pushed to the underlying stream,
- stream state remains usable after the call,
- the call reports success when the flush succeeds.

Testing focus:
- verify success return on a writable, flushable stream,
- verify no residual pending output remains in the stream abstraction after success, to the extent observable through the surrounding runtime.

### 2.2 Scenario: Flush a seekable stream whose flush can affect position tracking

A caller flushes a stream for which internal seek optimization or cached position state matters.

Expected behavior:
- any temporary state adjustment needed for a correct flush is applied before the flush,
- previous optimization state is restored after the flush path completes,
- cached file-position information is updated to remain consistent with the stream's resulting position.

Testing focus:
- verify that after a successful flush, subsequent position-sensitive operations observe the correct file position,
- verify behavior for streams where position tracking is meaningful.

### 2.3 Scenario: Flush all output streams using the null-stream form

A caller invokes the replacement flush operation with a null stream value.

Expected behavior:
- the operation delegates to the all-stream flush behavior associated with `fflush(NULL)`,
- success or failure matches the runtime's flush-all outcome.

Testing focus:
- verify that pending output across applicable output streams is flushed,
- verify return status matches the underlying flush-all result.

### 2.4 Scenario: Flush failure propagation

A caller flushes a stream and the underlying flush operation fails.

Expected behavior:
- the replacement flush operation reports failure,
- stream-related temporary state changes do not remain incorrectly applied after the operation completes.

Testing focus:
- induce flush failure through a stream that cannot successfully flush,
- verify failure return,
- verify the stream does not remain in a permanently altered optimization-state condition attributable to this module.

### 2.5 Scenario: Input or mixed-mode stream where position cache matters

A caller flushes a stream whose current position may need to be preserved or recomputed even if the stream is not a simple output-only case.

Expected behavior:
- if the module determines a known resulting position, it updates cached position state accordingly,
- if no known position is available, it does not claim an incorrect one.

Testing focus:
- validate that post-flush position observations remain consistent with actual file position semantics.

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Replacement flush entry point
The module shall provide the project's replacement flush behavior corresponding to `rpl_fflush`, operating on either a specific stream or the null-stream form.
**Traceability**: `rpl_fflush` in `fflush.c:126-233`

#### FR-2: Specific-stream flush behavior
When given a non-null stream, the module shall attempt to flush that stream and return a status indicating success or failure of the operation.
**Traceability**: `rpl_fflush` in `fflush.c:126-233`

#### FR-3: Null-stream flush behavior
When given a null stream, the module shall perform the flush-all-streams behavior associated with `fflush(NULL)` and return the corresponding status.
**Traceability**: `rpl_fflush` in `fflush.c:126-233`

#### FR-4: Temporary seek-optimization suspension
For stream cases requiring it, the module shall temporarily disable seek optimization before flush processing begins.
**Traceability**: `disable_seek_optimization` in `fflush.c:80-86`; `rpl_fflush` in `fflush.c:126-233`

#### FR-5: Seek-optimization restoration
After flush processing for a specific stream, the module shall restore the stream's prior seek-optimization state using the saved state from temporary suspension.
**Traceability**: `restore_seek_optimization` in `fflush.c:88-92`; `rpl_fflush` in `fflush.c:126-233`

#### FR-6: File-position cache maintenance
After a successful flush, when a resulting file position is known and relevant, the module shall update the stream's cached file-position state to match that position.
**Traceability**: `update_fpos_cache` in `fflush.c:96-120`; `rpl_fflush` in `fflush.c:126-233`

#### FR-7: No false position claim
If the module does not have a known resulting file position for the stream, it shall avoid updating cached position state to an incorrect value.
**Traceability**: `update_fpos_cache` in `fflush.c:96-120`

#### FR-8: Result preservation
The replacement flush operation shall propagate the flush result as its return value rather than masking underlying success or failure.
**Traceability**: `rpl_fflush` in `fflush.c:126-233`

### 3.2 Key Entities

#### Stream handle
The central entity is the C stream object represented by `FILE *`. All module behavior is defined in relation to a stream handle or the null-stream form.

Relationship to requirements:
- consumed by the replacement flush operation,
- may have seek-related optimization state temporarily modified,
- may have cached file-position state updated after flush.

**Traceability**: `disable_seek_optimization`, `restore_seek_optimization`, `update_fpos_cache`, `rpl_fflush`

#### Saved optimization-state value
A saved integer state is used to preserve a stream's pre-flush seek-optimization setting so it can be restored after processing.

Relationship to requirements:
- produced when disabling seek optimization,
- consumed when restoring seek optimization.

**Traceability**: `disable_seek_optimization`; `restore_seek_optimization`

#### Cached file position
A stream-associated position value of type `off_t` represents the known file position that may need to be synchronized after flush.

Relationship to requirements:
- used only when a valid resulting position is known,
- associated with the stream's internal position tracking.

**Traceability**: `update_fpos_cache`

#### Internal containing relationship
The module references a `containing` type relationship without defining it locally, indicating dependence on surrounding stream-internal structure access needed for stream state maintenance.

Relationship to requirements:
- supports stream state updates relevant to optimization and/or position cache behavior.

**Traceability**: referenced type `containing`; functions operating on `FILE *`

## 4. Success Criteria

### 4.1 Behavioral Equivalence

- For a valid non-null writable stream with buffered output, the Rust port returns success exactly when the underlying flush succeeds.
  **Traceability**: FR-1, FR-2, FR-8

- For a null-stream invocation, the Rust port exhibits flush-all behavior matching the original module's `fflush(NULL)` replacement semantics and returns the corresponding status.
  **Traceability**: FR-1, FR-3

### 4.2 Stream State Correctness

- For streams that require temporary seek-optimization suspension, the Rust port restores the prior optimization-related state after the flush attempt completes, including failure cases exercised by tests.
  **Traceability**: FR-4, FR-5

- After a successful flush where the resulting position is known, subsequent position-sensitive observation of the stream is consistent with the updated cached position state.
  **Traceability**: FR-6

- When no resulting position is known, the Rust port does not introduce an incorrect cached position observable through later stream-position behavior.
  **Traceability**: FR-7

### 4.3 Error Handling

- When the underlying flush operation fails, the Rust port returns failure and does not report success.
  **Traceability**: FR-2, FR-8

### 4.4 Scope Conformance

- The Rust port implements only the flush-related behavior evidenced by this module and does not require unrelated new public APIs or capabilities beyond replacement flush logic and its supporting internal state handling.
  **Traceability**: Feature Specification; `fflush.c`