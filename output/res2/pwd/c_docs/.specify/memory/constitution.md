# Constitution for the Rust Migration of `pwd`

## Purpose

This document defines the binding project-level principles for the Rust migration of the C project `pwd`. It is the governing standard for architecture, implementation, review, testing, and release decisions across all 29 module units and all supporting work.

Where a future spec, plan, task, or implementation conflicts with this constitution, this constitution takes precedence.

---

## 1. Core Principles

### 1.1 Behavioral Equivalence Principle

The Rust migration must preserve the externally observable behavior of the C program unless a deliberate, documented exception is approved.

This includes, at minimum:

- command-line control flow shape
- success and failure paths
- working-directory resolution behavior
- distinction between logical and robust path acquisition strategies
- usage and version flows
- output timing and finalization behavior
- quoting behavior and quoting API categories
- locale- and charset-sensitive behavior where present
- centralized allocation-failure behavior
- root detection and parent traversal stopping behavior
- broken-pipe handling policy

#### Required interpretation

1. The Rust version is a behavioral reimplementation, not a redesign.
2. If the C version has multiple operational modes, the Rust version must preserve those modes rather than collapsing them into one simpler path.
3. If the C version contains explicit state machines, the Rust design must preserve the same behavioral states and transitions even if represented with different Rust types.
4. Differences in internal structure are allowed only when external behavior remains equivalent.

#### Known behavior categories that must remain distinct

The migration must preserve the behavioral distinction between:

- `logical_getcwd` and `robust_getcwd`
- informational flows and normal execution flows
- buffer-based quoting and allocating quoting
- null-terminated string quoting and explicit-length memory quoting
- per-call quoting and slot-based retained quoting
- normal output errors and configured `EPIPE` special handling

#### Equivalence standard

Behavioral equivalence is judged by:

1. CLI-visible behavior
2. API-level behavior for preserved interfaces
3. error handling behavior
4. resource lifecycle behavior that affects outcomes
5. test and benchmark results against the C baseline

Where exact behavior is uncertain from available summaries, the project must prefer evidence over assumption.

---

### 1.2 Interface Compatibility First Principle

Public and migration-relevant interfaces must be preserved before internal cleanups are attempted.

This project includes a broad support surface around the main `pwd` behavior, including:

- path acquisition helpers
- quoting configuration and wrapper APIs
- stream finalization helpers
- locale and charset helpers
- memory allocation wrappers
- diagnostic/version helpers

The Rust migration must treat these interfaces as contractual unless explicitly scoped otherwise.

#### Rules

1. Preserve interface boundaries before optimizing internal structure.
2. Preserve function families as families where behavior depends on distinct entry points.
3. Do not remove wrapper APIs merely because they can be expressed through one shared Rust implementation.
4. Preserve stateful subsystems as explicit stateful subsystems when the C behavior depends on mutation or retained state.

#### Specific implications for `pwd`

The following interface groups must remain behaviorally recognizable:

- quoting options mutation APIs
- `quotearg*`, `quote*`, and slot-based quoting entry points
- `close_stdout*` and supporting flush/close behavior
- locale retrieval variants
- `x*alloc` failure-policy family
- file-name builder lifecycle helpers
- root device/inode support
- usage/version entry points

#### Compatibility priority order

When a tradeoff exists, prioritize in this order:

1. externally visible behavior
2. interface compatibility
3. safety improvements
4. maintainability improvements
5. internal elegance

Internal simplification is welcome only after the higher priorities are satisfied.

---

### 1.3 Safety First Principle

The Rust migration must maximize memory safety, type safety, and resource safety while preserving behavior.

Rust safety features are not optional embellishments; they are a primary reason for the migration. However, safety improvements must not silently alter behavior.

#### Mandatory safety expectations

1. No undefined behavior in Rust code.
2. No unchecked pointer-style logic where a safe abstraction can preserve the same semantics.
3. No unsound `unsafe` blocks without documented justification.
4. No panic-based replacement for expected runtime error paths unless the original C behavior is process-terminating and the replacement is explicitly approved.
5. No hidden lifetime or ownership shortcuts that make future maintenance unsafe.

#### `unsafe` usage policy

`unsafe` is allowed only when all of the following are true:

- there is a concrete need
- a safe alternative would materially prevent required compatibility or performance
- the scope is minimized
- invariants are documented next to the block
- tests exercise the behavior the `unsafe` code exists to preserve

All `unsafe` must be review-blocking and justification-bearing.

#### Safety and C semantics

Safety improvements must preserve the intent of these C behaviors:

- centralized failure handling instead of unchecked null propagation
- disciplined resource cleanup
- controlled retained state for quoting slots
- explicit output finalization
- careful filesystem traversal boundaries

Rust must express these safely, but must not erase their behavioral role.

---

### 1.4 Performance Constraint Principle

The Rust migration must not impose unjustified regressions in runtime, memory behavior, or startup characteristics for core execution paths.

This is a utility program. Fast startup, low overhead, and efficient hot paths matter.

#### Performance-sensitive areas already identified

Particular care is required for:

- robust current-directory reconstruction
- repeated parent traversal
- directory-entry lookup behavior
- incremental path building
- core quoting engine paths
- wrapper-heavy quoting call families
- stream flush/close finalization
- repeated allocation or resizing paths
- locale/charset helper overhead where used repeatedly

#### Rules

1. Preserve algorithmic complexity unless a change is proven neutral or better.
2. Avoid heap allocation where the original design intentionally supported caller-managed buffers.
3. Avoid duplicating logic across wrapper APIs when a shared core can preserve semantics.
4. Benchmark before approving changes to hot paths.
5. Do not trade large performance regressions for minor internal stylistic improvements.

#### Performance evaluation standard

A Rust implementation is acceptable when:

- correctness is preserved
- startup remains appropriate for a small CLI utility
- normal `pwd` execution shows no material degradation without justification
- robust traversal and quoting paths remain within approved regression limits

---

## 2. Migration Guidelines

### 2.1 C-to-Rust Mapping Rules

The migration must map C concepts into Rust in a way that preserves behavior and improves safety without flattening important distinctions.

#### 2.1.1 Functions and modules

- Each migration-relevant C module or interface family should map to a Rust module or tightly related Rust module set.
- Shared internals may be refactored, but the original public behavior must remain traceable.
- Wrapper families in C may share one Rust core implementation, but the wrapper entry points must remain behaviorally distinct where the C project exposed distinctions.

#### 2.1.2 Structs and mutable state

- C structs representing meaningful runtime state must map to named Rust structs or enums, not loose collections of unrelated variables.
- Mutation-heavy subsystems such as file-name building and quoting options must remain explicit stateful abstractions.
- Anonymous C struct usage in the summaries should be turned into clear Rust types with documented invariants.

#### 2.1.3 Ownership and resource management

- C allocation/free lifecycles must become Rust ownership lifecycles.
- Manual cleanup concepts must be represented through `Drop`, scoped ownership, or explicit cleanup methods where behavior requires explicit release timing.
- If explicit cleanup has observable behavior, retain explicit cleanup APIs even when Rust could clean up automatically later.

#### 2.1.4 Strings, paths, and byte data

- Null-terminated string behavior and explicit-length memory behavior must remain distinct when the C API distinguishes them.
- Path handling must not assume UTF-8 if the original behavior was byte-oriented or locale-sensitive.
- Quoting APIs that operate on arbitrary memory must use byte-oriented Rust types where needed rather than forcing text interpretation.

#### 2.1.5 Error handling

- C return-code or null-on-failure patterns should map to `Result`, `Option`, or explicit process-termination helpers as appropriate.
- Centralized fatal error behavior must remain centralized.
- Recoverable and non-recoverable failures must not be merged without evidence that the original behavior did so.

#### 2.1.6 Global or retained state

- Retained state such as quoting slots or output-close policy must be made explicit and controlled.
- If the C behavior depends on process-global state, the Rust design must preserve behavior while minimizing hidden mutability.
- Global mutable state must be encapsulated behind documented synchronization or single-threaded assumptions.

#### 2.1.7 FFI and libc interaction

- Interactions with OS or libc behavior must be wrapped behind narrow Rust interfaces.
- Raw FFI should not leak through the broader codebase.
- All ABI assumptions, ownership rules, and errno/OS error expectations must be documented at the boundary.

---

### 2.2 Principles for Handling Uncertain Behavior

Some details are explicitly underdetermined by the available summaries. In those cases, the project must follow a disciplined uncertainty policy.

#### 2.2.1 Evidence-first rule

When behavior is uncertain:

1. consult the C source
2. consult existing tests, if any
3. inspect call relationships and observable outputs
4. reproduce behavior empirically where practical
5. document the conclusion and evidence

Do not invent behavior to fill gaps.

#### 2.2.2 No speculative simplification

Unknown behavior must not be replaced with a simpler Rust behavior merely because the simpler behavior seems reasonable.

Examples of prohibited speculation include:

- collapsing logical and robust cwd retrieval into one path without proof
- replacing retained quotearg slot behavior with one-shot formatting only
- removing explicit final output handling because process exit "usually flushes"
- assuming UTF-8-only semantics where locale-sensitive byte behavior may matter
- normalizing all failures into one generic Rust error path

#### 2.2.3 Conservative preservation rule

If a behavior cannot yet be fully explained but is clearly present in the C design, preserve it until enough evidence exists to change it safely.

#### 2.2.4 Documentation requirement for uncertainty

Every unresolved or partially resolved behavior question must be tracked with:

- the uncertain behavior
- what evidence exists
- what remains unknown
- the temporary migration decision
- the test strategy used to guard the decision

#### 2.2.5 Approved change process

Any intentional deviation from the C behavior must include:

- explicit statement of the deviation
- reason for deviation
- safety, compatibility, and performance impact analysis
- approval in project review
- new or updated tests covering the chosen behavior

No silent deviations.

---

### 2.3 Test Verification Requirements

Testing is the primary enforcement mechanism for this constitution.

#### 2.3.1 Baseline comparison requirement

The Rust migration must be validated against the C implementation as the behavioral baseline wherever practical.

Required comparison styles include:

- CLI output comparison
- exit-status comparison
- stderr behavior comparison
- failure-path comparison
- edge-case comparison
- repeated-run consistency

#### 2.3.2 Layered testing requirement

Tests must exist at multiple levels:

1. **module/unit tests** for local logic
2. **integration tests** for subsystem interactions
3. **CLI behavior tests** for user-visible behavior
4. **regression tests** for discovered mismatches
5. **benchmark checks** for critical performance-sensitive paths

#### 2.3.3 Priority test targets for this project

At minimum, tests must cover:

- normal invocation printing current directory
- logical path resolution behavior
- robust path reconstruction behavior
- root boundary handling
- parent traversal behavior
- usage flow
- version flow
- stdout finalization behavior
- broken pipe policy behavior where applicable
- quoting subsystem representative families
- locale-sensitive helper behavior where observable
- allocation-failure policy behavior where testable

#### 2.3.4 Golden behavior policy

Where output is stable and observable, golden tests are preferred. Where behavior depends on environment, differential tests against the C binary are preferred.

#### 2.3.5 Regression permanence rule

Any discovered migration bug must be accompanied by a regression test before the fix is considered complete.

---

## 3. Quality Gates

### 3.1 Tests That Must Pass

No migration milestone is complete unless all applicable quality-gate tests pass.

#### 3.1.1 Mandatory passing categories

1. **Build correctness**
   - clean build via the project build system
   - reproducible build behavior in CI-supported environments

2. **Unit and integration tests**
   - all Rust unit tests pass
   - all integration tests pass
   - all targeted subsystem tests pass

3. **Behavioral parity tests**
   - Rust and C outputs match for approved comparison scenarios
   - Rust and C exit behavior matches for approved comparison scenarios
   - Rust and C stderr/diagnostic behavior matches where specified

4. **CLI acceptance tests**
   - normal execution path passes
   - usage path passes
   - version path passes
   - invalid invocation scenarios pass

5. **Regression suite**
   - all previously added regression tests pass with no quarantined failures unless explicitly approved

#### 3.1.2 Required test environments

Tests must include, as applicable:

- normal locale/default environment runs
- environment variations relevant to locale-sensitive behavior
- filesystem situations exercising logical vs robust retrieval paths
- pipe/output scenarios for close/finalization behavior

#### 3.1.3 Failure policy

A change that breaks parity, removes coverage, or introduces unexplained nondeterminism must not merge.

---

### 3.2 Code Review Standards

All migration code must undergo review against this constitution, not only for local correctness.

#### 3.2.1 Review checklist

Every review must verify:

- behavioral equivalence is preserved or deviations are documented
- interface compatibility is maintained
- safety properties are improved or preserved
- `unsafe` usage is justified and minimal
- error handling matches project rules
- tests adequately cover the change
- performance-sensitive changes include evidence
- uncertainty is documented rather than guessed through

#### 3.2.2 Review-blocking issues

The following are merge blockers:

- undocumented behavioral changes
- removal of an interface distinction present in C without approval
- unreviewed or unjustified `unsafe`
- panic-based control flow replacing expected runtime behavior
- inadequate tests for changed logic
- speculative behavior decisions with no evidence trail
- performance regressions on critical paths with no approval
- hidden global mutable state without clear invariants

#### 3.2.3 Review preference rules

Reviewers should prefer implementations that are:

1. behaviorally faithful
2. safe by construction
3. explicit about invariants
4. easy to test against the C baseline
5. modular without obscuring original semantics

---

### 3.3 Performance Benchmark Requirements

Performance must be checked, not assumed.

#### 3.3.1 Mandatory benchmark scope

Benchmarks or equivalent measurements must exist for:

- standard `pwd` invocation path
- path acquisition path(s), especially robust traversal if exercised
- quoting core path where relevant to migrated support libraries
- output finalization path if materially changed
- allocation-heavy helper behavior if materially changed

#### 3.3.2 Comparison baseline

Benchmark comparisons should use the C implementation as the primary baseline and prior Rust revisions as the secondary baseline.

#### 3.3.3 Benchmark approval rule

Performance regressions in critical paths require one of:

- demonstrated measurement noise only
- compensating safety or correctness benefit with explicit approval
- a documented follow-up optimization plan

#### 3.3.4 Regression thresholds

Exact numeric thresholds may be defined in later project documents, but no benchmarked regression may be ignored simply because the program is small. Small utilities still require disciplined performance.

---

## 4. Governance and Precedence

### 4.1 Binding force

This constitution is binding on:

- architecture decisions
- module migration plans
- task decomposition
- implementation choices
- review outcomes
- test design
- release readiness decisions

### 4.2 Amendment rule

This document may be amended only through explicit project-level approval. Silent drift through implementation is prohibited.

### 4.3 Interpretation rule

When a later document is ambiguous, interpret it in the way most consistent with:

1. behavioral equivalence
2. interface compatibility
3. safety
4. performance discipline
5. evidence-based decision making

### 4.4 Project maxim

For this migration, the governing rule is:

**Preserve behavior, preserve interfaces, improve safety, and prove performance.**