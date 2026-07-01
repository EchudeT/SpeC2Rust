# constitution.md

## Purpose

This document defines the binding project-level principles for the Rust migration of the `which` C project. It is the governing standard for all later specifications, plans, task breakdowns, implementations, reviews, and acceptance decisions.

Where this constitution conflicts with convenience, schedule, or stylistic preference, this constitution prevails.

---

## Scope

This constitution applies to all migration work across the identified project structure:

- Build system: `Makefile.in`
- C source files: 6
- Header files: 9
- Module units: 2
- Cluster units: 32

It applies especially to the two currently identified module areas:

- `main_root`
- `module_tilde`

It is written from the available interface and behavior summaries. Where the original C behavior is not fully evidenced, the project must preserve known behavior, avoid speculative redesign, and explicitly document uncertainty.

---

## 1. Core Principles

### 1.1 Behavioral Equivalence Principle

The Rust implementation must preserve the observable behavior of the C program unless a change is explicitly approved as a documented compatibility correction.

#### Requirements

- Program startup behavior must remain driven by a Rust `main` that is behaviorally equivalent to the C `main`.
- Command-line parsing behavior must preserve the existing public parsing modes and their distinctions.
- Path search behavior must preserve the stepwise semantics present in the C structure:
  1. classify direct path vs searchable name,
  2. iterate path units,
  3. construct candidate paths,
  4. evaluate candidate file status,
  5. continue or terminate consistently.
- User/group preparation and membership-related behavior must preserve setup/query ordering assumptions present in the C organization.
- Tilde-related behavior must preserve:
  - boundary detection,
  - home-directory lookup dependence,
  - fatal memory-failure semantics where the C implementation uses an abort path.
- Error outcomes, exit status behavior, stdout/stderr intent, and processing order must remain compatible wherever they are part of observable behavior.

#### Rules

- The project must prefer semantic equivalence over structural resemblance.
- Refactoring is allowed only if externally visible behavior remains compatible.
- Any proposed deviation must be:
  - explicitly identified,
  - justified,
  - tested,
  - approved before merge.

#### Prohibitions

- Do not simplify away stateful behavior merely because Rust enables cleaner APIs.
- Do not collapse distinct runtime phases if that could alter observable decisions.
- Do not infer behavior not supported by evidence and then implement that inference as fact.

---

### 1.2 Interface Compatibility First Principle

Public and project-internal interfaces that define behaviorally meaningful boundaries in the C program must be preserved or replaced with compatibility-preserving Rust interfaces.

#### Requirements

- The Rust project must maintain clear equivalents for the identified module boundaries:
  - `main_root`
  - `module_tilde`
- Parsing interface layering must remain explicit:
  - public parser entry behavior,
  - shared internal parsing engine behavior,
  - argument exchange/reordering behavior where applicable.
- Functions or routines that represent major behavioral gates in the C codebase must have traceable Rust equivalents, especially for:
  - direct-path classification,
  - path element extraction,
  - full pathname construction,
  - file-status evaluation,
  - home-directory lookup,
  - tilde boundary scanning,
  - fatal memory-error handling.
- If a C interface cannot be mirrored literally, the Rust replacement must preserve:
  - caller-visible inputs,
  - caller-visible outputs,
  - side-effect ordering,
  - failure semantics.

#### Rules

- Compatibility takes priority over idiomatic redesign at the boundary.
- Internals may become more idiomatic Rust only after compatibility is protected at the edges.
- Module reshaping is allowed only when traceability from C responsibilities to Rust responsibilities remains clear.

#### Prohibitions

- Do not remove a compatibility-relevant interface distinction because it appears redundant.
- Do not merge separate responsibilities if the separation reflects behaviorally important phases.
- Do not convert fatal behavior into recoverable behavior, or the reverse, without explicit approval.

---

### 1.3 Safety First Principle

The Rust migration must improve memory safety and state safety without changing required behavior.

#### Requirements

- Rust ownership, borrowing, and type systems must be used to eliminate undefined behavior risks where possible.
- Unsafe Rust is prohibited by default.
- Any use of `unsafe` must be:
  - unavoidable,
  - minimal in scope,
  - documented with a safety contract,
  - reviewed with heightened scrutiny,
  - covered by tests that exercise the boundary.
- Fallible operations must be expressed clearly using Rust result types or explicitly documented abort behavior, depending on the original semantics.
- String, path, and indexing logic must be implemented to prevent out-of-bounds access, invalid pointer arithmetic, and accidental aliasing errors.
- Initialization-sensitive logic must be encoded so that invalid state transitions are harder to represent than valid ones.

#### Rules

- Prefer total functions, validated inputs, and typed state over unchecked assumptions.
- Prefer enums over integer state codes internally when doing so does not change external behavior.
- Preserve externally observable return behavior even if the internal representation becomes safer and more expressive.

#### Prohibitions

- Do not reintroduce C-style memory hazards through unnecessary raw-pointer emulation.
- Do not use `unsafe` to mimic C layout or pointer flow unless strictly required.
- Do not expose partially initialized internal state across module boundaries.

---

### 1.4 Performance Constraint Principle

The Rust migration must not introduce unjustified regressions in runtime behavior, startup cost, or search-loop efficiency.

#### Requirements

- The migration must respect the known performance-sensitive areas:
  - core option parsing,
  - argument exchange/reordering,
  - path list scanning,
  - candidate pathname generation,
  - file-status checks,
  - repeated string boundary processing,
  - user/group setup and repeated membership checks.
- The Rust implementation must avoid extra passes, unnecessary allocations, or avoidable copies in hot paths.
- Incremental and streaming-style behavior must be preserved where the C design indicates iterative processing.
- Performance claims must be validated with measurements, not intuition.

#### Rules

- Preserve asymptotic behavior unless a change is proven neutral or beneficial.
- Prefer allocation-aware designs for repeated path and string operations.
- Cache or reuse only when doing so does not alter semantics.

#### Prohibitions

- Do not replace stepwise search logic with more expensive whole-structure preprocessing unless benchmarked and approved.
- Do not introduce abstraction layers in hot paths without justification.
- Do not accept measurable regressions without documented rationale and approval.

---

## 2. Migration Guidelines

### 2.1 C-to-Rust Mapping Rules

These rules govern how C constructs should be translated into Rust while preserving behavior.

#### 2.1.1 Functions and Modules

- Each C module responsibility must map to a clear Rust module or submodule.
- Large C files may be split into multiple Rust files if traceability is maintained.
- A migration mapping document must identify where each significant C function responsibility lives in Rust.
- Behaviorally central C routines should retain recognizable names or explicit mapping comments during migration.

#### 2.1.2 Data Representation

- C integer return codes may be represented internally as Rust enums or structured results, but compatibility-relevant output must be preserved.
- C structs should map to Rust structs with explicit invariants.
- Anonymous or implicit C data groupings should become named Rust types where that improves safety and reviewability.
- Global mutable state should be minimized. If the C design implies shared state, Rust must encapsulate it behind controlled APIs.

#### 2.1.3 Strings and Paths

- C string slicing and delimiter scanning behavior must be migrated carefully, especially for:
  - colon-delimited path parsing,
  - substring extraction,
  - tilde boundary detection,
  - home-directory substitution support.
- Rust string handling must preserve byte-level semantics where user-visible behavior depends on exact boundaries.
- Path construction logic must preserve ordering and candidate generation semantics.

#### 2.1.4 Error Handling

- Recoverable C-style return paths should generally map to `Result`, `Option`, or explicit status enums internally.
- Fatal C paths must remain fatal where the original behavior requires termination.
- Error translation must not hide compatibility-relevant distinctions.
- Internally richer error types are encouraged only if they do not alter externally visible behavior.

#### 2.1.5 State Machines

- C routines that imply stateful subsystems must be modeled explicitly in Rust.
- The option parser must retain a stateful design equivalent to the C parser layering and transitions.
- Search iteration logic must preserve its staged state progression.
- Setup-before-query flows must be represented so misuse is structurally discouraged.

#### 2.1.6 Memory and Allocation

- Replace manual allocation with owned Rust types wherever possible.
- Allocation failure behavior must be handled in a way consistent with project policy and original semantics.
- If the C code has an explicit abort-on-memory-error path in tilde logic, the Rust implementation must preserve that behavior for the equivalent path unless a deliberate compatibility decision says otherwise.

---

### 2.2 Principles for Handling Uncertain Behavior

The available summaries do not fully specify all runtime details. Therefore uncertainty must be handled conservatively.

#### Rules for Uncertainty

- Known behavior is binding.
- Unknown behavior is not permission to redesign.
- When behavior is insufficiently evidenced, the default project stance is:
  1. preserve structure suggested by the C interfaces,
  2. avoid irreversible abstraction,
  3. document the uncertainty,
  4. add validation work before finalizing behavior.

#### Required Actions When Behavior Is Unclear

- Mark the item as an open behavioral question.
- Identify the exact missing evidence.
- Prefer a temporary compatibility-oriented implementation over a speculative cleanup.
- Seek clarification through:
  - direct source inspection,
  - focused test extraction,
  - differential execution against the C version,
  - review of related headers and call sites.
- Do not merge behaviorally significant guesses as if they were established facts.

#### Decision Standard

If two Rust designs are both plausible and the evidence is incomplete, choose the one that is:

1. less behaviorally committal,
2. more easily validated against the C program,
3. more reversible if later evidence contradicts it.

---

### 2.3 Test Verification Requirements

Testing is mandatory and is part of the migration itself, not a follow-up activity.

#### Required Test Categories

##### A. Behavioral Equivalence Tests

The Rust implementation must be checked against the C implementation for:

- command-line parsing behavior,
- option mode differences,
- path lookup behavior,
- direct-path handling,
- search-list iteration behavior,
- file-status decision behavior where observable,
- tilde-related boundary behavior,
- fatal vs non-fatal failure behavior,
- exit code behavior.

##### B. Regression Tests

Every bug discovered during migration must add a regression test before or with the fix.

##### C. Edge and Boundary Tests

Tests must cover boundary-sensitive areas including:

- empty input cases,
- direct vs indirect program paths,
- colon-separated path edge shapes,
- repeated delimiters,
- leading/trailing delimiters,
- substring boundary conditions,
- tilde presence and absence,
- user/group setup/query ordering assumptions,
- parser mode transitions.

##### D. Negative Tests

Tests must validate invalid or failure-oriented paths where behavior is observable, including parser and file-evaluation failures when supported by evidence.

##### E. Cross-Implementation Differential Tests

Where practical, the same test inputs must run against both:
- the original C executable,
- the Rust executable,

and compare:
- stdout,
- stderr,
- exit status,
- relevant ordering-sensitive outputs.

#### Required Test Discipline

- No migration task is complete without tests proportional to behavioral risk.
- Untested compatibility claims are non-compliant.
- If exact behavior cannot yet be verified, the gap must be documented and tracked explicitly.

---

## 3. Quality Gates

No change may be accepted unless all applicable quality gates pass.

### 3.1 Tests That Must Pass

#### Minimum Required Passing Set

1. **Build Verification**
   - The Rust project must build successfully in the project-supported configuration.
   - The build integration path must remain compatible with the project’s `Makefile.in` driven environment or an approved equivalent integration layer.

2. **Unit Tests**
   - All unit tests for migrated modules must pass.

3. **Integration Tests**
   - All integration tests covering CLI behavior and module interaction must pass.

4. **Behavioral Equivalence Tests**
   - Differential or fixture-based compatibility tests against the C behavior must pass for all migrated functionality.

5. **Regression Tests**
   - All historical regression tests must pass.

6. **Platform-Relevant Tests**
   - Tests for filesystem, environment lookup, and user/group related behavior must pass on supported target environments as applicable.

#### Blocking Conditions

A change must not merge if it:
- changes observable behavior without approval,
- lacks tests for new compatibility-sensitive logic,
- disables failing tests instead of resolving the issue,
- leaves unexplained divergences from C behavior.

---

### 3.2 Code Review Standards

Every merged change must pass review for correctness, safety, compatibility, and maintainability.

#### Review Checklist

Reviewers must verify:

- the change preserves known C behavior,
- compatibility-relevant interfaces are respected,
- uncertainty is documented rather than hidden,
- safety improvements do not alter required semantics,
- `unsafe` usage is absent or fully justified,
- tests adequately cover the changed behavior,
- hot-path changes consider performance impact,
- module mapping remains traceable back to C responsibilities.

#### Required Review Standards

- Behaviorally significant changes require explicit reviewer attention to equivalence.
- Any deviation from the C structure in a compatibility-sensitive area must include rationale.
- Any `unsafe` block requires a dedicated safety review comment.
- Any performance-sensitive refactor requires benchmark evidence or a clear argument for non-impact.

#### Rejection Criteria

A review must reject code that:

- guesses at behavior without documentation,
- conflates distinct parser or search phases,
- changes failure semantics silently,
- introduces unnecessary allocation or copying in hot paths,
- obscures the mapping from C responsibilities to Rust code,
- lacks sufficient tests for edge conditions.

---

### 3.3 Performance Benchmark Requirements

Performance is a release gate for compatibility-sensitive paths.

#### Required Benchmark Areas

Benchmarks must cover, as applicable:

- option parsing throughput and overhead,
- argument reordering/exchange behavior cost,
- path search over multiple path elements,
- candidate pathname construction,
- repeated file-status checks,
- tilde scanning and related string boundary operations,
- repeated or cached user/group related queries where relevant.

#### Benchmark Standards

- Baselines must be recorded against the C implementation where practical.
- Rust benchmarks must be repeatable and documented.
- Performance must be evaluated using representative workloads, not toy-only inputs.
- If a benchmark cannot yet mirror the C implementation directly, that limitation must be documented.

#### Acceptance Standard

- No significant regression may be accepted in a known hot path without explicit approval.
- Any accepted regression must include:
  - measured data,
  - rationale,
  - proof that compatibility or safety goals required the tradeoff,
  - follow-up work if optimization is deferred.

---

## 4. Governance Rules

### 4.1 Hierarchy

This constitution governs all subordinate project documents, including:

- specs,
- migration plans,
- implementation tasks,
- testing plans,
- review checklists.

If a subordinate document conflicts with this constitution, the subordinate document must be corrected.

### 4.2 Amendment Rule

This constitution may be changed only when the project learns something materially new about:
- actual C behavior,
- supported compatibility scope,
- required safety policy,
- justified performance constraints.

Any amendment must be explicit and must not be smuggled in through implementation.

### 4.3 Documentation Rule

All major migration decisions must be documented when they affect:
- behavior,
- interface shape,
- safety model,
- performance characteristics,
- unresolved uncertainty.

Undocumented decisions are not considered stable project law.

---

## 5. Non-Negotiable Project Mandates

The following mandates are absolute unless this constitution is formally amended:

1. Preserve observable behavior first.
2. Preserve compatibility-relevant interfaces before pursuing idiomatic redesign.
3. Use Rust to improve safety, not to justify semantic drift.
4. Treat uncertain behavior conservatively.
5. Require tests for all compatibility-sensitive work.
6. Reject silent performance regressions in known hot paths.
7. Keep module and responsibility traceability from C to Rust.
8. Make all fatal-vs-recoverable behavior decisions explicit.
9. Prefer reversible implementation choices when evidence is incomplete.
10. Do not declare migration complete until behavior, tests, review, and performance gates all pass.