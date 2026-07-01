# constitution.md

## Purpose

This document is the governing constitution for the Rust migration of `sds`. It defines the non-negotiable principles, migration rules, and quality gates that all later specifications, plans, tasks, code changes, and reviews must follow.

This project is a behavioral migration of a compact C dynamic-string implementation into Rust. The goal is not to redesign the library, broaden the public contract, or substitute idiomatic Rust preferences for observed source behavior where compatibility matters. The goal is to preserve the C implementation’s externally relevant behavior while improving implementation safety and maintainability.

When any later document conflicts with this constitution, this constitution prevails.

---

## 1. Core Principles

### 1.1 Behavioral Equivalence Principle

The Rust migration must preserve the behavior of the C implementation as the primary source of truth.

#### Requirements

- The C source behavior is authoritative over assumptions, stylistic preferences, or inferred redesigns.
- Publicly observable semantics must remain equivalent unless a deliberate, approved compatibility exception is recorded.
- Distinct behavioral paths in C must remain distinct in Rust when they carry different semantics, including:
  - construction paths,
  - duplication paths,
  - append paths,
  - overwrite paths,
  - reserve/grow paths,
  - zero-growth paths,
  - length resynchronization paths,
  - clear vs free lifecycle paths,
  - allocation inspection paths,
  - size/type selection paths.
- State transitions implied by the C API must be preserved, including:
  - create,
  - mutate,
  - reserve,
  - compact,
  - clear,
  - free.
- Representation-sensitive behavior must be preserved where the C code makes size-based header/type decisions.
- Returned-object discipline must be preserved for APIs whose result may differ from the incoming handle after mutation or reallocation.

#### Prohibitions

- Do not collapse separate C operations into one Rust operation if doing so changes observable behavior.
- Do not normalize or “simplify” edge-case behavior unless tests and source evidence show equivalence.
- Do not invent stronger guarantees than the C implementation actually provides.
- Do not reinterpret uncertain C behavior as “undefined enough to redesign” without explicit project approval.

#### Decision rule

If a choice must be made between a more idiomatic Rust design and stricter behavioral equivalence, behavioral equivalence wins.

---

### 1.2 Interface Compatibility First Principle

Compatibility with the `sds` public interface comes before internal elegance.

#### Requirements

- The migration must preserve the conceptual public API surface and usage model of `sds`.
- Differences between explicit-length APIs and C-string-terminated APIs must remain intact.
- APIs with distinct C names and semantics must remain distinct in Rust, even if internally implemented by shared helpers.
- Functions that return an updated string handle in C must preserve the caller obligation to use the returned value where relocation or representation changes may occur.
- Inspection capabilities exposed by the C API must remain available in some compatibility-preserving form.
- Internal organization may change, but externally relevant categories must remain recognizable:
  - constructors,
  - duplication,
  - free/destruction,
  - clear/reset,
  - append,
  - copy/overwrite,
  - reserve,
  - compact,
  - manual length adjustment,
  - length resynchronization,
  - allocation inspection,
  - header/type selection support.

#### Prohibitions

- Do not remove API distinctions solely because Rust can encode them more elegantly.
- Do not silently replace length-based behavior with UTF-8 or Unicode-string semantics.
- Do not make nul-termination, binary-safe length handling, or allocation model assumptions that differ from C behavior.
- Do not expose a Rust-only abstraction as the primary contract if it hides required compatibility semantics.

#### Decision rule

When interface ergonomics conflict with compatibility, compatibility wins.

---

### 1.3 Safety First Principle

Rust safety is mandatory, but safety mechanisms must preserve compatibility rather than alter behavior accidentally.

#### Requirements

- The implementation must maximize memory safety, lifetime safety, and aliasing correctness.
- Unsafe Rust may be used only when required for compatibility, representation control, or performance, and only with documented invariants.
- Every unsafe block must have:
  - a specific necessity statement,
  - explicit invariants,
  - bounded scope,
  - tests covering its contract where practical.
- The migration must prevent Rust-introduced UB, data races, invalid pointer derivation, and unchecked layout assumptions.
- Internal invariants must be documented for:
  - header/metadata representation,
  - length/capacity coherence,
  - nul-termination policy if present,
  - reserve-then-write workflows,
  - signed length adjustment behavior,
  - ownership and deallocation rules.
- Safety wrappers may be introduced around unsafe internals, but they must not erase or alter required `sds` semantics.

#### Handling incompatibility between safety and exact exposure

If the original C API permits patterns that cannot be expressed safely in Rust without restriction:

1. preserve behavior as closely as possible,
2. isolate the compatibility layer,
3. document any unavoidable restriction,
4. require explicit approval for any public semantic change.

#### Prohibitions

- Do not use Rust panics as a substitute for C error behavior unless the C behavior is proven to abort equivalently.
- Do not rely on `unsafe` for convenience.
- Do not weaken invariants just to mimic C implementation style.
- Do not expose unsound public APIs merely because the C original was permissive.

#### Decision rule

Among behaviorally equivalent implementations, choose the safest one.

---

### 1.4 Performance Constraint Principle

The Rust migration must preserve the performance character of `sds`, especially in hot paths.

#### Requirements

- The implementation must preserve the cost model of common operations as closely as practical.
- Hot paths include at minimum:
  - append operations,
  - copy/overwrite operations,
  - room reservation,
  - manual length adjustment,
  - growth with zero-fill,
  - header/type calculation.
- The reserve-then-write-then-finalize workflow must remain viable and efficient.
- Growth and compaction must remain distinct strategies.
- The migration must avoid introducing unnecessary allocations, copies, bounds checks, branching, or abstraction overhead in hot paths.
- Data representation choices must be evaluated against:
  - allocation behavior,
  - pointer stability expectations,
  - metadata access costs,
  - growth strategy costs,
  - zero-fill costs,
  - compaction costs.
- Any deviation from C performance characteristics must be measured, justified, and approved.

#### Prohibitions

- Do not sacrifice core `sds` throughput characteristics for purely stylistic Rust abstractions.
- Do not replace allocation-aware mutation paths with always-copy or always-rebuild approaches.
- Do not hide hot operations behind generalized abstractions that prevent optimization without proof of equivalence.

#### Decision rule

Among behaviorally correct and safe options, choose the one that best preserves the original performance profile.

---

## 2. Migration Guidelines

### 2.1 C-to-Rust Mapping Rules

These rules define how C constructs and behaviors must be mapped into Rust.

#### 2.1.1 Public API mapping

- Preserve one-to-one public operation intent whenever possible.
- Keep explicit-length and nul-terminated entry points distinct.
- Preserve mutators that may relocate storage as return-value-significant operations.
- Preserve clear distinctions between:
  - empty creation,
  - content creation,
  - duplication,
  - append,
  - overwrite,
  - reserve,
  - compaction,
  - zero-growth,
  - direct length adjustment,
  - metadata resynchronization,
  - destruction.

#### 2.1.2 Data model mapping

- Treat `sds` as a byte-oriented dynamic string/buffer abstraction, not a Rust `String`.
- Length is byte length, not character count.
- Capacity/allocation metadata are semantically relevant and must not be erased from the design.
- Internal representation may use Rust structs/enums, but only if they preserve:
  - size-class or representation-class behavior,
  - metadata coherence,
  - allocation inspection semantics,
  - relocation semantics after growth/shrink.
- If the C implementation uses header-prefix allocation layout, Rust may emulate or abstract it, but the chosen form must preserve externally relevant behavior.

#### 2.1.3 Pointer and allocation mapping

- C pointer-sensitive behavior must be treated as compatibility-sensitive.
- Any API analogous to allocation pointer or allocation size inspection must reflect real storage behavior, not fabricated metadata.
- Reallocation-sensitive operations must preserve the possibility that the active handle changes.
- Ownership of allocated storage must be singular and explicit.

#### 2.1.4 Length and bounds mapping

- `size_t`-driven behavior maps to Rust size semantics without changing overflow expectations silently.
- Signed length adjustment semantics must be preserved for positive and negative cases.
- Reserve and growth behavior must maintain valid postconditions on length, free space, and visible content.
- Zero-fill semantics must be preserved for growth operations that imply initialization of the newly exposed region.

#### 2.1.5 Error model mapping

- Error behavior must follow the C implementation, not Rust conventions by default.
- If the C code returns null, preserves original state, or uses other failure signaling, the Rust API layer must preserve that behavior or provide a compatibility wrapper that does.
- Panics are not an acceptable replacement for ordinary operational failure.
- If exact failure behavior is uncertain, the project must not invent a stronger guarantee.

#### 2.1.6 Internal structure mapping

- Shared helpers may be introduced in Rust to reduce duplication.
- Such refactoring is allowed only if it does not erase required semantic distinctions or alter edge-case branching.
- Inline-style helper logic that is hot in C should remain low-overhead in Rust.

---

### 2.2 Principles for Handling Uncertain Behavior

The available summaries explicitly note areas where exact semantics are uncertain. This project must handle uncertainty conservatively.

#### 2.2.1 Source-first evidence rule

When behavior is uncertain:

1. inspect the original C source,
2. inspect headers and macros,
3. inspect tests,
4. inspect allocator interactions,
5. inspect call patterns,
6. only then define Rust behavior.

Behavior must never be guessed from naming alone if source inspection is possible.

#### 2.2.2 Conservative compatibility rule

If evidence remains incomplete:

- preserve the broadest behavior compatible with the known C behavior,
- avoid strengthening preconditions,
- avoid narrowing accepted inputs,
- avoid adding guarantees not proven by source or tests.

#### 2.2.3 No invention rule

Where the summaries state “insufficient to support a more detailed behavior judgment,” the migration must not fabricate semantics for:
- exact failure modes,
- null handling,
- repeated free behavior,
- exact threshold constants,
- hidden metadata layouts,
- exact realloc preservation guarantees,
- branch behavior of zero-length operations,
- exact recalculation source of truth in length resynchronization.

#### 2.2.4 Escalation rule

If a behavior materially affects compatibility and cannot be resolved conclusively:

- open a compatibility decision record,
- describe the uncertainty,
- list source evidence searched,
- document the proposed interpretation,
- obtain reviewer approval before implementation is treated as final.

#### 2.2.5 Documentation rule

Every resolved uncertainty must be documented in the migration notes with:
- the question,
- the evidence,
- the chosen interpretation,
- the tests added to lock in behavior.

---

### 2.3 Test Verification Requirements

Tests are part of the migration contract, not an afterthought.

#### 2.3.1 Required verification model

The Rust implementation must be verified against the C behavior through a layered test strategy:

- direct unit tests,
- behavior parity tests,
- edge-case tests,
- lifecycle tests,
- allocation/growth tests,
- regression tests,
- performance benchmarks.

#### 2.3.2 Parity test requirement

For each public operation or public operation family, tests must verify parity with the C implementation for:
- normal cases,
- empty cases,
- size boundary cases,
- repeated mutation cases,
- reserve/grow/compact interaction,
- clear vs free distinction,
- explicit-length vs C-string path distinction.

#### 2.3.3 State transition test requirement

Tests must validate the observable lifecycle and state transitions, including:
- create empty,
- create with content,
- duplicate,
- append,
- overwrite,
- reserve,
- write-then-increment-length workflows,
- zero-grow,
- update length,
- clear and reuse,
- compact after growth,
- final free/destruction semantics as applicable in Rust.

#### 2.3.4 Boundary-condition test requirement

Tests must explicitly cover:
- zero-length inputs,
- empty strings,
- large length thresholds relevant to representation selection,
- exact threshold crossing cases,
- positive and negative `sdsIncrLen` behavior,
- compaction after excess room,
- repeated appends and copies,
- binary-safe content with embedded nul bytes where explicit-length APIs imply it should work.

#### 2.3.5 Error-path test requirement

Where the C behavior can be observed or induced, tests must cover:
- allocation-sensitive failures,
- invalid size transitions where applicable,
- failure preservation rules for original objects,
- no-panic requirements for ordinary failure paths.

If low-memory behavior cannot be deterministically tested in normal CI, it must be covered by:
- fault-injection tests,
- allocator substitution tests,
- or dedicated stress test jobs.

#### 2.3.6 Regression-lock requirement

Every bug found during migration must be accompanied by:
- a reproducing test,
- a fix,
- and a regression guard preventing recurrence.

---

## 3. Quality Gates

No implementation is complete until all quality gates below pass.

### 3.1 Tests That Must Pass

#### 3.1.1 Build and baseline gates

The project must pass:

- `make`-based project build and test entry requirements,
- all Rust compilation checks,
- formatting checks,
- lint checks,
- documentation checks where applicable.

#### 3.1.2 Functional test gates

The following must pass before merge:

- unit tests for each migrated function family,
- integration tests covering public workflows,
- parity tests against the C implementation or recorded C behavior,
- edge-case and boundary-condition tests,
- regression tests for all known defects.

#### 3.1.3 Compatibility test gates

At minimum, tests must demonstrate preserved behavior for:

- `sdsnewlen`
- `sdsempty`
- `sdsnew`
- `sdsdup`
- `sdsfree`
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
- `sdsAllocSize`
- `sdsAllocPtr`
- `sdsHdrSize`
- `sdsReqType`

If any function is intentionally deferred, the deferral must be explicit and approved, and no merged claim of completion may omit that fact.

#### 3.1.4 Safety test gates

The project should pass, where applicable:

- Miri or equivalent safety-oriented checks for safe/unsafe interactions,
- sanitizer-backed tests for FFI or allocator-sensitive code,
- overflow-sensitive and debug-assert test runs,
- fault-injection tests for allocation failures if supported.

#### 3.1.5 CI gate policy

A change must not merge if it:
- breaks parity tests,
- removes edge-case coverage,
- introduces unreviewed unsafe code,
- degrades benchmark thresholds beyond approved limits,
- leaves unresolved compatibility decisions undocumented.

---

### 3.2 Code Review Standards

#### 3.2.1 Required review framing

Every code review must evaluate changes against this constitution, not only local correctness.

Reviewers must ask:

- Does this preserve C behavior?
- Does this preserve interface distinctions?
- Does this change edge-case handling?
- Does this alter failure semantics?
- Does this introduce unbounded overhead?
- Is every unsafe block justified and documented?
- Are tests sufficient to prove the behavior?

#### 3.2.2 Mandatory review content

Every non-trivial migration PR must include:

- the C function(s) or behavior(s) being migrated,
- the intended Rust mapping,
- known uncertainties,
- compatibility risks,
- tests added,
- benchmark impact if hot-path code changed.

#### 3.2.3 Unsafe code review standard

Unsafe code requires heightened review. Approval requires:

- proof that safe Rust is insufficient or materially worse,
- documented invariants,
- localized unsafe scope,
- tests exercising the invariants,
- reviewer acknowledgment of the unsafe rationale.

#### 3.2.4 Compatibility review standard

Any change affecting:
- public signatures,
- return-value significance,
- allocation layout assumptions,
- growth strategy,
- compaction behavior,
- zero-fill behavior,
- threshold behavior,
- length update logic

must receive explicit compatibility review, not just routine approval.

#### 3.2.5 Rejection criteria

A change must be rejected if it:

- replaces proven C behavior with speculation,
- conflates distinct APIs or lifecycle states,
- introduces panic-based normal control flow,
- erases return-value significance from mutators,
- weakens test coverage on migrated behavior,
- adds performance cost without measurement,
- introduces undocumented unsafe assumptions.

---

### 3.3 Performance Benchmark Requirements

#### 3.3.1 Benchmark obligation

Because `sds` is performance-sensitive, benchmark coverage is mandatory.

Benchmarks must cover at minimum:

- create empty,
- create from explicit-length content,
- duplicate,
- append small buffers repeatedly,
- append large buffers,
- overwrite existing content,
- reserve room then manual write then length increment,
- zero-growth,
- compact after growth,
- repeated growth/append loops.

#### 3.3.2 Comparison baseline

Performance must be evaluated against at least one of:

- the original C implementation,
- a verified compatibility wrapper baseline,
- an earlier accepted Rust baseline.

The comparison method must be documented.

#### 3.3.3 Regression thresholds

No change may introduce a material regression in hot paths without explicit approval.

Default policy:

- any statistically meaningful regression in a hot-path benchmark must be investigated,
- regressions beyond project-defined tolerance must block merge,
- approved regressions require written justification tied to safety or correctness needs.

#### 3.3.4 Allocation behavior benchmarking

Benchmarks must consider not only elapsed time but also:
- allocation count,
- reallocation behavior,
- copy volume where measurable,
- capacity-growth effectiveness.

#### 3.3.5 Threshold and representation benchmarking

Because `sds` behavior depends on size-sensitive representation choices, benchmarks must include sizes:
- below thresholds,
- at thresholds,
- just above thresholds,
- substantially above thresholds.

#### 3.3.6 Continuous performance discipline

Performance validation is not a one-time milestone. Hot-path changes must be benchmarked continuously throughout migration.

---

## 4. Governance and Precedence

### 4.1 Scope

This constitution applies to:

- project plans,
- design notes,
- compatibility decisions,
- implementation tasks,
- code reviews,
- tests,
- benchmarks,
- release-readiness judgments.

### 4.2 Precedence

If documents conflict, precedence is:

1. this constitution,
2. approved compatibility decision records,
3. detailed migration specs,
4. implementation plans,
5. task lists.

### 4.3 Amendment rule

This constitution may be changed only by an explicit amendment that:

- names the principle being changed,
- explains why the old rule is insufficient,
- describes compatibility impact,
- is reviewed as a project-wide governance change.

### 4.4 Completion rule

The migration is not complete merely when the code compiles. It is complete only when:

- behavioral compatibility is demonstrated,
- interface distinctions are preserved,
- safety requirements are satisfied,
- tests pass,
- benchmarks are acceptable,
- unresolved uncertainties are documented or closed.