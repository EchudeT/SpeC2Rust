# constitution.md

## Purpose

This document is the governing constitution for the Rust migration of `yank`. It defines the non-negotiable project-level principles, migration rules, and quality gates that every later specification, implementation plan, task list, and code change must follow.

Where the original C behavior is only partially known from available analysis, this constitution requires conservative preservation of observable behavior, explicit documentation of uncertainty, and proof-oriented validation before any intentional change is accepted.

---

## 1. Core Principles

### 1.1 Behavioral Equivalence Principle

The Rust implementation must preserve the observable behavior of the C program unless a deviation is explicitly approved and documented.

#### Required outcomes
- The Rust program must preserve the same top-level control ownership by `main`.
- The terminal lifecycle must remain explicitly bracketed by setup, interactive use, and teardown.
- The interactive controller role of `tmain` must be preserved in structure and effect.
- The terminal input boundary represented by `tgetc` must remain centralized rather than diffused across unrelated code paths.
- Counted-buffer behavior must remain counted-buffer behavior.
- Field comparison semantics represented by `fcmp` must remain the source of ordering and selection decisions.
- The interactive phase must continue to yield a field-oriented result rather than being reduced to pure side effects.

#### What counts as observable behavior
Observable behavior includes, at minimum:
- program startup and exit behavior
- command-line handling and usage behavior
- terminal setup and restoration behavior
- interactive loop behavior and termination behavior
- output destination, timing, and content
- byte-counted versus string-oriented output distinctions
- ordering, ranking, and selection outcomes
- error handling visible to callers or users
- cleanup behavior on normal and abnormal paths

#### Rules
- Do not simplify behavior merely because the C implementation looks awkward.
- Do not replace a dedicated behavior boundary with a more convenient Rust abstraction if the abstraction changes semantics.
- Do not treat undocumented behavior as unimportant; treat it as a compatibility risk.
- If exact behavior is unknown, prefer preserving the current structure until evidence justifies change.

---

### 1.2 Interface Compatibility First Principle

Compatibility with the original program’s externally visible interface takes precedence over internal design preferences.

#### Required outcomes
- The Rust binary must present compatible invocation behavior.
- Usage/help pathways must remain distinct and callable from the top-level control path.
- Publicly observable module responsibilities must remain recognizable in the Rust design, even if internal organization improves.
- Functionality corresponding to `input`, `strtopat`, `fcmp`, `xwrite`, `yank`, `twrite`, `tputs`, `tsetup`, `tmain`, `tgetc`, `tend`, `usage`, and `main` must remain present as explicit responsibilities.

#### Rules
- Preserve CLI shape and invocation expectations unless a change is approved as an intentional compatibility break.
- Preserve separation between:
  - counted writes and null-terminated string convenience output
  - terminal setup, active interaction, and teardown
  - preprocessing/transformation and interactive execution
- Preserve the semantic role of the field comparator rather than replacing it with incidental collection ordering.
- Avoid collapsing distinct C responsibilities into one Rust function when doing so makes compatibility auditing harder.

#### Interpretation guidance
Rust code does not need to preserve C syntax, file layout, or naming exactly. It must preserve interface meaning, boundaries, and observable effects.

---

### 1.3 Safety First Principle

The Rust migration must improve memory and resource safety without weakening compatibility.

#### Required outcomes
- Undefined behavior present or possible in C must not be carried forward into Rust.
- Terminal state restoration must be protected against early returns and error paths.
- Byte-counted operations must be represented safely and explicitly.
- Pointer-like C concepts must be modeled with Rust types that make invalid states difficult or impossible.

#### Rules
- Prefer ownership, borrowing, slices, enums, and RAII over raw-pointer-style emulation.
- Use `Result`, `Option`, and explicit state types instead of sentinel-driven hidden control flow where possible, as long as behavior remains compatible.
- Encapsulate unsafe code behind minimal, audited boundaries. Unsafe is allowed only when necessary for system interaction or proven compatibility needs.
- Every unsafe block must carry a justification comment explaining:
  - why it is required
  - what invariants it relies on
  - how those invariants are enforced
- Resource lifecycle pairs in C must become scope-guarded or otherwise mechanically enforced in Rust, especially terminal setup/teardown.
- Do not use panics for expected operational failures in normal program flow.
- Do not silently discard I/O failures, terminal failures, parse failures, or cleanup failures unless the C behavior demonstrably does so and that behavior is intentionally preserved.

#### Safety-compatible behavior preservation
Safety improvements must not redefine behavior. For example:
- replacing counted writes with lossy UTF-8-only strings is forbidden
- replacing nullable result behavior with unconditional success is forbidden
- replacing cleanup-sensitive flow with panic-prone shortcuts is forbidden

---

### 1.4 Performance Constraint Principle

The Rust migration must not introduce meaningful regressions in the program’s steady-state or interactive performance.

#### Performance-sensitive areas
Based on current evidence, the project must treat the following as performance-sensitive:
- the `tmain` interactive loop
- repeated `tgetc` input acquisition
- repeated terminal output via `twrite` and `tputs`
- low-level write behavior corresponding to `xwrite`
- repeated field comparison behavior corresponding to `fcmp`
- any repeated pattern conversion or matching behavior corresponding to `strtopat`

#### Rules
- Preserve low-overhead paths for terminal interaction and output.
- Avoid unnecessary allocation, copying, formatting, and conversion in the interactive loop.
- Do not replace byte-slice operations with string conversions unless required and benchmarked.
- Do not introduce locking, dynamic dispatch, or heavyweight abstractions in hot paths without evidence they are acceptable.
- Design for clear setup/steady-state/teardown cost separation.
- Prefer zero-copy or bounded-copy approaches where compatible.

#### Benchmark standard
Performance changes are acceptable only when:
- they preserve behavior, and
- they are neutral or better against established benchmarks, or
- a regression is explicitly approved because it buys required safety or correctness and no compatible alternative exists

---

## 2. Migration Guidelines

### 2.1 C-to-Rust Mapping Rules

These rules govern how C constructs and responsibilities must be translated into Rust.

#### 2.1.1 Program structure mapping
- The C `main` remains the top-level orchestration boundary in Rust.
- Distinct lifecycle responsibilities must remain distinct:
  - startup / argument interpretation
  - input and preprocessing
  - terminal initialization
  - terminal main loop
  - terminal teardown
  - final output / yank action
- Internal Rust modules may be reorganized, but the above responsibilities must remain traceable.

#### 2.1.2 Data representation mapping
- C `(const char *, size_t)` pairs map to `&[u8]` by default, not `&str`, unless text semantics are proven and required.
- Null-terminated text behavior maps to `&CStr`, `CString`, or validated Rust string handling only where the null-terminated semantic actually matters.
- C structs map to named Rust structs with documented invariants wherever possible.
- Anonymous or partially known C struct roles must be given descriptive Rust names based on responsibility, not arbitrary placeholder names, once enough evidence exists.
- Nullable pointer results map to `Option<T>` or `Option<&T>` style representations where lifetimes and ownership can accurately preserve semantics.
- Comparator behavior maps to explicit comparison functions or trait implementations only if doing so preserves the exact selection and ordering semantics.

#### 2.1.3 I/O mapping
- `xwrite`-like behavior must remain a low-level, explicit, status-bearing write layer.
- `twrite`-like behavior must preserve counted-buffer terminal output semantics.
- `tputs`-like behavior must preserve convenience string output semantics as distinct from counted writes.
- Partial-write semantics must not be ignored accidentally. Rust wrappers must intentionally model and handle them.
- Output functions must not assume UTF-8 unless that assumption is proven valid.

#### 2.1.4 Terminal lifecycle mapping
- `tsetup`, `tmain`, and `tend` must remain explicit conceptual phases.
- Rust should implement teardown using drop guards or equivalent scope-based restoration where possible.
- The design must guarantee that terminal restoration is attempted on all relevant exit paths.

#### 2.1.5 Input and preprocessing mapping
- `input` and `strtopat` represent explicit preprocessing boundaries and must remain explicit in the Rust architecture.
- Pattern conversion or parsing must not be silently merged into unrelated control flow if that makes behavior or testing less clear.
- If preprocessing caches or precomputes values in Rust, it must preserve user-visible results and ordering semantics.

#### 2.1.6 Error mapping
- C status returns, null checks, and sentinel values may be translated into Rust `Result` and `Option`, but error classification and externally visible behavior must remain compatible.
- Error paths must preserve required cleanup obligations.
- Usage-related exits must remain distinguishable from operational failures if that distinction exists in practice.

---

### 2.2 Principles for Handling Uncertain Behavior

The available behavioral evidence is incomplete. Therefore uncertainty handling is a first-class migration concern.

#### 2.2.1 Conservative interpretation rule
When behavior is uncertain:
1. do not invent a cleaner behavior,
2. do not assume the common-case Unix tool behavior is correct,
3. do not collapse branches or states,
4. preserve the strongest structure visible in the C analysis.

#### 2.2.2 Evidence hierarchy
When deciding what Rust must do, use the following priority order:
1. verified behavior from the original C source and executable tests
2. observed runtime behavior against fixtures and transcripts
3. documented interface and behavior summaries
4. naming and structural inference
5. developer preference

Lower-priority evidence must never override higher-priority evidence.

#### 2.2.3 Unknown behavior procedure
If a behavior is unclear:
- mark it explicitly as unknown
- create a compatibility question or issue
- add a characterization test against the C implementation where possible
- defer refactoring until the behavior is resolved or safely bounded

#### 2.2.4 No accidental semantic upgrades
The following are forbidden unless explicitly approved:
- changing byte-oriented behavior into text-only behavior
- changing comparator-driven outcomes into stable container insertion order
- changing nullable or fallible control flow into infallible success paths
- changing terminal lifecycle obligations into best-effort behavior
- changing output timing or ordering because buffering is more convenient in Rust

#### 2.2.5 Explicit deviation rule
Any intentional deviation from C behavior must include:
- the original behavior
- the evidence for that behavior
- the reason for changing it
- the user-visible impact
- the approval record
- the new tests that lock in the deviation

No undocumented deviation is acceptable.

---

### 2.3 Test Verification Requirements

Testing is the proof mechanism for compatibility.

#### 2.3.1 Baseline requirement
Before or during migration, the project must establish a characterization test suite against the C implementation wherever practical.

#### 2.3.2 Required test categories
The migration must include tests covering, at minimum:
- CLI invocation and usage behavior
- startup and exit behavior
- terminal lifecycle pairing
- interactive input handling
- output behavior for counted buffers
- output behavior for string-oriented helpers
- field comparison and ordering behavior
- preprocessing and pattern conversion behavior
- write failure and cleanup behavior
- null/empty/zero-length edge cases
- success, cancellation, and error exits where applicable

#### 2.3.3 Differential testing requirement
Where feasible, Rust behavior must be tested against the C implementation using the same inputs and expected outputs, including:
- stdout
- stderr
- exit status
- terminal-visible output or transcripts
- ordering/selection results

#### 2.3.4 Edge-case requirement
Tests must include edge conditions implied by the interface:
- empty inputs
- zero-length counted writes
- non-UTF-8 byte content where applicable
- ambiguous or invalid pattern strings
- comparator ties
- terminal interruption or early termination
- write failure simulation

#### 2.3.5 Regression discipline
Every bug found during migration must produce:
- a failing test reproducing the issue
- a fix
- a locked regression test in the Rust suite

---

## 3. Quality Gates

No work is complete until it passes the following gates.

### 3.1 Tests That Must Pass

A change may merge only if all applicable tests pass.

#### 3.1.1 Required passing test suites
- unit tests for local Rust logic
- integration tests for CLI-visible behavior
- characterization tests comparing against C behavior, where available
- regression tests for previously discovered issues
- error-path tests for cleanup and failure handling
- benchmark smoke tests for hot paths

#### 3.1.2 Mandatory compatibility checks
For behavior-affecting changes, reviewers must verify:
- exit codes remain compatible
- stdout/stderr remain compatible
- terminal lifecycle behavior remains paired and restored
- selection/order behavior remains comparator-driven
- counted output remains counted
- string helper behavior remains distinct where required

#### 3.1.3 Failure policy
A merge is blocked if:
- tests are missing for new observable behavior
- an existing characterization test regresses without an approved deviation record
- terminal cleanup behavior is not tested
- a hot-path change lands without benchmark evidence
- unsafe code lacks tests covering its invariants

---

### 3.2 Code Review Standards

Every code review must enforce this constitution, not just local correctness.

#### 3.2.1 Reviewer responsibilities
Reviewers must verify:
- compatibility is preserved or deviations are explicitly approved
- code structure still reflects required behavioral boundaries
- safety improvements do not hide semantic changes
- hot paths are not burdened by unnecessary allocations or conversions
- tests prove the claimed behavior
- comments and names reflect actual semantics, not guesses presented as facts

#### 3.2.2 Required review questions
For each substantial change, reviewers must ask:
1. What original C behavior does this map to?
2. What evidence supports this mapping?
3. What observable behavior could change?
4. How is terminal setup/teardown safety preserved?
5. Are counted bytes still treated as counted bytes?
6. Does this alter comparator or ordering semantics?
7. Is the interactive loop still efficient?
8. What tests prove compatibility?

#### 3.2.3 Unsafe code standard
Unsafe code requires heightened review and must include:
- minimal scope
- invariant documentation
- justification for necessity
- tests that exercise success and failure assumptions
- reviewer sign-off specifically acknowledging the unsafe boundary

#### 3.2.4 Refactoring standard
Pure refactors are allowed only when:
- behavior is proven unchanged
- tests remain intact or improve
- no compatibility boundary is blurred
- performance is not degraded in hot paths

---

### 3.3 Performance Benchmark Requirements

Performance claims must be measured, especially in the interactive and output paths.

#### 3.3.1 Required benchmark focus areas
Benchmarks must cover, as applicable:
- interactive loop throughput or latency
- terminal input read handling
- terminal output write behavior
- low-level counted write behavior
- field comparison throughput in representative workloads
- pattern conversion cost if repeated during operation

#### 3.3.2 Benchmark policy
- Benchmarks must compare Rust changes against the current Rust baseline.
- Early in migration, benchmarks should also compare against the C implementation where practical.
- A benchmark is required for any change touching:
  - `tmain`-equivalent logic
  - `tgetc`-equivalent input paths
  - `twrite` / `tputs` output paths
  - `xwrite`-equivalent low-level writes
  - comparator-intensive logic
  - repeated parsing or pattern conversion

#### 3.3.3 Regression threshold
Any meaningful regression in a hot path must be explained and approved. Unexplained regressions block merge.

“Meaningful” is determined by project benchmarks and workload context, but the default stance is conservative: user-facing latency and repeated-operation throughput matter.

#### 3.3.4 Acceptable tradeoffs
A performance regression may be accepted only if:
- it is necessary for correctness or safety,
- alternatives were considered,
- the regression is documented,
- the impact is bounded and understood,
- no compatible lower-cost design is available

---

## 4. Project-Wide Enforcement

These principles apply to all later documents and work products:
- specifications must derive requirements from this constitution
- plans must sequence work in a way consistent with compatibility and safety
- tasks must include testing and evidence collection where behavior is uncertain
- implementations must preserve the defined behavior boundaries
- reviews must enforce the quality gates before merge

If any later document conflicts with this constitution, this constitution wins unless formally amended.

---

## 5. Amendment Rule

This constitution may be amended only by an explicit project decision that:
- identifies the exact section being changed
- explains why the current rule is insufficient
- assesses compatibility, safety, and performance impact
- updates affected downstream documents
- records the approval

Until such an amendment is made, this document is binding.