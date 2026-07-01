# constitution.md

## Purpose

This document is the governing constitution for the Rust migration of `shc`. It defines the non-negotiable project principles, migration rules, and quality gates that all later specifications, plans, tasks, code, and reviews must follow.

Where the original C behavior is fully known, the Rust rewrite must preserve it. Where the source behavior is uncertain from available evidence, the project must preserve externally observable behavior conservatively and document every assumption explicitly.

This constitution applies to the whole project, including:
- the single execution-oriented module represented by `src/shc.c`
- command-line parsing behavior
- keyed state initialization and transformation flow
- script reading and shell evaluation flow
- output generation helpers and generated C emission
- build-step invocation behavior
- all tests, reviews, refactors, and release decisions

---

## Core Principles

### 1. Behavioral Equivalence Principle

The Rust implementation must preserve the observable behavior of the C program unless a deviation is explicitly approved and documented.

#### Required meaning of behavioral equivalence
Behavioral equivalence includes, at minimum:
- command-line driven control flow shape
- ordering of major phases
- success/failure branching behavior
- file I/O behavior visible to users
- generated output semantics
- build invocation semantics
- byte-length-driven processing behavior
- stateful transformation lifecycle
- exit status behavior
- stderr/stdout observable behavior where applicable

#### For this project specifically
The migration must preserve the staged execution model indicated by the C inventory:
1. process entry through `main`
2. orchestration through `do_all`
3. argument parsing via `parse_args` and `parse_an_arg`
4. state lifecycle via `stte_0`, `key`, `key_with_file`, and `arc4`
5. script acquisition via `read_script`
6. shell-related execution via `eval_shell`
7. output formatting and C emission via `prnt_bytes`, `prnt_array`, `dump_array`, and `write_C`
8. post-generation build stage via `make`

#### Non-permitted changes without explicit approval
The following are prohibited unless justified, reviewed, and documented as intentional behavior changes:
- changing argument parsing semantics
- collapsing distinct execution stages into opaque combined behavior
- replacing explicit length-based processing with string-termination-based logic
- changing return-value-driven branching into unconditional progression
- removing or weakening file-based keying behavior
- changing output generation ordering in ways visible to consumers
- altering build-stage invocation semantics
- changing shell evaluation timing or result interpretation

#### Evidence rule
When deciding whether a Rust behavior matches C behavior, the order of authority is:
1. observed behavior from executable tests against the C version
2. directly verified C source behavior
3. documented interface and behavior summaries
4. conservative inference from naming and structure

If evidence conflicts, the higher authority wins.

---

### 2. Interface Compatibility First Principle

The migration must preserve externally relevant interfaces before pursuing internal redesign.

#### Scope of interface compatibility
Interface compatibility includes:
- CLI arguments and argument interpretation
- exit codes
- file inputs and outputs
- generated artifact expectations
- environment and subprocess interaction expectations
- ordering and availability of major user-visible operations

For this project, compatibility must especially preserve:
- full-program entry semantics
- parser-driven invocation model
- file-path accepting behaviors for script/key/output operations
- return-bearing success/failure decisions
- generated C output role in the workflow

#### Internal freedom, external stability
Internal Rust structure may differ from the C layout, but only if:
- the external contract remains compatible
- phase ordering remains behaviorally consistent
- helper responsibilities remain understandable and traceable
- debugging and verification become easier, not harder

#### Mapping expectation
The rewrite should begin with a structure that makes C-to-Rust correspondence clear. Early migration stages should prefer obvious one-to-one or one-to-few mappings over ambitious abstraction.

Examples:
- parser logic should remain visibly separate from generation logic
- state initialization/keying/transformation should remain visibly separate phases
- output helper functions should remain subordinate to a higher-level generation routine
- build triggering should remain a distinct stage

#### Compatibility over elegance
If a more idiomatic Rust design would risk changing behavior, the less elegant but more compatible design must be chosen.

---

### 3. Safety First Principle

Rust safety benefits must be used aggressively, but never as an excuse to silently change behavior.

#### Safety priorities
The Rust implementation must eliminate, wherever possible:
- undefined behavior
- unchecked pointer arithmetic
- buffer overruns
- use-after-free patterns
- null dereference risks
- integer conversion hazards
- unchecked file/resource handling
- accidental aliasing and mutation hazards

#### Safety without behavioral drift
Safety improvements must preserve semantics. In particular:
- explicit lengths from C must remain explicit lengths in Rust
- byte buffers must be handled as bytes, not assumed UTF-8 strings, unless evidence proves text semantics
- null/absent/error conditions must be modeled explicitly
- result-bearing functions must surface failure paths clearly
- global mutable state from C must be isolated carefully and rewritten into controlled state ownership without reordering semantics

#### Unsafe code rule
`unsafe` Rust is permitted only when all of the following are true:
1. there is no practical safe alternative
2. the unsafe block is narrowly scoped
3. the invariants are documented immediately next to the code
4. tests cover the relevant behavior
5. review explicitly approves the unsafe usage

The default assumption for this project is that the migration should be achievable with little or no `unsafe`.

#### Security-sensitive behaviors
Because the module includes keying/transformation and shell-related execution behaviors:
- data handling must avoid accidental leakage through debug output
- shell invocation behavior must not be broadened beyond the C program’s scope
- file-derived keying behavior must not become more permissive by accident
- error paths must not expose sensitive intermediate data unnecessarily

---

### 4. Performance Constraint Principle

The Rust migration must not introduce unjustified regressions in the program’s important execution paths.

#### Performance applies to behavior-preserving paths
The following paths are considered performance-relevant:
- argument scanning loops
- byte-buffer keying and transformation
- file input and output
- output formatting and generated C emission
- randomness/noise-related repeated generation
- orchestration across generation and build steps

#### Default performance policy
The Rust version should aim for:
- no significant regression in steady-state processing cost for equivalent workloads
- no unnecessary copying of large buffers
- no repeated reparsing or redundant allocations in hot paths
- explicit control over byte-oriented processing
- stream-oriented output where the C design is stream-oriented

#### What counts as acceptable
A small overhead is acceptable if it buys memory safety, deterministic behavior, or greatly improved maintainability, but not if it changes user-visible throughput materially without justification.

#### What is forbidden
- replacing in-place or bounded processing with obviously wasteful whole-buffer churn
- converting byte workflows into repeated string allocations without evidence
- obscuring hot paths behind unnecessary abstraction
- ignoring benchmark regressions because “Rust is safer”

Performance is a requirement, not an optional polish step.

---

## Migration Guidelines

### 1. C-to-Rust Mapping Rules

The migration must preserve conceptual structure while adopting Rust ownership and error handling.

#### 1.1 Function mapping

Each significant C function must map to one of the following:
- a Rust function with the same responsibility
- a Rust method on a clearly named state type
- a small Rust cluster of functions whose combined responsibility is demonstrably equivalent

No C function with behaviorally significant branching may disappear without a traceable mapping note.

For this project, the following mappings must remain explicit:
- `main` -> Rust `main`
- `do_all` -> top-level orchestration function
- `parse_args` -> argument traversal coordinator
- `parse_an_arg` -> single-step parse routine
- `stte_0` -> state reset/init operation
- `key` -> apply key material operation
- `arc4` -> data transformation operation
- `key_with_file` -> file-based keying operation
- `read_script` -> script-loading operation
- `eval_shell` -> shell execution/evaluation operation
- `write_C` -> generated C emission operation
- `make` -> build invocation operation

Helper functions may be reorganized, but their responsibilities must still be identifiable.

#### 1.2 State mapping

Mutable C state must become explicit Rust state.

Rules:
- hidden global mutable state should be converted into owned structs where possible
- state reset must remain a distinct operation
- state transitions must remain visible in code
- callers must not be allowed to skip required initialization accidentally

For the keyed transformation lifecycle, the Rust design should prefer a dedicated state type whose API makes the sequence clear:
- initialize/reset
- key or key-from-file
- transform buffer

If the original behavior depends on cumulative mutation across calls, that must be preserved.

#### 1.3 Data mapping

C data must be mapped according to semantics, not superficial syntax.

Rules:
- `char *` used as byte storage -> `&[u8]`, `&mut [u8]`, or `Vec<u8>`
- `char *` used as text -> `String` or `&str` only when encoding assumptions are justified
- pointer-plus-length APIs -> slice-based APIs
- nullable return values -> `Option<T>` or `Result<T, E>`
- integer status returns -> `Result`, enums, or explicit status types internally, with compatibility preserved at boundaries

Length-bearing C functions must not be rewritten into sentinel-dependent Rust logic unless the original semantics are proven sentinel-based.

#### 1.4 Error mapping

C integer/error-pointer conventions should become explicit Rust error handling internally.

Rules:
- internal code should prefer `Result<T, E>`
- externally visible exit codes and branch outcomes must remain compatible
- lossy compression of multiple error modes into one generic error is not allowed if it changes control behavior
- all boundary conversions from structured Rust errors to CLI-visible statuses must be centralized and tested

#### 1.5 I/O mapping

File and process behavior must remain explicit and inspectable.

Rules:
- file reads and writes must use standard library APIs with clear error propagation
- stream-oriented formatting behavior must stay stream-oriented where practical
- subprocess/build invocation must be explicit and testable
- path handling must preserve the original acceptance/rejection behavior as closely as evidence allows

#### 1.6 Random/noise behavior mapping

Random-related helpers must preserve role and boundaries.

Rules:
- modulus-bounded generation must remain modulus-bounded
- any determinism or non-determinism present in the C behavior must be preserved when known
- if seeding behavior is unknown, the project must document the uncertainty and avoid inventing stronger guarantees

---

### 2. Principles for Handling Uncertain Behavior

Because available summaries do not fully define all internals, uncertainty must be handled conservatively.

#### 2.1 Do not invent semantics
When behavior is not established by source, tests, or authoritative documentation:
- do not guess
- do not “improve” semantics
- do not normalize edge cases speculatively
- do not replace ambiguity with convenient Rust idioms silently

#### 2.2 Conservative preservation rule
If multiple interpretations are possible, prefer the one that:
- best preserves observable C-era behavior
- least expands functionality
- least narrows accepted inputs
- least changes output shape
- least changes timing/order of side effects

#### 2.3 Document every unresolved assumption
Any unresolved behavior must be recorded in migration notes with:
- the uncertain area
- what evidence exists
- the chosen interpretation
- why that interpretation is the least risky
- what test would confirm or falsify it later

#### 2.4 Keep ambiguous boundaries explicit
If a function’s true semantics are uncertain, its Rust form should not overcommit.

Examples:
- use byte buffers instead of strings when text encoding is unclear
- preserve status-like return distinctions instead of flattening into boolean success
- preserve phase boundaries instead of merging steps into one opaque pipeline

#### 2.5 Golden behavior over theoretical correctness
If the original C behavior is odd but demonstrable, the Rust rewrite must reproduce it unless there is explicit approval to fix it.

This includes:
- unusual argument acceptance/rejection
- odd formatting details in generated output
- surprising ordering of steps
- non-idiomatic return code usage

---

### 3. Test Verification Requirements

Testing is the primary proof that the migration preserves behavior.

#### 3.1 Dual focus
The test strategy must cover both:
- black-box behavioral equivalence
- white-box Rust safety and invariants

#### 3.2 Required test categories

##### a. CLI behavior tests
Must verify:
- accepted argument patterns
- rejected argument patterns
- exit code behavior
- user-visible outputs and side effects
- phase ordering implications visible through outputs/artifacts

##### b. Parsing tests
Must verify:
- multi-argument traversal behavior
- per-argument control effects
- edge cases for short/minimal argument lists
- handling of invalid or unsupported forms

##### c. State lifecycle tests
Must verify, as applicable:
- state reset behavior
- key application behavior
- file-based keying behavior
- transformation dependence on prior state setup
- repeated-call behavior where state mutation matters

##### d. File I/O tests
Must verify:
- script read success and failure cases
- output file generation
- file-path error propagation
- handling of absent or inaccessible files

##### e. Output generation tests
Must verify:
- generated C output shape at a meaningful level
- helper-emitted formatting equivalence where behaviorally relevant
- stream-oriented behavior where applicable

##### f. Shell/build interaction tests
Must verify:
- shell evaluation outcome handling
- build-stage invocation success/failure propagation
- conditional sequencing if supported by the original behavior

##### g. Boundary and robustness tests
Must verify:
- zero or minimal inputs where meaningful
- explicit length-driven processing
- large-enough buffers to exercise scaling behavior
- modulus/random boundary handling if observable

#### 3.3 Differential testing requirement
Where feasible, the project must run equivalent inputs against:
- the original C implementation
- the Rust implementation

and compare:
- exit codes
- generated artifacts
- stdout/stderr
- observable file outputs
- major side effects

For uncertain behavior, differential tests are the preferred source of truth.

#### 3.4 Regression tests are mandatory
Every bug found during migration must produce:
1. a failing test
2. a fix
3. a passing test retained in the suite

---

## Quality Gates

### 1. Tests That Must Pass

No migration milestone is complete unless all applicable gates pass.

#### 1.1 Build and unit gates
Must pass:
- Rust project build with no errors
- all unit tests
- all integration tests
- all golden/differential tests marked required
- lint and formatting checks required by project tooling

#### 1.2 Behavioral gates
Must pass:
- CLI compatibility checks for supported scenarios
- exit-status compatibility checks
- generated-output equivalence checks for representative fixtures
- file error-path checks
- orchestration-path checks for main execution scenarios

#### 1.3 Safety gates
Must pass:
- no undefined behavior proxies introduced through unchecked assumptions
- no ignored `Result` values in behaviorally significant paths
- no panics in normal user-error paths unless explicitly intended and documented
- no unchecked integer narrowing in critical buffer/length paths without justification

#### 1.4 Completion gate for a migrated function/cluster
A migrated area is not complete until:
- its C counterpart is mapped
- tests exist for its success path
- tests exist for its failure or edge path, where meaningful
- observable behavior is compared against evidence
- unresolved assumptions are documented

---

### 2. Code Review Standards

All substantive changes require review against this constitution.

#### 2.1 Review criteria
Reviewers must verify:
- behavioral compatibility is preserved
- interface changes are absent or explicitly approved
- state transitions remain correct and visible
- error handling is explicit and complete
- byte/length semantics are preserved
- unsafe usage, if any, is justified and contained
- tests meaningfully cover the changed behavior
- performance-sensitive paths are not obviously degraded

#### 2.2 Required review questions
Every review should answer:
1. What C behavior is being preserved here?
2. What evidence supports that this Rust behavior is equivalent?
3. Did the change alter any external interface or side effect?
4. Are length-based and byte-based semantics preserved?
5. Are failure paths tested?
6. Did we accidentally replace uncertain behavior with invented behavior?
7. Is there any needless allocation, copying, or subprocess overhead?

#### 2.3 Review rejection conditions
A change must be rejected if it:
- changes behavior without documentation and approval
- removes phase boundaries important to the original design
- hides uncertain behavior behind unjustified assumptions
- lacks tests for altered control flow
- adds unsafe code without necessity and documented invariants
- introduces significant performance cost without evidence and justification

---

### 3. Performance Benchmark Requirements

Performance validation is required for behaviorally important paths.

#### 3.1 Minimum benchmark scope
Benchmarks must cover, where applicable:
- argument parsing with representative argument counts
- byte transformation on representative buffer sizes
- script read and generated output write paths
- output formatting/helpers on representative payloads
- end-to-end orchestration for representative fixtures

#### 3.2 Comparison standard
The Rust version should be compared against:
- the original C baseline when practical
- prior Rust baseline on subsequent changes

Benchmarks should focus on:
- elapsed time
- allocation behavior where measurable
- scaling with input size
- avoidance of pathological regressions

#### 3.3 Regression policy
Performance regressions in key paths must be:
- measured
- explained
- judged acceptable or fixed before merge

Unmeasured claims of “fast enough” are not sufficient for hot or repeated paths.

#### 3.4 Benchmark gate for merge
A change touching a known performance-sensitive area must not merge unless:
- benchmark impact is known, or
- the reviewer explicitly records why benchmark execution is unnecessary for that change

---

## Governance

### Amendment Rule

This constitution may be amended only when:
- the change is written explicitly
- the reason is documented
- the impact on existing specs and plans is identified
- reviewers confirm the amendment does not weaken behavioral preservation without deliberate approval

### Priority Rule

If any later project artifact conflicts with this constitution, this constitution wins.

Priority order:
1. `constitution.md`
2. approved migration specs
3. implementation plans
4. task breakdowns
5. local refactoring preferences

### Practical Rule

When forced to choose among:
- idiomatic Rust,
- minimal code,
- exact behavioral preservation,

the project must choose exact behavioral preservation first, then safety, then clarity, then idiomatic refinement.