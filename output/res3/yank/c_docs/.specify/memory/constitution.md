# Constitution for the Rust Migration of `yank`

## Purpose

This document defines the binding project-level principles for the Rust migration of the C project `yank`. It is the governing standard for all later specifications, plans, tasks, implementation decisions, reviews, and acceptance criteria.

Where the original C behavior is directly evidenced, the Rust migration must preserve it. Where the original behavior is not fully evidenced, the migration must proceed conservatively, make uncertainty explicit, and avoid accidental semantic invention.

---

## Scope

This constitution applies to the full migration of the current `yank` codebase, including:

- build and packaging integration,
- program entry and process behavior,
- terminal setup, interaction, and teardown,
- data ingestion and preparation,
- field comparison and selection logic,
- counted-byte output behavior,
- error handling and cleanup,
- tests, benchmarks, and review practices.

Project facts informing this constitution:

- Build system: `Makefile`
- C implementation units: 1
- Header files: 0
- Primary module cluster: 1
- Main module: `main_root`
- Known representative functions: `input`, `strtopat`, `fcmp`, `xwrite`, `yank`, `twrite`, `tputs`, `tsetup`
- Known lifecycle functions also evidenced in behavior summary: `main`, `usage`, `tend`, `tgetc`, `tmain`

---

# 1. Core Principles

## 1.1 Behavioral Equivalence Principle

The Rust implementation must preserve the observable behavior of the C program as the primary correctness target.

### Required interpretation
Observable behavior includes, at minimum:

- process entry through `main`,
- command-line usage path behavior,
- startup/preparation/interaction/teardown phase separation,
- terminal interaction model,
- field-oriented selection/result behavior,
- counted-byte output semantics,
- cleanup expectations after terminal activation,
- exit status and user-visible output where determinable.

### Binding rules
1. The Rust rewrite must reproduce the same externally visible behavior before it attempts architectural improvement.
2. Refactoring is allowed only when behavior remains equivalent.
3. Missing knowledge about internals is not permission to simplify semantics.
4. If the C version distinguishes phases such as `input`, `tsetup`, `tmain`, `tend`, and `yank`, the Rust version must preserve those behavioral boundaries even if internal structure changes.
5. Explicit byte-counted operations must remain byte-counted operations; they must not be weakened into null-terminated-only assumptions.
6. Dedicated paths, such as `usage` and terminal cleanup, must remain dedicated and intentional.

### For this project specifically
The migration must preserve the evidenced lifecycle:

1. `main` as orchestrator,
2. invocation/usage handling,
3. data preparation,
4. terminal setup,
5. terminal-driven interaction,
6. field-result resolution,
7. final yank/output,
8. terminal teardown,
9. exit.

If exact ordering is not fully proven by current evidence, the rewrite must preserve all known ordering constraints and avoid introducing contradictory behavior.

---

## 1.2 Interface Compatibility First Principle

The migration must preserve the effective interface contract before pursuing internal redesign.

### Required interpretation
Even though the current project is small and largely internal, its effective interface includes:

- CLI invocation shape,
- process startup and termination behavior,
- terminal-facing behavior,
- output destinations and byte-count semantics where evidenced,
- conceptual function boundaries that encode behavior contracts,
- data/result interpretation centered on field selection.

### Binding rules
1. User-facing behavior is the first interface and must remain stable unless a change is explicitly approved as non-equivalent.
2. Internal Rust APIs should map closely to the C program’s functional responsibilities during initial migration.
3. The migration must not collapse meaningful abstraction boundaries prematurely.
4. The semantics represented by `strtopat`, `fcmp`, `xwrite`, `tsetup`, `tmain`, `tend`, `twrite`, `tputs`, and `yank` must remain identifiable in the Rust design, even if names or module layout evolve.
5. The result of terminal interaction must remain field-oriented, not replaced with an unrelated or less expressive result shape without proof of equivalence.
6. Counted-byte output must remain explicitly modeled in Rust APIs.

### Preferred migration shape
The first Rust implementation should preserve a recognizable mapping from the C responsibilities into Rust modules and functions. Idiomatic Rust is desirable, but compatibility outweighs elegance during migration.

---

## 1.3 Safety First Principle

Rust safety is a project goal, but safety improvements must preserve behavior and operational guarantees.

### Required interpretation
Safety includes:

- memory safety,
- resource safety,
- terminal state restoration,
- explicit handling of fallible operations,
- avoidance of undefined behavior analogues,
- clear ownership of buffers and terminal/session resources.

### Binding rules
1. Safe Rust is the default.
2. `unsafe` is prohibited unless it is strictly necessary and justified in writing.
3. Any `unsafe` usage must:
   - have a minimal scope,
   - document invariants,
   - include tests covering the safety boundary,
   - be reviewed with special scrutiny.
4. Cleanup-critical resources, especially terminal state, must be managed with RAII or equivalent structured cleanup mechanisms whenever possible.
5. Error handling must be explicit; silent failure paths are not acceptable unless proven equivalent to the C behavior and documented.
6. The migration must remove accidental C risks, but not by changing intended behavior.
7. If the C implementation may rely on global mutable state, the Rust migration must constrain and document it rather than reproduce it carelessly.

### Project-specific safety priorities
Because `yank` appears terminal-oriented, terminal restoration is a safety concern, not merely a style concern. Entering terminal-active state must create a robust path to restoration on both success and failure wherever feasible.

---

## 1.4 Performance Constraint Principle

The Rust migration must not introduce unreasonable regressions in latency, throughput, or interaction responsiveness.

### Required interpretation
Performance constraints are especially important for the likely hot paths:

- terminal interaction loop (`tmain`, `tgetc`, `twrite`, `tputs`),
- low-level output (`xwrite`, `yank`),
- repeated comparisons (`fcmp`),
- potentially full-data ingestion (`input`).

### Binding rules
1. The first acceptable Rust version must be behaviorally correct and operationally responsive.
2. Rust abstractions must not add obvious avoidable overhead in hot paths.
3. Extra allocations, conversions, copying, or string normalization in repeated terminal paths are prohibited unless justified.
4. Counted-byte output paths must remain efficient and avoid unnecessary re-buffering.
5. Comparison logic must preserve semantics and remain suitable for repeated invocation.
6. Performance claims must be supported by measurement, not assumption.

### Acceptance posture
Minor implementation-level variance is acceptable. Noticeable user-facing slowdown, especially during interactive terminal use, is not.

---

# 2. Migration Guidelines

## 2.1 C-to-Rust Mapping Rules

The migration should preserve conceptual structure while translating into idiomatic, safe Rust.

### 2.1.1 Function responsibility mapping
Each meaningful C function responsibility must have a direct Rust equivalent during the initial migration.

Recommended responsibility-preserving mapping:

- `main` -> Rust `main` plus minimal orchestration helpers
- `usage` -> dedicated usage/help/error-reporting path
- `input` -> dedicated ingestion/preparation function or module
- `strtopat` -> explicit pattern transformation function
- `fcmp` -> explicit comparison function or `Ord`/`PartialOrd` implementation only if semantics are preserved exactly
- `tsetup` / `tend` -> terminal session guard or setup/teardown pair
- `tgetc` -> terminal input abstraction
- `twrite` / `tputs` -> terminal output abstraction
- `tmain` -> dedicated interaction controller
- `xwrite` -> centralized low-level counted-byte write helper
- `yank` -> higher-level emission/export helper using the centralized write path

### 2.1.2 State mapping
If the C implementation uses module-global state, the Rust implementation should migrate it into explicit state structures.

Rules:
1. Prefer structs over globals.
2. Prefer immutable data flow unless mutability is required by behavior.
3. Shared mutable state must be minimized and isolated.
4. State transitions implied by lifecycle phases should be representable in type structure or module boundaries when practical.

### 2.1.3 Struct mapping
Known C structs, including anonymous internal structures, must be translated into named Rust structs or enums with semantics preserved.

Rules:
1. Do not erase domain structure into generic tuples or maps.
2. Preserve field meaning and comparison semantics.
3. Use enums for mode/state distinctions when this clarifies behavior without changing it.
4. Represent absence explicitly with `Option` when pointer-null semantics are intended.

### 2.1.4 Pointer and buffer mapping
C pointers and raw memory behaviors must be translated conservatively.

Rules:
1. `const char *` is not automatically a Rust `String`; choose among `&str`, `&[u8]`, `Vec<u8>`, `OsString`, or owned byte buffers based on actual semantics.
2. Any path using explicit `size_t nmemb` must be modeled as counted bytes, typically `&[u8]` or equivalent.
3. Null-terminated assumptions may only be used where they are proven to be part of the original behavior.
4. Borrowed versus owned data must be chosen to preserve lifetime and mutation semantics, not convenience.

### 2.1.5 Error mapping
C implicit failure conventions must be made explicit in Rust.

Rules:
1. Use `Result` for fallible operations.
2. Use `Option` only for genuine presence/absence semantics, such as “no selected field” if that is what the C behavior represents.
3. Do not collapse usage errors, runtime I/O failures, and user cancellation into one undifferentiated error class unless the original behavior truly does so.
4. Preserve cleanup obligations on all failure paths.

### 2.1.6 I/O mapping
The low-level output model must remain centralized.

Rules:
1. All final byte emission equivalent to C `xwrite` must flow through one Rust helper or abstraction.
2. All terminal writes should flow through dedicated terminal output helpers.
3. Terminal input must flow through a dedicated abstraction equivalent to `tgetc`.
4. Direct ad hoc writes from unrelated code are prohibited unless specifically justified.

### 2.1.7 Build system mapping
The Rust migration must integrate with the existing `Makefile`-based workflow unless and until an explicit build-system change is approved.

Rules:
1. The project must remain buildable from the project’s standard top-level command path.
2. New tooling may be added, but not in a way that breaks existing automation expectations.
3. Rust build invocations should be wrapped or integrated cleanly with Make targets.

---

## 2.2 Principles for Handling Uncertain Behavior

The available behavioral evidence is incomplete. Therefore, uncertainty handling is a first-class migration discipline.

### 2.2.1 Conservative interpretation rule
When behavior is not fully known, choose the interpretation least likely to alter observable semantics.

Examples:
- preserve explicit lifecycle separation,
- preserve counted-byte handling,
- preserve centralized low-level output,
- preserve field-oriented result modeling,
- preserve explicit terminal setup/teardown.

### 2.2.2 No invention rule
Do not invent new semantics merely because Rust makes them convenient.

Prohibited examples:
- replacing counted bytes with plain UTF-8 strings everywhere,
- replacing field comparison semantics with arbitrary derived ordering,
- replacing explicit teardown with “best effort” informal cleanup,
- collapsing distinct terminal functions into hidden side effects with no testable boundary.

### 2.2.3 Evidence hierarchy
Implementation decisions must follow this priority order:

1. direct evidence from C source behavior,
2. documented interface/behavior summaries,
3. tests against the C binary,
4. conservative inference from function names/signatures,
5. explicit project decision records for unresolved ambiguity.

Lower-priority inference may not override higher-priority evidence.

### 2.2.4 Uncertainty recording requirement
Any unresolved behavior question must be documented in a migration note, issue, or ADR with:

- what is known,
- what is unknown,
- chosen temporary interpretation,
- risk of divergence,
- how it will be verified later.

### 2.2.5 Compatibility over idealization
When an idiomatic Rust redesign conflicts with possible original behavior, preserve compatibility first and defer redesign.

### 2.2.6 Terminal uncertainty rule
Because terminal behavior is easy to regress and hard to infer, any uncertain terminal interaction behavior must be validated empirically where possible against the C version.

### 2.2.7 Output uncertainty rule
For `yank` and `xwrite` equivalents, uncertainty about partial writes, zero-length writes, destination handling, or byte preservation must be resolved by tests before changing semantics.

---

## 2.3 Test Verification Requirements

Testing is mandatory evidence for behavioral equivalence.

## 2.3.1 General requirement
The Rust migration is not complete because it compiles. It is complete only when evidence shows it behaves equivalently enough for the migration stage.

## 2.3.2 Test categories
The project must maintain the following categories of tests where applicable:

1. **Build tests**
   - clean build from standard project workflow,
   - repeatable build in CI.

2. **CLI behavior tests**
   - invocation success and failure cases,
   - usage path behavior,
   - exit status verification where determinable.

3. **Golden output tests**
   - compare Rust output against C output for the same inputs,
   - especially for final yank/output behavior.

4. **Byte-semantics tests**
   - explicit-length writes,
   - zero-length behavior,
   - non-text or embedded-null behavior if applicable.

5. **Terminal interaction tests**
   - setup/teardown behavior,
   - key-input handling where automatable,
   - non-corruption of terminal state after completion/failure.

6. **State/result tests**
   - `tmain`-equivalent result resolution behavior,
   - no-selection / empty-data / canceled-interaction cases if supported by observed behavior.

7. **Comparison tests**
   - `fcmp` equivalence cases,
   - equal, less-than, greater-than, and edge-case comparisons.

8. **Pattern transformation tests**
   - `strtopat` input/output equivalence for known cases.

9. **Regression tests**
   - every bug discovered during migration must add a test.

## 2.3.3 Oracle requirement
Whenever feasible, the original C implementation should act as the behavioral oracle for test generation.

Preferred practice:
- run the same test fixture against C and Rust binaries,
- compare exit code,
- compare stdout/stderr,
- compare emitted bytes,
- compare behavior under terminal-like scenarios where practical.

## 2.3.4 Approval threshold
A migration stage may proceed only when tests cover all currently understood critical behaviors of the migrated area.

---

# 3. Quality Gates

## 3.1 Tests That Must Pass

No implementation may be merged unless all applicable gates below pass.

### 3.1.1 Mandatory baseline gates
1. Rust project builds successfully through the project-standard workflow.
2. All unit tests pass.
3. All integration tests pass.
4. All golden comparison tests against the C behavior pass.
5. All linting and formatting checks pass.
6. No unexplained ignored or flaky tests are introduced.

### 3.1.2 Behavior-critical gates
The following must be explicitly tested and passing before declaring migration-complete:

- `main`-level invocation behavior,
- dedicated `usage` path behavior,
- explicit terminal setup/use/teardown lifecycle,
- field-oriented selection/result behavior,
- `strtopat` transformation behavior for known cases,
- `fcmp` behavior for known cases,
- centralized low-level counted-byte output behavior,
- final yank/output behavior.

### 3.1.3 Cleanup and failure-path gates
Where behavior can be exercised, tests must verify:

- terminal restoration after normal exit,
- terminal restoration after handled failure,
- no double-output or truncated-output regressions in write paths,
- no panic on expected user-facing error conditions.

### 3.1.4 Cross-check gates
Before final acceptance of the migration, there must be at least one end-to-end comparison suite that runs both implementations against the same scenarios and reports divergences.

---

## 3.2 Code Review Standards

Every change must be reviewed against this constitution, not only for code style.

### 3.2.1 Minimum review criteria
Reviewers must verify:

1. behavior preservation is argued and evidenced,
2. uncertainty is documented where present,
3. tests are adequate for the changed behavior,
4. terminal lifecycle safety is preserved,
5. byte-counted output semantics are preserved,
6. no unnecessary semantic drift was introduced,
7. performance-sensitive paths were considered,
8. error handling is explicit and cleanup-safe.

### 3.2.2 Review questions
Each substantive migration PR should answer:

- What C behavior is being preserved?
- What evidence supports the mapping?
- What behavior remains uncertain?
- What tests prove equivalence?
- Does this change alter terminal behavior?
- Does this change alter counted-byte semantics?
- Does this change add allocations or overhead in hot paths?
- Is any `unsafe` used, and if so, why is it necessary?

### 3.2.3 `unsafe` review standard
Any `unsafe` block requires:
- explicit justification in code comments,
- reviewer acknowledgement of invariants,
- targeted tests,
- proof that a safe alternative is not practical.

### 3.2.4 Refactor review standard
Refactors that claim “no behavior change” must either:
- preserve existing tests unchanged, or
- add tests showing preserved behavior if prior coverage was insufficient.

### 3.2.5 Documentation review standard
If a change resolves or narrows uncertainty, the corresponding migration notes, ADRs, or behavior documents must be updated in the same change.

---

## 3.3 Performance Benchmark Requirements

Performance must be measured for behaviorally significant paths.

### 3.3.1 Required benchmark targets
Benchmarks should cover, where feasible:

- ingestion/preparation path corresponding to `input`,
- comparison-heavy behavior corresponding to `fcmp`,
- terminal redraw/input loop behavior corresponding to `tmain`/`twrite`/`tputs`/`tgetc`,
- final output path corresponding to `yank`/`xwrite`.

### 3.3.2 Comparison standard
Performance evaluation should compare Rust against the C implementation or, when direct comparison is impractical, against an agreed baseline from earlier Rust revisions.

### 3.3.3 Regression thresholds
The project must not accept:
- clear interactive responsiveness regressions,
- materially worse write-path efficiency without strong justification,
- obvious allocation-heavy behavior in hot paths,
- algorithmic regressions in repeated comparison or ingestion paths.

If a performance regression is accepted temporarily, it must be:
- measured,
- documented,
- justified,
- tracked for remediation.

### 3.3.4 Benchmark evidence
Performance-sensitive PRs must include one or more of:
- benchmark results,
- profiling output,
- allocation measurements,
- reasoned analysis tied to specific hot paths.

### 3.3.5 Optimization discipline
Performance optimizations are allowed only if they do not violate safety or behavioral equivalence. Cleverness without measurement is discouraged.

---

# Amendment and Precedence Rules

## Amendment rule
This constitution may be revised only by explicit project decision. Any amendment must be written, reviewed, and justified.

## Precedence rule
If any later project document conflicts with this constitution, this constitution prevails.

## Interpretation rule
When a later spec, plan, or task is ambiguous, interpret it in the way most consistent with:

1. behavioral equivalence,
2. interface compatibility,
3. safety,
4. performance discipline,
5. explicit handling of uncertainty.

---

# Non-Negotiable Project Laws

1. Preserve observable behavior first.
2. Preserve effective interfaces before redesigning internals.
3. Prefer safe Rust; tightly justify any `unsafe`.
4. Keep terminal setup and teardown explicit and reliable.
5. Preserve counted-byte output semantics.
6. Keep low-level write behavior centralized.
7. Preserve explicit pattern transformation and field comparison semantics.
8. Treat uncertainty as a tracked engineering problem, not a license to guess.
9. Require tests as evidence, not decoration.
10. Do not merge changes that violate this constitution.