# Constitution for the Rust Migration of `yank`

## Purpose

This document defines the non-negotiable project-level principles for migrating the C implementation of `yank` to Rust. It is the governing standard for all later specifications, plans, tasks, code changes, reviews, and release decisions.

Where this constitution conflicts with convenience, speed, stylistic preference, or architectural ambition, this constitution prevails.

---

## Project Context

`yank` is currently a small single-module C program centered in one source file with one primary entry point and a terminal-driven operational flow. Based on the available interface and behavior summaries, the migration must preserve:

- a lifecycle orchestrated by `main`
- a distinct usage/error path
- a dedicated input preparation phase
- explicit terminal setup and teardown phases
- a character/event-driven interactive loop
- a field-centered selection/result model
- layered output helpers
- explicit pattern conversion and field comparison logic

Because the available evidence is partial, the project must preserve demonstrated behavior conservatively and treat undocumented areas as compatibility-sensitive.

---

# 1. Core Principles

## 1.1 Behavioral Equivalence Principle

The Rust implementation must preserve the observable behavior of the C program unless a deviation is explicitly approved and documented.

### Required interpretation

Observable behavior includes, at minimum:

- process entry and exit behavior
- command-line invocation behavior
- usage-path behavior
- terminal setup and teardown behavior
- interactive control flow shape
- input handling shape
- output content and output ordering
- byte-counted write behavior
- selection/result behavior centered on field-like entities
- pattern conversion behavior
- field comparison outcomes
- error signaling visible to users or callers
- edge-case handling for empty, zero-length, or explicit-length data

### Rules

1. The Rust rewrite must not silently redesign the program.
2. The existing staged lifecycle must be preserved:
   - argument/usage handling
   - input preparation
   - terminal setup
   - interactive processing
   - selection/finalization
   - output/yank
   - terminal teardown
3. Functionality may be internally reorganized, but user-visible behavior must remain equivalent.
4. Where exact C behavior is known, Rust must match it.
5. Where exact C behavior is not yet known, Rust must choose the least-assumptive implementation and mark the area as behavior-sensitive.
6. Convenience-driven feature additions are prohibited during migration unless separately approved.
7. Refactors that alter semantics under the guise of “idiomatic Rust” are prohibited.

### Specific implications for `yank`

The migration must preserve these evidenced runtime characteristics:

- `main` remains the top-level orchestrator.
- usage handling remains a distinct path.
- terminal setup and teardown remain explicit lifecycle phases.
- terminal interaction remains character/event-driven.
- the main interactive routine remains conceptually distinct from process entry.
- result selection remains field-centered.
- output remains layered, with low-level explicit-length writes separated from terminal display helpers and final yank emission.
- pattern parsing/conversion remains explicit.
- field comparison remains explicit and semantically preserved.

---

## 1.2 Interface Compatibility First Principle

When behavior and implementation style are in tension, compatibility with the established interface model takes priority.

### Required interpretation

Even though the target language changes from C to Rust, the migration must preserve the program’s external contract and its internal architectural boundaries where those boundaries carry semantic meaning.

### Rules

1. The Rust binary must preserve the same CLI-facing contract unless a change is explicitly approved.
2. Distinct conceptual interfaces from the C code must remain distinct in Rust, even if the Rust implementation uses different types.
3. Rust modules and functions should map back clearly to the original C responsibilities.
4. The migration must not collapse semantically separate layers into one opaque routine if that would obscure behavior verification.
5. Publicly relevant semantics of:
   - input acquisition
   - pattern conversion
   - field comparison
   - low-level writing
   - terminal writing
   - terminal setup/teardown
   - interactive selection
   must remain individually traceable.

### Mapping expectation

The following conceptual C interfaces must remain identifiable in Rust:

- `input`
- `strtopat`
- `fcmp`
- `xwrite`
- `yank`
- `twrite`
- `tputs`
- `tsetup`
- `tend`
- `tgetc`
- `tmain`
- `usage`
- `main`

This does not require identical Rust function names in every case, but it does require a documented and reviewable mapping from each original responsibility to its Rust implementation.

### Compatibility preference order

When choosing among Rust designs:

1. preserve observable behavior
2. preserve interface intent and separation
3. improve safety
4. improve maintainability
5. improve elegance

Elegance never outranks compatibility.

---

## 1.3 Safety First Principle

The Rust implementation must maximize memory safety, resource safety, and failure-path safety without changing required behavior.

### Required interpretation

Rust is being adopted to improve implementation safety, but safety must be achieved in a way that preserves behavior rather than replaces it.

### Rules

1. Safe Rust is the default.
2. `unsafe` is forbidden unless strictly necessary.
3. Any `unsafe` must be:
   - narrowly scoped
   - justified in writing
   - reviewed with line-specific rationale
   - covered by tests that exercise the relevant contract
4. Resource cleanup must be reliable, especially for terminal state restoration.
5. Ownership and lifetime design must make field/result validity explicit.
6. Byte-length handling must be deliberate and correct.
7. Error paths must not leave terminal state corrupted when cleanup is expected.
8. Panics in normal operational paths are prohibited.
9. Recoverable runtime failures must use explicit error handling.
10. Hidden truncation, unchecked indexing, and lossy conversions are prohibited unless behaviorally required and documented.

### Safety priorities specific to `yank`

Given the available behavior evidence, the project must pay special attention to:

- terminal state restoration
- explicit-length buffer handling
- low-level write correctness
- partial-write and interruption semantics where relevant
- null/empty result conditions
- field selection references and ownership
- conversion between raw terminal input and internal Rust representations

### Safety and parity balance

When safety improvements expose ambiguity in the C behavior:

- first verify the C behavior through tests or source inspection
- then preserve that behavior safely in Rust if possible
- if exact preservation requires localized `unsafe` or low-level OS handling, isolate and document it
- do not replace uncertain behavior with a different “safer” behavior without approval

---

## 1.4 Performance Constraint Principle

The Rust migration must not introduce material regressions in responsiveness, throughput, or operational efficiency on the program’s hot paths.

### Required interpretation

The goal is behavior-preserving migration, not performance degradation in exchange for abstraction. Rust code must remain efficient, especially in terminal interaction and output paths.

### Performance-sensitive areas

Based on current evidence, the following are presumed hot or sensitivity-critical paths:

- interactive event/character input (`tgetc`-like behavior)
- the interactive control loop (`tmain`-like behavior)
- low-level output (`xwrite`-like behavior)
- terminal display output (`twrite`/`tputs`-like behavior)
- final yank/output emission
- repeated field comparison (`fcmp`-like behavior)
- potentially repeated pattern conversion (`strtopat`-like behavior)

### Rules

1. The interactive loop must remain responsive.
2. The output path must avoid unnecessary copying, allocation, and formatting overhead.
3. Explicit-length writes must remain efficient and correct.
4. Field comparison must avoid accidental allocation or expensive abstraction in repeated paths.
5. Pattern conversion must not be moved into a more expensive repeated path without evidence that behavior requires it.
6. Rust abstractions are acceptable only if they do not create material regressions.
7. Benchmarkable hot paths must be measured before declaring migration complete.

### Performance decision rule

If a safer or more idiomatic Rust implementation causes measurable regression, the team must prefer the safer implementation only if:

- the regression is insignificant for real use, or
- the change is required to preserve correctness, or
- no lower-regression safe design is available

Otherwise, the implementation must be revised.

---

# 2. Migration Guidelines

## 2.1 C-to-Rust Mapping Rules

The migration must preserve conceptual structure while adopting Rust ownership and type safety.

### Structural mapping rules

1. Each meaningful C responsibility must map to a named Rust responsibility.
2. The single-file C layout may become multiple Rust modules, but decomposition must reflect original behavior boundaries.
3. The Rust crate layout must remain understandable in terms of the original module inventory.
4. Since the project currently consists of one main module, premature over-modularization is discouraged.
5. The Rust entry point must remain centered on a `main`-orchestrated lifecycle.

### Function mapping rules

The following C concepts should generally map as follows:

- `main` -> Rust `main` plus minimal orchestration helpers
- `usage` -> dedicated usage/help/error-path function
- `input` -> explicit input acquisition/preparation function or module
- `strtopat` -> explicit pattern parsing/normalization function or type constructor
- `fcmp` -> dedicated comparator or ordering implementation with documented semantic equivalence
- `xwrite` -> low-level write helper preserving explicit-length semantics
- `yank` -> higher-level final output/emission function
- `twrite` / `tputs` -> terminal-output helpers
- `tsetup` / `tend` -> explicit terminal session setup/restore layer
- `tgetc` -> terminal input event reader
- `tmain` -> interactive controller returning a field-oriented result

### Type mapping rules

1. C pointers must not be mechanically translated; they must be re-expressed using ownership, borrowing, slices, enums, and structs.
2. C string-plus-length APIs should generally map to `&[u8]`, `&str`, or explicit owned buffers depending on actual semantics.
3. If content is semantically byte-oriented rather than text-oriented, prefer byte slices over `String`.
4. Nullability in C must map to `Option` in Rust unless stronger invariants are proven.
5. Integer conversions must be explicit and checked.
6. C struct semantics must be preserved, but anonymous or implicit C layouts may become named Rust structs if that improves clarity without changing behavior.
7. Comparison semantics from `fcmp` must be implemented in a way that preserves ordering rules exactly; if Rust `Ord` is used, its total ordering must match the C comparator’s effective behavior.
8. Pattern representation derived from `strtopat` must remain explicit and reviewable.

### I/O mapping rules

1. Explicit-length writing semantics are mandatory.
2. Rust must not accidentally reinterpret byte data as UTF-8 text unless the C behavior proves the data is textual.
3. Terminal output helpers must remain distinct from final yank/output behavior.
4. Low-level write wrappers must make partial-write and OS error behavior explicit.
5. Buffered versus unbuffered I/O choices must be justified in terms of parity and performance.

### Terminal handling rules

1. Terminal setup and restore must be explicit, paired, and testable.
2. Cleanup must use RAII where possible, provided behavior remains compatible.
3. Terminal state must not be left modified on normal exit; error-exit expectations must be tested and documented.
4. Event decoding logic must preserve the original interaction model as closely as evidence supports.

---

## 2.2 Principles for Handling Uncertain Behavior

The available behavior evidence is incomplete. Therefore, uncertainty must be managed systematically, not by guesswork.

### Uncertainty policy

1. Unknown behavior is not permission to redesign.
2. If behavior is unclear, the team must first attempt to resolve it through:
   - source inspection
   - direct C build and execution
   - characterization tests
   - input/output observation
   - terminal interaction tracing where practical
3. If uncertainty remains, choose the interpretation least likely to break compatibility.
4. Every unresolved behavior assumption must be recorded in migration notes or code comments adjacent to the relevant implementation.
5. Temporary assumptions must be treated as open compatibility risks until verified.

### Evidence hierarchy

When determining intended behavior, use this order of authority:

1. direct behavior of the existing C program
2. C source code semantics
3. existing interface and behavior analysis documents
4. regression tests derived from C behavior
5. project-approved migration notes
6. inferred design preference

Inference alone is weakest and must never override measured behavior.

### Rules for ambiguous cases

#### Ambiguous control flow
If it is unclear when usage, cleanup, or terminal restoration occurs:
- write characterization tests against the C version
- preserve the tested sequence in Rust

#### Ambiguous data semantics
If it is unclear whether data is text or bytes:
- default to byte-preserving handling
- only elevate to text semantics where evidence supports it

#### Ambiguous null/no-result behavior
If it is unclear whether no-selection can occur:
- represent the case explicitly in Rust using `Option`
- test the C behavior before deciding whether the case is reachable

#### Ambiguous comparison behavior
If `fcmp` edge behavior is unclear:
- test equality, ordering, and tie cases against the C implementation
- do not substitute a “natural” Rust ordering until verified

#### Ambiguous pattern behavior
If `strtopat` semantics are unclear:
- preserve transformation as a dedicated step
- characterize accepted and rejected inputs before optimization

### Prohibited responses to uncertainty

The following are prohibited:

- assuming undocumented behavior “does not matter”
- replacing difficult-to-model behavior with a simpler Rust-native alternative
- deleting behavior boundaries because the C code is small
- introducing user-visible changes without an explicit compatibility decision
- converting uncertain low-level behavior into panics

---

## 2.3 Test Verification Requirements

Tests are the primary enforcement tool for behavioral equivalence.

### Required test strategy

The migration must use layered verification:

1. characterization tests against the C implementation
2. equivalent regression tests against the Rust implementation
3. unit tests for isolated logic
4. integration tests for end-to-end behavior
5. targeted tests for error and boundary conditions
6. performance checks on sensitive paths

### Characterization testing requirements

Before declaring any migrated area complete, the team must create tests or captured observations for the corresponding C behavior where feasible.

At minimum, characterization must cover:

- CLI usage path behavior
- normal startup and exit behavior
- output for representative successful flows
- zero-length or empty-input relevant cases
- explicit-length write semantics where observable
- terminal lifecycle behavior to the extent testable
- comparison-sensitive behavior if field ordering matters
- pattern conversion acceptance/rejection cases if externally visible

### Rust equivalence testing requirements

For each characterized behavior, the Rust implementation must have a corresponding test proving equivalent output, exit behavior, or state transition effect.

Where exact terminal interaction is hard to test end-to-end, the project must test:
- terminal state wrapper behavior
- input decoding behavior
- interactive loop decisions at the function level
- cleanup guarantees under normal and failure conditions

### Test design rules

1. Tests must prefer black-box verification over implementation-detail assertions.
2. Golden outputs are acceptable when derived from the C implementation.
3. Snapshot-style tests must be reviewed for signal quality and stability.
4. Boundary tests must include empty and minimal cases.
5. Error-path tests are mandatory for low-level writing and terminal setup where practical.
6. Tests must not encode speculative behavior as truth without evidence.
7. If a behavior is unverified and assumed, tests must state that assumption explicitly.

### Minimum coverage expectation

Code coverage percentage alone is not sufficient, but the project must achieve coverage of all critical behavior zones:

- entry/usage path
- input preparation
- pattern conversion
- field comparison
- low-level write path
- terminal output helpers
- terminal setup/teardown
- interactive control logic
- final output/yank path

---

# 3. Quality Gates

No migration phase is complete until all applicable quality gates in this section are satisfied.

## 3.1 Tests That Must Pass

The following test classes are mandatory.

### A. Build and static validation

The Rust project must pass:

- `cargo build`
- `cargo test`
- `cargo fmt --check`
- `cargo clippy -- -D warnings`

If a Makefile remains part of the project workflow, it must also successfully build and run the Rust target through the project-standard command path.

### B. Behavioral regression tests

The Rust implementation must pass all approved regression tests derived from C behavior, including:

- usage-path tests
- representative successful execution tests
- output equivalence tests
- boundary-condition tests
- error-path tests where behavior is defined
- comparison and pattern-conversion tests where behavior is relevant

### C. Integration tests

The project must include end-to-end tests covering the expected lifecycle shape:

- invocation
- input preparation
- terminal-related setup path where testable
- main operational path
- final output path
- cleanup/exit path

### D. Terminal safety tests

Where terminal behavior exists, the project must verify as far as practical:

- setup occurs before interactive behavior
- teardown/restore occurs after interactive behavior
- restore happens on normal completion
- restore behavior on failure paths is tested or explicitly documented if not automatable

### E. Negative-path tests

The project must include tests for:

- invalid invocation
- empty or no-result conditions where possible
- write failures or simulated write interruptions where practical
- invalid or edge-case pattern input where relevant

### Gate rule

A migration change fails the quality gate if it:
- removes tests for a covered behavior
- changes expected behavior without an approved compatibility note
- leaves a known critical path untested

---

## 3.2 Code Review Standards

All migration code must be reviewed against this constitution, not just for syntax or style.

### Mandatory review checks

Every code review must confirm:

1. **Behavioral parity**
   - Does the change preserve known C behavior?
   - Is any deviation explicitly documented and approved?

2. **Interface traceability**
   - Can the Rust code be mapped back to the original C responsibility?
   - Are semantic layers still visible and understandable?

3. **Safety**
   - Is the code predominantly safe Rust?
   - Is every `unsafe` block justified, minimal, and tested?
   - Are ownership and lifetime choices clear?

4. **Error handling**
   - Are failures explicit rather than panic-driven?
   - Are cleanup guarantees preserved?

5. **Performance**
   - Does the change affect a hot path?
   - Has allocation, copying, or abstraction overhead been considered?

6. **Testing**
   - Are there sufficient tests for the changed behavior?
   - Were characterization expectations updated if new C behavior was verified?

7. **Simplicity**
   - Is the design no more complex than necessary for parity, safety, and performance?

### Review rejection criteria

A change must be rejected if it:

- introduces user-visible behavior changes without approval
- collapses important behavior boundaries without justification
- replaces explicit byte semantics with unjustified text semantics
- introduces panic-based control flow in operational paths
- adds undocumented assumptions about unclear C behavior
- uses `unsafe` without necessity and justification
- degrades hot-path performance without evidence it is acceptable
- weakens cleanup reliability for terminal state or output handling

### Documentation expectations in reviews

Reviewable changes must include, where applicable:

- mapping notes from C responsibility to Rust implementation
- identified behavior assumptions
- rationale for important type choices
- explanation of any low-level I/O or terminal handling approach
- benchmark notes for performance-sensitive changes
- test evidence

---

## 3.3 Performance Benchmark Requirements

Performance must be validated, not assumed.

### Required benchmark scope

Benchmarking or equivalent measurement must cover, as applicable:

- interactive input/event processing
- repeated display/output operations
- low-level explicit-length write behavior
- final yank/output path
- repeated comparison behavior
- repeated pattern conversion behavior if used dynamically

### Benchmark rules

1. Benchmarks must compare meaningful scenarios, not synthetic trivia only.
2. Benchmark inputs should reflect realistic operational sizes and repetitions where possible.
3. At least one benchmark or timed regression check must exercise the output path.
4. At least one benchmark or responsiveness check must exercise the interactive/event-processing path if practicable.
5. Performance-sensitive code changes require before/after evidence when they alter:
   - allocation behavior
   - data representation
   - write path logic
   - comparator implementation
   - terminal loop structure

### Acceptance standard

The Rust implementation must show no material regression relative to the C implementation or to prior Rust baselines for the same behavior, unless:

- the regression is documented,
- the cause is understood,
- the tradeoff is approved,
- and no better parity-preserving alternative is available

### Performance failure conditions

A change fails this gate if it:

- introduces repeated unnecessary allocations in a hot path
- adds avoidable buffering/copying to explicit-length write flows
- materially harms interactive responsiveness
- replaces an efficient comparator or parser with a significantly slower unchecked abstraction
- makes terminal output or final output observably slower without justification

---

# Enforcement

This constitution is binding on all future migration documents and implementation work.

## Enforcement rules

1. Specifications must cite and conform to these principles.
2. Plans must include work needed to satisfy these quality gates.
3. Tasks must not direct work that violates these principles.
4. Code reviews must use this document as acceptance criteria.
5. If a later document conflicts with this constitution, this constitution wins.
6. Any exception must be explicit, narrowly scoped, justified in writing, and approved before merge.

---

# Amendment Policy

This constitution may be amended only when the team determines that a principle is preventing correct migration, safe implementation, or verified compatibility.

Amendments must:

- be written explicitly
- state the reason for change
- identify affected sections
- describe impact on existing plans and tasks
- be approved before dependent implementation proceeds

Amendments must not be used to retroactively excuse unreviewed deviations.

---

# Non-Negotiable Summary

The Rust migration of `yank` must:

- preserve observable behavior first
- preserve interface intent and layer boundaries
- improve safety without changing semantics
- maintain performance on sensitive paths
- treat uncertainty conservatively
- verify behavior through characterization and regression tests
- enforce completion through hard quality gates