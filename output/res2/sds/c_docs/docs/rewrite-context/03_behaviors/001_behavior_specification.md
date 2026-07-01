# 03_behaviors

## Scope

This document describes dynamic behavior observable from the provided module summary for the root `sds` module, centered on `sds.c`. It is limited to behavior directly supported by the listed function set, file scope, and summary metadata. Where the summary does not expose implementation details, the document states that explicitly.

The module appears to implement runtime behavior around creation, resizing, updating, copying, concatenation, and release of an `sds` string object, with helper routines that determine header sizing and requested representation type.

---

## 1. Initialization flow and startup order

### 1.1 Module startup characteristics
No process-level startup entry point such as `main()` is listed in the module summary. The current module summary is insufficient to support a more detailed behavior judgment about program bootstrapping, global initialization, or subsystem registration.

### 1.2 Object creation flow
The observable runtime lifecycle begins when a caller creates an `sds` object through one of the constructor-style entry points:

- `sdsnewlen(const void *init, size_t initlen)`
- `sdsempty(void)`
- `sdsnew(const char *init)`
- `sdsdup(const sds s)`

From the function list and naming relationships, the creation/startup behavior of an individual string object is organized as follows:

1. A caller selects a creation path:
   - explicit byte length via `sdsnewlen`
   - empty object via `sdsempty`
   - C-string input via `sdsnew`
   - duplication of an existing `sds` via `sdsdup`

2. Internal helper selection occurs through:
   - `sdsReqType(size_t string_size)` to determine a requested type for a target size
   - `sdsHdrSize(char type)` to determine header size associated with that type

3. After construction, the new object enters an active usable state and can be passed to update, growth, append, copy, or release functions.

This is the only startup order directly supported by the summary: helper selection precedes or accompanies object construction, and constructed objects are then consumed by mutation operations.

### 1.3 Reinitialization-style flows
Several functions behave like reinitialization without destroying the object identity at the interface level:

- `sdsclear(sds s)` resets the logical content of an existing object.
- `sdsupdatelen(sds s)` refreshes internal length-related state from current content.
- `sdsRemoveFreeSpace(sds s)` adjusts allocation-related state while retaining the string object abstraction.

The summary supports describing these as state-refresh operations after initial construction, but not the exact internal field mutations.

---

## 2. Main user operation flows

### 2.1 Create -> use -> free lifecycle
The dominant runtime flow is:

1. Create an `sds`
2. Read or mutate it through append/copy/grow/update operations
3. Release it with `sdsfree`

This lifecycle is directly supported by the presence of constructors, mutators, allocation-management functions, and a destructor-style free routine.

### 2.2 Empty string flow
A caller can start from an empty object using `sdsempty()`.

Typical behavior sequence supported by the API set:

1. `sdsempty()`
2. capacity preparation or direct append:
   - `sdsMakeRoomFor`
   - `sdscatlen`
   - `sdscat`
   - `sdscatsds`
3. optional length update or explicit growth:
   - `sdsIncrLen`
   - `sdsupdatelen`
   - `sdsgrowzero`
4. eventual cleanup with `sdsfree`

The exact internal call chain is not available in the summary, but this is the operational flow implied by the available interfaces.

### 2.3 Construction from existing data
There are three main user-visible input forms:

#### a. Raw buffer with explicit length
- `sdsnewlen(const void *init, size_t initlen)`

Behaviorally, this is the most direct construction path for data that is not required to be null-terminated externally.

#### b. Null-terminated C string
- `sdsnew(const char *init)`

This path accepts conventional C-string input and produces an `sds`.

#### c. Existing `sds`
- `sdsdup(const sds s)`

This path creates a separate runtime object based on an already-existing `sds`.

### 2.4 Append and concatenation flow
Append-style runtime behavior is represented by:

- `sdscatlen(sds s, const void *t, size_t len)`
- `sdscat(sds s, const char *t)`
- `sdscatsds(sds s, const sds t)`

The summary supports the following behavioral sequence:

1. Determine whether the destination object has enough room for additional data.
2. If needed, trigger capacity expansion via `sdsMakeRoomFor`.
3. Add the supplied bytes or characters.
4. Update length-related state.
5. Return the resulting `sds`.

This append path is one of the module's core dynamic flows because it combines growth management and content mutation.

### 2.5 Copy/replace flow
Replacement-style operations are exposed through:

- `sdscpylen(sds s, const char *t, size_t len)`
- `sdscpy(sds s, const char *t)`

Behaviorally this differs from append:

1. The existing `sds` is reused as the destination.
2. Capacity may need to be expanded first.
3. Existing logical content is replaced by new content.
4. Length state is synchronized to the copied content.

The summary is insufficient to support a more detailed behavior judgment about whether previous content bytes beyond the new logical length are preserved, overwritten, or cleared.

### 2.6 Explicit growth and manual length-adjustment flow
The module exposes lower-level mutation control through:

- `sdsMakeRoomFor(sds s, size_t addlen)`
- `sdsgrowzero(sds s, size_t len)`
- `sdsIncrLen(sds s, ssize_t incr)`
- `sdsupdatelen(sds s)`

These indicate a runtime model where callers can:

1. Reserve extra room before writing.
2. Extend logical size while zero-growing through `sdsgrowzero`.
3. Manually increase or decrease logical length after direct buffer manipulation via `sdsIncrLen`.
4. Recompute the logical length from content via `sdsupdatelen`.

This is an important behavior pattern: the module supports both high-level string operations and lower-level direct-buffer workflows.

### 2.7 Allocation-state inspection flow
Two functions expose allocation-related runtime information:

- `sdsAllocSize(sds s)`
- `sdsAllocPtr(sds s)`

This supports workflows where callers inspect allocation properties or obtain an allocation-associated pointer for integration with external code. The current module summary is insufficient to support a more detailed behavior judgment about how that pointer is intended to be used or what invariants external code must preserve.

### 2.8 Space reclamation flow
The module contains a shrink/reclaim path:

- `sdsRemoveFreeSpace(sds s)`

Behaviorally:

1. A string has active content plus unused reserved space.
2. Caller requests removal of unused space.
3. The object transitions to a tighter allocation state.
4. The returned `sds` continues operation with less or no free capacity.

This forms the opposite runtime path to `sdsMakeRoomFor`.

---

## 3. State machines and state transitions

## 3.1 Object lifecycle state machine
The summary supports the following abstract lifecycle states for an `sds` object.

### States
- **Uninitialized / absent**
- **Allocated and active**
- **Active with spare capacity**
- **Active after logical clear**
- **Released**

### Transitions
- `sdsnewlen`, `sdsempty`, `sdsnew`, `sdsdup`:
  - `Uninitialized -> Allocated and active`

- `sdsMakeRoomFor`:
  - `Allocated and active -> Active with spare capacity`
  - or remains in the same effective state if enough room already exists; the summary is insufficient to support a more detailed behavior judgment

- `sdscatlen`, `sdscat`, `sdscatsds`, `sdscpylen`, `sdscpy`, `sdsgrowzero`, `sdsIncrLen`, `sdsupdatelen`:
  - `Allocated and active -> Allocated and active`
  - `Active with spare capacity -> Allocated and active` or remains with spare capacity depending on consumed room; the summary is insufficient to support a more detailed behavior judgment

- `sdsclear`:
  - `Allocated and active -> Active after logical clear`
  - `Active with spare capacity -> Active after logical clear`

- `sdsRemoveFreeSpace`:
  - `Active with spare capacity -> Allocated and active` with reduced extra space

- `sdsfree`:
  - `Allocated and active -> Released`
  - `Active with spare capacity -> Released`
  - `Active after logical clear -> Released`

No post-release behavior is defined by the summary.

## 3.2 Content-length state transitions
A second state dimension concerns logical content length.

### Observable transitions
- **Creation**
  - `sdsempty` creates a zero-length active object.
  - `sdsnewlen`, `sdsnew`, `sdsdup` create active objects whose initial length corresponds to input content.

- **Reset**
  - `sdsclear` transitions logical length to empty while retaining object usability.

- **Explicit recomputation**
  - `sdsupdatelen` synchronizes stored length with current content representation.

- **Incremental adjustment**
  - `sdsIncrLen` changes logical length by a signed increment.

- **Growth**
  - `sdsgrowzero` enlarges object length to a requested target and fills newly grown area with zeros, as indicated by the function name.

- **Append**
  - `sdscatlen`, `sdscat`, `sdscatsds` move logical length upward.

- **Copy/replace**
  - `sdscpylen`, `sdscpy` set logical length according to replacement data.

## 3.3 Representation-selection state
The helper pair

- `sdsReqType(size_t string_size)`
- `sdsHdrSize(char type)`

indicates that object representation depends on target string size. The observable dynamic state is:

1. a target size is known or requested,
2. a type is selected from that size,
3. a header size is obtained from the type,
4. construction or reallocation proceeds based on that representation choice.

The current module summary is insufficient to support a more detailed behavior judgment about the exact type set, thresholds, or transition table.

---

## 4. Error-handling flows

### 4.1 General limits of observable error behavior
The summary lists function signatures but does not expose implementation bodies, explicit error branches, assertions, or allocation-failure handling. The current module summary is insufficient to support a more detailed behavior judgment about exact error propagation semantics, return-on-failure policy, or whether failures are recoverable.

### 4.2 Error-sensitive operation groups
The following runtime paths are structurally error-sensitive because they imply allocation changes or state updates:

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

The summary supports identifying these as error-relevant paths because they create or resize string state. It does not support describing exact failure outcomes.

### 4.3 Input-validity related handling
Some functions take pointers and lengths:

- `sdsnewlen(const void *init, size_t initlen)`
- `sdscatlen(sds s, const void *t, size_t len)`
- `sdscpylen(sds s, const char *t, size_t len)`

Other functions take signed or size-based length modifiers:

- `sdsIncrLen(sds s, ssize_t incr)`
- `sdsgrowzero(sds s, size_t len)`
- `sdsMakeRoomFor(sds s, size_t addlen)`

These interfaces imply that runtime behavior depends on input size and object state. The current module summary is insufficient to support a more detailed behavior judgment about validation rules for null pointers, zero lengths, negative increments, overflow checks, or out-of-range transitions.

### 4.4 Post-error state guarantees
No guarantee can be described from the summary alone about whether the original `sds` remains valid after a failed growth or copy operation. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 5. Boundary conditions and special-case handling

### 5.1 Zero-length creation and empty-state handling
The presence of `sdsempty()` and `sdsnewlen(..., initlen)` indicates explicit support for empty-content states.

Special boundary-relevant behaviors supported by the summary:

- creating an empty string directly,
- clearing a non-empty string back to empty with `sdsclear`,
- updating a string whose logical length may have become zero,
- removing free space from a logically empty string.

The current module summary is insufficient to support a more detailed behavior judgment about whether all of these share the same internal representation.

### 5.2 Small vs. large string representation boundaries
The helper functions `sdsReqType` and `sdsHdrSize` show that runtime behavior changes at size thresholds.

Boundary behavior that must be recognized:

1. a requested string size is evaluated,
2. a representation type is chosen,
3. header size depends on that type,
4. creation or resize paths behave differently across these thresholds.

The exact thresholds and type transitions are not available in the summary.

### 5.3 No-op or low-change transitions
Several operations may become boundary cases when requested change is minimal:

- `sdsMakeRoomFor` when existing capacity is already sufficient
- `sdsRemoveFreeSpace` when little or no free space remains
- `sdsIncrLen` when increment is zero
- `sdscatlen` when append length is zero
- `sdscpylen` when copy length is zero
- `sdsgrowzero` when target length does not exceed current length

These are relevant special cases because the interfaces explicitly express size-based behavior. The current module summary is insufficient to support a more detailed behavior judgment about whether these are optimized as no-ops or still rewrite metadata.

### 5.4 Manual-buffer synchronization edge
The coexistence of:
- `sdsAllocPtr`
- `sdsIncrLen`
- `sdsupdatelen`

shows that the module supports workflows where content may be manipulated outside the highest-level append/copy API, after which internal logical length must be synchronized.

This creates an important boundary behavior:

- after direct buffer writes, caller-visible consistency depends on explicit synchronization functions rather than automatic discovery in all cases.

The summary supports this behavioral statement, but not the exact required call ordering for every use case.

### 5.5 Clear versus free
The API distinguishes:
- `sdsclear` for retaining the object in an empty-but-usable state
- `sdsfree` for terminating object lifetime

This is an important behavioral boundary. An empty active string and a released string are different runtime states and must not be conflated.

---

## 6. Behaviors that must remain consistent with the C version

The following behavioral properties are directly grounded in the listed function set and must remain consistent.

### 6.1 Constructor and destructor lifecycle
The C version provides explicit object lifecycle control:

- creation via `sdsnewlen`, `sdsempty`, `sdsnew`, `sdsdup`
- release via `sdsfree`

Any preserved behavior must keep this explicit lifecycle model.

### 6.2 Size-dependent representation behavior
The existence of `sdsReqType` and `sdsHdrSize` means representation choice is part of runtime behavior, not just an implementation detail hidden from all flow analysis. Behavior must preserve:

- requested-size evaluation
- type selection from size
- header-size lookup from type
- use of that representation information during creation/resizing paths

### 6.3 Separation between logical length and allocation state
The C module clearly distinguishes at least two dimensions of state:

- logical content length
- allocation/free-space state

This must remain consistent because different functions operate on each dimension:

- logical length operations: `sdsupdatelen`, `sdsclear`, `sdsIncrLen`
- capacity operations: `sdsMakeRoomFor`, `sdsRemoveFreeSpace`, `sdsAllocSize`
- combined content/capacity operations: `sdsgrowzero`, `sdscatlen`, `sdscpylen`

### 6.4 Append, copy, and duplicate are distinct flows
The C behavior distinguishes:
- creating a new object from source content,
- appending source content to existing destination,
- replacing destination content,
- duplicating an existing `sds`.

These flows must not be merged semantically.

### 6.5 Manual synchronization support
The C version exposes low-level synchronization paths:
- direct allocation pointer access via `sdsAllocPtr`
- manual length adjustment via `sdsIncrLen`
- recomputation via `sdsupdatelen`

These behaviors indicate that the implementation supports controlled mutation outside only the highest-level copy/append APIs. Preserved behavior must keep this low-level usage model available.

### 6.6 Empty-state reusability
Because both `sdsempty` and `sdsclear` exist, the C behavior treats an empty string as a first-class active state. Preserved behavior must keep:
- construction of empty objects,
- transition to empty without destruction,
- continued usability after being cleared.

### 6.7 Returned-object continuity across resizing operations
Functions such as:
- `sdsMakeRoomFor`
- `sdsRemoveFreeSpace`
- `sdsgrowzero`
- `sdscatlen`
- `sdscpylen`

return `sds`, which indicates that the caller must continue using the returned object after operations that may alter storage state. Any preserved behavior must keep this return-driven continuation model.

The summary is insufficient to support a more detailed behavior judgment about whether returned values may differ in address identity from the input object, but the returned-object usage pattern itself is explicit.

---

## 7. Performance-sensitive paths

### 7.1 Allocation and reallocation paths
The most performance-sensitive flows are those that change storage characteristics:

- `sdsnewlen`
- `sdsMakeRoomFor`
- `sdsRemoveFreeSpace`
- `sdsgrowzero`
- append/copy functions that depend on available room

These operations are central because they combine data movement or storage preparation with metadata updates.

### 7.2 Fast-path significance of capacity reservation
The presence of `sdsMakeRoomFor` as a standalone public function indicates an intended performance pattern:

1. reserve room in advance,
2. perform one or more writes/appends,
3. avoid repeated growth work.

This preallocation/reservation flow is a performance-relevant behavior that should remain intact.

### 7.3 Helper-based representation selection
`SdsReqType` and `sdsHdrSize` are `static inline` helpers, which indicates that size-to-representation calculations are on hot paths such as allocation and resize decisions. The behavior significance is:

- representation selection is expected to be lightweight,
- construction and growth depend on fast header/type computation.

The summary does not support further claims about exact cost.

### 7.4 Append-heavy workloads
The grouped append functions:

- `sdscatlen`
- `sdscat`
- `sdscatsds`

show that appending is a primary workload path. Performance sensitivity here comes from:

- capacity checks,
- growth decisions,
- data extension,
- length updates.

Preserved behavior should maintain append as a direct path rather than forcing all concatenation through more expensive generic reconstruction flows.

### 7.5 Copy/overwrite reuse path
`sdscpylen` and `sdscpy` indicate a performance-relevant reuse scenario where an existing `sds` is updated instead of discarded and recreated. This reuse path is behaviorally important for avoiding unnecessary object lifecycle churn.

### 7.6 Space trimming tradeoff path
The module exposes both:
- `sdsMakeRoomFor` to increase spare room,
- `sdsRemoveFreeSpace` to eliminate spare room.

This means runtime behavior supports explicit performance/memory tradeoff control by the caller. Preserving this dual-path model is important:
- one path favors future growth efficiency,
- the other favors reduced unused allocation.

The current module summary is insufficient to support a more detailed behavior judgment about any exact resizing policy.

---

## 8. Overall behavioral picture

From the available summary, `sds.c` implements a mutable string-object lifecycle with these major runtime characteristics:

1. **Objects are explicitly created and explicitly freed.**
2. **String state includes both logical content length and allocation/free-space state.**
3. **Runtime representation depends on requested string size.**
4. **Mutation flows include append, copy/replace, zero-growth, clear, and manual length synchronization.**
5. **The caller can explicitly manage spare capacity and reclaim unused space.**
6. **The API supports both high-level string operations and lower-level direct-buffer workflows.**

Anything more specific than this about branch behavior, failure outcomes, or exact storage transitions is not supported by the current module summary.