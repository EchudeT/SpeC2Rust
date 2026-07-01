# constitution.md

# c4 Rust Migration Constitution

This document defines the binding project-level principles for the Rust migration of the C project `c4`. All later specification, planning, implementation, testing, and review documents must conform to this constitution. Where tradeoffs arise, this document takes precedence unless formally amended.

---

## 1. Core Principles

### 1.1 Behavioral Equivalence Principle

The Rust migration must preserve the observable behavior of the C project as the primary success criterion.

#### Requirements

- The Rust version must preserve all behavior that is directly evidenced by the analyzed C project.
- The following behavior shape is mandatory to preserve:
  - `c4`-style execution begins at `main(int argc, char **argv)` in the C source and must have an equivalent Rust CLI entry behavior.
  - `hello`-style execution begins at `main()` in the C source and must have an equivalent Rust entry behavior.
  - `next`, `expr(int lev)`, and `stmt` remain distinct behavioral phases in the Rust design, even if represented by idiomatic Rust functions, methods, or internal modules.
  - `expr` must remain level-parameterized in behavior.
  - program termination must remain return-based from the top-level entry flow.
  - root and test copies must remain behaviorally mirrored.

#### Interpretation Rules

- Behavioral preservation is defined in terms of externally observable results, control-flow responsibilities, accepted inputs, produced outputs, exit behavior, and test-visible semantics.
- Internal rewrites are allowed only if they do not change observable behavior.
- Idiomatic Rust is encouraged, but only after behavioral fidelity is protected.
- Where exact implementation detail is unknown, the project must preserve the strongest behavior supported by available evidence and avoid speculative semantic changes.

#### Non-Negotiable Constraint

No migration change may knowingly alter:
- startup shape of the executables,
- presence and role of the `next` / `expr` / `stmt` phases for `c4`,
- level-sensitive expression handling,
- correspondence between root and test behavior.

---

### 1.2 Interface Compatibility First Principle

Public and project-visible interfaces must be preserved before internal refactoring is pursued.

#### Requirements

- Migration work must begin from interface preservation, not from internal redesign.
- The Rust project structure must reflect the discovered module boundaries:
  - `main_root`
  - `module_test`
- Separate executable-style flows identified in C must remain separate in Rust.
- CLI-facing behavior, argument flow, return conventions, and top-level execution patterns must remain compatible with the original C project.

#### Mapping Expectations

- Each C source file with an independent `main` represents a distinct executable or executable-mode entry and must not be silently collapsed into a single unrelated runtime path.
- `c4.c` and `hello.c` must remain distinguishable artifacts in the Rust migration, whether as separate binaries, feature-gated binaries, or another explicitly documented equivalent.
- Test-side mirrored copies must remain traceable to their source counterparts.

#### Design Rule

If a proposed Rust abstraction improves code elegance but obscures or changes the original interface shape, the abstraction must be rejected or deferred.

---

### 1.3 Safety First Principle

The migration must improve memory and type safety without changing required behavior.

#### Requirements

- Rust safety guarantees must be used as a project advantage, not bypassed casually.
- Safe Rust is the default.
- `unsafe` is forbidden unless all of the following are true:
  1. it is strictly necessary,
  2. no reasonable safe alternative exists,
  3. the reason is documented inline and in review,
  4. the unsafe scope is minimized,
  5. tests cover the affected behavior.
- Panics must not be introduced in place of expected C-style control flow unless explicitly justified and behaviorally validated.
- Integer conversions, indexing, buffer handling, and argument parsing must be explicit and reviewed for semantic equivalence.

#### Behavioral Safety Rule

Safety improvements must not erase behaviors that callers or tests rely on. In particular:
- invalid or edge-case inputs must not be silently normalized unless the C behavior is proven to do so,
- failure handling must not be invented without evidence,
- implicit assumptions in the C code must be surfaced and documented before being changed.

---

### 1.4 Performance Constraint Principle

The Rust migration must not introduce unjustified regressions in structurally sensitive execution paths.

#### Requirements

- Performance-sensitive paths must be treated conservatively, especially around:
  - `main`
  - `next`
  - `expr`
  - `stmt`
- Since the analyzed modules show all observed behavior is internal, the Rust rewrite must assume runtime cost is concentrated in these internal paths.
- `expr(int lev)` is considered structurally performance-sensitive and must not be burdened by avoidable allocations, excessive cloning, or unnecessary dynamic dispatch.
- The `hello` path must remain lightweight and simple.

#### Allowed Changes

- Performance-neutral safety improvements are allowed.
- Performance improvements are allowed if they do not alter behavior.
- Data structure substitutions are allowed only when benchmarked or clearly justified as non-regressive.

#### Forbidden Changes

- Repeated heap allocation in tight helper paths without justification.
- Unreviewed introduction of recursion, boxing, trait-object dispatch, or cloning in core paths.
- Refactors that make the Rust version materially slower without measured evidence and explicit approval.

---

## 2. Migration Guidelines

### 2.1 C-to-Rust Mapping Rules

The migration must follow these default mapping rules unless a deviation is formally documented.

#### File and Module Mapping

- Preserve recognizable correspondence between C files and Rust modules.
- Maintain a clear mapping for:
  - `c4.c`
  - `hello.c`
  - `test/c4.c`
  - `test/hello.c`
- The project layout must make it possible for reviewers to trace each Rust artifact back to its C origin.

#### Function Mapping

- C functions should map initially to Rust functions with corresponding names where practical:
  - `next` -> `next` or a clearly equivalent name if `next` conflicts with idiomatic or reserved usage concerns.
  - `expr(int lev)` -> `expr(lev: ...)`
  - `stmt()` -> `stmt()`
  - `main(...)` -> Rust binary entry with documented argument handling equivalence.
- Distinct C functions must not be prematurely fused into one Rust routine if that would hide behavioral phases.

#### State and Data Mapping

- Global C state, if present in source implementation, should be migrated into explicit Rust state structures as early as practical.
- Shared mutable state must be minimized and made explicit.
- Integer widths and signedness must be selected deliberately to preserve C semantics where behavior depends on them.
- Pointer-based iteration or buffer traversal in C should become explicit slice, index, iterator, or state-machine logic in Rust, with semantics preserved.

#### Build Mapping

- The Makefile remains part of the project contract until replaced by an approved equivalent.
- If Cargo is introduced, the project must either:
  - preserve existing Makefile-driven workflows, or
  - provide a Makefile wrapper that preserves the original developer and CI entry points.
- Build system changes must not break existing invocation expectations without explicit project approval.

---

### 2.2 Principles for Handling Uncertain Behavior

The source analysis contains evidence gaps. Those gaps must be handled conservatively.

#### Default Rule

When behavior is uncertain, do not invent semantics.

#### Required Approach

- Prefer direct source inspection over assumption.
- Prefer characterization tests over interpretation.
- Prefer preserving control-flow shape over “cleaning up” unknown behavior.
- Prefer explicit documentation of uncertainty over silent guessing.

#### Decision Order for Uncertain Cases

1. Check the original C source body.
2. Add or run characterization tests against the C implementation.
3. Compare root and test copies for mirrored behavior.
4. Select the narrowest Rust interpretation consistent with evidence.
5. Document the uncertainty in code comments or migration notes.

#### Forbidden Responses to Uncertainty

- “Fixing” behavior because it appears odd without proof it is a bug.
- Replacing unknown failure behavior with Rust panics by default.
- Replacing low-level control flow with a higher-level abstraction that changes sequencing assumptions.
- Removing parameters such as `lev` because their exact meaning is not yet known.

#### Specific Rule for `expr(int lev)`

Because `lev` is the only observed semantic control parameter:
- its presence is mandatory,
- its effect must remain behaviorally meaningful,
- its valid range must not be narrowed without evidence,
- callers must continue to supply it explicitly or through a documented equivalent.

---

### 2.3 Test Verification Requirements

Testing is the authoritative mechanism for proving migration correctness.

#### Required Test Strategy

The project must maintain three layers of verification:

1. **Build verification**
   - the Rust project builds cleanly through the approved project workflow,
   - the original C project remains buildable during migration unless formally retired.

2. **Behavioral verification**
   - existing tests must continue to pass,
   - mirrored root/test behavior must be checked,
   - executable entry behavior must be validated.

3. **Comparative verification**
   - where feasible, the Rust output and exit behavior must be compared against the C implementation for the same inputs.

#### Characterization Test Rule

If behavior is not already covered by tests and is relevant to migration risk, a characterization test must be added before refactoring or semantic restructuring.

#### Minimum Areas to Verify

- startup behavior of `c4` flow,
- startup behavior of `hello` flow,
- participation and preservation of `next`, `expr`, and `stmt` paths,
- argument-bearing entry behavior for `c4`,
- return and exit behavior,
- root/test mirrored behavior.

---

## 3. Quality Gates

No migration change is complete until all applicable quality gates pass.

### 3.1 Tests That Must Pass

#### Mandatory Gates

- All existing project tests must pass.
- All newly added Rust unit tests must pass.
- All integration or end-to-end tests comparing executable behavior must pass.
- Any mirrored tests for `test/c4.c` and `test/hello.c` equivalents must pass.
- Build verification for the approved Makefile-driven workflow must pass.

#### Required Comparative Coverage

At minimum, test coverage must demonstrate:
- the Rust `c4` entry path accepts and processes command-line arguments in a behaviorally equivalent way to the C version for covered cases,
- the Rust `hello` entry path preserves its single-function execution model at the observable level,
- the Rust migration preserves the distinct behavior roles of `next`, `expr`, and `stmt`,
- the Rust test-side implementation mirrors the root-side behavior for equivalent scenarios.

#### Regression Rule

A PR that reduces behavioral coverage in a touched area must not merge unless:
- an equivalent stronger test replaces the removed one, and
- reviewers explicitly approve the replacement.

---

### 3.2 Code Review Standards

Every migration change must be reviewed against this constitution.

#### Required Review Questions

Reviewers must verify:

- Does this change preserve observed C behavior?
- Does this change preserve interface shape before improving internals?
- Does this change avoid speculative semantics?
- Does this change use safe Rust by default?
- Is every conversion, ownership transfer, and mutable state path understandable?
- Does this change preserve or justify performance in core paths?
- Are root and test behaviors still mirrored?

#### Mandatory Review Standards

- No unreviewed `unsafe`.
- No unexplained panic paths in migrated control flow.
- No undocumented deviations from C structure or semantics.
- No hidden behavior changes inside “cleanup” commits.
- No merging with failing tests or missing benchmark evidence where performance-sensitive code changed.

#### Documentation Standard

Any intentional deviation from a literal C-to-Rust translation must include:
- what changed,
- why it is safe,
- why it preserves behavior,
- how it was tested.

---

### 3.3 Performance Benchmark Requirements

Performance must be checked whenever changes affect structurally sensitive paths.

#### Benchmark Trigger Conditions

Benchmarks are required when a change touches:
- `next`
- `expr`
- `stmt`
- parsing/stepping loops
- argument processing in the main execution path
- allocation strategy in core runtime code

#### Benchmark Expectations

- Compare before/after Rust performance for touched paths.
- Where feasible, compare Rust behavior against the C baseline on representative inputs.
- Record benchmark method and input shape.
- Reject conclusions based on anecdotal timing.

#### Acceptance Rule

A change may merge with a measured regression only if:
- the regression is small and justified, or
- it buys necessary correctness/safety, and
- the tradeoff is documented and approved.

#### Performance Hygiene Rules

- Avoid unnecessary allocation in hot paths.
- Avoid cloning where borrowing suffices.
- Avoid dynamic dispatch in core parser/execution paths unless justified.
- Prefer explicit, simple control flow in performance-sensitive routines.

---

## 4. Governance and Precedence

### 4.1 Authority

This constitution is the governing standard for the `c4` Rust migration project.

### 4.2 Precedence

If any future spec, plan, task, or implementation conflicts with this document, this constitution prevails.

### 4.3 Amendment Rule

Any amendment to this constitution must:
- be explicit,
- state the reason,
- identify affected sections,
- be reviewed with the same rigor as a core architectural change.

### 4.4 Practical Standard

When in doubt, choose the option that best satisfies all of the following in order:

1. preserves observed behavior,
2. preserves interface compatibility,
3. improves safety without semantic drift,
4. avoids unjustified performance regression,
5. remains reviewable and testable.