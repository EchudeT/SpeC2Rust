# Constitution for the Rust Migration of `cflow-new`

## Purpose and Scope

This document defines the binding project-level principles for migrating the C project `cflow-new` to Rust.

It applies to all work across:

- the main executable in `src/`
- option/config parsing in `src/parseopt/`
- shell-like expansion and splitting in `src/wordsplit/`
- portability and runtime support in `gnu/`
- standalone programs in `doc/`
- standalone programs in `test/`

This constitution governs all later specifications, plans, designs, tasks, code, tests, reviews, and acceptance decisions. If a later document conflicts with this constitution, this constitution prevails.

---

## Core Principles

### 1. Behavioral Equivalence Principle

The Rust migration must preserve the observable behavior of the C project unless a deviation is explicitly approved and documented.

#### 1.1 What “observable behavior” includes

Observable behavior includes, at minimum:

- command-line parsing behavior
- startup ordering and configuration loading order
- environment and rc/profile file handling
- lexer and parser behavior
- token buffering and scanner state transitions
- symbol installation, mutation, pruning, and deletion behavior
- output driver registration and selection behavior
- output content, ordering, and formatting semantics
- wordsplit scanning, expansion, coalescing, and cleanup order
- error reporting paths and termination behavior
- return codes and failure modes
- standalone `doc/` and `test/` executable behavior where present

#### 1.2 What must be preserved specifically in `cflow-new`

The Rust implementation must preserve the behaviorally significant properties identified in the source analysis, including:

- staged startup:
  - global/program init
  - config ingestion
  - parse/apply options
  - lexer/parser init
  - source processing
  - output emission
- mutable parser semantics:
  - token push/insert/delete
  - mark/restore
  - save stack
  - skip/recovery behavior
  - balance tracking
- scanner buffer-stack semantics:
  - create/switch/push/pop/flush/delete buffers
  - string/bytes scanning behavior
- symbol lifecycle semantics:
  - lookup/install
  - storage-class mutation
  - scope-based deletion
  - caller/target marking
  - collection/pruning
- output driver dispatch semantics:
  - GNU/POSIX/DOT modes
  - xref vs tree mode distinctions
- wordsplit pipeline ordering:
  - scan
  - expand
  - quote removal
  - coalesce
  - trim/null elimination
  - finish/export
  - free
- parser edge cases:
  - K&R declarations
  - typedef parsing
  - function/variable disambiguation
  - struct skipping/faking
  - balanced delimiter handling
- platform mediation behavior currently carried by `gnu/` wrappers

#### 1.3 No unapproved semantic cleanup

The migration is not a license to “improve” behavior by changing semantics that appear odd, legacy, stateful, or C-shaped.

The following are prohibited unless explicitly approved:

- changing parse ordering because a Rust design seems cleaner
- collapsing subsystem-specific errors into a generic error model
- reordering wordsplit phases
- simplifying output traversal in ways that change emitted structure
- normalizing option behavior beyond what the C program does
- removing legacy syntax branches because they are hard to model

#### 1.4 Allowed changes

Changes are allowed only when they do not alter required observable behavior, for example:

- replacing unsafe internal representations with safe Rust abstractions
- restructuring implementation internals
- improving readability or maintainability
- removing undefined behavior while preserving externally visible results
- adding internal assertions, diagnostics, or instrumentation that do not affect normal observable outputs

Any behavior-affecting change must be documented as a deviation and approved before merge.

---

### 2. Interface Compatibility First Principle

Compatibility with the existing project interface takes priority over internal elegance.

#### 2.1 Compatibility targets

The Rust migration must preserve, as applicable:

- executable name and invocation model
- supported CLI options and option semantics
- config/environment ingestion behavior
- output modes and expected formats
- file-processing entry points
- standalone demo/test program behavior where retained
- build and automation integration expectations

#### 2.2 Public and de facto interfaces both matter

For this project, interface compatibility includes both formally exposed interfaces and operationally relied-on behavior, including:

- CLI syntax
- option aliases and negation behavior
- ordering sensitivity during option application
- rc/profile processing flows
- error text categories where scripts may depend on them
- output structure consumed by users or tooling

If the C project has de facto behavior that users may rely on, that behavior must be treated as part of the interface until proven otherwise.

#### 2.3 Build-system compatibility

Because the current project uses `Makefile.in`, the migration must preserve a build and packaging story compatible with existing project workflows as far as practical.

At minimum:

- the project must remain buildable in automated environments
- migration steps must not require ad hoc manual intervention
- Rust integration must not silently drop existing build products or targets
- any new Cargo usage must be coordinated with, not blindly replace, the existing build system until transition is explicitly approved

#### 2.4 Module-boundary discipline

The project contains approximately 120 module units/clusters. Migration work must respect existing subsystem boundaries before attempting re-architecture.

Priority order:

1. preserve interface behavior
2. preserve subsystem boundaries where behavior depends on them
3. only then simplify internals

Large cross-cutting rewrites that erase module boundaries are discouraged unless they are behaviorally proven and explicitly approved.

---

### 3. Safety First Principle

Rust safety benefits are mandatory, but safety must be introduced without violating required behavior.

#### 3.1 Default to safe Rust

All new Rust code must use safe Rust by default.

`unsafe` is allowed only when:

- it is necessary
- the safe alternative is not viable for required compatibility or performance
- the unsafe boundary is tightly scoped
- the safety invariants are written down in code comments
- tests exercise the unsafe path

#### 3.2 Safe replacement over mechanical translation

The goal is not line-by-line C imitation. The goal is a behaviorally equivalent Rust implementation using the safest design that preserves semantics.

Preferred replacements include:

- ownership instead of manual free discipline
- enums instead of flag soup where semantics remain unchanged
- slices and iterators instead of pointer arithmetic where semantics remain unchanged
- `Result`/`Option` internally, with outward behavior adapted to match the C project
- explicit state machines for scanner/parser behavior where useful

#### 3.3 Unsafe containment policy

If unsafe code is required, it must satisfy all of the following:

- isolated in the smallest possible module/function
- documented with:
  - why unsafe is needed
  - what invariants are required
  - who upholds those invariants
- covered by tests at the boundary
- reviewed by at least one reviewer specifically for safety invariants
- never used merely for convenience

#### 3.4 No accidental panic boundaries

The C program generally expresses failure through explicit error paths, not Rust panics. Therefore:

- panics must not replace normal user-facing error handling
- `unwrap`, `expect`, and unchecked indexing are forbidden in production paths unless proven unreachable and documented
- recoverable failures must map to explicit project error behavior
- fatal behavior, where required, must match intended C semantics as closely as practical

#### 3.5 Memory safety is necessary, not sufficient

A Rust implementation is not acceptable merely because it is memory-safe. It must also be:

- behaviorally correct
- interface-compatible
- performance-aware
- test-verified

---

### 4. Performance Constraint Principle

The Rust migration must not cause unacceptable regressions in performance, especially on known hot paths.

#### 4.1 Performance is a project requirement

Performance is a first-class migration constraint, not a post-hoc optimization goal.

The migration must respect hot paths identified in the source behavior analysis, including:

- lexical scanning
- parser token handling
- symbol lookup/insertion
- graph/output traversal
- wordsplit transformation passes
- dependency closure computation
- formatting-heavy support paths where relevant

#### 4.2 No premature pessimization

The project must avoid design choices that predictably degrade performance, including:

- excessive heap allocation in token hot paths
- unnecessary string copying in lexer/parser/output paths
- replacing mutable state machines with expensive recomputation
- hash table substitutions that materially worsen lookup/insert behavior
- abstracting hot paths before benchmarks confirm acceptability

#### 4.3 Preserve asymptotic behavior unless approved

Algorithmic complexity must not worsen without explicit approval.

Examples:

- O(1)/amortized mutable operations must not become O(n) by accident
- repeated symbol-table lookups must not gain avoidable indirection
- node-list transformations in wordsplit must not gain unnecessary extra passes
- transitive closure logic must not regress in asymptotic behavior without a compelling documented reason

#### 4.4 Measure, do not guess

Performance claims must be backed by benchmarks or comparable evidence, especially for:

- scanner throughput
- parser throughput
- symbol graph construction
- output generation over larger inputs
- wordsplit-heavy scenarios

#### 4.5 Compatibility over micro-optimization, but not over negligence

Behavioral compatibility wins over superficial speedups. However, compatibility does not excuse careless slowdowns.

The correct standard is:

- preserve behavior first
- achieve acceptable performance second
- optimize only where measurement justifies it

---

## Migration Guidelines

### 5. C-to-Rust Mapping Rules

These rules define the default translation strategy from C concepts to Rust concepts.

#### 5.1 General mapping rule

Map C implementation patterns to Rust constructs by preserving behavior and state transitions, not by preserving syntax.

#### 5.2 Data structure mapping

Default mappings:

- C structs -> Rust `struct`
- tagged integer state -> Rust `enum` where external semantics are preserved
- linked lists -> Rust-owned list structures or vectors only if ordering, mutation, iterator invalidation, and removal semantics remain correct
- C hash tables -> Rust implementation with equivalent lookup/insertion/removal semantics and acceptable performance
- manual allocation/free -> ownership/borrowing with explicit lifecycle boundaries
- raw buffers -> `Vec<u8>`, slices, or dedicated buffer structs when compatible with scanner semantics

Do not replace a mutable structure with a different abstraction if that changes ordering, identity, mutation timing, or deletion behavior.

#### 5.3 Function mapping

C functions may map to:

- free functions
- inherent methods
- trait implementations
- module-private helpers

But call semantics must remain compatible where behavior depends on:

- ordering
- side effects
- state mutation
- error propagation
- shared mutable context

#### 5.4 Global state mapping

Because the C project uses staged initialization and mutable subsystem state, global state must be migrated carefully.

Allowed approaches include:

- explicit context objects
- subsystem state structs
- controlled interior mutability where justified

Prohibited approaches include:

- hidden thread-local or global state introduced without need
- lazy initialization that changes startup ordering
- splitting formerly unified state in ways that alter sequencing or ownership semantics

#### 5.5 Error handling mapping

Map C error handling to Rust in a way that preserves user-visible behavior.

Preferred internal model:

- typed internal errors
- explicit propagation
- boundary adaptation to match existing output and exit semantics

Must preserve:

- subsystem-specific reporting identity
- fatal vs recoverable distinctions
- parser recovery paths where they exist
- allocation-failure semantics where still relevant to behavior

#### 5.6 Scanner and parser mapping

The lexer/parser subsystems are stateful and must be modeled explicitly.

Required rules:

- preserve scanner buffer-stack semantics
- preserve token save/restore behavior
- preserve backtracking-like parser operations
- preserve balanced-delimiter tracking
- preserve old-style C grammar branches

The scanner/parser must not be replaced with a “cleaner” parser generator approach unless full behavior parity is demonstrated and approved.

#### 5.7 Platform wrapper mapping

The `gnu/` portability layer must be migrated based on behavior categories:

- wrappers still required for compatibility should remain as Rust compatibility modules
- wrappers obsolete due to Rust/std behavior may be reduced only after proving no compatibility loss
- platform-specific behavior must remain explicit, not implicit

#### 5.8 Incremental migration rule

Migration should proceed incrementally by subsystem or behavior slice, with each increment remaining testable.

Preferred migration slices:

- isolated support modules
- parseopt subsystem
- wordsplit subsystem
- output subsystem
- symbol subsystem
- scanner/parser-adjacent subsystems
- main executable orchestration

Avoid “big bang” rewrites across many modules at once.

---

### 6. Principles for Handling Uncertain Behavior

The project analysis identifies areas where exact behavior is not fully known. Uncertainty must be handled conservatively.

#### 6.1 When behavior is uncertain, preserve first

If exact behavior is uncertain:

1. assume it may matter
2. preserve existing structure and sequencing as much as possible
3. add characterization tests
4. delay simplification until evidence exists

#### 6.2 Sources of evidence, in order of authority

When resolving uncertainty, use evidence in this order:

1. actual C source behavior
2. executable behavior under tests
3. existing project documentation
4. observed outputs from fixtures/golden files
5. inferred behavior from static analysis

Inference alone is never enough to justify a semantic change where executable evidence can be gathered.

#### 6.3 Characterization before redesign

For unclear areas such as:

- `init()` details
- exit-code rules
- parser edge recovery
- wordsplit subprocess/expansion details
- exact output formatting nuances
- memory lifetime assumptions tied to behavior

the team must first create characterization tests or trace captures before redesigning.

#### 6.4 Unknown behavior log

Every uncertain behavior encountered during migration must be recorded in a project-visible log containing:

- subsystem/module
- observed uncertainty
- current assumption
- evidence collected
- decision taken
- whether behavior parity is proven, provisional, or intentionally changed

No unresolved uncertainty may be silently buried in implementation code.

#### 6.5 Deviation approval rule

If exact parity cannot be achieved or is judged harmful, the change must be documented as a deviation including:

- original C behavior
- new Rust behavior
- reason for deviation
- affected users/interfaces
- compatibility impact
- mitigation/tests

No deviation is valid without explicit approval.

---

### 7. Test Verification Requirements

Testing is mandatory evidence for migration correctness.

#### 7.1 Minimum testing philosophy

Each migrated subsystem must be verified by a combination of:

- characterization tests against C behavior
- unit tests for Rust internals
- integration tests at subsystem boundaries
- end-to-end tests on executable behavior

#### 7.2 Required test categories

The project must maintain, at minimum, the following categories:

##### a. CLI and startup tests
Must verify:

- option parsing
- environment ingestion
- rc/profile loading
- startup order-sensitive behavior
- help/version/usage behavior

##### b. Lexer/parser tests
Must verify:

- tokenization behavior
- save/restore semantics
- balanced delimiter handling
- declaration parsing branches
- K&R handling
- recovery/skip behavior where observable

##### c. Symbol lifecycle tests
Must verify:

- lookup/install
- storage mutation
- scope deletion
- caller/target marking
- collection/pruning behavior

##### d. Output tests
Must verify:

- GNU/POSIX/DOT modes
- xref/tree mode distinctions
- output ordering
- formatting-sensitive results where scripts or users depend on them

##### e. Wordsplit tests
Must verify:

- scanning
- quoted strings
- variable expansion
- command expansion where supported
- tilde/path expansion
- coalescing
- null elimination
- final word export behavior

##### f. Portability/wrapper tests
Must verify, where applicable:

- file descriptor behavior
- stat/open/close/fcntl compatibility
- error text/errno-related behavior
- platform-specific cases retained from `gnu/`

##### g. Standalone executable tests
If `doc/` and `test/` programs remain in scope, their observable behavior must also be tested.

#### 7.3 Golden testing rule

Where output is user-visible and structured, golden tests should be used.

Golden tests are especially required for:

- output driver modes
- help/usage text with wrapping-sensitive behavior
- selected parser/output scenarios
- known wordsplit edge cases

Golden outputs must be derived from the C implementation unless an approved deviation exists.

#### 7.4 Differential testing rule

Where practical, the Rust executable must be tested against the C executable on the same inputs.

Differential tests are strongly preferred for:

- CLI behavior
- parser behavior on representative code samples
- output generation
- wordsplit transformations

#### 7.5 Regression test rule

Every discovered migration bug must add a regression test before or alongside the fix.

---

## Quality Gates

### 8. Tests That Must Pass

No migration work is complete until all required quality gates pass.

#### 8.1 Mandatory passing conditions for merge

A change may not merge unless all applicable tests pass:

- existing unit tests
- new unit tests for changed Rust code
- integration tests for affected subsystem boundaries
- end-to-end tests for affected executable behavior
- regression tests for any bug being fixed
- golden/differential tests for output-affecting changes

#### 8.2 Subsystem-specific gate

A subsystem migration is not complete until its behavior is verified against the C baseline for the primary relevant scenarios.

Minimum examples:

- `src/parseopt/`: option parsing parity
- `src/wordsplit/`: transformation pipeline parity
- `src/output*`: output mode parity
- `src/parser*` and `src/c*`: scanner/parser parity
- `src/symbol*` and table/list modules: symbol lifecycle parity
- `gnu/`: compatibility wrapper parity where retained

#### 8.3 Whole-program gate

Before declaring the main migration complete, the Rust implementation must pass:

- representative end-to-end CLI scenarios
- representative source analysis scenarios
- all supported output mode scenarios
- configuration ingestion scenarios
- error-path scenarios
- performance gate benchmarks

#### 8.4 No waived test failures by default

Failing tests may not be ignored, quarantined, or waived except with explicit temporary approval and a tracked remediation plan.

Temporary waivers must include:

- reason
- owner
- deadline
- risk assessment

---

### 9. Code Review Standards

Every merged change must satisfy these review standards.

#### 9.1 Required review questions

Reviewers must verify:

- Does this preserve observable behavior?
- Does this preserve interface compatibility?
- Does this introduce unnecessary semantic drift?
- Is the safety story sound?
- Is `unsafe` minimized and justified?
- Are error paths correct and compatible?
- Are tests sufficient and targeted?
- Is performance risk acceptable and measured where needed?

#### 9.2 Review depth by change type

##### Low-risk internal refactor
Requires:
- correctness review
- test adequacy review

##### Behavior-adjacent subsystem change
Requires:
- behavior parity review
- test evidence review
- interface review

##### Unsafe code change
Requires:
- dedicated unsafe review
- invariant review
- boundary tests review

##### Hot path or algorithm change
Requires:
- performance review
- benchmark evidence review
- allocation/copying review

#### 9.3 Prohibited review outcomes

The following are not acceptable reasons to approve code:

- “more idiomatic Rust” without compatibility proof
- “simpler design” without behavioral evidence
- “tests probably cover it” without specific coverage
- “unsafe seems fine” without invariants
- “performance is probably okay” without measurement on hot paths

#### 9.4 Documentation requirements in reviewed code

Complex or behavior-sensitive code must include comments describing:

- externally required behavior being preserved
- key state transitions
- invariants
- compatibility constraints
- reasons a strange-looking implementation exists

The project must prefer explicitness over cleverness.

---

### 10. Performance Benchmark Requirements

Performance benchmarks are mandatory for behaviorally significant or hot-path migrations.

#### 10.1 Benchmark gate scope

Benchmarks are required when changes affect:

- scanner internals
- parser internals
- symbol lookup/storage
- output traversal
- wordsplit core passes
- dependency graph closure
- core allocation-heavy helpers
- platform wrappers on hot execution paths

#### 10.2 Baseline comparison

Benchmarks should compare, as applicable:

- Rust current branch vs Rust mainline
- Rust implementation vs C baseline for representative workloads

The comparison must use representative input sizes and scenarios, not toy-only cases.

#### 10.3 Required benchmark dimensions

At minimum, benchmark reports should include relevant measures such as:

- execution time
- throughput
- memory allocation count or volume where practical
- peak memory where relevant
- output equivalence confirmation for benchmarked scenarios

#### 10.4 Acceptance rule for regressions

A measurable regression must be treated as a defect unless one of the following is true:

- the regression is negligible and documented
- the regression is justified by a substantial safety/correctness gain
- the regression is temporary and tracked with an explicit remediation plan
- the previous behavior was itself unacceptable and the tradeoff is approved

#### 10.5 No benchmark-free hot path rewrites

Hot path rewrites must not merge without benchmark evidence.

---

## Governance and Amendment

### 11. Authority

This constitution is the governing document for the migration project.

All downstream documents must conform to it, including:

- specs
- plans
- task lists
- design notes
- implementation PRs
- test strategies

### 12. Amendment Rule

This constitution may be amended only by explicit project decision.

Any amendment must state:

- the previous rule
- the new rule
- the reason
- affected project areas
- whether the change is temporary or permanent

### 13. Decision Rule

When tradeoffs arise, decisions must be made in this order:

1. behavioral equivalence
2. interface compatibility
3. safety
4. performance
5. internal elegance

Internal elegance is never sufficient reason to violate a higher-order principle.