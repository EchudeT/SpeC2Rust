# 03_behaviors

## Scope and evidence basis

This behavior description is limited to the observed function list and module summary for `sds.c` in the `main_root` module. The source-level function names and their grouping strongly indicate a dynamic string subsystem with creation, growth, copy, append, length maintenance, and release behaviors.

Where the module summary does not expose implementation branches, field layouts, or exact failure returns, the document explicitly states that the current module summary is insufficient to support a more detailed behavior judgment.

---

## 1. Initialization flow and startup order

### 1.1 Module startup model

No process-level `main` entry point or global initializer is listed in the module summary. The observable startup behavior is therefore API-driven rather than automatic.

The module appears to start participating in execution only when a caller invokes one of the string-construction entry points:

- `sdsnewlen`
- `sdsempty`
- `sdsnew`
- `sdsdup`

These are the first observable creation points for runtime objects managed by this module.

### 1.2 Object creation flow

The startup sequence for a string object is centered on construction:

1. A caller selects a constructor:
   - `sdsnewlen(const void *init, size_t initlen)`
   - `sdsempty(void)`
   - `sdsnew(const char *init)`
   - `sdsdup(const sds s)`

2. Internal size-selection helpers are available:
   - `sdsReqType(size_t string_size)`
   - `sdsHdrSize(char type)`

3. Based on the function names and helper structure, the construction path includes:
   - selecting a representation class from the requested string size,
   - determining header size for that representation,
   - producing a new `sds` object.

The exact memory layout and field initialization steps are not present in the summary. The current module summary is insufficient to support a more detailed behavior judgment.

### 1.3 Empty-string startup path

`sdsempty()` provides a dedicated empty-object initialization path. This indicates that the module supports a canonical startup flow where a valid string object can exist with zero content before subsequent mutations such as append, copy, clear, or growth.

### 1.4 Duplication-based startup path

`sdsdup(const sds s)` indicates an alternate startup flow in which an existing object becomes the source of a new object. Dynamic behavior here is object cloning rather than in-place mutation.

The exact extent of copied metadata versus payload is not visible from the summary. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 2. Main user operation flows

## 2.1 Create-use-destroy lifecycle

The primary user-visible runtime flow is:

1. Create a string object.
2. Read or adjust its length-related state.
3. Ensure capacity if needed.
4. Modify content.
5. Optionally shrink or clear.
6. Free the object.

This lifecycle is supported by the following functions:

- Creation:
  - `sdsnewlen`
  - `sdsempty`
  - `sdsnew`
  - `sdsdup`

- Length and state maintenance:
  - `sdsupdatelen`
  - `sdsIncrLen`

- Capacity management:
  - `sdsMakeRoomFor`
  - `sdsRemoveFreeSpace`
  - `sdsAllocSize`
  - `sdsAllocPtr`

- Content mutation:
  - `sdsgrowzero`
  - `sdscatlen`
  - `sdscat`
  - `sdscatsds`
  - `sdscpylen`
  - `sdscpy`
  - `sdsclear`

- Release:
  - `sdsfree`

### 2.2 Creation from raw data

`sdsnewlen(const void *init, size_t initlen)` is the most fundamental creation flow in the listed interface.

Behaviorally, this path accepts an explicit byte source and explicit length. That means runtime behavior is not limited to NUL-terminated text input. It supports a controlled content size at creation time.

The visible helper pairing `sdsReqType` and `sdsHdrSize` indicates that the constructor first classifies the requested size and then derives internal overhead before creating the object.

The summary does not expose what happens when `init` is null, when `initlen` is zero, or how terminators are handled. The current module summary is insufficient to support a more detailed behavior judgment.

### 2.3 Creation from C string input

`sdsnew(const char *init)` provides a second create flow that uses a C-string source rather than explicit length. Dynamic behavior differs from `sdsnewlen` in that the content extent must be derived from the source string rather than provided by the caller.

Exact rules for null input are not available in the summary. The current module summary is insufficient to support a more detailed behavior judgment.

### 2.4 Append flow

Three append operations are exposed:

- `sdscatlen(sds s, const void *t, size_t len)`
- `sdscat(sds s, const char *t)`
- `sdscatsds(sds s, const sds t)`

Observed dynamic flow:

1. Caller provides an existing string object.
2. Additional content is selected in one of three source forms:
   - raw bytes with explicit length,
   - C string,
   - another `sds`.
3. The destination string must have enough room for the appended content.
4. After successful append, the destination’s logical content length changes.

Because `sdsMakeRoomFor` exists as a dedicated capacity routine and append functions are separate APIs, append behavior is closely tied to growth management. The exact internal sequence for when and how append calls trigger capacity growth is not shown in the summary. The current module summary is insufficient to support a more detailed behavior judgment.

### 2.5 Copy/replace flow

Two overwrite-style operations are listed:

- `sdscpylen(sds s, const char *t, size_t len)`
- `sdscpy(sds s, const char *t)`

Their dynamic role differs from append:

1. Caller provides an existing destination string object.
2. New source content is supplied.
3. Destination content is replaced rather than extended.
4. Destination length becomes the copied length.

As with append, replacement may require room growth if the new content is larger than current capacity. The exact growth and truncation rules are not exposed in the summary.

### 2.6 Zero-growth flow

`sdsgrowzero(sds s, size_t len)` describes a specific mutation path:

1. Caller requests that a string object reach a target logical length.
2. If the current string is shorter than `len`, the object is extended.
3. Newly exposed content area is zero-filled.

This path is distinct from append because the caller supplies a target size rather than new content bytes. It is also distinct from copy because the resulting bytes in the grown region are determined by zero-fill behavior.

The detailed behavior when `len` is equal to or less than current length is not provided in the summary. The current module summary is insufficient to support a more detailed behavior judgment.

### 2.7 Manual length adjustment flow

Two functions expose direct length-state adjustment:

- `sdsupdatelen(sds s)`
- `sdsIncrLen(sds s, ssize_t incr)`

#### `sdsupdatelen`
This function indicates a synchronization flow where the module recomputes the string’s logical length from its current content representation. Dynamic behavior is:

1. Caller has modified content externally or through a path that requires re-evaluation.
2. `sdsupdatelen` recalculates and updates internal length state.

The exact source of truth used during recalculation is not visible in the summary.

#### `sdsIncrLen`
This function indicates incremental length movement, not full recomputation:

1. Caller provides a signed increment.
2. Internal length state is adjusted forward or backward.
3. The string object transitions to a new logical-length state without replacing the entire object.

This is a key mutable-state operation because it exposes negative as well as positive movement through `ssize_t incr`.

The exact validation rules, underflow/overflow protection, and content-termination handling are not visible from the summary.

### 2.8 Clear flow

`sdsclear(sds s)` provides a fast reset path within the same object lifecycle:

1. Caller keeps the object alive.
2. Logical content is reset to empty.
3. The object remains reusable for later append/copy operations.

This is behaviorally different from `sdsfree`, which ends the lifecycle.

Whether clear preserves allocated space for reuse is suggested by the surrounding API shape but not directly confirmed by the summary. The current module summary is insufficient to support a more detailed behavior judgment.

### 2.9 Free flow

`sdsfree(sds s)` is the terminal lifecycle operation:

1. Caller passes an existing object.
2. Module releases the object’s backing storage.
3. The object is no longer valid for further state transitions.

The exact treatment of null pointers is not available in the summary.

---

## 3. State machines and state transitions

## 3.1 String object lifecycle states

The visible API supports the following high-level runtime states:

1. **Nonexistent**
   - No `sds` object has been created yet.

2. **Allocated and initialized**
   - Reached through `sdsnewlen`, `sdsempty`, `sdsnew`, or `sdsdup`.

3. **Allocated with mutable content**
   - Object can be changed through append, copy, growth, clear, or length-adjustment calls.

4. **Allocated but logically empty**
   - Reached through `sdsempty` at creation or later through `sdsclear`.

5. **Allocated with reserved/free space characteristics**
   - Exposed through `sdsMakeRoomFor`, `sdsRemoveFreeSpace`, `sdsAllocSize`, and `sdsAllocPtr`.

6. **Freed**
   - Reached through `sdsfree`.

Allowed high-level transitions include:

- Nonexistent -> Allocated and initialized
- Allocated and initialized -> Allocated with mutable content
- Allocated with mutable content -> Allocated but logically empty
- Allocated but logically empty -> Allocated with mutable content
- Any allocated state -> Freed

### 3.2 Length-state transitions

The API exposes multiple ways to move between logical-length states:

- Constructor establishes initial length.
- `sdscatlen`, `sdscat`, `sdscatsds` increase length.
- `sdscpylen`, `sdscpy` set length to source length.
- `sdsgrowzero` raises length to a requested target if expansion occurs.
- `sdsIncrLen` changes length by signed delta.
- `sdsclear` sets length to zero.
- `sdsupdatelen` synchronizes length with current content.

This is a strong indication that logical length is a first-class mutable state in the module, not a derived property computed on every access.

### 3.3 Capacity/free-space transitions

The functions

- `sdsMakeRoomFor`
- `sdsRemoveFreeSpace`
- `sdsAllocSize`
- `sdsAllocPtr`

show that the object has an allocation-related state distinct from logical content length.

Observable transitions:

- `sdsMakeRoomFor`: moves an object into a state with increased available room.
- `sdsRemoveFreeSpace`: moves an object into a tighter allocation state with reduced extra room.
- `sdsclear`: may alter logical length without necessarily changing allocation state, but the summary does not explicitly confirm the allocation effect.
- `sdsAllocSize`: observes current allocation state.
- `sdsAllocPtr`: exposes the allocation base for the current object.

The exact internal state variables and transition conditions are not visible in the summary.

### 3.4 Representation-type transitions

Two internal helpers imply multiple storage representations:

- `sdsReqType(size_t string_size)`
- `sdsHdrSize(char type)`

This means objects can be associated with a representation type selected from string size.

Runtime transition implications:

- A new object begins in a representation chosen for its initial size.
- Operations that enlarge or shrink allocation may cause the representation choice to be reconsidered.

The module summary does not show whether representation changes can occur after creation, or under exactly which calls. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 4. Error-handling flows

## 4.1 General error-handling visibility

The module summary shows many functions returning `sds`, which indicates that operations may return an updated object handle rather than mutating through a stable pointer alone. This is especially relevant for:

- `sdsnewlen`
- `sdsempty`
- `sdsnew`
- `sdsdup`
- `sdsMakeRoomFor`
- `sdsRemoveFreeSpace`
- `sdsgrowzero`
- `sdscatlen`
- `sdscat`
- `sdscatsds`
- `sdscpylen`
- `sdscpy`

However, the summary does not expose explicit failure branches, sentinel values, or allocation-failure conventions. The current module summary is insufficient to support a more detailed behavior judgment.

## 4.2 Reallocation-sensitive flows

Functions that return `sds` after mutation are behaviorally significant because callers must treat the returned value as the current valid object reference for continued execution.

This matters most for:

- `sdsMakeRoomFor`
- `sdsRemoveFreeSpace`
- `sdsgrowzero`
- append/copy functions

Even without implementation details, preserving this runtime usage pattern is important: the flow continues with the returned handle, not merely the original argument value.

## 4.3 Signed length adjustment risks

`sdsIncrLen(sds s, ssize_t incr)` exposes an error-sensitive path because it accepts a signed increment. Runtime misuse could involve shrinking below zero logical length or growing beyond available space.

The implementation-level guard behavior is not shown in the summary. The current module summary is insufficient to support a more detailed behavior judgment.

## 4.4 Pointer and input validation

Several APIs accept raw pointers:

- `sdsnewlen(const void *init, size_t initlen)`
- `sdsnew(const char *init)`
- `sdscatlen(sds s, const void *t, size_t len)`
- `sdscat(sds s, const char *t)`
- `sdscpylen(sds s, const char *t, size_t len)`
- `sdscpy(sds s, const char *t)`

The summary does not expose null-handling, aliasing rules, or invalid-pointer behavior. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 5. Boundary conditions and special-case handling

## 5.1 Zero-length content

The API explicitly includes empty-string support through `sdsempty()`, so zero-length content is a normal, supported state rather than an exceptional one.

Other boundary-relevant paths likely include:

- `sdsnewlen(..., 0)`
- `sdscatlen(..., len = 0)`
- `sdscpylen(..., len = 0)`
- `sdsgrowzero(..., len = current length or 0)`
- `sdsIncrLen(..., incr = 0)`

The exact behavior for each zero-value boundary is not detailed in the summary. The current module summary is insufficient to support a more detailed behavior judgment.

## 5.2 Large-size boundaries

The presence of `sdsReqType(size_t string_size)` indicates explicit size-class boundaries in representation selection. This means behavior depends on string size ranges.

Important boundary points therefore include:

- thresholds where requested size maps to a different internal type,
- thresholds where header size differs,
- transitions caused by growth or shrink operations.

The actual threshold values are not in the summary.

## 5.3 Exact-fit versus extra-room cases

Because the API distinguishes logical length from allocation room, boundary behavior includes:

- operations when current free room is exactly sufficient,
- operations when no additional room exists,
- operations when excess free room should be removed.

These cases are managed through `sdsMakeRoomFor` and `sdsRemoveFreeSpace`. The current module summary is insufficient to support a more detailed behavior judgment about exact branch behavior.

## 5.4 Clear versus free special case

`sdsclear` and `sdsfree` represent two different terminal-looking but behaviorally distinct cases:

- `sdsclear` keeps the object reusable.
- `sdsfree` ends object validity.

This distinction must remain stable because callers may rely on clearing as a non-destructive reset rather than a release.

## 5.5 External content modification boundary

The presence of both `sdsAllocPtr(sds s)` and `sdsupdatelen(sds s)` indicates a special operating mode:

1. Caller obtains access related to the allocation.
2. Content may be modified outside the higher-level append/copy APIs.
3. Caller then resynchronizes logical length using `sdsupdatelen`.

This is an important boundary behavior because it allows transitions from externally modified raw storage back into a valid logical string state.

The exact guarantees around safe external mutation are not described in the summary.

---

## 6. Behaviors that must remain consistent with the C version

## 6.1 Constructor-driven object establishment

The C behavior establishes valid objects through explicit constructor functions, not implicit stack-local initialization or hidden startup hooks. Any equivalent implementation must preserve that lifecycle entry pattern.

## 6.2 Returned-handle mutation model

Functions returning `sds` after mutation must preserve the behavior that the caller receives the current object handle to continue using. This is a core runtime property of the API shape.

## 6.3 Distinction between logical length and allocation state

The C interface clearly separates:

- content length control: `sdsupdatelen`, `sdsIncrLen`, `sdsclear`
- allocation/capacity control: `sdsMakeRoomFor`, `sdsRemoveFreeSpace`, `sdsAllocSize`, `sdsAllocPtr`

This separation must remain behaviorally intact.

## 6.4 Empty string as a valid persistent state

The existence of `sdsempty` and `sdsclear` means an empty string is a first-class valid state. It must not be collapsed into a null-object convention unless the C implementation explicitly does so, which is not shown in the summary.

## 6.5 Multiple input modes for content mutation

The C version supports mutation from:
- raw bytes with explicit length,
- C strings,
- other `sds` objects,
- zero-fill growth.

These are distinct runtime flows and must remain distinct.

## 6.6 Manual and automatic length-maintenance coexistence

The API exposes both:
- higher-level operations that change length as part of append/copy/grow/clear,
- low-level operations that let callers explicitly recompute or adjust length.

That coexistence is part of the module’s behavior and should not be simplified away.

## 6.7 Representation-type selection logic

Internal helpers for requested type and header size indicate that object representation depends on requested content size. Even if hidden from external callers, this selection logic is part of the C runtime behavior and must remain semantically consistent at the level of size-dependent representation choice.

The exact type taxonomy is not visible in the summary, so only this high-level consistency can be stated.

---

## 7. Performance-sensitive paths

## 7.1 Creation and append are hot-path candidates

The API composition suggests that these are likely performance-critical flows:

- `sdsnewlen`
- `sdsempty`
- `sdscatlen`
- `sdscat`
- `sdscatsds`
- `sdscpylen`
- `sdscpy`

This conclusion is grounded in their centrality to the object lifecycle and the observed internal/external call activity totals.

The summary does not provide profiling data, so no stronger ranking should be asserted.

## 7.2 Inline size/type helpers

The functions

- `static inline int sdsHdrSize(char type);`
- `static inline char sdsReqType(size_t string_size);`

are explicitly marked inline in the interface list. This indicates that representation-size calculation is on a performance-sensitive path and is intended to be inexpensive at runtime.

## 7.3 Capacity reservation path

`sdsMakeRoomFor` is performance-sensitive because it sits at the boundary between in-place mutation and allocation-related adjustment. Efficient handling here affects append and copy workloads.

The module summary does not expose the exact growth policy. The current module summary is insufficient to support a more detailed behavior judgment.

## 7.4 Space reclamation path

`sdsRemoveFreeSpace` is performance-sensitive in a different way: it trades retained free space for tighter allocation. This matters in workloads alternating between heavy growth and compaction.

No exact strategy is visible from the summary.

## 7.5 Manual length updates as low-overhead maintenance

`sdsIncrLen` and `sdsupdatelen` appear to exist to avoid full object reconstruction during content changes:

- `sdsIncrLen` supports direct incremental adjustment.
- `sdsupdatelen` supports synchronization after content changes.

These are likely intended as lower-overhead maintenance paths compared with rebuilding content through higher-level APIs, but the current module summary is insufficient to support a more detailed behavior judgment.

## 7.6 Reuse-oriented flow

`sdsclear` is performance-relevant because it resets object content without ending the lifecycle. This supports reuse-oriented execution patterns where the caller retains an existing object across multiple operations.

Whether allocated room is retained across clear is not explicitly confirmed by the summary, so only the reuse-oriented role can be stated safely.

---

## 8. Consolidated behavioral picture

From the available evidence, `sds.c` behaves as a mutable string-object runtime with these core dynamic properties:

- objects are explicitly created, mutated, queried for allocation-related state, and freed;
- logical length is maintained as mutable state, not merely implicit content scanning;
- allocation/capacity state is managed separately from logical content;
- content can be extended, replaced, zero-grown, cleared, duplicated, and manually length-adjusted;
- size-dependent representation selection exists internally through helper logic;
- several mutating operations return the current object handle and therefore participate in relocation-aware execution flow.

For implementation details beyond these observed behaviors—especially exact failure outcomes, null handling, allocation policy, and concrete state-transition guards—the current module summary is insufficient to support a more detailed behavior judgment.