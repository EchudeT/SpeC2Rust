# Constitution for the Rust Migration of `shc`

## Purpose

This document defines the non-negotiable project-level principles for the Rust migration of the C project `shc`. It is the governing standard for all later specifications, plans, tasks, code, tests, reviews, and release decisions.

Where this constitution conflicts with convenience, speed, stylistic preference, or architectural novelty, this constitution wins.

Because the source project is small in file count but behaviorally dense and stateful, this migration must prioritize correctness of observed behavior over aggressive redesign.

---

## Scope

This constitution applies to:

- the full Rust rewrite of the current C implementation
- all modules, helper functions, state handling, and build-facing behavior
- all generated artifacts and user-visible command-line behavior
- all testing, benchmarking, review, and acceptance criteria
- all later project documents derived from the migration effort

This project currently centers around a single analyzed module with a single orchestration pipeline. That simplicity of file layout does **not** reduce the rigor required for preserving semantics.

---

# 1. Core Principles

## 1.1 Behavioral Equivalence Principle

The Rust implementation must preserve the observable behavior of the C implementation unless a deliberate, documented exception is approved.

### Rules

1. **Preserve runtime phase ordering.**
   The Rust rewrite must preserve the high-level orchestration flow supported by the current analysis:
   - program entry
   - top-level orchestration
   - argument parsing
   - transformation-state initialization and keying
   - script acquisition
   - shell-related evaluation
   - content transformation
   - output generation
   - optional build execution
   - process exit

2. **Preserve distinct behavioral stages.**
   The following stages must remain distinguishable in implementation and tests:
   - whole-argument parsing vs per-argument parsing
   - state initialization vs key injection vs byte transformation
   - script reading vs shell evaluation
   - generation vs build execution
   - random helper production vs noise-buffer population
   - low-level emission helpers vs whole-file generation

3. **Preserve control-point separations.**
   Operations that are independently meaningful in C must remain independently meaningful in Rust, especially where failure can occur at separate points.

4. **Preserve error timing as closely as evidence permits.**
   If a failure would have been detected during argument parsing, file loading, generation, or build invocation in C, the Rust version should fail at the same phase rather than silently deferring or reclassifying the failure.

5. **Preserve boundary behavior.**
   Zero-length inputs, empty argument sets, small buffers, file failures, formatting edge cases, and random-generation edge conditions must be preserved or, if not fully known, explicitly investigated and pinned down by tests before behavior is changed.

6. **Preserve stateful transformation semantics.**
   The relationship among initialization, keying, and transformation routines must remain consistent with the C version’s mutable-state pipeline. Rust may encapsulate state more safely, but must not alter the externally visible sequence semantics.

7. **Do not “simplify away” behaviorally meaningful helper layers.**
   Rust may restructure internals, but only if tests demonstrate that helper-role distinctions and output semantics remain intact.

### What this forbids

- rewriting the tool into a materially different workflow because Rust makes it cleaner
- merging separate stages into a single opaque function when that changes timing, failure points, or debuggability
- replacing uncertain C behavior with guessed “better behavior” without evidence
- changing output formatting merely for aesthetic reasons
- replacing stateful processing with a new abstraction that changes results or ordering

---

## 1.2 Interface Compatibility First Principle

User-visible and integration-visible compatibility takes precedence over internal Rust elegance.

### Rules

1. **Preserve CLI expectations first.**
   The Rust implementation must match the C program’s command-line contract as discovered through source review and execution-based verification.

2. **Preserve invocation shape.**
   Input conventions, argument ordering expectations, optional build behavior, and file-driven workflows must remain compatible unless a documented compatibility break is explicitly approved.

3. **Preserve generated-output compatibility.**
   If the tool emits C source, arrays, bytes, declarations, or other textual artifacts, the Rust version must preserve compatibility at the artifact level, not merely at the conceptual level.

4. **Preserve exit-code meaning where discoverable.**
   Exit status must remain compatible where the original behavior can be determined.

5. **Preserve build-system integration.**
   The migration must fit the project’s current build environment centered on `Makefile.in`. Any Rust integration must not require consumers to adopt a radically different top-level build workflow unless formally approved.

6. **Prefer wrappers and adaptation over forced external changes.**
   When Rust tooling differs from C conventions, the project should absorb that complexity internally rather than pushing it onto users or downstream build scripts.

### What this forbids

- introducing a new CLI just because the original is awkward
- changing output filenames, formatting, or artifact structure without compatibility review
- requiring Cargo-only workflows if that breaks expected Make-based project usage
- redefining failure modes in ways visible to scripts or automation without approval

---

## 1.3 Safety First Principle

Rust safety improvements are mandatory, but they must be introduced in ways that preserve intended semantics.

### Rules

1. **Memory safety is a hard requirement.**
   Safe Rust is the default. `unsafe` is allowed only when strictly necessary and must be minimal, localized, documented, and review-justified.

2. **Undefined C behavior must not be reproduced if safe containment is possible.**
   If the original C relies on fragile pointer arithmetic, unchecked lengths, implicit aliasing, or unchecked file/stream assumptions, the Rust rewrite must implement the intended behavior safely rather than reproducing memory unsafety.

3. **Input validation must not alter valid behavior.**
   Rust may reject invalid memory states internally, but must not begin rejecting previously valid user inputs without evidence that the C version did so.

4. **State must be explicit.**
   Hidden mutable global behavior from C should be made explicit in Rust structures where possible, especially for the transformation pipeline. This is encouraged if and only if externally visible semantics remain unchanged.

5. **Errors must be represented explicitly.**
   Rust error types, enums, and result propagation should replace ambiguous internal failure handling where feasible, while preserving the original user-visible behavior.

6. **External process and file interactions must be constrained.**
   All shell, file, and build interactions must be implemented carefully, with explicit handling of failure, paths, encodings, and command invocation assumptions.

7. **Randomness must be controlled intentionally.**
   Any random-related behavior in the original must be preserved safely. The rewrite must not accidentally introduce nondeterminism where the original was deterministic, nor determinism where the original was runtime-variable, without evidence.

### `unsafe` policy

`unsafe` may be used only if all of the following are true:

- there is no practical safe alternative without material regression
- the reason is documented adjacent to the code
- invariants are documented
- tests cover the behavior guarded by the unsafe block
- code review explicitly signs off on the unsafe usage

---

## 1.4 Performance Constraint Principle

The Rust migration must not introduce unjustified regressions in the tool’s performance-sensitive paths.

### Rules

1. **Match or improve practical performance.**
   The Rust version should be at least comparable to the C version in realistic usage, especially in:
   - byte-wise transformation
   - script reading
   - output formatting and emission
   - random/noise generation
   - end-to-end generation workflow

2. **Preserve linear work where the C behavior is linear.**
   Buffer-processing and formatting paths must remain efficient with respect to input length.

3. **Avoid allocation-heavy rewrites.**
   Do not replace streaming or in-place style operations with excessive temporary allocations unless measured and justified.

4. **Preserve output throughput.**
   Emission helpers must avoid avoidable per-byte overhead explosions beyond what is necessary for correctness.

5. **Benchmark before claiming improvement.**
   Perceived elegance or idiomatic structure does not count as a performance win; measured evidence does.

6. **Do not optimize away semantics.**
   Performance changes are permitted only if behavioral equivalence remains intact.

### Default performance targets

Unless stronger project-specific baselines are later established, acceptance should assume:

- no significant regression in end-to-end execution for representative workloads
- no significant regression in large-buffer transformation throughput
- no significant regression in generated-output emission time
- no pathological memory growth relative to input size

---

# 2. Migration Guidelines

## 2.1 C-to-Rust Mapping Rules

The migration must follow disciplined mapping from C concepts to Rust concepts.

### 2.1.1 Function mapping

1. Each behaviorally significant C function must have a traceable Rust counterpart.
2. One-to-one mapping is preferred for:
   - orchestration entry points
   - parsing layers
   - transformation-state lifecycle functions
   - output-generation helpers
   - build/generation split points
3. Consolidation is allowed only when:
   - no user-visible behavior changes
   - no meaningful failure point disappears
   - no testability is lost
   - documentation records the mapping

### 2.1.2 State mapping

1. Mutable transformation state should be represented by an explicit Rust struct.
2. State transitions corresponding to initialization, keying, and transformation must be visible in code.
3. If C used globals or file-static state, Rust should encapsulate them in owned state objects unless global behavior is itself externally meaningful.
4. State-reset semantics must be preserved exactly once they are determined.

### 2.1.3 Data mapping

1. `char *` and `void *` inputs in buffer-oriented paths should generally map to byte-oriented Rust types such as slices or owned byte buffers.
2. Textual data should be modeled as bytes first when the original semantics are byte-level rather than Unicode-level.
3. Do not assume UTF-8 if the C program treated data as arbitrary bytes.
4. Length-bearing APIs must remain length-aware; never infer semantics from null termination unless the original behavior requires it.

### 2.1.4 I/O mapping

1. File reads must preserve byte content faithfully.
2. Output generation must preserve textual formatting and ordering.
3. Stream-like behavior in formatting helpers should remain stream-capable where practical.
4. External build invocation must preserve command semantics as determined from the original.

### 2.1.5 Error mapping

1. Ambiguous C integer return statuses should be mapped internally to Rust error/result types.
2. The final user-visible status and messages must remain compatible once understood.
3. Distinct failure classes must remain distinct:
   - argument failure
   - file failure
   - shell-evaluation failure
   - generation failure
   - build failure

### 2.1.6 Random/helper mapping

1. Random-related helpers must remain explicit and testable.
2. Noise generation must not be silently absorbed into unrelated formatting or generation code.
3. If the original random behavior depends on process-global seeding or library defaults, the Rust implementation must determine and preserve that behavior deliberately.

---

## 2.2 Principles for Handling Uncertain Behavior

The supplied analysis explicitly indicates many areas where detailed behavior is not yet fully known. The migration must treat uncertainty as a first-class engineering concern.

### Rules

1. **Do not guess.**
   Unknown behavior must not be replaced with preferred behavior by assumption.

2. **Evidence order is mandatory.**
   When behavior is uncertain, teams must resolve it using this order:
   1. direct C source inspection
   2. execution of the C program
   3. fixture-based differential testing
   4. historical project documentation, if available
   5. explicit maintainership decision when evidence remains insufficient

3. **Unknowns must be recorded.**
   Every unresolved semantic question must be tracked in writing until resolved.

4. **Tests must lock in resolved behavior.**
   Once uncertain behavior is determined, a test must be added before or alongside the Rust implementation.

5. **Conservative preservation is the default.**
   If evidence is partial but points toward a narrower preservation choice, prefer the option least likely to change user-visible behavior.

6. **Document intentional deviations.**
   If the project intentionally changes behavior, that change must be:
   - explicitly documented
   - justified
   - approved
   - reflected in tests and release notes

### Uncertainty hotspots for this project

At minimum, the following areas require explicit verification rather than assumption:

- exact CLI parsing semantics
- zero-argument behavior
- invalid-argument handling
- exact ordering among script read, shell evaluation, transformation, output, and build
- state-reset and multi-call transformation behavior
- file-failure semantics
- exact output formatting details
- build-step trigger conditions
- random seeding and reproducibility behavior
- edge behavior for zero lengths, empty paths, and zero modulus

---

## 2.3 Test Verification Requirements

Testing is not a support activity; it is the mechanism for proving migration correctness.

### Required test layers

#### 1. Characterization tests against the C implementation
These establish the original behavior before or during rewrite.

They must cover, at minimum:

- successful representative invocations
- invalid argument cases
- zero or minimal input cases
- file-related failure cases
- output-generation behavior
- build-phase enabled/disabled behavior if applicable
- representative random/noise-dependent outputs where determinable

#### 2. Differential tests
For every supported scenario, the Rust implementation must be compared against the C implementation on:

- exit status
- stdout
- stderr
- generated files
- side effects relevant to build output and file creation

#### 3. Unit tests
Rust units must cover:

- argument parsing components
- transformation state lifecycle
- keying and byte transformation boundaries
- script reading behavior
- output formatting helpers
- random/noise helper constraints
- error mapping and propagation

#### 4. Integration tests
Integration tests must exercise the full pipeline from invocation through output generation and, where feasible, build execution.

#### 5. Edge and boundary tests
These must cover, at minimum:

- empty inputs
- zero-length buffers
- single-byte buffers
- invalid paths
- inaccessible files
- zero and small modulus inputs
- empty or minimal generated arrays
- repeated stateful operations

#### 6. Regression tests
Every defect found during migration must add a regression test before closure.

### Test evidence standards

A behavior is not considered preserved unless there is evidence through one or more of:

- direct differential output comparison
- stable characterization fixture
- targeted unit proof for an internal invariant
- benchmark comparison for performance-sensitive paths

---

# 3. Quality Gates

## 3.1 Tests That Must Pass

No migration milestone is complete unless all applicable gates below pass.

### Mandatory gate set

1. **Rust build must succeed cleanly.**
2. **All unit tests must pass.**
3. **All integration tests must pass.**
4. **All differential tests against the C implementation must pass for the approved fixture set.**
5. **All regression tests must pass.**
6. **Generated-output comparison tests must pass byte-for-byte where byte-for-byte preservation is required.**
7. **Exit-code compatibility tests must pass for covered scenarios.**
8. **Failure-path tests must pass for covered error classes.**
9. **No unresolved critical behavioral unknowns may remain for migrated code paths.**

### Warning and lint expectations

The Rust codebase must pass project-approved static checks, with the default expectation of:

- `cargo test`
- `cargo fmt --check`
- `cargo clippy` with agreed strict settings
- no ignored warnings without written justification

If the project is built through `Makefile.in`, equivalent Make targets or wrappers must enforce the same standards.

---

## 3.2 Code Review Standards

Every merged change must satisfy review against this constitution.

### Required review criteria

1. **Behavioral preservation**
   - Does the change preserve the known C behavior?
   - If behavior differs, is that difference documented and approved?

2. **Traceability**
   - Can the reviewer map the Rust code to the original behaviorally significant C logic?

3. **Safety**
   - Is the implementation safe by default?
   - Is every `unsafe` use necessary, minimal, and documented?

4. **Error semantics**
   - Are distinct failure modes preserved and represented clearly?
   - Does user-visible behavior remain compatible?

5. **Boundary handling**
   - Are zero-length, invalid-path, and small-input conditions handled consistently with known behavior?

6. **Performance awareness**
   - Has the change avoided obvious allocation or throughput regressions?
   - If not, is there benchmark justification?

7. **Test sufficiency**
   - Does the change include or update tests that prove the intended preservation?

8. **Build integration**
   - Does the change remain compatible with the project’s Make-based workflow expectations?

### Review prohibitions

A reviewer must reject changes that:

- rely on assumed behavior without evidence
- remove behaviorally significant layers without proof
- introduce undocumented compatibility changes
- add unjustified `unsafe`
- weaken tests or differential coverage
- replace byte semantics with Unicode/text semantics without proof that it is safe

---

## 3.3 Performance Benchmark Requirements

Performance must be measured, not assumed.

### Required benchmark categories

At minimum, benchmark coverage must exist for:

1. **Byte transformation throughput**
   - representative buffer sizes
   - repeated invocation behavior where relevant

2. **Script read performance**
   - small and larger representative files

3. **Output generation performance**
   - formatting and emitting representative generated content
   - larger array/byte emission cases

4. **End-to-end execution time**
   - generation-only workflow
   - generation-plus-build workflow where feasible

5. **Memory behavior**
   - representative peak memory use
   - avoidance of avoidable temporary-buffer growth

### Benchmark rules

1. Benchmarks must compare Rust against the C baseline where feasible.
2. Any material regression must be explained and approved before merge.
3. “Material” should be judged in the context of user-visible workload impact, not microbenchmark noise.
4. Optimizations must not be accepted if they reduce readability and safety without measurable need.
5. If build execution dominates runtime, benchmark reports must separate:
   - internal tool time
   - delegated external build time

---

# 4. Governance Rules

## 4.1 Priority Order

When principles appear to compete, the default priority order is:

1. behavioral equivalence
2. interface compatibility
3. safety
4. performance
5. internal elegance

This order applies unless a documented project decision explicitly overrides it.

---

## 4.2 Change Control

Any intentional deviation from this constitution requires:

- written rationale
- explicit statement of impacted behavior or interface
- test updates
- reviewer approval
- maintainership approval for user-visible compatibility changes

---

## 4.3 Definition of Done

A migrated feature, module, or release is only done when:

- the Rust implementation is behaviorally validated
- interface compatibility is demonstrated
- safety expectations are met
- benchmarks show no unapproved regression
- tests and reviews satisfy all quality gates
- unresolved uncertainties for that scope are either closed or formally accepted as deferred and non-blocking

---

## 4.4 Project-wide Non-Negotiables

The migration must not:

- become a redesign project disguised as a port
- erase behavior that has not been disproven as accidental
- trade compatibility for idiomatic Rust by default
- trade safety for speed without proof of necessity
- trade correctness for schedule pressure

---

# 5. Immediate Application to `shc`

Given the current project evidence, all planning and implementation must explicitly preserve:

- `main` to top-level orchestration flow
- the distinction between `parse_args` and `parse_an_arg`
- the distinction between `stte_0`, `key`, `key_with_file`, and `arc4`
- the distinction between `read_script` and `eval_shell`
- the helper layering of `prnt_bytes`, `prnt_array`, `dump_array`, and `write_C`
- the separation of generation and build through `write_C` and `make`
- the explicit role of `rand_mod`, `rand_chr`, and `noise`
- behavior at phase boundaries, error boundaries, and output boundaries

All later project documents must treat these as protected migration constraints unless future evidence proves otherwise.