# constitution.md

## Purpose

This document is the governing constitution for the Rust migration of the `pwd` C project. It defines the non-negotiable project-level principles, migration rules, and quality gates that all later specifications, plans, task breakdowns, implementations, and reviews must follow.

Where this constitution conflicts with convenience, schedule, or stylistic preference, this constitution wins.

---

## Scope

This constitution applies to the full migration surface of the project, including:

- the `pwd` executable behavior
- command startup and dispatch
- logical and robust current-directory acquisition flows
- pathname reconstruction helpers
- quoting and quote-option subsystems
- localization and charset-sensitive behavior
- version/help/diagnostic output flows
- stream flushing and closing behavior
- allocator and fatal-allocation conventions
- Makefile-based build and verification integration
- all migrated modules, support utilities, tests, and benchmarks

It governs all 29 module units and all cross-module interactions, not only the main user-visible command path.

---

## Article I: Core Principles

### 1. Behavioral Equivalence Principle

The Rust rewrite must preserve the externally observable behavior of the C project unless a deviation is explicitly approved and documented.

#### 1.1 What must be preserved

Behavioral equivalence includes, at minimum:

- startup sequencing relevant to behavior
- command dispatch behavior
- success and failure paths
- mode distinctions such as logical versus robust/physical directory retrieval
- root detection and parent traversal behavior
- path assembly semantics
- user-visible output shape and ordering
- help/version branching behavior
- quoting behavior and option mutation effects
- locale-sensitive behavior
- delayed output failure detection during flush/close
- fatal allocation policy
- important edge-case handling and boundary conditions

#### 1.2 Observable behavior takes priority over implementation similarity

The Rust code does **not** need to mimic C syntax, memory layout, or internal helper structure unless those are required to preserve behavior. However, where internal statefulness affects later outputs, that state model must be preserved.

Examples from this project include:

- mutable quoting options and their effect on subsequent quoting calls
- slot-based quoting behavior in `quotearg_n*`
- `close_stdout` behavior depending on prior setter calls
- the distinction between direct logical cwd retrieval and robust reconstruction

#### 1.3 No silent semantic simplification

The migration must not collapse distinct C behaviors into a simpler Rust implementation if the simplification can change results in normal, edge, locale-sensitive, or failure cases.

Prohibited examples include:

- replacing logical and robust cwd paths with one generic path lookup
- reducing the quoting subsystem to a single escape routine
- ignoring stream-finalization errors because writing appeared to succeed
- replacing fatal allocator semantics with silent propagation of allocation errors in code paths that were fail-fast in C

#### 1.4 Unknown behavior must remain unknown until proven

If the available analysis does not establish exact C behavior, the Rust migration must not invent certainty. Such areas must be treated as unresolved and preserved conservatively until confirmed by source inspection, test evidence, or differential execution.

---

### 2. Interface Compatibility First Principle

Compatibility with the C project's effective interfaces comes before idiomatic restructuring.

#### 2.1 Public and cross-module interfaces are migration anchors

All significant C interfaces must be mapped deliberately before refactoring. This includes:

- executable entry behavior driven by `main`
- helper interfaces used across module boundaries
- quoting APIs and option-object APIs
- version/help/diagnostic helpers
- allocator wrappers
- stream-close helpers
- locale/charset helpers
- path-construction helpers

#### 2.2 Preserve semantic contracts, not just names

Even when Rust types or module boundaries differ, the migration must preserve:

- parameter meaning
- return-value meaning
- lifetime and ownership expectations visible to callers
- mutation semantics
- side effects
- cleanup expectations
- failure policy

If a direct one-to-one signature mapping is not possible, the Rust abstraction must still preserve the original contract at the call boundary.

#### 2.3 Wrapper compatibility before consolidation

When multiple C wrappers expose distinct behavior over a shared engine, the Rust version must first preserve those wrappers and prove equivalence before consolidating internals.

This is especially mandatory for:

- `quotearg*` wrapper families
- style/custom/colon/char quote variants
- allocation-based versus buffer-based quote paths
- stdout closing and flushing helpers

#### 2.4 Entry-point behavior is normative

The executable's effective contract is defined by runtime behavior, not by the elegance of a Rust API. `main`-level sequencing and branching are therefore normative and must be preserved.

---

### 3. Safety First Principle

Rust safety improvements are mandatory, but they must be introduced without altering required behavior.

#### 3.1 Memory safety is required

The migration must eliminate C-origin memory unsafety by default through Rust ownership, borrowing, bounds checking, and safe abstractions where possible.

#### 3.2 Unsafe Rust is allowed only by exception

`unsafe` is permitted only when all of the following are true:

- there is a clear behavioral or interoperability need
- a safe alternative is not practical
- the unsafe region is minimized
- the invariants are documented in comments adjacent to the block
- tests exercise the relevant behavior
- review explicitly approves the unsafe usage

#### 3.3 Safety must not erase important failure semantics

Replacing unsafe C patterns with safe Rust must not accidentally alter:

- fail-fast allocation behavior
- late detection of I/O errors
- stateful quoting semantics
- locale-sensitive branching
- behavior under embedded NUL or explicit-length buffers
- error paths that are part of the observable contract

#### 3.4 Panic policy

The migrated program must not rely on uncontrolled panics for ordinary error handling. Fatal behavior must be explicit and aligned with the original command semantics.

Allowed panic scenarios are limited to irrecoverable internal invariant violations that indicate a bug in the Rust implementation rather than a user-facing operational error.

#### 3.5 Security and robustness

The Rust implementation must be robust under malformed inputs, unusual filesystem states, locale variation, and output stream failures. Safer implementation must strengthen resistance to undefined behavior, but not at the cost of changing intended outcomes.

---

### 4. Performance Constraint Principle

The Rust migration must remain within acceptable performance bounds relative to the C project, especially on behaviorally critical paths.

#### 4.1 Performance is a constraint, not a luxury

The migration must avoid regressions that materially worsen:

- cwd reconstruction loops
- directory traversal and parent ascent
- path-component prepending or equivalent assembly
- central quoting transformations
- repeated allocation/growth behavior
- stream finalization overhead
- repeated locale/charset lookups on hot paths

#### 4.2 Correctness first, then bounded cost

Behavioral equivalence has priority over micro-optimization. However, once correctness is established, the implementation must avoid avoidable overhead, duplicate scans, unnecessary allocation churn, and repeated work not present in the original design intent.

#### 4.3 Preserve algorithmic shape where behavior depends on it

If the C behavior implies a specific algorithmic model that affects cost and edge handling, the Rust version should preserve that model unless a replacement is proven equivalent in both behavior and cost profile.

This applies especially to:

- robust path reconstruction
- quote transformation reuse through a central engine
- growth-oriented allocation helpers and buffer management

#### 4.4 No hidden performance cliffs from abstraction

Rust abstractions are acceptable only if they do not introduce pathological copying, repeated UTF-8 assumptions on byte-oriented paths, or unnecessary heap traffic on critical flows.

---

## Article II: Migration Guidelines

### 5. C-to-Rust Mapping Rules

#### 5.1 Map by semantic role

Each C module and function must be mapped according to what it does, not merely what it is named.

Recommended mapping categories:

- executable control flow
- path acquisition and reconstruction
- mutable path accumulator state
- quote engine and quote-option state
- locale and charset helpers
- allocator/fatal-policy helpers
- stream finalization wrappers
- diagnostics/help/version formatting

#### 5.2 Preserve byte-oriented behavior where required

Any C API operating on raw bytes, explicit lengths, embedded NULs, or non-text data must remain byte-correct in Rust. Do not force such paths into `String` or UTF-8-only representations unless equivalence is proven.

This is mandatory for the quoting subsystem and any buffer-oriented helpers.

#### 5.3 Represent mutable state explicitly

Where the C code uses mutable structs or global/shared state that affects later outputs, the Rust version must represent that state explicitly and reviewably.

Examples include:

- quoting options objects
- slot-managed quote state
- `close_stdout` configuration state
- path accumulator state during reconstruction

#### 5.4 Preserve lifecycle semantics

C create/mutate/use/free lifecycles must be mapped into Rust ownership and drop semantics without losing behavior.

Typical mapping expectations:

- `init/free` pairs become owned Rust types with deterministic cleanup
- temporary buffers become scoped owned values
- global cleanup functions remain present when required by behavior, even if Rust could otherwise rely on drop

If explicit cleanup is observable or contractually meaningful, it must remain explicit.

#### 5.5 Error mapping must preserve policy

C APIs that encode:
- fatal-on-failure behavior,
- nullable returns,
- size-return conventions,
- errno-sensitive behavior,
- delayed stream failure,

must be mapped deliberately. Do not normalize all errors into a single Rust style if that erases semantics.

#### 5.6 FFI is not the default migration target

The default goal is a native Rust implementation with preserved behavior, not a thin Rust wrapper over the C implementation. FFI may be used temporarily for verification or staging, but not as a substitute for migration completion unless explicitly approved.

#### 5.7 Build-system compatibility

The project uses a Makefile. Rust integration must support Make-driven builds, tests, and benchmarks. New tooling may be added only if it does not weaken reproducibility or complicate standard project entry points.

---

### 6. Principles for Handling Uncertain Behavior

#### 6.1 Do not guess

When behavior is uncertain, contributors must not guess based on preference or generic expectations. They must instead mark the uncertainty and resolve it through evidence.

#### 6.2 Acceptable evidence sources

Uncertain behavior may be resolved only through one or more of:

- direct inspection of the C source
- existing project documentation
- differential execution against the C binary
- targeted characterization tests
- platform-aware experiments where relevant
- review consensus grounded in evidence

#### 6.3 Conservative default

Until resolved, ambiguous areas must be implemented conservatively in a way that minimizes the risk of observable divergence.

#### 6.4 Required uncertainty log

Every unresolved semantic question affecting implementation must be tracked in project documentation with:

- the question
- impacted modules/functions
- current assumption, if any
- evidence status
- verification plan
- final resolution when known

#### 6.5 Areas already known to require caution

The following must be treated as high-risk until confirmed:

- exact command-line option precedence in `main`
- fallback relationships between logical and robust cwd retrieval
- exact syscalls and traversal behavior in robust reconstruction
- exact diagnostics and exit status details
- precise quoting escape rules and return semantics
- allocator growth formulas
- stream-close behavior by errno case
- locale-specific quote and text selection details

---

### 7. Test Verification Requirements

#### 7.1 Differential verification is mandatory

The Rust implementation must be verified against the C implementation wherever behavior can be observed. For each migrated feature, tests should compare Rust results to C results under the same conditions.

#### 7.2 Tests must follow behavior, not implementation

Tests must assert externally meaningful outcomes:

- stdout
- stderr
- exit status
- path results
- quoting output
- locale-sensitive formatting
- cleanup/finalization error behavior where observable

Tests must not be satisfied merely because the Rust internals look elegant or because types appear safe.

#### 7.3 Layered testing is required

The project must maintain three levels of tests:

1. **module tests**
   - focused behavior of individual helpers
2. **integration tests**
   - cross-module flows such as startup, dispatch, path retrieval, and output
3. **differential/end-to-end tests**
   - compare Rust executable behavior with the C executable

#### 7.4 Edge-case coverage is mandatory

Tests must include boundary and failure conditions relevant to this project, including where feasible:

- root directory cases
- deep parent traversal
- minimal and multi-component path assembly
- logical versus robust mode distinctions
- quoting with explicit sizes and embedded NULs
- custom quote delimiters
- locale-sensitive variations
- final flush/close failure cases
- allocation-failure policy for wrappers, if safely testable

#### 7.5 Regression tests are required for every discovered mismatch

Any bug, divergence, or uncertainty resolved during migration must produce a regression test unless impossible for documented reasons.

---

## Article III: Quality Gates

### 8. Tests That Must Pass

No change is considered complete unless all applicable gates pass.

#### 8.1 Build gate

The project must:

- build successfully through the project-sanctioned Makefile entry points
- produce the intended Rust target artifacts
- complete without unresolved warnings elevated by project policy

#### 8.2 Static quality gate

The project must pass all required static checks, including as applicable:

- formatting
- linting
- compiler warnings policy
- unsafe usage review requirements
- dead-code or unused-interface review where relevant to migration staging

#### 8.3 Unit and module behavior gate

All module-level tests for migrated code must pass, especially for:

- quoting behavior families
- path assembly helpers
- root and parent traversal helpers
- locale/charset helpers
- stream and close helpers
- allocator behavior wrappers

#### 8.4 Integration gate

All integration tests covering executable flows must pass, including:

- startup and initialization sequence assumptions
- help/version branches
- normal path printing
- logical-path flow
- robust reconstruction flow
- output finalization behavior
- error-path handling

#### 8.5 Differential gate

Where a C reference binary is available, differential tests against the C implementation must pass for all migrated behavior in scope. Any approved exceptions must be documented explicitly and linked to a decision record.

#### 8.6 Platform gate

If behavior varies by platform or libc characteristics, tests must pass on all supported target environments or document approved, evidence-based differences.

---

### 9. Code Review Standards

#### 9.1 Constitution compliance review

Every non-trivial change must be reviewed for compliance with this constitution, not only for syntax or style.

#### 9.2 Required review questions

Reviewers must explicitly check:

- Does this preserve observable C behavior?
- Does this preserve interface semantics?
- Does this introduce any silent simplification?
- Does this handle uncertain behavior with evidence rather than guesswork?
- Does this preserve byte-oriented behavior where required?
- Does this alter error, flush, or allocation policy?
- Does this create performance risk on critical paths?
- Is any `unsafe` justified, minimized, and documented?
- Are tests sufficient and behavior-focused?

#### 9.3 Evidence requirement for semantic changes

Any intentional deviation from the C behavior requires:

- explicit documentation
- rationale
- impact analysis
- approval by project maintainers
- updated tests reflecting the approved new contract

Unexplained semantic changes must be rejected.

#### 9.4 Review standard for unsafe code

Any `unsafe` code requires heightened review, including:

- invariant documentation
- proof that safe alternatives were considered
- tests targeting the unsafe boundary
- confirmation that the block is as small as possible

#### 9.5 Review standard for refactoring

Pure refactors are allowed only when they do not weaken behavioral evidence or test coverage. If a refactor changes semantics and cleanup behavior cannot be ruled out, it must be treated as a behavioral change.

---

### 10. Performance Benchmark Requirements

#### 10.1 Benchmark coverage

Benchmarks must cover, at minimum, representative cases for:

- normal pwd execution
- deep directory traversal / robust reconstruction scenarios
- repeated quoting transformations
- allocation/growth-heavy helper behavior where applicable
- output finalization overhead if measurable

#### 10.2 Baseline comparison

Rust performance must be compared against:

- the C implementation when available, and/or
- the previously accepted Rust baseline

#### 10.3 Regression thresholds

Performance regressions on critical flows must be investigated before merge. Any material slowdown must be justified by correctness or safety needs and explicitly approved.

As a default rule, contributors must treat the following as requiring explanation and approval:

- noticeable algorithmic regression
- extra full scans of data on hot paths
- repeated avoidable allocations
- path reconstruction slowdowns on deep directory trees
- quote engine slowdowns from duplicated transformation work

#### 10.4 Benchmark discipline

Benchmarks must be repeatable, documented, and tied to representative workloads. One-off anecdotal runs are not sufficient evidence for accepting performance-sensitive changes.

---

## Article IV: Enforcement and Decision Rules

### 11. Hierarchy of Authority

When interpreting migration work, apply this precedence:

1. this constitution
2. verified C behavior
3. approved project decisions and exceptions
4. later migration specs and plans
5. contributor preference

Later documents may refine this constitution, but they may not violate it.

---

### 12. Definition of Done

A migrated component is done only when:

- behavior is implemented
- interfaces are preserved semantically
- uncertainties are resolved or explicitly tracked
- tests pass at all required levels
- code review standards are satisfied
- performance requirements are met
- documentation is updated where behavior or assumptions matter

Code that compiles but lacks behavioral proof is not done.

---

### 13. Approved Deviation Process

If strict equivalence is impossible or undesirable, the team may approve a deviation only when all of the following are provided:

- precise description of the original behavior
- precise description of the new behavior
- reason equivalence is infeasible or harmful
- user impact assessment
- compatibility impact assessment
- testing updates
- maintainer approval

Undocumented deviation is a defect.

---

### 14. Non-Negotiable Project Rules

The following rules are absolute for this migration:

- No guessing about uncertain C semantics.
- No silent collapse of distinct behavioral paths.
- No byte-to-text coercion where byte semantics matter.
- No ignoring final stream errors.
- No weakening of fatal allocation policy without approval.
- No unjustified `unsafe`.
- No merge without behavior-focused tests.
- No merge of performance-sensitive changes without benchmark evidence when applicable.

---

## Article V: Project-Specific Directives for `pwd`

### 15. Directives for the `pwd` Migration

The following directives are especially binding for this project:

#### 15.1 Preserve dual cwd acquisition behavior

The distinction between logical retrieval and robust reconstruction is fundamental and must remain visible in design, tests, and implementation.

#### 15.2 Preserve robust reconstruction semantics

The Rust version must respect the behavioral model of:

- mutable path accumulator creation
- iterative parent ascent
- directory entry discovery within parents
- front-assembly of path components or a proven-equivalent mechanism
- root detection via identity semantics, not naive string logic

#### 15.3 Preserve quoting subsystem richness

The quoting subsystem must not be reduced to a simplistic string-escape helper. Option mutation, wrapper equivalence, slot behavior, custom delimiters, and locale-sensitive quote selection must all be treated as real contract surface.

#### 15.4 Preserve startup and shutdown behavior

Program-name setup, locale-related setup, command dispatch, output emission, and close/flush finalization are all part of the executable contract.

#### 15.5 Preserve failure timing where visible

Errors discovered only on flush or close remain real failures. Allocation wrappers that were fail-fast remain fail-fast unless an explicit deviation is approved.

---

## Ratification

This constitution is effective immediately for all migration work on the Rust rewrite of `pwd`. All future spec, plan, task, implementation, review, and benchmark artifacts must comply with it.