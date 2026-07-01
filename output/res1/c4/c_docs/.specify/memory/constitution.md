# constitution.md

## Purpose

This document defines the binding project-level principles for the Rust migration of the C project `c4`. It is the governing constitution for all later specification, planning, implementation, review, and validation work.

Where later documents conflict with this constitution, this constitution takes precedence.

Because the available evidence is limited to module structure, public interface shape, build organization, and behavior summaries, this constitution is intentionally strict about what may be preserved, what must be proved, and what must not be invented.

---

## Scope

This constitution applies to all migration work covering the observed source tree:

- `c4.c`
- `hello.c`
- `test/c4.c`
- `test/hello.c`

It also applies to all Rust replacements, adapters, tests, build changes, benchmarks, and review artifacts related to those files.

The project currently exposes multiple executable entry paths and duplicated root/test structure. Therefore, all principles in this document apply per build target, not only at repository level.

---

## Project Facts That Constrain the Constitution

The following facts are treated as project constraints:

- Build system: `Makefile`
- C files: 4
- Header files: 0
- Module units: 2
- Cluster units: 11
- Public module inventory:
  - `main_root`
  - `module_test`

Observed callable structure includes:

- `next()`
- `expr(int lev)`
- `stmt()`
- `main(int argc, char **argv)` for `c4` variants
- `main()` for `hello` variants

Observed structural behavior includes:

- multiple `main` definitions depending on chosen build target
- a staged `c4` path centered on `next`, `expr`, and `stmt`
- a simpler `hello` path centered only on `main`
- mirrored root and `test/` implementations

These facts are sufficient to define migration law even where body-level runtime details remain unknown.

---

# 1. Core Principles

## 1.1 Behavioral Equivalence Principle

### Law

The Rust migration must preserve the observable behavior of each selected C build target to the maximum extent supported by available evidence.

### Required interpretation

1. Behavioral equivalence is defined per executable path, not per repository.
2. The `c4` path and the `hello` path are distinct products and must remain distinct.
3. Root and test variants must preserve their corresponding structural relationships.
4. Preservation applies first to:
   - entry-point shape
   - control-flow layering
   - target selection behavior
   - externally observable success/failure behavior that can be demonstrated
   - test-observed outputs and effects
5. Where exact behavior is not evidenced, the migration must not claim equivalence without proof.

### Specific obligations

- The `c4` Rust implementation must preserve the staged processing model reflected by:
  - `main`
  - `next`
  - `expr(int lev)`
  - `stmt`
- The `hello` Rust implementation must preserve its minimal single-entry character and must not be artificially expanded into an unrelated multi-stage design if that alters observable behavior.
- The existence of a level-sensitive expression-processing API is part of the observable contract and must remain represented in Rust design.
- Main/test correspondence must remain intact. Test-side replacements must reflect the same decomposition as their paired non-test targets unless a deviation is explicitly justified and proven behavior-preserving.

### Prohibitions

The migration must not:

- merge distinct executable behaviors into one unified runtime if target behavior changes
- erase the observed layered `c4` control flow into an unrelated architecture without proof of behavioral preservation
- invent undocumented semantics and then treat them as required compatibility targets
- use “idiomatic Rust” as justification for changing proven behavior

---

## 1.2 Interface Compatibility First Principle

### Law

Before internal refactoring goals, Rust code must preserve the effective interface contract exposed by the C project’s build targets, callable routines, and invocation forms.

### Required interpretation

Interface compatibility includes:

- executable entry-point behavior
- command-line shape where `main(int argc, char **argv)` exists
- function-role compatibility for `next`, `expr(int lev)`, and `stmt`
- per-target build selection semantics
- test harness compatibility as required by the Makefile and project workflow

### Specific obligations

1. The Rust migration must preserve target distinctions:
   - `c4`-style executable path with command-line arguments
   - `hello`-style executable path with minimal entry behavior
   - mirrored `test/` target behavior
2. The Rust build integration must allow the Makefile-centered workflow to continue operating.
3. If a Rust module restructures internals, it must still expose equivalent top-level execution and callable layering.
4. `expr(int lev)` must remain level-parameterized in some explicit and reviewable form.
5. Any replacement of direct C-style procedural flow with Rust types or methods must still preserve call-role boundaries.

### Ranking rule

When there is tension between:
- internal elegance
- API redesign
- behavior compatibility

the priority order is:

1. behavioral compatibility
2. interface compatibility
3. safety improvement
4. implementation elegance

unless a formally approved exception says otherwise.

### Prohibitions

The migration must not:

- remove command-line based entry behavior from `c4`
- collapse `hello` and `c4` into one ambiguous executable mode
- hide `expr` level semantics inside an opaque mechanism that cannot be traced back to the C contract
- change build/entry conventions in ways that break existing project usage without an approved compatibility layer

---

## 1.3 Safety First Principle

### Law

The Rust migration must prefer memory safety, type safety, and explicit failure handling, while preserving proven C behavior.

### Required interpretation

Safety is mandatory, but safety changes do not authorize unverified behavioral drift.

The project should use Rust’s safety guarantees to eliminate classes of C failure where possible, but must do so in a way that preserves externally visible behavior unless the C behavior is both:
- demonstrably unsafe, and
- not part of a required compatibility contract

### Specific obligations

1. Safe Rust is the default.
2. `unsafe` is allowed only when:
   - it is necessary for interop, performance, or low-level behavior preservation, and
   - the necessity is documented inline and in review.
3. All state transitions that were implicit in C should be made explicit in Rust wherever practical.
4. Integer conversions, indexing, buffer handling, and pointer-like behavior must be validated rather than assumed.
5. Error handling must be explicit in code even when C behavior evidence is limited.
6. Panics must not be used as ordinary control flow in migrated runtime paths.
7. If the original C behavior for invalid input is unknown, Rust code must avoid introducing hidden undefined behavior or silent memory-unsafe assumptions.

### Uncertain-safety rule

When behavior is unclear and one candidate implementation is safer than another, the project shall choose the safer implementation only if it does not contradict known evidence and is covered by tests or documented assumptions.

### Prohibitions

The migration must not:

- introduce `unsafe` to mimic C style when safe Rust is sufficient
- rely on unchecked indexing, unchecked conversions, or unwrap-driven control flow in core runtime paths without justification
- preserve undefined C behavior as an implementation goal unless required by a proven compatibility test
- treat absence of evidence as permission for unsafe shortcuts

---

## 1.4 Performance Constraint Principle

### Law

The Rust migration must preserve the structural performance profile of the C program’s hot paths and must avoid avoidable regressions in repeatedly used runtime routines.

### Required interpretation

The available evidence identifies likely hot paths structurally, not quantitatively. Therefore performance preservation focuses on the routines most likely to dominate runtime behavior:

- `next()`
- `expr(int lev)`
- `stmt()`

### Specific obligations

1. The migration must treat token/input progression and expression/statement processing as performance-sensitive paths.
2. Rust design must avoid unnecessary heap allocation, cloning, string copying, or abstraction overhead in these paths unless justified.
3. The staged internal interaction among these routines must remain efficient and direct.
4. Performance claims must be benchmarked, not assumed.
5. Any deliberate slowdown must be justified by measurable safety or correctness benefit and approved during review.

### Constraint model

Because exact C complexity is not fully evidenced, the project must preserve:

- the existence of a staged processing pipeline
- direct internal interactions among hot routines
- reasonable efficiency for repeated parsing/processing operations

It is acceptable to improve performance, but not by changing observable behavior.

### Prohibitions

The migration must not:

- introduce obviously avoidable per-token or per-expression allocations in hot paths
- replace direct processing with grossly more expensive generalized frameworks without proof of need
- declare performance “non-goal” for core processing routines
- sacrifice tested compatibility for micro-optimizations

---

# 2. Migration Guidelines

## 2.1 C-to-Rust Mapping Rules

### 2.1.1 General mapping rule

Map C constructs to Rust constructs in ways that preserve role, sequencing, and observability before pursuing abstraction.

### 2.1.2 Function mapping

The following function-role expectations are binding:

- `main(int argc, char **argv)` -> Rust executable entry preserving command-line-driven behavior
- `main()` in `hello` paths -> Rust executable entry preserving minimal single-entry behavior
- `next()` -> explicit token/input advancement routine or equivalent clearly named/internalized unit
- `expr(int lev)` -> explicit level-parameterized expression-processing routine
- `stmt()` -> explicit statement-processing routine

Equivalent Rust implementations may use:
- free functions
- methods
- modules
- small structs holding parser/interpreter state

But the original role boundaries must remain visible in code structure and review artifacts.

### 2.1.3 State mapping

Implicit C global or procedural state should be mapped into explicit Rust state holders where practical, such as:

- parser state structs
- input cursor structs
- enums for mode/state transitions
- typed wrappers for level or token state

This is encouraged because it improves safety and reviewability, provided observed behavior is preserved.

### 2.1.4 Data representation rule

Use Rust types that encode invariants explicitly.

Preferred approaches include:

- enums instead of integer tags when semantics are known
- slices/iterators instead of raw pointer arithmetic when behavior is preserved
- `Result` and `Option` instead of sentinel values when externally visible behavior is not altered
- newtypes for semantically meaningful numeric/state parameters where useful

### 2.1.5 Build mapping rule

The Rust migration must fit into the existing `Makefile` workflow.

Allowed approaches include:

- invoking `cargo` from `make`
- compiling Rust targets directly through Makefile rules
- mixed Rust/C transitional builds if needed

Not allowed:

- replacing the Makefile-centered workflow without explicit project approval
- requiring developers or CI to use undocumented manual steps

### 2.1.6 Multi-target rule

Because the project contains multiple `main` definitions, the migration must maintain explicit separation of build targets.

At minimum, the project must preserve the ability to build and validate the equivalent of:

- root `c4`
- root `hello`
- test `c4`
- test `hello`

If consolidation is proposed, it must preserve per-target behavior and be proven by tests and review.

---

## 2.2 Principles for Handling Uncertain Behavior

### 2.2.1 Evidence-first rule

If behavior is not supported by source evidence, interface evidence, tests, or observed execution, it must be treated as uncertain.

Uncertain behavior must never be silently converted into a hard compatibility claim.

### 2.2.2 Hierarchy of truth

When determining what to preserve, use this priority order:

1. existing executable behavior demonstrated by tests or captured runs
2. source code evidence
3. build-target behavior
4. interface summaries
5. behavior summaries
6. documented migration assumptions

### 2.2.3 No invention rule

Do not invent:
- grammar details
- diagnostics
- exit code meanings
- unsupported error recovery rules
- hidden initialization phases
- undocumented input constraints

### 2.2.4 Required action on uncertainty

When behavior is uncertain, the team must do one of the following:

1. investigate and obtain evidence from the C implementation
2. add characterization tests against the C behavior
3. document the uncertainty and choose the least-assumptive compatible implementation
4. escalate the ambiguity for design approval before merging

### 2.2.5 Least-assumption implementation rule

If immediate migration is necessary before full clarification:

- preserve known entry points
- preserve known control-flow layering
- preserve known parameterization
- avoid stronger claims than evidence supports
- choose explicit, reviewable behavior over implicit guessed behavior

### 2.2.6 Unsupported specifics rule

Later specs and tasks must not assert specifics unsupported by evidence, including but not limited to:

- exact output strings
- exact parse grammar
- exact error message text
- exact recursion depth behavior
- exact memory lifetime behavior
- exact end-of-input semantics
- exact argument validation rules

unless they are first established by code inspection or characterization tests.

### 2.2.7 Documentation rule for uncertainty

Every uncertain migration decision must record:

- what is known
- what is unknown
- what evidence was used
- what implementation choice was made
- how that choice will be validated later

---

## 2.3 Test Verification Requirements

### 2.3.1 General rule

No migrated target is complete until equivalence is tested at the appropriate target level.

### 2.3.2 Required categories of tests

The migration must maintain or introduce the following test categories as applicable:

1. **Build tests**
   - each intended target builds through the Makefile workflow

2. **Characterization tests**
   - capture observed C behavior before or during migration
   - especially for uncertain or parser-related logic

3. **Target equivalence tests**
   - compare C and Rust outputs/exit behavior for the same inputs where feasible

4. **Unit tests**
   - verify Rust-level state transitions and helper logic

5. **Integration tests**
   - validate executable behavior per target

6. **Regression tests**
   - cover every bug fixed during migration

### 2.3.3 Required target coverage

At minimum, verification must cover the executable equivalents of:

- `c4`
- `hello`
- `test/c4`
- `test/hello`

### 2.3.4 Hot-path verification rule

Changes affecting `next`, `expr`, or `stmt` require focused tests because those routines represent the observed core processing structure.

### 2.3.5 Uncertainty-driven testing rule

Any area marked uncertain must gain one of:

- a characterization test against C behavior
- a direct source-backed proof note
- an approved temporary assumption with follow-up task

### 2.3.6 Output comparison rule

Where exact output is part of observed behavior, compare exact output.
Where exact output is not yet evidenced, compare only validated aspects and document the gap.

### 2.3.7 Exit behavior rule

Where exit status or failure mode is known, it must be tested.
Where unknown, it must not be asserted as a requirement without evidence.

---

# 3. Quality Gates

## 3.1 Tests That Must Pass

No migration change may merge unless all applicable gates below pass.

### 3.1.1 Build gate

The repository must successfully build the migrated targets through the project’s Makefile-based workflow.

### 3.1.2 Target behavior gate

For every migrated target, all defined equivalence and integration tests must pass.

This includes, as applicable:

- root target tests
- mirrored test target tests
- `hello` path tests
- `c4` path tests

### 3.1.3 Regression gate

All existing regression tests and all newly added migration regression tests must pass.

### 3.1.4 Characterization gate

For behavior that was uncertain at design time, required characterization tests must be present and passing before the relevant migration is considered complete.

### 3.1.5 Safety gate

The Rust code must pass the project’s required static and dynamic checks, including as applicable:

- `cargo test`
- `cargo check`
- `cargo clippy` with project-approved lint policy
- `cargo fmt --check`

If mixed-language transition code exists, the equivalent C/Rust build and test checks must also pass.

### 3.1.6 Panic and error-path gate

Core runtime tests must demonstrate that expected failure handling does not rely on uncontrolled panics unless explicitly approved and documented.

---

## 3.2 Code Review Standards

### 3.2.1 Constitutional compliance gate

Every pull request must state how it satisfies:

- behavioral equivalence
- interface compatibility
- safety
- performance constraints

### 3.2.2 Review checklist

Reviewers must verify:

1. the change preserves the correct target boundary
2. entry-point behavior remains compatible
3. `c4` staged control flow remains traceable
4. `expr` level semantics remain explicit
5. uncertain behavior is not presented as fact
6. tests cover changed behavior
7. `unsafe` usage, if any, is justified and minimized
8. hot-path performance risks are considered
9. Makefile workflow remains functional
10. documentation is updated where assumptions changed

### 3.2.3 Review evidence standard

A claim of compatibility must be backed by at least one of:

- direct source correspondence
- characterization test
- integration test
- benchmark result
- approved design note

### 3.2.4 Required scrutiny for unsafe or structural changes

Any PR that:
- adds `unsafe`
- changes build targets
- changes entry behavior
- restructures `next` / `expr` / `stmt`
- alters error propagation strategy
- changes target output behavior

requires heightened review and explicit approval.

### 3.2.5 Rejection conditions

A change must be rejected if it:

- weakens known behavioral compatibility without approved justification
- hides uncertain behavior behind undocumented assumptions
- breaks Makefile-driven usage
- introduces unjustified unsafe code
- lacks tests for changed observable behavior
- degrades hot-path performance without measurement or approval

---

## 3.3 Performance Benchmark Requirements

### 3.3.1 Benchmark trigger rule

Benchmarks are required for any change that may affect:

- `next()`
- `expr(int lev)`
- `stmt()`
- target startup overhead in `c4`
- repeated processing throughput
- allocation behavior in core loops

### 3.3.2 Baseline rule

Performance comparison should be made against one of:

- the original C implementation
- the previously accepted Rust baseline
- both, when available

### 3.3.3 Measurement rule

Performance must be measured using repeatable inputs and documented commands. Ad hoc impressions are not acceptable evidence.

### 3.3.4 Acceptance rule

A performance regression in a hot path must not be merged unless one of the following is true:

1. the regression is negligible and documented
2. the regression is offset by clear safety/correctness gains
3. the regression is temporary, approved, and tracked with a follow-up task
4. the benchmark shows no meaningful regression under representative workloads

### 3.3.5 Allocation rule

Review and benchmarking must pay specific attention to new allocations, clones, and string/materialization costs introduced into parsing or statement/expression processing paths.

### 3.3.6 Benchmark documentation rule

Benchmark-affecting PRs must document:

- scenario measured
- baseline used
- result summary
- observed regressions or improvements
- rationale for acceptance

---

# 4. Governance and Precedence

## 4.1 Binding force

This constitution is binding on:

- specifications
- migration plans
- task breakdowns
- implementation PRs
- review decisions
- test strategy
- benchmark policy

## 4.2 Amendment rule

This constitution may be amended only by an explicit project decision that documents:

- the current rule
- the reason it is insufficient
- the proposed replacement
- the migration impact
- how existing work will be re-evaluated

## 4.3 Conflict resolution

If later documents conflict with this constitution:

1. this constitution wins
2. the conflicting document must be updated
3. any implementation based on the conflicting rule must be re-reviewed

## 4.4 Default stance

If a situation is not explicitly covered here, the project default is:

- preserve proven behavior
- do not invent unsupported semantics
- keep interfaces compatible
- choose safe Rust
- verify with tests
- measure performance-sensitive changes

---

# 5. Non-Negotiable Summary

The Rust migration of `c4` shall:

- preserve per-target behavior rather than approximate repository-wide intent
- keep `c4` and `hello` target paths distinct
- preserve the staged `c4` processing structure around `next`, `expr`, and `stmt`
- keep `expr(int lev)` level-sensitive semantics explicit
- maintain Makefile-centered build usability
- use safe Rust by default
- treat uncertainty as something to test or document, not invent
- require proof for behavior, safety, and performance claims
- reject changes that compromise compatibility without evidence and approval