# constitution.md

## Purpose

This document defines the non-optional project-level principles for the Rust migration of `sds`. It is the governing constitution for all later specifications, plans, tasks, implementations, reviews, and release decisions.

Where future documents conflict with this constitution, this constitution takes precedence.

The migration target is a small but behavior-sensitive C dynamic string subsystem with a compact public surface, allocation-sensitive mutation flows, size-dependent internal representation behavior, and APIs whose semantics depend on preserving distinctions among logical length, allocation size, free space, and returned-handle mutation patterns.

---

## 1. Core Principles

### 1.1 Behavioral Equivalence Principle

The Rust migration must preserve the externally observable behavior of the C implementation before pursuing elegance, refactoring, API redesign, or idiomatic simplification.

#### Mandatory rules

- The Rust implementation must preserve the semantic contract of the C `sds` subsystem at the public API level.
- Constructor, mutation, allocation-management, length-management, clear, duplication, and free/release behaviors must remain meaningfully equivalent.
- Distinctions visible in the C design must remain distinct in Rust, including:
  - empty versus freed state,
  - logical length versus allocation capacity,
  - append versus overwrite,
  - automatic length-changing operations versus manual length-maintenance operations,
  - raw-byte input versus C-string input versus `sds` input,
  - growth-for-capacity versus shrink-to-remove-free-space.
- Any operation in C that may relocate storage and requires callers to use the returned handle must retain that practical semantic in Rust, even if implemented through different internal machinery.
- Size-dependent representation behavior implied by `sdsReqType` and `sdsHdrSize` must remain semantically consistent unless the project has proof that a different internal strategy is fully behavior-preserving for all supported cases.

#### Required interpretation

- “Equivalent” means equivalent in results, valid state transitions, error outcomes, edge-case handling, and caller obligations.
- Internal implementation does not need to mimic the exact C layout unless layout itself is observable or required for compatibility.
- If a simpler Rust design would erase a behavior distinction present in C, that simplification is disallowed unless proven behavior-neutral.

#### Specific implications for `sds`

The migration must preserve, at minimum:

- explicit constructor-driven object creation,
- empty string as a valid first-class state,
- mutable string lifecycle behavior,
- explicit capacity-management operations,
- explicit length resynchronization and signed length adjustment behaviors,
- zero-growth semantics,
- duplication semantics,
- clear-versus-free distinction,
- support for binary-safe data paths where length is explicit.

---

### 1.2 Interface Compatibility First Principle

The project must prioritize compatibility with the original public interface and calling expectations before introducing Rust-native convenience layers.

#### Mandatory rules

- The first deliverable interface must preserve the original C-facing API shape closely enough to support drop-in or near-drop-in adoption for existing users.
- Public names, argument meaning, return meaning, and call sequencing expectations must remain aligned with the C version unless a documented compatibility exception is approved.
- Compatibility must be judged from the perspective of a caller relying on documented and de facto behavior, not from the perspective of internal Rust aesthetics.
- If the project exposes both a Rust-native API and a C-compatible API, the C-compatible API is authoritative for behavioral correctness.
- FFI boundaries, pointer ownership expectations, nullability behavior, and mutation/update obligations must be explicitly documented and tested.

#### Mandatory preservation targets

The migration must preserve compatibility for the semantics of functions in the observed public surface, including behavior corresponding to:

- `sdsnewlen`
- `sdsempty`
- `sdsnew`
- `sdsdup`
- `sdsfree`
- `sdsupdatelen`
- `sdsIncrLen`
- `sdsMakeRoomFor`
- `sdsRemoveFreeSpace`
- `sdsAllocSize`
- `sdsAllocPtr`
- `sdsgrowzero`
- `sdscatlen`
- `sdscat`
- `sdscatsds`
- `sdscpylen`
- `sdscpy`
- `sdsclear`
- and the behavior implied by `sdsReqType` and `sdsHdrSize`.

#### Compatibility hierarchy

When tradeoffs arise, the project must prefer, in order:

1. behavioral compatibility,
2. interface compatibility,
3. memory safety,
4. performance parity or improvement,
5. implementation elegance.

If safety and compatibility conflict, see the Safety First Principle.

---

### 1.3 Safety First Principle

The Rust migration must improve memory safety and misuse resistance wherever possible without silently changing required behavior.

#### Mandatory rules

- The default implementation must be safe Rust unless a lower-level boundary requires `unsafe`.
- Every `unsafe` block must be:
  - necessary,
  - minimal in scope,
  - documented with its safety invariants,
  - reviewed specifically for soundness,
  - covered by tests exercising the relevant invariants where feasible.
- No undefined behavior present in the original C implementation may be reproduced intentionally inside safe Rust abstractions.
- Dangerous C patterns may be encapsulated, but never normalized as casual internal practice.
- Pointer arithmetic, raw allocation interaction, manual length adjustment, and external buffer mutation paths require special scrutiny.
- The project must distinguish:
  - compatibility-required behavior,
  - legacy permissiveness,
  - undefined or unsupported misuse.
- If C permits behavior that would violate Rust soundness, the Rust project must preserve the nearest safe compatible behavior and document the deviation explicitly.

#### Safety policy for compatibility-sensitive cases

When the C behavior is known and valid but risky:
- preserve it with strong tests and minimal `unsafe`.

When the C behavior is uncertain:
- do not guess in a way that risks unsoundness,
- document uncertainty,
- add characterization tests against the C implementation,
- select the safest implementation consistent with observed evidence.

When the C behavior appears to depend on undefined behavior:
- do not replicate UB,
- preserve observable outcomes where possible,
- document the incompatibility and rationale.

#### Required safety focus areas for `sds`

The following areas are constitutionally high-risk and must receive explicit review:

- binary-safe data creation and append paths,
- length updates after external mutation,
- signed length increments and decrements,
- capacity growth and shrink logic,
- representation-type transitions,
- terminator maintenance,
- allocation base exposure behavior,
- any FFI adapter exposing raw pointers.

---

### 1.4 Performance Constraint Principle

The Rust migration must not achieve correctness by ignoring the performance character of `sds`.

`sds` is not merely a string API; it is a compact mutable string subsystem whose value depends on efficient creation, append, copy, growth, clearing, and allocation management.

#### Mandatory rules

- Performance-sensitive operations must remain efficient in asymptotic behavior and practical constant factors.
- The migration must preserve the distinction between logical length and spare capacity in a way that enables efficient append and reuse patterns.
- The implementation must avoid unnecessary allocation, copying, scanning, or revalidation on hot paths.
- Operations corresponding to inline or helper-based size/type selection in C must remain cheap enough to avoid distorting hot-path behavior.
- Any safety layer that adds measurable hot-path cost must be justified and benchmarked.
- “Rust-idiomatic” is not an excuse for replacing a mutable dynamic-string design with a less compatible but slower abstraction.

#### Performance-sensitive paths

At minimum, the following must be treated as benchmarked hot or warm paths:

- `sdsnewlen`
- `sdsempty`
- `sdsnew`
- `sdsdup`
- `sdscatlen`
- `sdscat`
- `sdscatsds`
- `sdscpylen`
- `sdscpy`
- `sdsgrowzero`
- `sdsMakeRoomFor`
- `sdsRemoveFreeSpace`
- `sdsIncrLen`
- `sdsupdatelen`
- `sdsclear`

#### Performance decision rule

A behavior-preserving implementation that is materially slower on representative workloads is not acceptable without explicit approval and a written rationale. Correctness is necessary, but not sufficient.

---

## 2. Migration Guidelines

### 2.1 C-to-Rust Mapping Rules

The migration must map C concepts into Rust deliberately, preserving semantics rather than performing a literal syntax translation.

#### 2.1.1 API mapping rules

- Each public C function must map to a clearly identifiable Rust implementation unit.
- Public C semantics must remain traceable one-to-one, even if internal helpers are reorganized.
- If the project provides FFI-compatible exported functions, those functions must preserve C-compatible parameter and return semantics.
- If a Rust-native wrapper exists, it must be layered on top of a compatibility-preserving core, not replace it.

#### 2.1.2 Data model mapping rules

- C string-like storage must map to a Rust representation capable of:
  - binary-safe content,
  - explicit length tracking,
  - reserved capacity tracking,
  - internal mutation without forcing UTF-8 interpretation.
- The implementation must not model `sds` as `String` if doing so would change binary semantics, terminator handling, or mutation cost characteristics.
- A byte-oriented representation is the default unless contrary source evidence is found.
- Logical length must remain a first-class tracked property if required for compatibility.
- Any hidden terminator semantics required by C compatibility must be maintained.

#### 2.1.3 Allocation and relocation rules

- Functions that may change the active storage location in C must be implemented so that callers are required to use the resulting current handle/value.
- Internal reallocation behavior may be represented differently in Rust, but not in a way that breaks caller-observable semantics.
- Capacity reservation and free-space removal must remain explicit behaviors, not incidental side effects only.

#### 2.1.4 Error model mapping rules

- Allocation failure, invalid-input handling, and boundary-condition behavior must be mapped explicitly.
- The Rust code must not silently convert C-style failure modes into panics in normal runtime paths.
- Panics are not a substitute for specified API behavior.
- If an infallible Rust internal path wraps a fallible C-compatible surface, the boundary behavior must still match the contract of the C-facing API.

#### 2.1.5 Representation-selection rules

- Behavior corresponding to size-based representation selection must be preserved semantically.
- If exact original representations are reproduced, they must be justified by compatibility or performance need.
- If exact representations are not reproduced, the project must prove that externally visible semantics, thresholds, and performance expectations are not materially violated.

---

### 2.2 Principles for Handling Uncertain Behavior

The provided summaries identify several areas where the current evidence is insufficient to support a precise judgment. Uncertainty must be treated as a first-class migration concern.

#### 2.2.1 Do not invent semantics

- When behavior is not confirmed, the project must not guess and encode assumptions as if they were fact.
- Ambiguity must be recorded explicitly in specs and task documents.

#### 2.2.2 Characterize before changing

For any uncertain behavior, the project must attempt to resolve uncertainty through one or more of:

- direct inspection of the C source,
- characterization tests against the C implementation,
- header and macro analysis,
- targeted runtime probes,
- review of upstream test expectations.

#### 2.2.3 Evidence hierarchy

When resolving ambiguity, evidence must be trusted in this order:

1. direct behavior of the C implementation,
2. explicit public headers and authoritative comments,
3. existing tests,
4. call-site expectations,
5. inferred behavior from naming and API shape.

Inference alone is insufficient for high-risk decisions when better evidence is obtainable.

#### 2.2.4 Safe fallback rule

If uncertainty cannot be fully resolved in time:
- choose the safest behavior that does not contradict known evidence,
- document the uncertainty,
- gate release on explicit acknowledgement of the remaining risk.

#### 2.2.5 Uncertainty hotspots for this project

The following areas must be presumed uncertain until verified from source or tests:

- null input handling,
- zero-length special cases beyond explicitly named empty constructors,
- exact allocation-failure returns,
- exact growth policy,
- exact shrink policy,
- representation thresholds,
- aliasing behavior during append/copy,
- exact semantics of external mutation followed by `sdsupdatelen`,
- signed length adjustment validation,
- exact treatment of freed or null-like handles.

---

### 2.3 Test Verification Requirements

Tests are the main enforcement mechanism for this constitution.

#### 2.3.1 Required test philosophy

- Tests must validate behavioral equivalence, not just Rust internal correctness.
- Every public operation must have direct tests.
- Edge cases and state transitions must be treated as required coverage, not optional hardening.
- Tests must distinguish content correctness, length correctness, capacity/allocation correctness, and handle/update correctness.

#### 2.3.2 Required test categories

The project must maintain, at minimum:

1. **Characterization tests**
   - Validate observed behavior of the original C implementation.
   - Required for uncertain or compatibility-sensitive behaviors.

2. **Conformance tests**
   - Assert that the Rust implementation matches characterized behavior.

3. **Boundary tests**
   - Zero-length, exact-fit, reallocation thresholds, large-size transitions, signed increment/decrement boundaries.

4. **Lifecycle tests**
   - create, mutate, clear, reuse, shrink, free.

5. **Mutation model tests**
   - append, copy, zero-growth, manual length update, duplicate.

6. **Allocation-state tests**
   - reserve room, remove free space, allocation size observation, allocation base behavior where applicable.

7. **FFI compatibility tests**
   - If C ABI compatibility is exposed.

8. **Negative and misuse-adjacent tests**
   - For documented invalid inputs or unsupported cases, ensuring the implementation fails in the intended way without unsoundness.

9. **Regression tests**
   - Every discovered migration bug must add a permanent regression test.

#### 2.3.3 Required coverage expectations

The project must achieve full function-level coverage of the public surface and targeted branch/edge coverage for all high-risk paths. Numeric coverage percentage alone is not sufficient; behavioral coverage is required.

---

## 3. Quality Gates

No change may be merged, and no milestone may be declared complete, unless the following quality gates are met.

### 3.1 Tests That Must Pass

#### 3.1.1 Baseline required tests

The following must pass for every merge affecting behavior, memory handling, FFI, or performance-sensitive code:

- all unit tests,
- all integration tests,
- all characterization tests,
- all conformance tests,
- all regression tests,
- all benchmark smoke checks,
- all configured sanitization or dynamic-analysis checks for the C reference side where applicable,
- `cargo test` and any Makefile-driven compatibility test targets,
- formatting and lint checks,
- documentation examples if any are compiled or tested.

#### 3.1.2 Required behavioral pass criteria

At minimum, tests must demonstrate:

- constructors create valid objects across empty, non-empty, and binary-data cases,
- append and copy operations preserve expected content and length,
- capacity-management operations preserve content while changing allocation state appropriately,
- clear preserves reusability,
- duplication preserves source semantics without aliasing unintended mutable state,
- manual length updates behave consistently with the C implementation,
- reallocation-sensitive flows require and honor the updated handle semantics,
- zero-growth behavior is preserved,
- boundary and threshold behavior is stable,
- no tested path introduces Rust panics where the API contract requires a normal result or error return.

#### 3.1.3 Unsafe-code pass criteria

Any module containing `unsafe` must additionally pass:

- targeted tests exercising stated safety invariants,
- Miri where feasible for the relevant Rust-only surfaces,
- sanitizer-backed test runs where applicable at the FFI boundary,
- reviewer signoff explicitly acknowledging the `unsafe` reasoning.

---

### 3.2 Code Review Standards

Every change must be reviewed against this constitution, not merely for local code quality.

#### 3.2.1 Mandatory review questions

Reviewers must explicitly verify:

- Does this change preserve the observable C behavior?
- Does it preserve interface compatibility or document a justified exception?
- Does it improve or at least not weaken safety?
- Does it preserve performance expectations on relevant paths?
- Are any assumptions based on inference rather than evidence?
- Are new edge cases covered by tests?
- If `unsafe` is present, is it minimal and soundly justified?
- If allocation or length logic changed, were boundary cases re-tested?

#### 3.2.2 Required review standards

- No behaviorally significant PR may merge without at least one reviewer examining compatibility implications.
- No `unsafe` PR may merge without review by a reviewer explicitly comfortable evaluating Rust soundness.
- No performance-sensitive PR may merge without benchmark consideration.
- Reviewer approval requires evidence, not intuition.

#### 3.2.3 Disallowed review outcomes

The following are insufficient grounds for approval by themselves:

- “Looks idiomatic.”
- “Tests pass” without checking behavioral adequacy.
- “This is simpler.”
- “The old C behavior seems strange.”
- “The optimization is probably fine.”
- “Unsafe is unavoidable” without invariant documentation.

---

### 3.3 Performance Benchmark Requirements

Performance is a release gate, not a postscript.

#### 3.3.1 Required benchmark scope

The project must maintain representative benchmarks for:

- empty construction,
- construction from known-length bytes,
- construction from C-string-like input,
- duplication,
- append of small payloads,
- append of large payloads,
- repeated append with growth,
- overwrite/copy workloads,
- zero-growth workloads,
- clear-and-reuse workloads,
- reserve-then-append workloads,
- shrink/remove-free-space workloads,
- length resynchronization workloads,
- signed length adjustment workloads where valid.

#### 3.3.2 Comparison baseline

Benchmarks must compare, where feasible:

- Rust current branch versus Rust main baseline,
- Rust implementation versus C reference behavior/cost envelope on representative workloads.

The goal is not bit-identical timing but prevention of unjustified regressions.

#### 3.3.3 Required performance acceptance rule

A change affecting hot paths must not merge if it causes a material regression without:

- benchmark evidence,
- explanation,
- approval documenting why the tradeoff is acceptable.

#### 3.3.4 Regression thresholds

Exact thresholds may be defined in project benchmarking docs, but until then the default rule is:

- no clearly measurable regression on hot-path microbenchmarks without explicit signoff,
- no asymptotic regression in any public operation,
- no avoidable additional allocation or full-buffer scan on common paths.

---

## 4. Enforcement and Precedence

### 4.1 Binding effect

This constitution is binding on:

- specs,
- plans,
- tasks,
- implementation PRs,
- tests,
- benchmark suites,
- review decisions.

### 4.2 Conflict resolution

If a later document conflicts with this constitution, the later document must be revised unless this constitution is formally amended.

### 4.3 Amendment rule

Any amendment to this constitution must:

- state the exact principle being changed,
- justify the reason,
- describe compatibility, safety, and performance impact,
- be reviewed with the same rigor as a major architectural change.

### 4.4 Default migration stance

When in doubt, the project must choose the option that is:

1. more behaviorally faithful,
2. more interface-compatible,
3. safer,
4. faster or no slower,
5. better documented.

---

## 5. Non-Negotiable Project Summary

The Rust migration of `sds` shall:

- preserve the behavior of the C subsystem,
- preserve interface expectations before redesign,
- improve safety without inventing incompatible semantics,
- maintain the performance character of a mutable dynamic string runtime,
- resolve uncertainty through evidence,
- enforce correctness through characterization and conformance tests,
- reject merges that compromise compatibility, safety, or justified performance.

All later project artifacts must comply with these principles. Violations are defects in project governance, not merely implementation details.