# 03_behaviors

## Scope and evidence basis

This behavior document is limited to the observed `main_root` module summary for `sds.c`. It describes runtime-relevant behavior only where supported by the listed function set, names, source ranges, and module-level constraints.

The current module summary is insufficient to support a more detailed behavior judgment for:
- process-level startup beyond function-local construction behavior,
- external I/O behavior,
- complete return-value/error semantics,
- exact internal header layout and field update rules,
- any behavior of functions not listed in the visible interface excerpt.

---

## 1. Initialization flow and startup order

### 1.1 Module startup characteristics

No process entrypoint or global initializer is listed in the module summary. The current module summary is insufficient to support a more detailed behavior judgment about application startup order.

What is observable is that this module behaves as a string object lifecycle provider. Runtime activity begins when a caller invokes one of the constructors or helpers in `sds.c`.

### 1.2 Object creation flow

The creation path is centered on these functions:

- `sdsnewlen`
- `sdsempty`
- `sdsnew`
- `sdsdup`

The startup order for an individual string object is:

1. A caller requests creation through a constructor.
2. Size/type-selection helpers are available in-module:
   - `sdsReqType`
   - `sdsHdrSize`
3. A new string object is initialized with a requested initial content/length form.
4. The created object becomes eligible for later mutation, growth, shrink, duplication, clearing, and release.

This is the only clear initialization flow supported by the summary.

### 1.3 Constructor specialization flow

The constructor set indicates several entry patterns:

- `sdsnewlen(const void *init, size_t initlen)`
  General creation path using explicit content pointer and length.

- `sdsempty(void)`
  Empty-object initialization path.

- `sdsnew(const char *init)`
  Creation path using C-string input.

- `sdsdup(const sds s)`
  Copy-based initialization from an existing string object.

Dynamic behavior that must be preserved:
- all higher-level creation forms remain distinguishable by their entry conditions,
- duplication remains a create-from-existing flow rather than an in-place mutation flow,
- empty creation remains available as its own fast initialization path.

---

## 2. Main user operation flows

### 2.1 Lifecycle flow

The visible lifecycle is:

1. **Create**
   - `sdsnewlen`
   - `sdsempty`
   - `sdsnew`
   - `sdsdup`

2. **Inspect allocation context**
   - `sdsAllocSize`
   - `sdsAllocPtr`

3. **Mutate content or logical size**
   - `sdsupdatelen`
   - `sdsclear`
   - `sdsMakeRoomFor`
   - `sdsRemoveFreeSpace`
   - `sdsIncrLen`
   - `sdsgrowzero`
   - `sdscatlen`
   - `sdscat`
   - `sdscatsds`
   - `sdscpylen`
   - `sdscpy`

4. **Destroy**
   - `sdsfree`

This lifecycle-oriented flow is the clearest runtime pattern exposed by the summary.

### 2.2 Append flow

Append operations are represented by:

- `sdscatlen`
- `sdscat`
- `sdscatsds`

Behaviorally, the append sequence is:

1. Caller holds an existing `sds`.
2. Caller requests concatenation using either:
   - explicit buffer + explicit length,
   - C-string input,
   - another `sds`.
3. Capacity-related preparation is supported in the module by `sdsMakeRoomFor`.
4. The logical string length is extended to include appended content.
5. The resulting string object is returned to the caller.

The current module summary is insufficient to support a more detailed behavior judgment about whether append always calls `sdsMakeRoomFor` directly or whether some variants share a common lower-level routine, but these functions clearly form a content-growth path.

### 2.3 Copy/overwrite flow

Overwrite-style mutation is represented by:

- `sdscpylen`
- `sdscpy`

Behavioral sequence:

1. Caller provides an existing string object.
2. Caller supplies replacement content in explicit-length or C-string form.
3. If current capacity is insufficient, space-management functions in the same module are available to support expansion.
4. Existing logical content is replaced by the new content.
5. The updated string object is returned.

This flow differs from append because the intended state transition is content replacement rather than extension.

### 2.4 Explicit growth flow

Growth-oriented operations include:

- `sdsMakeRoomFor`
- `sdsgrowzero`
- `sdsIncrLen`

These indicate three different user-visible growth behaviors:

#### A. Reserve-style growth
`SdsMakeRoomFor` provides a path where capacity is increased ahead of content writing.

Dynamic sequence:
1. Caller requests additional room for `addlen`.
2. The object transitions from current-capacity state to larger-capacity state.
3. Logical content need not be fully rewritten by this operation itself.
4. A string object is returned for continued use.

#### B. Zero-growth to target length
`sdsgrowzero` indicates a path that grows an object up to a target length while defining behavior for the newly added region.

Dynamic sequence:
1. Caller requests length `len`.
2. If the current object is shorter, the object is expanded.
3. Newly added range is handled in a zero-fill style, based on the function name.
4. The object returns in a valid grown state.

Because the function is explicitly named `growzero`, preserving zero-initialized extension behavior is part of compatibility.

#### C. Manual logical-length adjustment
`sdsIncrLen` provides direct control over logical length by signed increment.

Dynamic sequence:
1. Caller already has an allocated object and has some reason to adjust visible length.
2. Caller supplies positive or negative `incr`.
3. The object’s logical size is updated in place.
4. The operation completes without changing API category; it remains a mutating in-place state update.

This function is particularly important in workflows where content may be written into reserved space first and length finalized afterward.

### 2.5 Length resynchronization flow

Two functions indicate correction/reset behavior:

- `sdsupdatelen`
- `sdsclear`

#### A. `sdsupdatelen`
This is a resynchronization path:
1. Caller has an existing string object.
2. The function recomputes or refreshes the logical length from current content state.
3. Metadata and visible length are brought back into agreement.

The current module summary is insufficient to support a more detailed behavior judgment about the exact source of truth used during recalculation.

#### B. `sdsclear`
This is a reset path:
1. Caller keeps the object.
2. The logical content is cleared.
3. The object remains reusable after clearing.

This is a lifecycle-preserving reset, not a destructor.

### 2.6 Shrink/compaction flow

Shrink-related behavior is exposed by:

- `sdsRemoveFreeSpace`

Dynamic sequence:
1. Caller holds an object with current content and some amount of extra allocation.
2. Caller requests removal of free space.
3. The object transitions from content-plus-slack state to tighter-allocation state.
4. A string object is returned.

This is the complementary flow to `sdsMakeRoomFor`: one expands available capacity, the other compacts it.

### 2.7 Destruction flow

`Sdsfree` terminates the lifecycle:

1. Caller passes an existing string object.
2. Storage associated with that object is released.
3. The object must no longer be used by normal flows afterward.

The current module summary is insufficient to support a more detailed behavior judgment about null-handling or idempotence for repeated frees.

---

## 3. State machines and state transitions

### 3.1 High-level object state model

Based on the function set, the string object supports the following runtime states:

- **Uninitialized / absent**
  Before constructor return.

- **Allocated-empty**
  Reached via `sdsempty`, or via `sdsclear` from a non-empty state.

- **Allocated-with-content**
  Reached via `sdsnewlen`, `sdsnew`, `sdsdup`, append, copy, or growth operations that produce visible content length.

- **Allocated-with-extra-room**
  Reached via `sdsMakeRoomFor`; content remains valid while spare capacity exists.

- **Allocated-compacted**
  Reached via `sdsRemoveFreeSpace`; spare room is reduced or removed.

- **Released**
  Reached via `sdsfree`.

### 3.2 State transitions

#### Creation transitions
- absent → allocated-empty
- absent → allocated-with-content

#### Mutation transitions
- allocated-empty → allocated-with-content
  via append/copy/grow operations

- allocated-with-content → allocated-with-content
  via append, overwrite, resync, manual length adjustment

- allocated-with-content → allocated-empty
  via `sdsclear`

#### Capacity transitions
- allocated-empty/content → allocated-with-extra-room
  via `sdsMakeRoomFor`

- allocated-with-extra-room → allocated-compacted
  via `sdsRemoveFreeSpace`

- allocated-compacted → allocated-with-extra-room
  via later `sdsMakeRoomFor`

#### Terminal transition
- any allocated state → released
  via `sdsfree`

### 3.3 Metadata/type transition behavior

The helpers:

- `sdsHdrSize(char type)`
- `sdsReqType(size_t string_size)`

show that object metadata is not monolithic; there is some type-dependent header sizing and a size-to-type selection rule.

Behaviorally this means:
- creation and resizing flows can involve a metadata/header-class decision,
- growth or shrink operations may trigger transitions not only in payload capacity but also in the internal representation class.

The current module summary is insufficient to support a more detailed behavior judgment about exact type values, thresholds, or whether all mutation paths can change representation type.

---

## 4. Error-handling flows

### 4.1 Evidence limits

The current module summary is insufficient to support a more detailed behavior judgment about:
- exact failure triggers,
- whether failures return null, preserve the original object, or abort,
- how allocation failures propagate,
- any logging/assertion behavior.

### 4.2 Observable error-related categories

Even without exact semantics, the function set exposes likely error-sensitive runtime zones that must preserve control-flow structure in the C version:

#### A. Allocation-sensitive flows
These include:
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

These functions either create objects or can require resized storage. Any C-version-consistent behavior must preserve how such calls react when they cannot complete normally.

#### B. Length/bounds-sensitive flows
These include:
- `sdsIncrLen`
- `sdsupdatelen`
- `sdsgrowzero`
- `sdscpylen`
- `sdscatlen`

These functions act on explicit sizes or change logical length. Their runtime behavior depends on maintaining valid object metadata/content agreement.

### 4.3 Error containment behavior

The summary does support one preservation rule: control flow and return conventions must be preserved from the source implementation body.

Therefore, the behavior document must record:
- callers should treat mutating-return functions as operations whose returned object matters,
- failures, if any, must not be re-described with invented semantics,
- compatibility requires preserving source-level branching around growth, shrink, and mutation operations.

---

## 5. Boundary conditions and special-case handling

### 5.1 Empty input and empty object handling

Boundary-aware APIs are clearly present:

- `sdsempty`
- `sdsnewlen(..., initlen)`
- `sdsnew(const char *init)`
- `sdscatlen(..., len)`
- `sdscpylen(..., len)`

This indicates explicit handling exists for empty-length content and empty objects.

Behavior that must remain stable:
- empty-object creation remains distinct and valid,
- length-based APIs continue to accept length as the controlling boundary parameter,
- zero-length growth/copy/append cases must continue to follow the C implementation’s branch behavior.

The current module summary is insufficient to support a more detailed behavior judgment about whether zero-length operations are strict no-ops or metadata-touching operations.

### 5.2 Explicit-length versus C-string paths

There are paired APIs for:
- creation: `sdsnewlen` vs `sdsnew`
- append: `sdscatlen` vs `sdscat`
- copy: `sdscpylen` vs `sdscpy`

This means runtime behavior distinguishes:
- operations bounded by caller-supplied length,
- operations bounded by C-string termination.

This distinction is behaviorally important and must remain intact.

### 5.3 Signed length adjustment boundary

`SdsIncrLen(sds s, ssize_t incr)` explicitly accepts signed adjustments.

Therefore the state machine must preserve:
- positive-length extension behavior,
- negative-length reduction behavior,
- in-place metadata change semantics rather than treating this as a full reconstructive operation.

The current module summary is insufficient to support a more detailed behavior judgment about the exact valid range checks or failure mode when `incr` would move length out of bounds.

### 5.4 Minimum/maximum representation thresholds

The presence of:
- `sdsReqType(size_t string_size)`
- `sdsHdrSize(char type)`

shows that object behavior changes at size-related thresholds.

Boundary behavior that must remain consistent:
- size-class selection is based on requested string size,
- header size depends on representation type,
- transitions at size thresholds are part of normal runtime behavior, not undefined side effects.

The current module summary is insufficient to support a more detailed behavior judgment about exact threshold constants.

### 5.5 Clear versus free

Two special-case object-ending operations exist with different behavior:

- `sdsclear`: object remains alive and reusable
- `sdsfree`: object lifecycle ends

This distinction is essential. Any reimplementation must not collapse these into the same state transition.

---

## 6. Behaviors that must remain consistent with the C version

### 6.1 Object lifecycle semantics

The following lifecycle distinctions must be preserved exactly at the behavioral level:

- create new object,
- duplicate existing object,
- mutate existing object in place or via returned replacement object,
- clear without destruction,
- free and end lifecycle.

### 6.2 Explicit control-flow categories

The C version clearly separates these runtime categories:

- header/type selection: `sdsReqType`, `sdsHdrSize`
- construction: `sdsnewlen`, `sdsempty`, `sdsnew`, `sdsdup`
- destruction: `sdsfree`
- length synchronization/reset: `sdsupdatelen`, `sdsclear`
- capacity expansion: `sdsMakeRoomFor`
- capacity compaction: `sdsRemoveFreeSpace`
- allocation inspection: `sdsAllocSize`, `sdsAllocPtr`
- direct length mutation: `sdsIncrLen`
- growth with zero-fill semantics: `sdsgrowzero`
- append: `sdscatlen`, `sdscat`, `sdscatsds`
- overwrite: `sdscpylen`, `sdscpy`

A behaviorally compatible version must preserve these as meaningfully distinct runtime paths.

### 6.3 Returned-object discipline

Many mutators return `sds` rather than `void`. That implies a runtime discipline where the post-operation object reference is significant.

Behaviors to preserve:
- callers receive the current valid object reference from mutating-return APIs,
- operations that can affect storage/layout continue to expose that through the returned value,
- `void` functions remain pure side-effect operations at the API level.

### 6.4 Size-aware representation behavior

The internal helper pairing demonstrates that representation details are selected according to string size and header form. A consistent implementation must preserve:
- size-based representation choice,
- representation-aware size calculation,
- use of these decisions during creation and resize-related flows.

### 6.5 Data/content and metadata coherence

The presence of:
- `sdsupdatelen`
- `sdsIncrLen`
- `sdsgrowzero`
- append/copy functions

shows the C version treats content bytes, visible length, and allocation metadata as coordinated state.

Required consistency:
- content mutation and length mutation must remain synchronized according to each function’s purpose,
- manual length adjustments remain distinct from content-copy operations,
- clear/update/grow operations must maintain a valid object state after completion.

---

## 7. Performance-sensitive paths

### 7.1 High-frequency mutation paths

The most performance-sensitive flows suggested by the interface are:

- `sdscatlen`
- `sdscat`
- `sdscatsds`
- `sdscpylen`
- `sdscpy`
- `sdsMakeRoomFor`
- `sdsIncrLen`

These are the paths most likely to appear in repeated string-building or buffer-management loops.

### 7.2 Fast-path support helpers

The helpers:
- `sdsHdrSize`
- `sdsReqType`

are declared `static inline`, which indicates they participate in hot internal decision paths.

Behaviorally relevant performance observation:
- header/type calculation is treated as low-overhead support logic for creation and resizing paths,
- these decisions are embedded in runtime flows rather than delegated to distant subsystems.

### 7.3 Reserve-then-write workflow

The presence of both:
- `sdsMakeRoomFor`
- `sdsIncrLen`

indicates an important performance-oriented behavioral pattern:

1. reserve extra capacity,
2. write content into available space through caller-controlled means,
3. finalize visible length explicitly.

This avoids forcing every incremental write through a full append/copy API. That workflow is part of the module’s dynamic behavior and should remain available.

### 7.4 Growth versus compaction tradeoff

The paired existence of:
- `sdsMakeRoomFor`
- `sdsRemoveFreeSpace`

shows the module supports two opposing performance modes:

- **throughput-oriented mode**: keep or add spare room to support future writes,
- **space-oriented mode**: remove slack when compactness is preferred.

A compatible implementation must preserve both runtime strategies rather than reducing behavior to always-grow or always-compact.

### 7.5 Zero-fill growth cost center

`sdsgrowzero` is a specialized path whose name indicates that newly exposed range handling is more than simple capacity reservation.

Behaviorally, this makes it a distinct cost profile from plain room reservation:
- reserve-only growth prepares space,
- growzero both extends visible length semantics and handles the new region in a zeroing style.

This distinction matters for both correctness and performance.

### 7.6 Inspection without mutation

`sdsAllocSize` and `sdsAllocPtr` provide allocation-state inspection paths. These appear intended to expose allocation context without forcing a copy or content rewrite. Their presence suggests performance-sensitive callers may query storage properties directly before choosing a mutation strategy.

The current module summary is insufficient to support a more detailed behavior judgment about how these inspection paths are used internally or externally.

---

## 8. Consolidated behavior summary

This module behaves as a dynamic string runtime with:
- constructor flows,
- mutation flows for append/overwrite/grow/clear,
- capacity-management flows for expansion and compaction,
- metadata-sensitive representation selection,
- explicit logical-length maintenance,
- final release.

The strongest dynamic themes supported by the summary are:
1. string objects are created in multiple forms,
2. object state includes both visible content length and allocation characteristics,
3. some operations change content while others change only capacity or visible length,
4. representation choice depends on size-related rules,
5. performance-sensitive usage includes reserving room, mutating data, and then adjusting length,
6. clear and free are distinct lifecycle transitions.

Where exact runtime decisions are not visible from the summary, the current module summary is insufficient to support a more detailed behavior judgment.