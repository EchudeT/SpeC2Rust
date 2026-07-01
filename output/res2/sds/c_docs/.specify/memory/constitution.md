# constitution.md

## Purpose

This document defines the non-optional project-level principles for the Rust migration of `sds`. It is the governing standard for all later specifications, plans, implementation tasks, reviews, and acceptance decisions.

Where tradeoffs arise, this constitution takes precedence unless it is explicitly amended.

---

## 1. Core Principles

### 1.1 Behavioral Equivalence Principle

The Rust migration must preserve the externally observable behavior of the C implementation.

#### Requirements

- The Rust implementation must preserve the lifecycle model exposed by the C API:
  - creation,
  - mutation,
  - capacity management,
  - explicit release or equivalent ownership-driven finalization semantics at the interface boundary.
- The Rust implementation must preserve the distinction between:
  - duplication,
  - append,
  - copy/replace,
  - clear,
  - grow,
  - free-space removal,
  - manual length synchronization.
- The Rust implementation must preserve size-dependent representation behavior where it is part of observable semantics or performance-critical operation.
- The Rust implementation must preserve the returned-object continuity model for operations that may resize or relocate storage. If the C contract requires callers to use the returned handle after mutation, the Rust-facing compatibility layer must preserve that usage model.
- Empty active strings and destroyed/released strings must remain distinct states.
- Manual-buffer workflows supported by the C API must remain supported where they are part of the public contract.

#### Interpretation rules

- “Equivalent” means equivalent in externally observable results, valid state transitions, error behavior, and documented edge-case behavior.
- Internal Rust implementation details may differ from C if and only if observable behavior, compatibility requirements, and performance constraints remain satisfied.
- Where the C implementation exposes behavior indirectly through pointer stability, returned handles, null-termination, size reporting, or allocation-related inspection, those behaviors are part of the migration contract unless proven otherwise.

#### Prohibitions

- Do not simplify the API by merging semantically distinct operations.
- Do not remove low-level capabilities solely because Rust offers safer abstractions.
- Do not replace specified behavior with “more idiomatic Rust” when that changes caller-visible semantics.

---

### 1.2 Interface Compatibility First Principle

Compatibility with the existing C-facing interface takes priority over internal elegance.

#### Requirements

- The project must preserve the public API surface expected from `sds.h`, `sdsalloc.h`, and relevant exposed behavior inferred from current usage.
- Function names, argument meanings, return-value meanings, and call sequencing expectations must remain compatible unless an explicitly approved compatibility layer provides exact legacy behavior.
- The Rust migration must support integration with the existing build system (`Makefile`) and existing consumers.
- C-callable interfaces must be stable, explicit, and documented through FFI boundaries where required.
- Null-termination expectations, length semantics, and buffer mutability expectations must remain compatible with the original library contract.
- Public behavior must be specified module-by-module before implementation changes that could alter compatibility.

#### Compatibility hierarchy

When forced to choose, the project must prioritize in this order:

1. Correctness of externally observable behavior
2. Source and binary interface compatibility where required
3. Safety
4. Performance
5. Internal implementation elegance

If binary compatibility is not fully required in a given integration mode, source and behavioral compatibility still remain mandatory unless explicitly waived.

#### Prohibitions

- Do not redesign the library into a purely Rust-native string type while calling it a migration.
- Do not expose only Rust-native APIs and defer C compatibility to a later phase.
- Do not treat undocumented but relied-upon interface behavior as disposable without evidence.

---

### 1.3 Safety First Principle

Rust must be used to reduce unsoundness, undefined behavior risk, and memory hazards without changing required semantics.

#### Requirements

- Safe Rust is the default.
- `unsafe` is allowed only where required for:
  - FFI boundaries,
  - raw pointer interoperability,
  - representation-sensitive operations,
  - performance-critical internals that cannot be expressed safely without unacceptable cost.
- Every `unsafe` block must have:
  - a documented safety invariant,
  - minimal scope,
  - a justification tied to required compatibility or performance,
  - tests that exercise the relevant boundary behavior where practical.
- Ownership, aliasing, length, capacity, and null-termination invariants must be explicit in code and documentation.
- The implementation must avoid introducing panic-based control flow across FFI boundaries.
- Overflow, underflow, invalid pointer arithmetic, and out-of-bounds writes must be prevented or explicitly guarded.
- Error handling must be deterministic and documented at the interface boundary.

#### Safety invariants to preserve or define

At minimum, the project must maintain explicit invariants for:

- logical length,
- allocation size and spare capacity,
- writable buffer range,
- trailing null byte expectations,
- valid mutation after growth operations,
- valid state after clear operations,
- valid state after free-space trimming,
- post-failure object validity guarantees, if any.

#### Prohibitions

- No unchecked `unsafe` convenience shortcuts.
- No reliance on panic for routine contract enforcement across public C-facing functions.
- No silent divergence between internal invariants and exposed C semantics.

---

### 1.4 Performance Constraint Principle

The Rust migration must preserve the intended performance profile of `sds`, especially on append, growth, copy, and capacity-management paths.

#### Requirements

- The migration must preserve the performance significance of:
  - size-based representation selection,
  - capacity reservation,
  - append-heavy workflows,
  - in-place reuse for copy/replace operations,
  - explicit free-space trimming.
- The Rust implementation must not introduce avoidable heap allocations, copies, scans, or abstraction overhead on hot paths.
- Null-terminated buffer access must remain efficient for C interop and string-style workloads.
- Preallocation and returned-handle growth workflows must remain efficient and direct.
- Any safety layer added around hot paths must be measured, not assumed acceptable.

#### Performance interpretation

- Exact byte-for-byte internal layout need not match C unless required for compatibility, but the operational cost model must remain comparable.
- Representation-selection logic must remain lightweight.
- Append and copy operations must remain direct operations, not decomposed into higher-cost generic abstractions.

#### Prohibitions

- Do not replace mutable-buffer workflows with repeatedly reconstructing owned strings.
- Do not force all public operations through expensive validation layers on hot paths when equivalent safety can be achieved with tighter invariants and localized checks.
- Do not accept measurable regressions in primary workloads without explicit approval and documented justification.

---

## 2. Migration Guidelines

### 2.1 C-to-Rust Mapping Rules

The migration must map C concepts into Rust deliberately, not mechanically.

#### API mapping rules

- Public C API semantics are the source of truth.
- Rust internal types may be redesigned, but the exposed compatibility contract must remain equivalent.
- If a C function can reallocate and return a new handle, the Rust implementation must preserve that model in the C-facing API.
- If a C function exposes direct buffer mutation workflows, Rust must provide a controlled equivalent at the compatibility boundary.
- Functions with distinct semantics in C must remain distinct in Rust, even if they share implementation internals.

#### Data mapping rules

- `sds` must be modeled as a mutable byte string abstraction, not as a Unicode-only string type.
- Length is byte length.
- Embedded zero bytes must be supported where supported by the C constructor and length-based APIs.
- Null-terminated interoperability must remain available where expected.
- Capacity and allocation metadata must remain conceptually separate from logical length.

#### Representation rules

- The Rust design must account for size-based representation behavior exposed by helper logic such as `sdsReqType` and `sdsHdrSize`.
- If the exact internal header layout is not required externally, an alternative Rust representation is acceptable only if:
  - observable behavior remains equivalent,
  - performance constraints are met,
  - allocation and capacity queries remain compatible in meaning.
- If external consumers depend on raw allocation layout or pointer conventions, the Rust design must preserve or faithfully emulate them.

#### Build and integration rules

- The project must remain buildable from the existing `Makefile`-driven workflow during migration phases unless an approved transitional build path is documented.
- FFI boundaries, crate type, symbol export strategy, and test integration must be explicitly defined before broad implementation proceeds.
- The migration for this project must remain manageable as a single-module migration, but internal Rust submodules may be introduced for clarity if they do not alter the public contract.

---

### 2.2 Principles for Handling Uncertain Behavior

Where current summaries or interface documents do not fully define behavior, the project must act conservatively.

#### Decision order for uncertainty

When behavior is uncertain, resolve in this order:

1. Actual C source behavior
2. Public headers and API contract
3. Existing tests and usage patterns
4. Observable behavior from targeted experiments
5. Minimal-change conservative interpretation

#### Required actions

- Do not guess when the C source can be inspected.
- Do not “improve” unspecified behavior in ways that may break compatibility.
- Document every uncertain behavior that affects implementation choices.
- Add characterization tests for any behavior discovered from source reading or empirical validation.
- If two interpretations are possible, choose the one least likely to break existing callers until evidence justifies a stricter change.
- Mark unresolved uncertainty explicitly in planning and review documents.

#### Conservative defaults

Unless disproven, treat the following as compatibility-sensitive:

- null input handling,
- zero-length operations,
- spare-capacity semantics,
- reallocation return semantics,
- pointer invalidation behavior after resize,
- manual length-adjustment invariants,
- post-failure validity of the original object,
- exact meaning of allocation inspection functions.

#### Escalation rule

Any uncertainty affecting memory safety, FFI correctness, or public compatibility must block final acceptance until resolved by:
- source confirmation,
- tests,
- or explicit project decision recorded in writing.

---

### 2.3 Test Verification Requirements

Tests are mandatory evidence of equivalence, not optional support material.

#### Required test layers

The project must maintain all of the following where applicable:

1. **Characterization tests**
   - Capture existing C behavior before or during migration.
2. **Compatibility tests**
   - Validate the Rust implementation against the C API contract.
3. **Regression tests**
   - Prevent reintroduction of discovered bugs.
4. **Boundary and edge-case tests**
   - Cover zero lengths, empty strings, growth edges, shrink edges, representation thresholds, and manual buffer workflows.
5. **FFI tests**
   - Validate C-to-Rust calling conventions and lifecycle behavior.
6. **Performance benchmarks**
   - Measure hot-path behavior against baseline expectations.

#### Required behavioral coverage

Tests must cover at minimum:

- construction from explicit length,
- construction from C strings,
- duplication,
- free/release behavior,
- clear versus free distinction,
- append operations,
- copy/replace operations,
- room reservation,
- free-space removal,
- zero growth,
- manual length increment/decrement where supported,
- length recomputation from buffer content,
- allocation size inspection,
- allocation pointer access semantics where exposed,
- empty string reuse,
- threshold-sensitive representation behavior,
- return-handle usage after resizing operations.

#### Cross-validation rule

Where practical, tests must compare Rust behavior directly against the C implementation for identical inputs and operation sequences.

---

## 3. Quality Gates

### 3.1 Tests That Must Pass

No migration milestone is complete unless the required tests pass.

#### Mandatory passing criteria

- All existing relevant C project tests must pass under the migrated build or compatibility setup.
- All newly added characterization tests must pass.
- All Rust unit tests must pass.
- All integration and FFI tests must pass.
- All regression tests added during migration must pass.
- All edge-case and threshold tests required by this constitution must pass.
- No test may be silently removed to make the migration appear complete.

#### Minimum acceptance scenarios

Before any release candidate or equivalent milestone, the following scenarios must be verified:

- create → use → free lifecycle,
- empty-string creation and reuse,
- append after preallocation,
- copy/replace into existing buffer,
- grow then manual write then length synchronization,
- remove free space after growth,
- operations around representation-size boundaries,
- failure-path behavior for allocation-sensitive operations, to the extent observable and testable,
- compatibility of returned handle semantics after resize-capable calls.

#### Tooling expectations

At minimum, CI or equivalent automation must run:
- build checks,
- test suite,
- formatting checks,
- lint checks,
- benchmark smoke checks.

---

### 3.2 Code Review Standards

No code may merge unless it satisfies the project review standards.

#### Required review checks

Every change must be reviewed for:

- behavioral equivalence with the C implementation,
- public interface compatibility,
- soundness of ownership and lifetime handling,
- correctness of length/capacity/null-termination invariants,
- adequacy of tests,
- impact on performance-sensitive paths,
- clarity of unsafe justifications,
- conformance with documented migration decisions.

#### Unsafe review rules

Any code containing `unsafe` must be reviewed with additional scrutiny:

- each unsafe block must be as small as possible,
- safety comments must state required invariants,
- reviewers must verify that those invariants are established and maintained,
- tests must exist for the boundary conditions related to that unsafe code,
- unsafe code must not be accepted merely because it “matches the C version.”

#### Review rejection conditions

A change must be rejected if it:

- alters externally visible semantics without explicit approval,
- reduces API compatibility without an approved transition plan,
- introduces unbounded or unmeasured performance cost on hot paths,
- adds undocumented unsafe assumptions,
- weakens tests or removes characterization coverage without replacement,
- relies on speculation about uncertain C behavior.

---

### 3.3 Performance Benchmark Requirements

Performance must be demonstrated, not asserted.

#### Required benchmark categories

Benchmarks must cover at least:

- construction from explicit-length buffers,
- construction from null-terminated strings,
- duplication,
- append of small and large payloads,
- repeated append with and without preallocation,
- copy/replace into reused buffers,
- zero-growth extension,
- free-space removal,
- workloads near representation-size thresholds.

#### Benchmark policy

- A baseline must be established from the C implementation or from an approved reference build.
- Rust results must be compared against that baseline for key operations.
- Any material regression on hot paths must be investigated and documented before acceptance.
- Claimed improvements are welcome but do not waive compatibility requirements.
- Benchmarks must reflect realistic usage patterns, especially append-heavy and reuse-heavy flows.

#### Regression thresholds

Exact numeric thresholds may be defined in later benchmark plans, but this constitution requires:

- no unreviewed regression in core append/grow/copy paths,
- no hidden asymptotic degradation,
- no acceptance of avoidable extra allocation churn,
- no acceptance of repeated full-buffer scans where the C model maintains explicit lengths.

---

## 4. Project Enforcement

### 4.1 Binding Authority

This constitution is binding on:
- specs,
- migration plans,
- implementation tasks,
- test plans,
- review decisions,
- release acceptance.

Any subordinate document that conflicts with this constitution must be corrected.

### 4.2 Amendment Rule

If project learning reveals a need to change these principles, the change must be made explicitly by amending this document. Silent drift is not allowed.

### 4.3 Completion Standard

The Rust migration of `sds` is complete only when:

- required behavior is preserved,
- interface compatibility is satisfied,
- safety invariants are explicit and enforced,
- tests pass,
- benchmarks are acceptable,
- review standards have been met,
- remaining uncertainties have been resolved or formally accepted.