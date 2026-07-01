# Constitution for the Rust Migration of `cat`

## Purpose

This document defines the non-negotiable project-level principles for the Rust migration of the C project `cat`. It is the governing document for all later specifications, plans, tasks, designs, and code reviews.

Where there is tension between convenience, speed of implementation, idiomatic Rust, and fidelity to the original program, this constitution defines the order of priority.

---

## 1. Core Principles

### 1.1 Behavioral Equivalence Principle

The Rust implementation must preserve the observable behavior of the C implementation unless a deviation is explicitly approved and documented.

#### Rules

- The Rust binary must match the C program’s externally visible behavior for:
  - exit codes
  - stdout output
  - stderr diagnostics
  - option behavior
  - file processing behavior
  - buffering-visible effects where externally observable
  - locale-sensitive behavior where externally observable
  - error/fatal distinction
- Behavioral preservation applies to both:
  - normal flows
  - edge and failure flows
- When exact internal implementation details differ, the Rust version is acceptable only if externally observable behavior remains equivalent.
- For this project, “equivalent” means:
  - same results for the same valid inputs and environment
  - same failure class for the same invalid inputs or runtime faults
  - no silent semantic drift in corner cases already handled by the C version

#### Project-specific implications

The following behaviors are especially protected and must not be altered without explicit approval:

- Separation of top-level control between:
  - `main`
  - copy orchestration
  - simple fast-path copy
  - formatted/feature-rich copy
  - stdout/finalization logic
- Stateful formatted output behavior, including:
  - line numbering
  - blank-line squeezing
  - tab/nonprinting/end-of-line rendering
  - pending output flush behavior
- Slot-based quoting behavior and cleanup semantics
- Locale-aware branches and charset-sensitive behavior
- Distinction between:
  - recoverable operational failures
  - unrecoverable fatal support failures
- Stream finalization and close behavior, especially around stdout/stderr handling

#### Prohibitions

- Do not “simplify” behavior solely because Rust offers a cleaner abstraction.
- Do not collapse semantically distinct code paths if doing so risks changing output, timing of errors, or edge-case handling.
- Do not replace nuanced C behavior with approximate behavior based only on likely user expectations.

---

### 1.2 Interface Compatibility First Principle

Migration must preserve the original program structure and public behavior before pursuing internal redesign.

#### Rules

- The Rust project must retain a module decomposition that maps clearly to the C module inventory.
- Rust code should preserve the conceptual boundaries of the original units, especially for:
  - copy path logic
  - quoting subsystem
  - locale helpers
  - stream cleanup/finalization
  - allocation/fatal helpers
  - binary-mode and descriptor helpers
- Public and cross-module interfaces must be designed to maintain traceability to the C interfaces.
- If a C module represents an independent behavioral unit, the Rust implementation should preserve that unit as a Rust module or equivalent clearly bounded component.
- The Makefile build flow must remain supported unless explicitly superseded by an approved transitional build design.

#### Project-specific module guidance

The following areas must remain explicitly identifiable in the Rust codebase:

- entry/control flow corresponding to `main`
- copy engine corresponding to:
  - `simple_cat`
  - `cat`
  - `copy_cat`
  - `write_pending`
  - `next_line_num`
- quoting subsystem corresponding to:
  - quoting options configuration
  - quote/quotearg wrappers
  - slot-indexed quoting state
  - cleanup
- locale/charset subsystem corresponding to:
  - `setlocale_null*`
  - `hard_locale`
  - `locale_charset`
  - `mbrtoc32`
- stream/finalization subsystem corresponding to:
  - `close_stdout`
  - `close_stream`
  - `rpl_fclose`
  - `rpl_fflush`
  - `fpurge`
  - `clear_ungetc_buffer*`
- low-level I/O helpers corresponding to:
  - `safe_rw`
  - `full_rw`
  - `copy_file_range`
  - `fadvise`
  - `set_binary_mode`

#### Prohibitions

- Do not begin by rewriting the system into a fully new architecture.
- Do not merge unrelated modules merely to reduce file count.
- Do not hide behavior-critical state transitions inside generic helpers if that reduces auditability against the C source.

---

### 1.3 Safety First Principle

Rust safety advantages must be used aggressively, but never in ways that change required semantics.

#### Rules

- Prefer safe Rust by default.
- Use `unsafe` only when required for:
  - OS interop
  - FFI
  - raw descriptor or stream handling
  - memory/layout constraints that cannot be expressed safely
- Every `unsafe` block must have:
  - a narrow scope
  - a documented safety invariant
  - a clear reason it is necessary
- Eliminate C undefined behavior where possible, but preserve the program’s specified or de facto observable behavior.
- Replace implicit memory hazards with explicit Rust types and invariants where behavior is unchanged.
- Global mutable state must be minimized, but where original semantics require process-global retained state, the Rust version must model it explicitly and safely.

#### Project-specific safety expectations

Special care is required in:

- retained quoting slot storage
- persistent line-number state
- stream finalization semantics
- descriptor and file-handle ownership
- buffer boundary management in copy paths
- locale and multibyte conversion state
- faithful handling of partial reads/writes and interrupted syscalls
- any emulation of C APIs involving `FILE *`, `fcntl`, or binary mode

#### Prohibitions

- Do not introduce panics in place of controlled operational error handling.
- Do not use `.unwrap()` or `.expect()` in runtime paths where the C program would emit a diagnostic or return a status.
- Do not treat fatal support failures and ordinary I/O failures as the same category.
- Do not rely on undefined ordering of destructors for behavior-critical finalization.

---

### 1.4 Performance Constraint Principle

The Rust migration must not materially regress the performance characteristics of `cat`, especially on the primary copy paths.

#### Rules

- Preserve fast-path behavior as a first-class design concern.
- The simple copy path must remain optimized for throughput and low overhead.
- The formatted path must preserve buffered transformation behavior rather than degrade into excessive per-byte syscalls or allocations.
- Low-level read/write loops must be designed with attention to:
  - syscall count
  - partial-transfer handling
  - buffer reuse
  - avoiding unnecessary copies
- Allocation behavior on hot paths must be minimized and predictable.
- Rust abstractions are acceptable only when they compile to behavior consistent with performance expectations for a utility like `cat`.

#### Project-specific performance priorities

Highest sensitivity areas:

1. simple copy path
2. formatted copy path with pending output buffering
3. low-level safe/full read-write helpers
4. file-to-file optimized transfer opportunities
5. startup overhead that affects short-lived invocations
6. quoting behavior only where it is on active runtime paths

#### Prohibitions

- Do not replace buffered loops with character-by-character I/O.
- Do not introduce unnecessary heap allocation in copy loops.
- Do not sacrifice throughput for abstraction purity.
- Do not make locale or quoting support impose overhead on paths that do not require them.

---

## 2. Migration Guidelines

### 2.1 C-to-Rust Mapping Rules

The migration must proceed by disciplined semantic mapping, not by unconstrained rewrite.

#### 2.1.1 Module mapping

- Each C implementation unit must map to:
  - one Rust module, or
  - one small Rust module group with documented correspondence
- Mapping from C file to Rust module must be recorded and reviewable.
- Cross-module dependencies in Rust should remain understandable relative to the original C dependency graph.

#### 2.1.2 Function mapping

- Behaviorally important C functions must have an identifiable Rust equivalent.
- High-risk functions should keep names closely aligned to the C originals unless there is a strong reason not to.
- Wrapper families in C should remain wrapper families in Rust where that relationship is behaviorally meaningful.

This is especially required for:

- `main`
- `copy_cat`
- `simple_cat`
- `cat`
- `write_pending`
- `next_line_num`
- `close_stdout*`
- `close_stream`
- `safe_rw`
- `full_rw`
- `quotearg*` / `quote*`
- `setlocale_null*`
- `hard_locale`
- `locale_charset`
- allocation/fatal helpers

#### 2.1.3 Type mapping

- C integer, flag, and status types must be mapped with attention to width, signedness, and syscall/API expectations.
- Rust types must preserve:
  - sentinel distinctions where externally meaningful
  - platform-sized semantics where required
  - byte-oriented behavior in copy paths
- Do not replace byte-processing logic with Unicode scalar semantics unless the original behavior is truly character-based.
- For text/locale subsystems, preserve the original boundary between bytes, multibyte sequences, and character abstractions.

#### 2.1.4 State mapping

- C static/module-retained state must be explicitly modeled in Rust.
- Stateful subsystems must document:
  - owner
  - lifetime
  - mutability model
  - cleanup/reset semantics
- If the C design uses process-global state and behavior depends on that state, Rust must not silently convert it into transient local state.

#### 2.1.5 Error mapping

- C return-code semantics must be preserved in Rust, even if implemented using `Result`.
- Internal Rust error types must map back to the original program’s externally visible status model.
- Fatal helpers in C must remain fatal in Rust.
- Recoverable failures in C must not become panics or hidden logs in Rust.

#### 2.1.6 OS and libc boundary mapping

- When replacing libc-facing C code, choose the narrowest Rust abstraction that preserves behavior.
- For syscalls and descriptor operations:
  - preserve retry and partial-transfer semantics
  - preserve errno-relevant distinctions where observable
  - preserve close/finalization ordering
- If direct libc or platform API calls are necessary, isolate them behind reviewed interfaces.

---

### 2.2 Principles for Handling Uncertain Behavior

Not all behavior is fully known from summaries alone. Uncertainty must be handled conservatively.

#### 2.2.1 Conservative default

When behavior is uncertain, assume the C implementation’s exact behavior matters.

- Prefer postponing redesign over guessing.
- Prefer reading source and constructing tests over inferring intent.
- Prefer preserving strange behavior over normalizing it.

#### 2.2.2 Evidence hierarchy

When deciding behavior, use this order of authority:

1. actual C source behavior
2. existing tests and fixtures
3. observed binary behavior against representative inputs
4. documented interface and behavior summaries
5. comments and historical project context
6. developer intuition

#### 2.2.3 Required response to ambiguity

If behavior is ambiguous:

- document the ambiguity
- create a characterization test if possible
- compare C and Rust outputs directly
- defer irreversible refactoring until ambiguity is resolved

#### 2.2.4 Allowed deviations

A deviation is allowed only if all of the following are true:

- exact original behavior cannot be preserved reasonably
- the deviation is documented
- the deviation is reviewed and approved
- tests are updated to lock the new intended behavior
- the change does not violate the core principles above

#### 2.2.5 Unknown platform-specific behavior

For platform-conditional areas such as:

- `fcntl` variants
- `copy_file_range`
- binary mode
- locale locking behavior
- stream internals

the project must preserve behavior on the supported target platform first. Additional portability improvements may be layered later only if they do not break baseline equivalence.

---

### 2.3 Test Verification Requirements

Testing is mandatory evidence of equivalence, not a cleanup task for later.

#### 2.3.1 Characterization-first testing

Before changing a behaviorally significant area, establish tests that characterize the C version when practical.

Priority targets:

- option combinations
- stdout/stderr/exit-code combinations
- line numbering behavior
- blank-line squeezing
- show-tabs/show-ends/show-nonprinting interactions
- file and stdin handling
- broken pipe / close behavior
- locale-sensitive outputs where relevant
- quoting subsystem output and slot behavior

#### 2.3.2 Golden behavioral comparisons

The Rust binary must be validated against the C binary using golden or differential tests for:

- identical inputs
- identical environment where required
- identical arguments
- identical expected exit status
- stdout match
- stderr match, or documented accepted equivalence where formatting differences are approved

#### 2.3.3 Edge-case coverage

Tests must include, as applicable:

- empty input
- large input
- binary input
- inputs without trailing newline
- repeated blank lines
- mixed tabs and control bytes
- multiple files
- stdin plus file arguments as supported
- failing reads/writes where reproducible
- output close failures where reproducible
- interrupted or partial I/O scenarios where harnessable
- locale variants relevant to quoting or multibyte behavior

#### 2.3.4 Stateful subsystem coverage

The following require explicit state-oriented tests:

- line-number progression
- pending-output flush boundaries
- quotearg slot reuse and cleanup
- close-stdout retained policy
- locale query helpers if implemented directly in Rust
- multibyte conversion state where behavior is preserved internally

#### 2.3.5 Regression expectations

Every bug fixed during migration must be accompanied by:
- a reproducing test
- a fix
- verification that the change does not alter unrelated behavior

---

## 3. Quality Gates

No migration step is complete until it passes all applicable quality gates in this section.

### 3.1 Tests That Must Pass

#### 3.1.1 Build and baseline gates

The project must successfully:

- build via the project-approved build flow
- produce a runnable Rust binary
- run the required automated test suites in CI
- support side-by-side comparison against the reference C implementation where defined

#### 3.1.2 Behavioral test gates

At minimum, the following must pass before a module or milestone is considered complete:

- unit tests for internal logic introduced in Rust
- differential tests against the C implementation for behaviorally significant paths
- integration tests covering CLI behavior
- regression tests for all fixed defects
- snapshot/golden tests for stable textual outputs where appropriate

#### 3.1.3 Protected behavior gate list

The following behaviors must have passing evidence before release-equivalent status is claimed:

- help/version branches
- normal copy path
- simple fast path
- formatted copy path
- option interaction behavior
- exit status behavior
- stdout finalization behavior
- error reporting behavior
- quoting subsystem outputs used by the program
- locale-sensitive behavior relied on by the implementation

#### 3.1.4 Negative gate

A change fails quality gates if:

- it removes tests covering existing behavior without replacement
- it changes observable behavior without approval
- it adds panic-prone runtime behavior to operational paths
- it lacks differential evidence for behaviorally significant rewrites
- it cannot explain discrepancies from the C binary

---

### 3.2 Code Review Standards

Code review is a semantic audit, not merely a style check.

#### 3.2.1 Required review criteria

Every review must evaluate:

- behavioral equivalence risk
- interface traceability to the C implementation
- safety of memory/ownership/concurrency decisions
- correctness of error and fatal-path handling
- performance impact on hot paths
- adequacy of tests
- clarity of documented assumptions and invariants

#### 3.2.2 Review requirements for sensitive areas

Changes touching any of the following require heightened scrutiny:

- `main` or top-level dispatch
- copy loops
- buffering logic
- read/write syscall wrappers
- stdout/stderr close behavior
- quoting slot management
- locale/multibyte logic
- `unsafe` code
- OS interop and descriptor ownership
- performance-sensitive allocations

#### 3.2.3 Unsafe review rule

No `unsafe` code may be merged unless the review explicitly confirms:

- why it is necessary
- the exact invariants relied upon
- why safe Rust alternatives were insufficient
- how the invariants are enforced by surrounding code and tests

#### 3.2.4 Panic review rule

Reviews must reject runtime `.unwrap()`, `.expect()`, or equivalent shortcuts in code paths that process user input, files, descriptors, locale state, or output finalization unless the panic is provably unreachable and documented as such.

#### 3.2.5 Traceability rule

For behaviorally significant changes, the review should be able to answer:

- what C behavior this maps to
- what tests prove equivalence
- what, if anything, intentionally changed

If those answers are not clear, the change is not ready.

---

### 3.3 Performance Benchmark Requirements

Performance must be measured, not assumed.

#### 3.3.1 Benchmark gate policy

Any change affecting hot paths must include benchmark evidence or justified exemption.

Hot paths include:

- simple copy path
- formatted copy path
- low-level read/write wrappers
- file transfer strategy selection
- startup path for short invocations

#### 3.3.2 Benchmark comparison baseline

Benchmarks should compare:

- Rust current branch vs Rust mainline baseline
- Rust implementation vs reference C implementation where feasible

#### 3.3.3 Required benchmark scenarios

As applicable, benchmark at least:

- large file simple copy
- stdin-to-stdout simple streaming
- formatted output path with representative transformations enabled
- small-invocation overhead for short-lived command execution
- repeated file processing where setup costs matter

#### 3.3.4 Performance acceptance standard

A performance regression is acceptable only if:

- it is measured
- it is documented
- it is justified by correctness or safety necessity
- it is approved

Unmeasured regressions are not acceptable.

#### 3.3.5 Allocation benchmark expectation

For hot paths, reviews should consider:

- allocation count
- buffer reuse
- hidden copies
- formatting overhead
- whether abstractions force extra work relative to the C behavior

---

## 4. Governance and Precedence

### 4.1 Constitutional Authority

This document is the highest-level engineering authority for the migration project.

- Specs must conform to it.
- Plans must conform to it.
- Tasks must conform to it.
- Code changes must conform to it.
- Review decisions must use it as a standard.

### 4.2 Conflict Resolution

If a later document conflicts with this constitution:

- this constitution takes precedence
- the later document must be revised
- implementation must not proceed based on the conflicting instruction

### 4.3 Amendment Rule

This constitution may be changed only through explicit project-level approval. Amendments must be:

- intentional
- documented
- justified
- narrow in scope where possible

### 4.4 Default Decision Rule

When no explicit guidance exists, choose the option that best satisfies this order:

1. behavioral equivalence
2. interface compatibility and traceability
3. safety
4. performance preservation
5. implementation elegance

---

## 5. Non-Negotiable Summary

The Rust migration of `cat` shall:

- preserve observable behavior first
- preserve interface and module traceability second
- use Rust safety to eliminate accidental hazards without altering semantics
- protect fast-path performance
- treat uncertainty conservatively
- require test evidence for claims of equivalence
- require strict review for unsafe, error-path, buffering, locale, quoting, and I/O behavior
- reject unmeasured regressions and undocumented semantic drift

All subsequent project documents and implementation work must comply with this constitution.