# constitution.md

## Purpose

This document defines the non-negotiable project-level principles for the Rust migration of `pwd`. It is the governing standard for all later specification, planning, implementation, testing, and review work.

When trade-offs arise, this document takes precedence over convenience, stylistic preference, and local optimization. Any exception must be explicit, justified in writing, narrowly scoped, and approved at project review level.

---

## Scope

This constitution applies to the full migration of the C `pwd` project into Rust, including:

- command behavior,
- module boundaries,
- public and internal interfaces,
- error handling,
- memory and resource management,
- locale and quoting behavior,
- stream finalization behavior,
- build and test integration through the existing `Makefile`,
- performance-sensitive paths.

It applies especially to the observed behavior split among:

- help/version flows,
- logical current-directory resolution,
- robust physical directory reconstruction,
- quoting subsystem behavior,
- allocation and failure behavior,
- output finalization behavior.

---

## 1. Core Principles

### 1.1 Behavioral Equivalence Principle

The Rust implementation must preserve the observable behavior of the C program unless a deviation is both:

1. required for correctness or safety, and
2. documented and approved.

Observable behavior includes, at minimum:

- command-line mode selection,
- output text content and structure,
- exit status behavior,
- error reporting behavior,
- path resolution semantics,
- root detection behavior,
- quoting behavior,
- locale-sensitive behavior,
- stream flush/close behavior,
- allocation-failure policy where behavior is user-visible.

#### Required interpretations

- The Rust version must preserve the separation between logical path generation and robust physical reconstruction.
- The Rust version must preserve the reverse-construction traversal model of robust path assembly where behavior depends on climbing parents, identifying entries, prepending components, and stopping at root identity.
- The Rust version must preserve the distinction between fixed-buffer, allocated-result, and reusable-slot quoting models where these distinctions are externally or semantically observable.
- The Rust version must preserve explicit-size data handling and must not silently collapse all APIs into NUL-terminated string semantics.
- The Rust version must preserve late output verification behavior rather than treating the last write as sufficient completion.

#### Rules

- “Equivalent” means equivalent to user-observable semantics, not source-level resemblance.
- If the C implementation exposes multiple behavior paths, the Rust version must not collapse them into one path unless equivalence is proven by tests and design review.
- If exact behavior is uncertain, the project must prefer evidence collection over assumption.
- No document may redefine behavior merely because Rust offers a simpler abstraction.

---

### 1.2 Interface Compatibility First Principle

Migration must preserve interface intent before pursuing abstraction cleanup.

This project contains many helper modules and subsystem entry points, especially around quoting, locale handling, allocation, flushing, and cwd reconstruction. The Rust rewrite must first represent these interfaces faithfully enough to support behavior-preserving migration and verification.

#### Rules

- Existing module responsibilities must remain recognizable in Rust.
- Public and cross-module entry points must not be merged prematurely.
- Function families with differentiated semantics must remain differentiated in Rust, even when implemented atop shared internals.
- Internal APIs may become more idiomatic only after compatibility is established and verified.
- Wrapper functions that appear redundant in C must still be preserved if they encode distinct ownership, error, or behavior contracts.

#### Specific project implications

The Rust design must preserve clear counterparts for:

- `logical_getcwd` versus `robust_getcwd`,
- `file_name_init` / `file_name_prepend` / `file_name_free`,
- quoting configuration mutation APIs,
- quoting rendering APIs with distinct result models,
- `close_stdout` and lower-level stream finalization layers,
- shared allocation helpers and shared fatal allocation policy,
- locale and charset query helpers.

#### Prohibited shortcuts

- Do not replace broad interface families with a single “do everything” helper.
- Do not eliminate slot-based quoting semantics solely because Rust ownership makes fresh allocation easier.
- Do not erase explicit-size interfaces by converting everything to `String`.
- Do not bypass output finalization wrappers with direct “println and return” behavior.

---

### 1.3 Safety First Principle

Rust safety is a project requirement, not an optional enhancement. However, safety improvements must preserve behavior unless deviation is approved.

The migration must prefer safe Rust by default and use `unsafe` only when required by system interaction, ABI boundaries, or demonstrable low-level necessity.

#### Rules

- Safe Rust is the default.
- Every `unsafe` block must be:
  - minimal,
  - documented with a safety justification,
  - covered by tests exercising the relevant path.
- No undefined behavior from the C source may be carried forward intentionally.
- Resource lifetime correctness is mandatory for:
  - file descriptors,
  - directory handles,
  - buffers,
  - locale-related temporary state,
  - output streams,
  - reusable quoting storage.
- Pointer-oriented C APIs must be translated into Rust ownership and borrowing models that make invalid states harder to express.

#### Safety priorities for this project

Highest care is required in areas corresponding to:

- cwd reconstruction across directory traversal,
- metadata comparisons for root detection,
- explicit-size buffer handling,
- multibyte and locale-sensitive conversion,
- stream flush/close interactions,
- any emulation of C stdio state repair behavior,
- cached or reusable quoting result storage.

#### Unsafe policy

`unsafe` is allowed only when one of the following is true:

- direct libc or OS interaction requires it,
- compatibility with required C behavior cannot be achieved otherwise,
- performance-critical implementation requires it and the safe alternative is demonstrably inadequate.

Even then:

- the safe interface boundary must remain as high-level as possible,
- invariants must be stated in comments,
- tests must target failure and edge paths,
- reviewers must verify that the `unsafe` scope is the smallest practical scope.

---

### 1.4 Performance Constraint Principle

The Rust rewrite must not introduce material regressions in performance, allocation behavior, or scaling characteristics on the program’s important paths.

This is a migration, not a functional rewrite that may ignore runtime cost.

#### Performance-sensitive areas

The project must treat the following as priority paths:

- robust current-directory reconstruction,
- repeated parent traversal and entry matching,
- path accumulator growth,
- quoting core formatting,
- reusable quoting-slot behavior,
- buffer-based formatting paths,
- output finalization logic,
- locale-sensitive text conversion where used.

#### Rules

- Preserve asymptotic behavior of the C implementation wherever observable.
- Avoid unnecessary heap allocation on hot paths.
- Avoid replacing reusable storage with always-fresh allocation unless benchmarked and approved.
- Avoid converting byte-oriented logic into Unicode-heavy logic unless behavior requires it.
- Avoid hidden copies when preserving explicit-size APIs.
- Preserve fast paths that avoid extra formatting or allocation work.

#### Acceptance standard

A change is not acceptable merely because it is “idiomatic Rust.” It must also satisfy one of:

- equivalent performance,
- measurably better performance,
- slightly worse performance with documented and approved safety/correctness justification.

---

## 2. Migration Guidelines

### 2.1 C-to-Rust Mapping Rules

The migration must map C concepts into Rust in a way that preserves semantics first and idiom second.

#### 2.1.1 Modules and file structure

- Start from the observed module inventory.
- Maintain traceable correspondence between C modules and Rust modules.
- Shared internals may be factored, but original boundaries must remain reconstructible for review and testing.
- Each Rust module should state which C module(s) it corresponds to.

#### 2.1.2 Data representation

- Use `String` only for guaranteed text data with valid UTF-8 semantics.
- Use byte-oriented types such as `Vec<u8>`, `&[u8]`, `OsString`, `OsStr`, `PathBuf`, and platform string abstractions where the C behavior is byte-oriented or filesystem-oriented.
- Preserve explicit-size semantics with byte slices or equivalent typed representations.
- Do not assume UTF-8 where the C code does not require it.
- Structs that encode state machines in C should remain explicit state holders in Rust rather than dissolving into ad hoc local variables.

#### 2.1.3 Ownership mapping

- C allocation-returning helpers should generally become owned Rust values.
- C mutable state objects should become explicit Rust structs with controlled mutation methods.
- C global reusable state may become managed Rust global or thread-local state only when required by behavior; otherwise prefer explicit state passing.
- Any replacement of implicit C state with Rust-managed state must preserve externally visible semantics.

#### 2.1.4 Error mapping

- Prefer `Result` internally.
- Preserve C-visible fatal/non-fatal boundaries at the program behavior level.
- Shared fatal allocation behavior must remain centralized in design, even if implemented with Rust panic-free termination paths.
- Do not convert fatal C paths into silent recoverable Rust paths unless behavior review explicitly approves the change.

#### 2.1.5 Strings, buffers, and quoting

- Separate NUL-terminated and explicit-length semantics.
- Preserve APIs that conceptually operate on arbitrary bytes.
- Preserve caller-buffer versus owned-result versus reusable-slot models as distinct layers.
- Quoting option mutation must remain stateful and explicit.

#### 2.1.6 System and libc interaction

- Encapsulate OS-facing logic behind narrow Rust interfaces.
- Keep device/inode comparisons explicit.
- Do not replace filesystem identity logic with path-string comparison.
- Any interaction that depends on platform behavior must be documented at the wrapper boundary.

#### 2.1.7 Stream and shutdown behavior

- Model output completion as a distinct finalization stage.
- Preserve flush-and-close error checking semantics.
- Do not rely solely on Rust buffered I/O drop behavior when the C code performs explicit close validation.
- Any simplification of stream internals requires proof that observable behavior remains equivalent.

---

### 2.2 Principles for Handling Uncertain Behavior

The source summaries show some areas where exact behavior is not fully known. Uncertainty must be handled systematically.

#### Rules

- Do not guess if evidence can be obtained.
- Prefer the following order:
  1. inspect C source,
  2. inspect tests and build behavior,
  3. run the C binary and capture outputs,
  4. add characterization tests,
  5. only then choose an implementation approach.
- Where behavior remains uncertain, preserve flexibility in Rust design rather than freezing an assumption into the API.

#### Required practice

For each uncertain area, the migration record must include:

- what is known,
- what is unknown,
- why it matters,
- how it will be verified,
- whether implementation is blocked or may proceed behind a provisional interface.

#### Explicitly sensitive uncertainty areas

This project must treat the following as evidence-first areas:

- exact command-line branching details,
- exact diagnostics and exit statuses,
- locale startup sequencing,
- quoting edge cases,
- malformed multibyte handling,
- pointer invalidation or storage lifetime rules in reusable quoting slots,
- stream finalization edge conditions,
- fallback behavior in cwd reconstruction failures.

#### Prohibited behavior under uncertainty

- No speculative cleanup that changes semantics.
- No collapsing of interfaces because “the difference probably does not matter.”
- No introduction of UTF-8-only assumptions in byte-sensitive code.
- No replacing root-identity logic with textual shortcuts.

---

### 2.3 Test Verification Requirements

Testing is the primary proof that migration satisfies this constitution.

#### 2.3.1 Characterization before substitution

Before replacing a C behavior path with Rust, the project should have characterization tests for that path whenever feasible.

Priority characterization targets:

- help output,
- version output,
- logical cwd mode,
- robust cwd mode,
- root boundary behavior,
- deep-directory behavior,
- quoting behaviors,
- explicit-size quoting paths,
- allocation-failure policy where testable,
- output finalization behavior where testable.

#### 2.3.2 Differential testing

Where practical, the Rust implementation must be tested against the C implementation using the same inputs and environment.

Differential testing should compare:

- stdout,
- stderr,
- exit code,
- filesystem-side effects if any,
- behavior under locale variation where relevant.

#### 2.3.3 Edge-case coverage

Tests must include edge and boundary cases, especially:

- root directory,
- deep nesting,
- unusual path component names,
- names requiring quoting,
- embedded special characters,
- explicit-size data cases,
- locale-sensitive output differences,
- stream error behavior if reproducible.

#### 2.3.4 Regression discipline

Every bug found during migration must produce:

- a failing test,
- a fix,
- a non-regression test added to the suite.

---

## 3. Quality Gates

No implementation work is complete until it passes the quality gates in this section.

### 3.1 Tests That Must Pass

The following are mandatory gates for merged work.

#### 3.1.1 Build and integration gates

- The project must build through the established `Makefile` workflow or an approved compatible extension of it.
- The Rust build must integrate cleanly with project automation.
- No merged change may break the baseline build, test, or packaging flow.

#### 3.1.2 Behavioral test gates

At minimum, the following test classes must pass:

- unit tests for migrated module logic,
- integration tests for top-level command behavior,
- differential tests against the C implementation for migrated paths,
- regression tests for previously fixed defects.

#### 3.1.3 Required behavior categories

The test suite must cover and pass for:

- help flow,
- version flow,
- logical cwd flow,
- robust cwd reconstruction flow,
- root stopping behavior,
- path accumulator behavior,
- quoting option mutation behavior,
- quoting rendering behavior across result models,
- explicit-size input behavior,
- locale-sensitive behavior where applicable,
- output finalization behavior.

#### 3.1.4 Platform and environment gates

Where the original C behavior is platform-sensitive, tests must either:

- run on the supported target platforms, or
- explicitly document why coverage is restricted and what compensating evidence exists.

---

### 3.2 Code Review Standards

Every merge requires review against this constitution, not only against local code correctness.

#### 3.2.1 Required review questions

Reviewers must explicitly verify:

- Does the change preserve observable behavior?
- Does it preserve interface intent?
- Does it improve or at least maintain safety?
- Does it maintain performance characteristics on relevant paths?
- Does it add or update sufficient tests?
- Does it introduce any undocumented assumption about uncertain C behavior?

#### 3.2.2 Review criteria

A change must be rejected if it:

- removes behavior distinctions without proof,
- replaces byte semantics with text semantics incorrectly,
- introduces unnecessary `unsafe`,
- widens `unsafe` scope without justification,
- hides allocation or copying on hot paths,
- weakens output finalization behavior,
- weakens root identity checks,
- weakens test coverage for changed behavior,
- lacks traceability to the corresponding C behavior.

#### 3.2.3 Documentation expectations

Code under review must include, where relevant:

- mapping notes to C functions/modules,
- safety comments for `unsafe`,
- rationale for behavior-preserving design choices,
- notes on any approved deviation.

---

### 3.3 Performance Benchmark Requirements

Performance claims must be measured, not asserted.

#### 3.3.1 Benchmark scope

Benchmarks must exist for important paths, especially:

- logical cwd retrieval,
- robust cwd reconstruction in shallow trees,
- robust cwd reconstruction in deep trees,
- path accumulation growth behavior,
- quoting core formatting,
- repeated quoting through reusable slots or equivalent preserved model,
- output finalization overhead where measurable.

#### 3.3.2 Comparison baseline

Performance must be compared against:

- the C implementation where practical,
- the prior Rust state for regression detection.

#### 3.3.3 Benchmark quality rules

Benchmarks must:

- use representative workloads,
- include deep and boundary-shaped inputs where relevant,
- measure allocation behavior where possible,
- report environment assumptions,
- be reproducible in CI or documented benchmark environments.

#### 3.3.4 Acceptance thresholds

A performance regression requires explicit review if it is:

- material on a hot path,
- caused by additional allocations or copies,
- caused by loss of reusable-state behavior,
- caused by replacing byte operations with heavier abstractions.

Unapproved regressions must not merge.

---

## Enforcement

This constitution is binding on all subsequent migration documents and implementation work.

Therefore:

- specifications must derive requirements from these principles,
- plans must sequence work to satisfy these principles,
- tasks must be written so compliance can be checked,
- reviews must block changes that violate these principles,
- deviations must be documented and approved before merge.

If a later document conflicts with this constitution, the constitution wins.

---

## Project-Wide Non-Negotiables

1. Preserve behavior before beautifying design.
2. Preserve interface meaning before consolidating APIs.
3. Prefer safe Rust; isolate and justify every `unsafe`.
4. Preserve performance characteristics on important paths.
5. Treat uncertainty as a reason to measure, not assume.
6. Require tests as evidence, not decoration.
7. Keep build integration stable through the existing project workflow.
8. Do not merge behavior-changing shortcuts under the label of idiomatic Rust.