# Constitution for the Rust Migration of `cat`

## Purpose

This document defines the non-negotiable project-level principles for the Rust migration of the C project `cat`. It is the governing standard for all later specs, plans, tasks, implementation decisions, reviews, and acceptance criteria.

Where this constitution conflicts with convenience, preference, or speed, this constitution wins.

---

## 1. Core Principles

### 1.1 Behavioral Equivalence Principle

The Rust implementation must preserve the observable behavior of the C program unless a deliberate, explicitly approved deviation is documented.

#### Mandatory implications

- The Rust executable must preserve the role of `main` as the single top-level executable entry path.
- The program must preserve the same high-level runtime split between:
  - informational execution paths,
  - content-copy execution paths,
  - startup/setup failure paths,
  - shutdown/finalization paths.
- The Rust port must preserve the distinct copy-engine modes identified in the C implementation:
  - simple/plain copy behavior,
  - transformation-capable copy behavior.
- The transformation-capable path must preserve the behavioral effect of flags controlling:
  - nonprinting rendering,
  - tab rendering,
  - line numbering,
  - nonblank-only numbering,
  - end-of-line marking,
  - blank-line squeezing.
- Buffered output behavior must remain semantically equivalent. The port must not casually replace buffered staged emission with materially different per-byte or per-line output behavior if that could change ordering, flushing, or failure propagation.
- The quoting subsystem must preserve its configuration lifecycle and dynamic behavior, including:
  - mutable quoting options,
  - style-based configuration,
  - per-character quoting overrides,
  - custom delimiters,
  - numbered-slot style APIs,
  - byte-slice/memory-length-aware operations,
  - cleanup behavior corresponding to `quotearg_free`.
- Locale- and charset-dependent behavior must remain equivalent at the externally visible level.
- Stream finalization policy must remain equivalent, including centralized close behavior and shutdown error handling policy.
- Fatal helper semantics must remain fatal. Logic corresponding to `xalloc_die` and `xset_binary_mode_error` must not be silently weakened.

#### Observable behavior takes priority over internal structure

The Rust code does not need to mimic C syntax, file structure, macro style, or memory layout by default. It must mimic externally visible behavior, error propagation, lifecycle rules, and important performance characteristics.

#### No accidental semantic simplification

The migration must not flatten meaningful distinctions found in the C program merely because Rust offers a cleaner abstraction. In particular, the project must not erase:
- simple vs transforming copy paths,
- string vs explicit-length memory APIs,
- configurable quoting state,
- locale query variants,
- compatibility wrapper boundaries,
- shutdown error policy distinctions.

---

### 1.2 Interface Compatibility First Principle

Public and project-internal interfaces that carry semantic meaning in the C codebase must be preserved or intentionally mapped, not improvised away.

#### Scope

This applies to:
- CLI behavior,
- module responsibilities,
- function-level semantic contracts,
- stateful subsystems,
- error and shutdown boundaries,
- compatibility wrappers.

#### Rules

- The Rust migration must begin from the C interface and behavior inventory, not from a greenfield redesign.
- Module responsibilities identified in the source inventory must remain recognizable in Rust, even if merged, split, or wrapped idiomatically.
- Where multiple C entry points express distinct semantics, the Rust code must preserve those semantics through:
  - equivalent public functions,
  - equivalent internal APIs,
  - or a documented compatibility layer.
- Byte-oriented and string-oriented variants must remain distinct where the C project distinguishes them.
- Stateful subsystems such as quoting options and stream shutdown policy must retain explicit state boundaries. They must not become hidden, ad hoc globals without review.
- Compatibility functions such as `rpl_fflush`, `rpl_fclose`, `copy_file_range`, `set_binary_mode`, `fcntl` wrappers, and shutdown helpers must remain explicit semantic boundaries in the Rust design even if implemented differently underneath.

#### CLI compatibility

The Rust executable must preserve user-visible command behavior to the maximum extent supported by evidence:
- same invocation role,
- same operational modes,
- same output category behavior,
- same success/failure meaning,
- same transformation feature set.

If exact CLI details are uncertain, they must be resolved by testing against the C implementation before finalization.

---

### 1.3 Safety First Principle

Rust safety is a project requirement, not an optional enhancement, but safety must be achieved without breaking required behavior.

#### Memory safety

- Safe Rust is the default.
- `unsafe` is allowed only when necessary for:
  - FFI,
  - platform-specific syscalls,
  - performance-critical low-level operations that cannot be expressed safely without unacceptable cost,
  - precisely bounded compatibility behavior.
- Every `unsafe` block must have:
  - a clear safety comment,
  - a narrowly scoped boundary,
  - documented invariants,
  - tests that exercise the relevant behavior where practical.

#### Semantic safety

The migration must also preserve operational safety properties present in the C project:
- robust bounded-buffer handling,
- correct explicit-length memory processing,
- reliable error propagation,
- correct shutdown handling,
- no silent truncation,
- no dropped I/O errors,
- no weakening of fatal allocation/error paths into undefined or inconsistent behavior.

#### Error handling

- Rust error handling must reflect the semantic intent of the C code.
- Recoverable errors must remain recoverable.
- Fatal paths must remain fatal where required.
- Result types, custom error types, and explicit propagation are preferred over hidden global flags or silent fallthrough.
- No `.unwrap()` or `.expect()` is allowed in production code unless it is proving an invariant that is:
  - documented,
  - review-approved,
  - and effectively equivalent to a C fatal invariant boundary.

#### Concurrency and shared state

If the Rust design introduces synchronization or shared state for locale, quoting, or shutdown policy, it must not introduce races, deadlocks, or behavior changes. Do not add concurrency merely because Rust makes it available.

---

### 1.4 Performance Constraint Principle

The Rust migration must remain performance-conscious, especially in the main copy paths.

#### Performance-sensitive areas

The following are treated as critical paths:
- `copy_cat`
- `simple_cat`
- `cat`
- `safe_rw`
- `full_rw`
- `write_pending`
- optional direct copy paths such as `copy_file_range`
- alignment-sensitive allocation paths
- quoting APIs where buffer reuse or slot reuse matters

#### Rules

- The migration must preserve the separation between fast/plain copy and transformation-heavy copy.
- The Rust implementation must avoid unnecessary allocations in streaming paths.
- Buffered output must be preserved as a performance and semantic property.
- Large-input behavior matters more than micro-cleanliness of abstraction.
- Idiomatic Rust is desirable, but not if it introduces avoidable per-byte overhead, extra copies, hidden buffering changes, or allocation churn.
- Benchmark regressions in critical paths require explicit justification and approval.

#### Performance is constrained by equivalence

Performance work must not change behavior. Conversely, behavioral fidelity must not be used as an excuse for careless slowdowns in the main data path.

---

## 2. Migration Guidelines

### 2.1 C-to-Rust Mapping Rules

These rules define how C constructs should be translated into Rust while preserving semantics.

#### 2.1.1 Functions and module boundaries

- Each C module or module cluster must be mapped to a Rust module or set of modules with traceable responsibility.
- Major behavioral clusters must remain explicit:
  - core cat/copy engine,
  - quoting subsystem,
  - locale/charset subsystem,
  - stream/fd compatibility subsystem,
  - allocation/fatal helper subsystem,
  - program metadata/version/help subsystem.
- Mapping may be many-to-one or one-to-many, but it must be documented when semantics are redistributed.

#### 2.1.2 Data representation

- C structs with semantic state must become explicit Rust structs.
- Anonymous or implicit C state must not remain conceptually anonymous; Rust code should give such state meaningful names.
- C boolean flags should become Rust booleans or typed config structs.
- Raw byte buffers should become slices, `Vec<u8>`, or fixed buffers as appropriate, but explicit-length semantics must be preserved.
- NUL-terminated string assumptions must not be introduced where the C code supports explicit memory spans.

#### 2.1.3 Pointers and memory

- Raw pointers in C should map to references, slices, owned buffers, or smart pointers wherever possible.
- Pointer arithmetic must be replaced with index- or slice-based logic unless `unsafe` is required for performance or FFI.
- Ownership and lifetime must be explicit.
- Allocation helpers in the `xmalloc` family should map to Rust allocation patterns that preserve fatal or checked behavior as required by semantics.
- If the C code uses alignment-sensitive allocation for meaningful runtime behavior, the Rust port must preserve the alignment guarantees.

#### 2.1.4 I/O and file descriptor handling

- Descriptor-oriented behavior must remain descriptor-aware where semantics depend on it.
- Do not blindly replace low-level fd logic with high-level `std::fs` abstractions if that would change retry behavior, buffering, binary mode semantics, or close/finalization behavior.
- Direct syscall or libc-level access is acceptable when needed for equivalence or performance.
- Stream and file-descriptor compatibility layers should be represented explicitly in Rust, even if internally simplified on supported platforms.

#### 2.1.5 Global and process-level state

- Program-name state, quoting slot state, locale state, and shutdown policy state must be explicit and reviewable.
- Global mutable state must be minimized.
- If retained, global state must be encapsulated and justified by compatibility requirements.

#### 2.1.6 Macros, constants, and compile-time behavior

- C macros should map to Rust constants, helper functions, enums, or traits depending on semantic role.
- Feature gating and platform differences should be explicit in Rust through `cfg`, compatibility modules, or portable wrappers.
- Do not hide behaviorally meaningful platform branching inside undocumented helper code.

#### 2.1.7 Fatal semantics

- C fatal helpers should map to Rust functions with explicit non-returning behavior where required, e.g. returning `!` or terminating via a clearly designated fatal path.
- Fatal allocation and fatal binary-mode setup must preserve their semantic role.

---

### 2.2 Principles for Handling Uncertain Behavior

The source summary explicitly identifies areas of insufficient evidence. These areas must be handled conservatively.

#### 2.2.1 Evidence hierarchy

When behavior is uncertain, teams must resolve questions in this order:

1. Actual C source code
2. Existing tests, if any
3. Differential execution against the built C binary
4. Tooling-assisted observation of the C program
5. Project-approved interpretation documented in writing

Assumption without evidence is prohibited.

#### 2.2.2 Conservative default

If behavior is not fully known:
- do not invent nicer behavior,
- do not modernize semantics by guesswork,
- do not simplify edge cases away,
- do not infer from function names alone when observation is possible.

#### 2.2.3 Required documentation for uncertain areas

Any migration decision involving incomplete evidence must record:
- what is known,
- what is unknown,
- what evidence was consulted,
- what assumption was made,
- why the assumption is believed safe,
- how the assumption will be validated.

#### 2.2.4 Known uncertainty zones

The following must be treated as requiring verification before final acceptance if they affect implementation:
- exact option parsing details,
- exact startup call order,
- exact file iteration behavior,
- precise `safe_rw` and `full_rw` retry/error semantics,
- exact rendering details of transformation flags,
- exact line-number formatting and initial state,
- exact quoting escape rules and slot lifetime behavior,
- exact locale-locking mechanism,
- exact stream error/exit mapping,
- exact memory growth formulas,
- exact dispatch conditions for optimized copy or advisory I/O.

#### 2.2.5 Compatibility over elegance

In uncertainty zones, choose the path most likely to preserve compatibility with the C implementation, not the path that produces the cleanest Rust API.

---

### 2.3 Test Verification Requirements

Testing is a required mechanism of migration truth, not a cleanup step.

#### 2.3.1 Differential verification requirement

For every implemented behavior that can be observed from outside the program, the Rust version must be verifiable against the C version by differential tests, fixtures, or recorded comparisons.

#### 2.3.2 Minimum required test categories

The test suite must include, as applicable:

- **CLI behavior tests**
  - no-argument behavior
  - stdin/stdout path behavior
  - file operand behavior
  - informational output paths
  - error path behavior

- **Transformation behavior tests**
  - plain copy mode
  - line numbering behavior
  - nonblank-only numbering
  - tab rendering behavior
  - end-of-line marking behavior
  - nonprinting rendering behavior
  - blank-line squeezing behavior
  - representative flag combinations

- **I/O robustness tests**
  - large input handling
  - partial read/write simulation where feasible
  - output flushing and finalization behavior
  - descriptor-oriented behavior where semantically relevant

- **Quoting subsystem tests**
  - style-based behavior
  - custom quoting behavior
  - colon-related behavior
  - explicit-length vs NUL-terminated input behavior
  - numbered-slot behavior
  - cleanup/free lifecycle behavior where applicable

- **Locale and encoding tests**
  - locale query behavior
  - hard locale classification behavior
  - charset-dependent behavior where observable
  - multibyte conversion behavior where relevant

- **Failure-path tests**
  - allocation-failure policy where practical through injection/mocking
  - binary-mode setup failure paths where feasible
  - stream close/finalization failures where feasible
  - invalid input or conversion failure paths where observable

- **Regression tests for uncertain behaviors**
  - every previously uncertain area, once resolved, must gain a regression test if practical.

#### 2.3.3 Golden and differential outputs

- Golden outputs are allowed, but when possible they should be generated or validated against the C implementation.
- If platform-specific differences exist, the test suite must explicitly document and isolate them.

#### 2.3.4 Property and fuzz testing

For parsing, quoting, bounded-buffer logic, and transformation logic, property-based or fuzz testing is strongly encouraged. It does not replace behavioral equivalence tests.

---

## 3. Quality Gates

No migration work is complete until it passes all applicable quality gates.

### 3.1 Tests That Must Pass

#### 3.1.1 Build and static checks

Every change set must pass:
- `cargo fmt --check`
- `cargo clippy` with project-configured deny-level lints
- `cargo test`
- any Makefile-integrated build/test targets defined for the migration
- documentation or spec consistency checks if established by the project

#### 3.1.2 Behavioral test gate

Before a module, subsystem, or milestone is considered complete:
- all relevant Rust tests must pass,
- all relevant differential tests against the C implementation must pass,
- all previously passing regression tests must remain passing.

#### 3.1.3 Platform gate

If the project supports multiple target environments, behaviorally relevant tests must pass on all required targets. Platform-specific skips must be explicit and justified.

#### 3.1.4 No unresolved critical uncertainty

A feature cannot be marked complete if it still depends on undocumented assumptions about user-visible behavior.

---

### 3.2 Code Review Standards

Every non-trivial change must be reviewed against this constitution.

#### Required review checks

Reviewers must verify:

- **Behavioral fidelity**
  - Does the change preserve known C behavior?
  - If behavior differs, is the deviation documented and approved?

- **Interface traceability**
  - Can the Rust implementation be traced back to the relevant C module/function responsibility?
  - Are meaningful semantic boundaries preserved?

- **Safety**
  - Is safe Rust used by default?
  - Is every `unsafe` block justified, minimal, and documented?
  - Are error paths explicit and sound?

- **Testing**
  - Are new behaviors covered by tests?
  - Are uncertain behaviors backed by evidence?
  - Are regressions prevented by targeted tests?

- **Performance**
  - Does the change affect a critical path?
  - Does it introduce avoidable allocations, copies, or buffering changes?
  - Has performance impact been measured where appropriate?

- **Readability and maintainability**
  - Is the mapping from C semantics to Rust design understandable?
  - Are invariants documented?
  - Are compatibility hacks isolated instead of infecting unrelated code?

#### Review rejection conditions

A change must be rejected if it:
- changes behavior without evidence or approval,
- removes semantically meaningful interface distinctions,
- introduces unjustified `unsafe`,
- introduces `.unwrap()`/`.expect()` on externally influenced paths without approved invariant reasoning,
- lacks tests for new or changed behavior,
- causes unexplained performance regression in critical paths,
- obscures compatibility logic so that reviewers cannot verify equivalence.

---

### 3.3 Performance Benchmark Requirements

Performance is a release gate for the migration, especially for the copy engine.

#### 3.3.1 Mandatory benchmark scope

Benchmarks must cover at minimum:
- plain copy path on large input,
- transformation-heavy path on large input,
- mixed-size file workloads,
- stdin-to-stdout streaming path if supported,
- representative quoting hot paths if they are on user-visible or repeated diagnostic paths.

#### 3.3.2 Baseline

The C implementation is the baseline reference unless the project explicitly defines an alternate benchmark baseline.

#### 3.3.3 Regression policy

- No significant regression in critical-path throughput, allocation rate, or latency is allowed without explicit review approval.
- Any measurable regression must be explained in terms of:
  - behavioral correctness requirement,
  - platform constraint,
  - or safety necessity.
- “Rust is slower” is not an acceptable justification by itself.

#### 3.3.4 Measurement discipline

Benchmarks must:
- use repeatable inputs,
- distinguish warm-up from measured runs where relevant,
- record environment details,
- compare like-for-like modes between C and Rust,
- separate plain copy from transforming copy.

#### 3.3.5 Optimization discipline

Performance optimization is allowed only if:
- behavior remains equivalent,
- tests remain passing,
- code remains reviewable,
- unsafe optimizations follow the project’s safety rules.

---

## 4. Governance and Precedence

### 4.1 This document is binding

All later documents, including:
- specifications,
- architecture notes,
- migration plans,
- implementation tasks,
- review checklists,
- acceptance criteria,

must conform to this constitution.

### 4.2 Conflict resolution

If two goals conflict, precedence is:

1. Behavioral equivalence
2. Interface compatibility
3. Safety
4. Performance
5. Implementation convenience

Safety may justify internal redesign, but not silent user-visible incompatibility. Performance may justify lower-level implementation choices, but not correctness loss.

### 4.3 Amendment rule

Any change to these principles requires explicit project-level approval and must state:
- what principle is changing,
- why the current rule is insufficient,
- what risks are introduced,
- how downstream documents and work items will be updated.

### 4.4 Definition of done

A migrated subsystem is done only when:
- its behavior is evidenced and implemented,
- its interfaces are traceable to the C semantics,
- its tests pass,
- its reviews pass,
- its performance is acceptable for its role,
- its unresolved uncertainties are either closed or explicitly tracked and gated.