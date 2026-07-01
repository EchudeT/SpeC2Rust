# Constitution for the Rust Migration of `which`

## Purpose

This document defines the binding project-level principles for the Rust migration of the C project `which`. It is the governing standard for all later specifications, plans, task breakdowns, implementation work, testing, and review.

Where this constitution conflicts with convenience, speed, stylistic preference, or unchecked modernization, this constitution prevails.

---

## Scope

This constitution applies to the full migration surface described by the available project evidence:

- Build system: `Makefile.in`
- C files: 6
- Header files: 9
- Module units: 2
- Cluster units: 32

Covered module inventory:

- `main_root`
  - primary areas: command-line parsing, path scanning, file status evaluation, user/group/environment lookup
- `module_tilde`
  - primary areas: tilde scanning, home-directory lookup, fatal memory-error path

This constitution is intentionally evidence-based. It must not invent semantics not supported by the analyzed C project and its extracted behavior/interface summaries.

---

# 1. Core Principles

## 1.1 Behavioral Equivalence Principle

The Rust implementation must preserve the observable behavior of the C implementation unless a deliberate deviation is explicitly approved and documented.

### Required interpretation

Behavioral equivalence includes, at minimum:

- command-line parsing behavior
- token consumption order
- option handling mode distinctions
- argument permutation behavior indicated by `exchange`
- startup behavior exposed through the visible `main`
- absolute vs non-absolute path branching
- colon-delimited path scanning behavior
- candidate pathname construction and status checking flow
- user/home/environment lookup effects
- tilde prefix/suffix boundary handling
- fatal memory-failure behavior where evidenced
- output, exit status, and diagnostics to the extent they are observable from source and tests

### Evidence-backed obligations

The migration must preserve these C behaviors specifically evidenced by the analysis:

1. Parsing remains stateful across `argv`.
2. Long-option entry points remain semantically routed to the same core parser behavior as short-option flows.
3. Argument reordering semantics must not be lost if the C implementation performs permutation.
4. Absolute paths remain a distinct control-flow case from searched names.
5. PATH-like colon-separated scanning remains boundary-sensitive.
6. String slicing and tilde scanning remain boundary-sensitive.
7. User/group/home lookup remains behaviorally consistent with the C implementation.
8. At least one tilde-module allocation failure path is fatal and must not silently become recoverable.

### Prohibited behaviors

The migration must not:

- simplify away behavior that appears redundant but is externally visible
- replace parser behavior with "close enough" behavior from a convenience crate unless equivalence is demonstrated
- normalize path handling in ways that change delimiter, empty-segment, or boundary semantics
- change fatal error paths into warnings or recoverable errors without proof from source bodies that such change is correct
- remove argument permutation solely because Rust APIs make immutable argument handling more convenient

### Standard of proof

If a proposed Rust behavior differs from the C behavior, the burden of proof is on the proposer to show one of:

- the C source bodies support the deviation as equivalent
- the difference is unobservable
- the difference is required for safety and is explicitly documented and accepted
- the difference is backed by conformance tests that prove user-visible compatibility is preserved

Absent such proof, preserve the C behavior.

---

## 1.2 Interface Compatibility First Principle

Public and project-internal interfaces that matter to build, integration, and externally observable behavior must be preserved before internal redesign is considered.

### Required interpretation

The migration is not primarily an API redesign project. It is a compatibility-preserving rewrite.

Interface compatibility includes:

- command-line interface shape and semantics
- option names and option parsing behavior
- environment-variable interaction patterns
- pathname input/output expectations
- module boundaries that are needed for validation and review
- build integration behavior expected by `Makefile.in`
- any exposed symbols or integration assumptions needed during staged migration

### Interface preservation rules

1. Preserve CLI behavior first.
   - Flags, argument forms, and parsing semantics must remain compatible.
   - Long and short option behavior must remain aligned with the C implementation.

2. Preserve input interpretation before refactoring internals.
   - Absolute paths, path lists, and tilde-bearing strings must be interpreted the same way.

3. Preserve boundary-sensitive helper semantics.
   - Helpers corresponding to `substring`, `extract_colon_unit`, `get_next_path_element`, `tilde_find_prefix`, and `tilde_find_suffix` must keep equivalent contracts.

4. Preserve error-surface compatibility where observable.
   - Exit codes, fatality, and diagnostics must not drift casually.

5. Preserve build compatibility.
   - The Rust migration must fit the project’s existing build expectations or provide a documented compatibility layer during transition.

### Practical rule

When choosing between:
- a more idiomatic Rust API, and
- a more C-compatible behavior-preserving interface,

the project must choose the behavior-preserving interface unless and until compatibility has been proven through tests and review.

---

## 1.3 Safety First Principle

Rust’s safety guarantees are a project requirement, but safety improvements must be delivered without changing validated behavior except where necessary and documented.

### Required interpretation

The migration must eliminate classes of C risk where possible, including:

- buffer overflows
- use-after-free
- double free
- null dereference from unchecked pointer use
- integer misuse around indexing and slicing
- ownership confusion in dynamically allocated strings and buffers

### Safety rules

1. Prefer safe Rust by default.
   - `unsafe` is disallowed unless clearly justified.
   - Every `unsafe` block must document:
     - why it is necessary
     - the invariants required
     - why safe alternatives were insufficient

2. Make invalid states unrepresentable where behavior permits.
   - Use enums, newtypes, and structured data for parser modes, file status categories, and state-machine phases where that does not alter external behavior.

3. Preserve boundary behavior while enforcing memory safety.
   - Boundary-sensitive logic must be reimplemented safely, not simplified away.

4. Explicitly model fatal paths.
   - Where the C code aborts on certain memory failures, the Rust code must preserve fatality semantics if still behaviorally relevant.
   - Rust must not accidentally convert fatal behavior into silent recovery.

5. Avoid panics as a substitute for designed behavior.
   - Panics are not a compatibility strategy.
   - Expected user-input errors and normal failure modes must be handled through explicit program logic.
   - If panic/abort behavior is required to match the C program’s fatal semantics, that choice must be intentional and documented.

### Safety override rule

If exact C behavior would require unsound Rust, the project must:

1. preserve observable behavior as closely as possible,
2. choose the sound implementation,
3. document the difference,
4. add targeted tests proving the behavioral surface remains compatible.

Soundness is mandatory; undocumented unsoundness is forbidden.

---

## 1.4 Performance Constraint Principle

The Rust rewrite must not introduce material regressions in the performance-sensitive paths evidenced by the C analysis.

### Performance-sensitive areas

The following paths are project-designated performance-sensitive:

1. Command-line parsing
   - especially the `_getopt_internal` equivalent
   - including repeated token classification and argument permutation

2. Path search iteration
   - colon-delimited path scanning
   - candidate path construction
   - repeated `file_status`-like checks

3. Tilde scanning
   - repeated prefix/suffix boundary scans over strings

4. User/group initialization and repeated membership checks
   - especially if setup/check separation is part of the original design

### Performance obligations

1. Preserve algorithmic shape unless a replacement is proven equivalent and not worse.
2. Avoid unnecessary allocations in repeated scan loops.
3. Avoid path normalization or string cloning that changes complexity characteristics in hot paths.
4. Avoid replacing incremental scanning with repeated full rescans unless benchmarked and approved.
5. Avoid dependency choices that hide incompatible or slower behavior behind convenience.

### Acceptable performance change

A performance change is acceptable only if:

- it preserves behavior, and
- it is neutral or better under representative benchmarks, or
- a small regression is justified by a major safety or maintainability gain and explicitly approved

Unmeasured performance optimism does not satisfy this principle.

---

# 2. Migration Guidelines

## 2.1 C-to-Rust Mapping Rules

The migration must translate C concepts into Rust in a way that preserves semantics first and idioms second.

### General mapping rules

#### 2.1.1 Functions and modules

- Each C module family must map to Rust modules that preserve traceability.
- It must remain easy to identify where the Rust equivalent of each significant C function lives.
- For critical behavior clusters, maintain a near-1:1 mapping until compatibility is established.

Examples of behavior clusters that should remain easy to trace:

- getopt family and internal parser core
- path scanning helpers
- file status evaluation
- user/group/home lookup helpers
- tilde scanning helpers
- fatal memory-error path handling

#### 2.1.2 Data structures

- Anonymous C structs should be replaced with named Rust structs or enums reflecting the actual role they play.
- Introduce stronger typing where it clarifies semantics without changing behavior.
- Use `Option`, `Result`, enums, slices, and owned/borrowed string distinctions deliberately.

#### 2.1.3 Strings and path data

- Treat boundary semantics as first-class.
- C string operations that depend on positional ranges must be mapped carefully to Rust byte/character handling.
- Do not assume Unicode-oriented semantics if the C logic is byte-oriented.
- Where C behavior is byte-indexed, Rust should preserve byte-level interpretation unless evidence proves otherwise.

#### 2.1.4 Global and mutable state

- C global parser or runtime state may be modeled with explicit structs in Rust.
- Hidden state should become explicit where possible.
- However, visible semantics of state progression must remain unchanged.

#### 2.1.5 Error handling

- Convert raw C error signaling into Rust `Result`/enum forms internally where practical.
- At the external behavior boundary, preserve C-compatible fatality, diagnostics, and exit behavior.
- Internal elegance must not change observable behavior.

#### 2.1.6 Memory management

- Replace manual allocation with Rust ownership.
- Preserve ownership-sensitive behavior at boundaries.
- If the C implementation allocates returned strings for callers, the Rust design must preserve equivalent lifetime and mutability expectations at the API boundary used by the port.

### Preferred implementation pattern

Use a two-step migration posture:

1. semantic port
   - preserve structure and behavior closely
2. controlled idiomatization
   - only after equivalence is demonstrated with tests and benchmarks

---

## 2.2 Principles for Handling Uncertain Behavior

The supplied summaries explicitly note areas where behavior is not fully known. This uncertainty must be handled conservatively.

### Rule: do not invent semantics

Where exact behavior is not visible from the summaries or interface extracts:

- do not guess
- do not "improve" based on assumptions
- do not import common behavior from unrelated tools or libraries

### Required response to uncertainty

When behavior is uncertain, teams must do one or more of the following before finalizing implementation:

1. inspect the original C source bodies
2. derive behavior from targeted execution tests
3. compare with existing program output and exit codes
4. document the uncertainty in the migration notes
5. add compatibility tests that lock the discovered behavior

### Conservative fallback order

When uncertainty remains, decisions must follow this order:

1. source evidence from the C implementation
2. existing observable behavior from compiled C binary
3. existing tests or reproducible fixtures
4. least-assumptive implementation that preserves known behavior surface
5. explicit documentation of unresolved ambiguity

### Specific uncertainty areas requiring caution

The project must treat the following as high-risk ambiguity zones:

- exact startup sequencing among parser helpers
- exact parser state variables and completion criteria
- exact `exchange` trigger conditions
- exact `file_status` status categories
- path-search stop conditions
- empty or repeated `:` semantics in path lists
- substring invalid-range behavior
- precise tilde-eligibility rules
- user/group caching and initialization lifetime
- environment/home lookup precedence
- exact diagnostics and error propagation details

### Rule for ambiguity resolution in code review

Any pull request that resolves one of the above ambiguities must cite the evidence used:
- C source reference,
- observed C behavior,
- or explicit approved migration note.

No evidence, no merge.

---

## 2.3 Test Verification Requirements

Tests are the primary enforcement mechanism for this constitution.

### Required test philosophy

Testing must validate equivalence, not merely functionality.

That means tests must answer:
- does it work?
- does it work the same way as the C version?
- does it fail the same way where failure semantics matter?
- does it maintain the same boundaries and control-flow distinctions?

### Mandatory test categories

#### 2.3.1 Golden CLI compatibility tests

The Rust binary must be compared against the C binary for:

- valid short options
- valid long options
- long-only option handling where applicable
- malformed options
- mixed option/non-option argument cases
- argument order permutations where relevant
- representative exit codes
- representative diagnostics

#### 2.3.2 Path behavior tests

Must cover:

- absolute path inputs
- non-absolute program names
- path list iteration
- multi-element colon-separated paths
- empty path elements if supported by the C behavior
- repeated delimiters
- trailing delimiters
- candidate construction and selection behavior
- file accessibility/status distinctions as observable

#### 2.3.3 String-boundary tests

Must cover equivalents of:

- `substring`
- `extract_colon_unit`
- `get_next_path_element`
- `tilde_find_prefix`
- `tilde_find_suffix`

Tests must include:
- start boundary
- end boundary
- empty region
- adjacent delimiters
- invalid or edge index cases where discoverable from C behavior

#### 2.3.4 Tilde behavior tests

Must cover:

- strings with no tilde region
- strings with a tilde prefix
- strings containing tilde-like internal text
- boundary detection behavior
- home-directory substitution behavior as observable
- fatal memory-error behavior if practically testable through isolated abstraction or documented proof

#### 2.3.5 User/group/environment tests

Must cover as feasible:

- environment lookup presence/absence
- home directory lookup behavior
- group initialization and membership queries
- behavior under missing or limited user context, where reproducible

#### 2.3.6 Regression tests

Every bug found during migration must add a regression test before the fix is merged, unless technically impossible. If impossible, the pull request must document why.

### Comparative testing rule

Whenever practical, tests must run both:

- the original C implementation, and
- the Rust implementation,

against the same inputs and compare:
- stdout
- stderr
- exit status
- observable side effects

---

# 3. Quality Gates

## 3.1 Tests That Must Pass

No migration milestone is complete unless all applicable quality-gate tests pass.

### Required passing conditions

#### 3.1.1 Build and integration gates

- The Rust implementation must build reproducibly in the project environment.
- Integration with the existing `Makefile.in` workflow must succeed, or a documented temporary bridge must be in place and validated.
- All project-defined CI jobs must pass.

#### 3.1.2 Unit test gates

All unit tests for:
- parser behavior
- path scanning helpers
- string-boundary helpers
- tilde helpers
- environment/user/group helpers

must pass.

#### 3.1.3 Integration test gates

All integration tests covering end-to-end CLI behavior must pass.

#### 3.1.4 Compatibility test gates

All golden comparison tests against the C implementation must pass unless an approved exception exists.

Each approved exception must include:
- exact behavior difference
- reason for deviation
- safety or correctness justification
- stakeholder approval
- updated documentation

#### 3.1.5 Regression test gates

All historical regression tests must pass on every merge.

### Minimum merge bar

A change may merge only if:

- relevant tests exist,
- all relevant tests pass,
- no known compatibility regression is left undocumented,
- and no failing benchmark gate is waived without approval.

---

## 3.2 Code Review Standards

Code review is a compliance activity against this constitution, not just a style check.

### Required reviewer checks

Every review must verify:

1. **Behavior**
   - Does the change preserve C-observable behavior?
   - Is any deviation explicitly documented and justified?

2. **Traceability**
   - Can reviewers map the Rust code to the corresponding C behavior/function cluster?

3. **Safety**
   - Does the code avoid unnecessary `unsafe`?
   - Are ownership and lifetime rules clear?
   - Are panics avoided in normal error paths?

4. **Boundary correctness**
   - Are indexing, slicing, delimiter handling, and path semantics explicitly correct?

5. **Testing**
   - Are the right tests present?
   - Are edge cases and regressions covered?
   - Are C-vs-Rust comparisons used where appropriate?

6. **Performance**
   - Does the change add unnecessary allocation, cloning, rescanning, or abstraction overhead in hot paths?

7. **Build compatibility**
   - Does the change preserve or improve compatibility with the existing build/integration flow?

### Review rejection criteria

A change must be rejected if it:

- changes behavior without proof or approval
- replaces evidence-backed logic with assumptions
- introduces unexplained `unsafe`
- removes compatibility tests
- weakens fatal behavior where the C code is evidenced as fatal
- obscures traceability to the C implementation too early
- introduces performance regressions without measurement
- uses third-party crates to replace core behavior without equivalence proof

### Documentation requirements in pull requests

Every substantial PR must include:

- affected behavior area
- C source/function mapping
- behavior preserved or changed
- uncertainty encountered
- tests added or updated
- benchmark impact for hot-path changes
- any remaining risks

---

## 3.3 Performance Benchmark Requirements

Benchmarks are mandatory for performance-sensitive areas.

### Benchmark targets

The project must maintain representative benchmarks for:

1. command-line parsing
   - varied argument counts
   - mixed options and operands
   - cases that trigger argument permutation if applicable

2. path search
   - short and long PATH-like inputs
   - early success and late success cases
   - all-failure cases
   - repeated delimiter and empty-segment cases if behaviorally relevant

3. tilde scanning
   - strings with and without tilde regions
   - repeated scans over many inputs
   - short and long strings

4. repeated lookup/helper scenarios
   - initialization followed by repeated membership or lookup checks where applicable

### Benchmark rules

1. Any change touching a hot path must run relevant benchmarks.
2. Any statistically meaningful regression must be explained before merge.
3. Any major refactor must compare before/after results.
4. Allocation-sensitive paths should include allocation profiling where practical.
5. Microbenchmarks are not sufficient alone; representative end-to-end scenarios are also required.

### Performance acceptance standard

A change passes the performance gate if:

- it shows no material regression in relevant benchmarks, or
- a regression is minor, measured, justified, and explicitly approved

### Forbidden performance practices

The project must not:

- accept hot-path regressions based only on intuition
- trade repeated scanning loops for repeated heap allocation without evidence
- use abstraction layers that hide repeated conversions or cloning in core loops
- declare performance "good enough" without measurement on affected paths

---

# Amendment and Precedence Rules

## Amendment rule

This constitution may be amended only through an explicit project decision. Amendments must be written, reviewable, and justified against actual migration needs.

## Precedence rule

If any later document conflicts with this constitution, this constitution wins.

The order of precedence is:

1. `constitution.md`
2. approved migration decisions and amendments
3. project plans
4. task documents
5. implementation preferences

---

# Non-Negotiable Summary

The Rust migration of `which` must:

- preserve observable C behavior
- preserve compatibility before redesign
- improve safety without losing correctness
- respect measured performance constraints
- treat uncertainty conservatively
- prove equivalence with tests
- block merges that violate these rules