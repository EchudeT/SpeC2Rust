# Constitution for the Rust Migration of `c4`

## Purpose

This document defines the binding project-level principles for the Rust migration of the C project `c4`. It is the governing standard for all later specifications, plans, implementation tasks, reviews, and release decisions.

Where a later document conflicts with this constitution, this constitution takes precedence.

---

## 1. Core Principles

### 1.1 Behavioral Equivalence Principle

The Rust migration shall preserve the observable behavior of the C project as the primary correctness target.

#### Requirements

- The Rust implementation must preserve the runtime roles and control decomposition visible in the current C codebase:
  - `main(int argc, char **argv)`-driven `c4` flow
  - `main()`-driven `hello` flow
  - distinct internal behavior roles for:
    - `next()`
    - `expr(int lev)`
    - `stmt()`
- The migration must preserve externally observable outcomes for supported inputs, including:
  - process exit behavior
  - stdout/stderr behavior
  - argument-driven behavior
  - success/failure distinctions
  - ordering-sensitive behavior where observable
- The migration must preserve the high-level shared-state execution model where required by semantics, even if the internal Rust representation changes.
- Root and test code paths that are mirrored in C must remain behaviorally aligned in Rust unless a later approved document explicitly establishes and justifies a divergence.

#### Rules

- Equivalent behavior is required; textual or structural similarity is not.
- Internal refactoring is allowed only if behavior remains equivalent.
- Unknown behavior must not be silently reinterpreted into a more idiomatic Rust behavior without evidence.
- If the C behavior appears odd, fragile, global-state-driven, or non-idiomatic, it must still be preserved unless there is explicit approval to change it.

#### Specific application to `c4`

Based on available evidence, the migration must preserve:

- separate startup paths for `c4` and `hello`
- separation of token/state advancement, expression handling, and statement handling
- level-sensitive expression processing through an equivalent of `expr(int lev)`
- internally self-contained module behavior with no newly introduced cross-module semantic dependencies

---

### 1.2 Interface Compatibility First Principle

Public and build-visible interfaces shall be preserved before internal redesign is attempted.

#### Requirements

- The Rust project must provide build targets and executable behaviors compatible with the existing Makefile-oriented project structure, unless and until the build migration is explicitly changed in an approved plan.
- Function-role compatibility matters even where direct function-name export compatibility is not required.
- Entry-point compatibility must be preserved:
  - `c4`-family executable path must continue to accept command-line arguments
  - `hello`-family executable path must continue to behave as an argument-less direct main-style program where applicable
- Any Rust module structure must reflect the original module boundaries enough to preserve traceability:
  - `main_root`
  - `module_test`

#### Rules

- Do not collapse distinct C responsibilities into a single Rust routine if doing so obscures traceability or changes semantics.
- Preserve conceptual interfaces before improving ergonomics.
- Make compatibility decisions in this order:
  1. preserve observable behavior
  2. preserve build and execution interfaces
  3. preserve module traceability
  4. improve internal Rust design

#### Compatibility interpretation

Because the source set contains multiple `main` functions across root and test files, the Rust migration must not assume a single executable layout without build evidence. If build-target selection is unclear, the migration must preserve the ambiguity safely rather than inventing a simplified target model.

---

### 1.3 Safety First Principle

Rust safety improvements are mandatory, but safety must be introduced without changing required behavior.

#### Requirements

- Safe Rust is the default.
- `unsafe` is allowed only when strictly necessary and must be:
  - minimal in scope
  - documented with a safety justification
  - reviewed with special scrutiny
- Undefined, implementation-defined, or brittle C behavior must be investigated before being rewritten into a different semantic model.
- Shared mutable state inferred from the C design must be represented in Rust in a way that:
  - avoids unsound aliasing
  - preserves sequencing semantics
  - makes state transitions explicit enough for review

#### Rules

- Never introduce memory unsafety to mimic C.
- Never use `unsafe` to avoid understanding the original semantics.
- Never replace uncertain C behavior with panic-driven Rust behavior unless panic is verified to match intended termination semantics.
- Error handling in Rust must not invent recoverability that the C program does not exhibit.
- Absence of explicit C error returns does not authorize silent ignoring of errors in Rust.

#### Practical safety standards

- Prefer explicit state structs over hidden global mutation where semantics permit.
- Prefer enums and typed state over magic integers where semantics are known.
- Preserve integer-width behavior carefully; do not widen or narrow without checking semantic consequences.
- Any translation of parser or state-machine logic must keep control flow auditable and deterministic.

---

### 1.4 Performance Constraint Principle

The Rust migration must not materially degrade the performance characteristics of the original C project in its core paths.

#### Requirements

- Performance work shall focus first on the likely hot path identified by the current evidence:
  - `next()`
  - `expr(int lev)`
  - `stmt()`
- Rust abstractions are acceptable only when they do not impose unjustified overhead on core processing paths.
- Allocation, copying, and indirection introduced during migration must be justified, especially in repeated parsing or state-advance loops.
- The `hello` flow may prioritize simplicity, but the `c4` flow must prioritize preserving tight internal execution characteristics.

#### Rules

- Do not introduce heap allocation into per-token or per-step paths without evidence it is acceptable.
- Do not replace compact shared-state transitions with heavyweight abstraction layers in the hot path.
- Do not trade large performance regressions for idiomatic purity.
- Benchmark before and after any major architectural rewrite affecting parsing flow.

#### Performance interpretation under uncertainty

Where exact C performance baselines are not yet established, the migration shall assume that the dense internal interaction among `next`, `expr`, and `stmt` is performance-sensitive and design accordingly.

---

## 2. Migration Guidelines

### 2.1 C-to-Rust Mapping Rules

These rules govern how C implementation patterns shall be translated into Rust.

#### 2.1.1 Function and control-flow mapping

- Each behaviorally meaningful C function must map to one of:
  - a Rust function
  - a Rust method
  - a small, traceable Rust state-machine transition
- The following functions must remain separately identifiable in the Rust design, even if not as exact symbols:
  - `next`
  - `expr`
  - `stmt`
  - `main` variants
- `expr(int lev)` must retain an explicit level/precedence/mode parameter or an equivalent representation that preserves caller-controlled semantics.

#### 2.1.2 Shared-state mapping

Because the C behavior strongly suggests non-local state:

- Shared parsing or execution state should be represented explicitly in Rust.
- Preferred representations:
  - a dedicated state struct
  - scoped mutable context passed through functions
  - tightly controlled interior mutability only when necessary
- Hidden global state should not be reintroduced unless required for equivalence and explicitly justified.

#### 2.1.3 Module mapping

- Rust modules should preserve original source-family separation:
  - root/source behavior
  - test/mirrored behavior
- Mirrored C source families should remain traceable to their Rust equivalents.
- Cross-cutting abstractions may be introduced only if they do not erase source-to-source accountability.

#### 2.1.4 Type mapping

- C integer usage must be mapped deliberately, not casually.
- Pointer-driven state in C must be translated into Rust ownership and borrowing models with behavior preservation as the deciding factor.
- Sentinel-style state, if discovered, must be modeled explicitly rather than hidden behind default values.
- Void-returning helper functions must not automatically become result-returning functions unless that change does not affect external or internal control semantics.

#### 2.1.5 Build mapping

- The Makefile remains the authoritative build contract unless superseded by an approved migration plan.
- Rust integration must fit the existing build workflow during migration stages where mixed validation against C is required.
- Build targets for root and test behavior must remain distinguishable.

---

### 2.2 Principles for Handling Uncertain Behavior

Where the provided analysis is incomplete, uncertainty shall be handled conservatively.

#### Requirements

- Unknowns must be documented explicitly.
- No developer may fill semantic gaps with assumption disguised as fact.
- If evidence is insufficient, preserve structure first and defer semantic optimization.
- Every unresolved behavior question must be classified as one of:
  - blocked by missing source inspection
  - blocked by missing runtime evidence
  - blocked by missing build-target evidence
  - intentionally deferred with bounded risk

#### Conservative interpretation rules

- Prefer preserving call decomposition over collapsing logic.
- Prefer preserving ordering over rearranging control flow.
- Prefer preserving statefulness over forcing pure-functional structure.
- Prefer preserving ambiguous behavior over prematurely normalizing it.

#### Required actions when behavior is uncertain

At least one of the following must occur before implementation is considered complete:

- inspect the original C source directly
- add characterization tests against the C binary
- compare root and test variants for equivalence
- document the uncertainty and freeze the Rust implementation to the narrowest safe interpretation

#### Forbidden responses to uncertainty

- inventing simplified parsing rules
- deleting apparently unused behavior without proof
- replacing silent C behavior with warnings or panics by default
- assuming test files are disposable duplicates without evidence
- assuming all `main` variants compile into a single binary target

---

### 2.3 Test Verification Requirements

Testing is mandatory evidence of equivalence, not an optional cleanup step.

#### Required verification layers

##### A. Build verification

- The project must build cleanly through the intended Makefile-driven workflow.
- All Rust-integrated targets must compile without warnings that indicate likely semantic issues.
- Root and test target selection must be validated explicitly.

##### B. Behavioral equivalence verification

For each migrated executable flow, the project must verify equivalence against the C implementation where feasible:

- command-line behavior
- exit status behavior
- output behavior
- representative success cases
- representative invalid or boundary cases when identifiable

##### C. Functional trace verification

For `c4` behavior, tests must cover the coordinated operation of:

- `next`
- `expr`
- `stmt`

This may be via unit tests, integration tests, snapshot tests, or characterization harnesses, but the decomposition must remain testable.

##### D. Mirrored behavior verification

- Root and test family behavior must be checked for intended alignment.
- If divergence is discovered, it must be documented and approved rather than silently encoded.

#### Minimum testing expectations

- Characterization tests for current C behavior before or during migration
- Rust unit tests for internal state transitions where feasible
- Integration tests for executable-level behavior
- Regression tests for every bug found during migration
- Performance checks for hot-path-sensitive changes

---

## 3. Quality Gates

No migration milestone is complete unless all applicable quality gates pass.

### 3.1 Tests That Must Pass

#### Mandatory gates for all merged changes

- All existing project tests must pass.
- All newly added Rust unit tests must pass.
- All integration tests for executable behavior must pass.
- Any C-vs-Rust characterization comparison tests must pass for covered cases.
- Build validation through the project Makefile must pass.

#### Mandatory gates for parser/state-flow changes

Any change affecting `next`, `expr`, `stmt`, argument processing, or shared execution state must also pass:

- targeted regression tests for the affected flow
- boundary-condition tests for the changed control path
- test coverage demonstrating unchanged behavior on representative known-good inputs

#### Mandatory gates for mirrored source-family changes

Where root and test families are both migrated or modified:

- both variants must compile
- both variants must execute under their intended targets
- alignment checks or documented divergence approval must be present

#### Failure policy

If equivalence evidence and Rust implementation disagree:

- the change does not pass
- the discrepancy must be explained
- either the implementation changes, or the constitution-level principle must be explicitly superseded by approved project governance

---

### 3.2 Code Review Standards

Every merged change must pass review against these standards.

#### Review criteria

- **Behavioral fidelity:** Does the change preserve observed C behavior?
- **Traceability:** Can the reviewer map Rust logic back to the original C responsibility?
- **Uncertainty discipline:** Are assumptions clearly marked and justified?
- **Safety:** Is the Rust code sound, with `unsafe` minimized and justified?
- **Performance awareness:** Does the change avoid unjustified regressions in hot paths?
- **Test adequacy:** Is there sufficient evidence for correctness?

#### Required review practices

- Any nontrivial translation of parser or state logic must include explanation of how it maps to C behavior.
- Any use of `unsafe` requires an explicit safety comment and reviewer acknowledgement.
- Any intentional behavioral deviation requires:
  - clear documentation
  - justification
  - explicit approval
- Any major restructuring must preserve or improve testability and traceability.

#### Review rejection conditions

A change must be rejected if it:

- changes behavior without evidence or approval
- removes the visible distinction among `next`, `expr`, and `stmt` without justification
- assumes facts not supported by source or tests
- introduces panic-based control flow in place of unknown C behavior without validation
- adds avoidable allocation or abstraction overhead in core paths
- bypasses Makefile compatibility without an approved build-migration decision

---

### 3.3 Performance Benchmark Requirements

Performance validation is required for changes that may affect execution characteristics.

#### Benchmark scope

At minimum, benchmark-sensitive work includes changes to:

- `next` logic
- `expr` logic
- `stmt` logic
- shared parser/execution state representation
- argument handling for the `c4` executable path
- any refactor that changes data ownership or allocation behavior in core flow

#### Benchmark expectations

- Establish a baseline using the C implementation where practical.
- Compare Rust changes against the latest accepted Rust baseline at minimum.
- Measure representative workloads for the `c4` flow.
- Prefer stable, repeatable benchmark inputs over anecdotal timing.

#### Acceptance standard

- No significant regression may be merged without explicit approval and justification.
- If a regression is accepted temporarily, a follow-up remediation task must be created.
- Claimed performance improvements must be supported by measured results, not intuition.

#### Performance review triggers

A benchmark review is required when a change introduces:

- new allocations in hot paths
- new dynamic dispatch in hot paths
- new cloning or copying of parser state
- replacement of direct control flow with layered abstractions
- synchronization primitives in formerly local execution paths

---

## 4. Governance and Precedence

### 4.1 Binding effect

This constitution is binding on:

- specifications
- migration plans
- implementation tasks
- code reviews
- test strategy
- build integration decisions

### 4.2 Conflict resolution

If a later document conflicts with this constitution:

1. this constitution wins by default
2. the conflict must be surfaced explicitly
3. any exception must be approved as a deliberate constitutional override

### 4.3 Amendment standard

Amendments to this document must be rare and justified by one or more of:

- direct evidence from the C source
- verified runtime behavior
- proven incompatibility in the current migration approach
- explicit project-scope change in goals

### 4.4 Non-negotiable project laws

The following are non-negotiable unless formally amended:

- behavior first
- interface compatibility before redesign
- safety without semantic drift
- no unjustified performance regression
- no silent assumption in areas of uncertainty
- no merge without test evidence