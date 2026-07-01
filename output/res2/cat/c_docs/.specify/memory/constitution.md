# Constitution for the Rust Migration of `cat`

## Purpose

This document defines the non-negotiable project-level principles for the Rust migration of the C project `cat`. It is the governing constitution for all later specifications, plans, tasks, implementation decisions, and reviews.

Where there is tension between elegance, modernization, and faithfulness, this constitution defines how that tension is resolved.

---

## Scope

This constitution applies to the full migration surface of the project, including:

- the `cat` executable behavior
- all 39 module units and 39 cluster units
- public and semi-public interfaces reflected in the module inventory
- startup, runtime, shutdown, and error behavior
- low-level I/O, stream handling, locale handling, quoting, and allocation support behavior
- build, test, benchmarking, and review standards

This migration is not a greenfield rewrite. It is a behavior-preserving Rust reimplementation of an existing C program and its support modules.

---

## 1. Core Principles

### 1.1 Behavioral Equivalence Principle

The Rust implementation must preserve the observable behavior of the C implementation unless an intentional deviation is explicitly approved and documented.

#### Required interpretation

Observable behavior includes, at minimum:

- command-line semantics
- exit codes
- stdout and stderr content
- help and version output structure
- file processing order
- buffering-visible behavior where it affects correctness
- error timing where externally visible
- locale-sensitive behavior
- quoting behavior
- end-of-program flush and close behavior
- fatal vs recoverable failure behavior
- platform-sensitive mode-setting behavior where applicable

#### Project-specific behavioral invariants

The following are mandatory migration invariants for this project:

- Control flow begins in `main` and preserves the same top-level runtime role.
- Help/version/reporting remain alternate top-level flows.
- The dual copy-engine structure must remain behaviorally visible:
  - `simple_cat` for plain copying
  - `cat` for decorated/transforming copying
- Flag-driven transformation semantics must remain faithful for:
  - `show_nonprinting`
  - `show_tabs`
  - `number`
  - `number_nonblank`
  - `show_ends`
  - `squeeze_blank`
- Explicit line-number state and progression behavior must be preserved.
- Buffered output commit behavior represented by `write_pending` must remain semantically intact.
- Low-level transfer layering must remain intact:
  - guarded primitive behavior corresponding to `safe_rw`
  - completion-oriented loop behavior corresponding to `full_rw`
- Quoting subsystem behavior families must remain intact, including mutable options, `_mem` forms, custom delimiters, indexed storage behavior, and cleanup semantics.
- Locale query behavior must remain distinct from locale hardness checks, charset lookup, and multibyte conversion.
- Final stdout close and flush behavior must preserve close-time error surfacing and configurable EPIPE policy.
- Fatal helper paths equivalent to `xalloc_die` and `xset_binary_mode_error` must remain fatal.

#### Decision rule

If a Rust design is more idiomatic but risks changing externally visible behavior, behavior wins.

If exact behavior is known, preserve it.
If behavior is inferred but strongly supported by module and behavior evidence, preserve it.
If behavior is uncertain, do not invent new semantics; see Section 2.2.

---

### 1.2 Interface Compatibility First Principle

The migration must preserve interface compatibility before pursuing internal restructuring.

#### Required interpretation

Compatibility applies to:

- executable CLI behavior
- module boundaries significant to migration planning
- semantic contracts exposed by the C interfaces
- function family relationships, especially where multiple wrappers share one semantic core
- data and option flow implied by the original design

Rust does not need to mimic C syntax or unsafe memory layouts unless required, but it must preserve the interface contract visible to callers, tests, and users.

#### Project-specific interface obligations

The following interface families must retain recognizable semantic structure:

- main execution and copy orchestration:
  - `main`
  - `copy_cat`
  - `simple_cat`
  - `cat`
  - `write_pending`
  - `next_line_num`
- stdout and stream-finalization family:
  - `close_stdout_set_file_name`
  - `close_stdout_set_ignore_EPIPE`
  - `close_stdout`
  - `close_stream`
  - `rpl_fflush`
  - `rpl_fclose`
- quoting family:
  - `clone_quoting_options`
  - `get_quoting_style`
  - `set_quoting_style`
  - `set_char_quoting`
  - `set_quoting_flags`
  - `set_custom_quoting`
  - `quoting_options_from_style`
  - `quotearg_buffer_restyled`
  - wrapper forms such as `quotearg`, `quote`, `quotearg_n`, `quotearg_style`, `quotearg_colon`, `quotearg_custom`, `_mem` variants, and `quotearg_free`
- locale and character handling family:
  - `setlocale_null*`
  - `hard_locale`
  - `locale_charset`
  - `mbrtoc32`
- transfer and descriptor family:
  - `safe_rw`
  - `full_rw`
  - `dupfd`
  - `rpl_fcntl_*`
  - `copy_file_range`
  - `set_binary_mode`
- allocation family:
  - aligned allocation semantics
  - fail-fast `x*alloc` semantics
  - duplication/reallocation helper semantics

#### Structure rule

Rust modules may be reorganized for clarity, but reorganization must not erase:
- semantic separations that matter to behavior
- distinct error policies
- state machine boundaries
- convenience-wrapper families that callers or tests depend on

The project must prefer preserving conceptual API shape over collapsing everything into a monolith.

---

### 1.3 Safety First Principle

Rust safety benefits are a primary reason for migration, but safety must be achieved without violating behavioral equivalence.

#### Required interpretation

The default implementation stance is:

- safe Rust first
- `unsafe` only when justified
- minimal FFI and unsafe boundaries
- explicit invariants wherever unsafe code exists

#### Mandatory safety rules

1. No undefined behavior may be introduced.
2. No memory-unsafe emulation of C behavior is permitted merely for similarity.
3. Integer conversions, allocation sizing, indexing, and buffer growth must be checked explicitly.
4. Error-prone C patterns must be replaced with sound Rust equivalents while preserving observable semantics.
5. Global mutable state must be minimized and encapsulated.
6. Any `unsafe` block must have:
   - a written safety comment
   - a narrow scope
   - a testable justification
7. Panic behavior must not accidentally replace controlled error behavior in user-visible paths.
8. Fatal paths in C must map to controlled Rust termination behavior, not accidental unwinding unless explicitly intended and validated.

#### Project-specific safety focus areas

Special scrutiny is required for:

- low-level I/O loops corresponding to `safe_rw` and `full_rw`
- output buffering in `cat`
- line-number state progression
- indexed reusable storage in the quoting subsystem
- locale and multibyte handling
- aligned allocation replacement
- stream close/flush handling
- platform-specific descriptor operations
- any behavior previously relying on pointer arithmetic, manual allocation, or internal `FILE *` assumptions

#### Safety versus fidelity rule

Rust safety improvements are mandatory where they do not change external behavior.

If safety and exact internal mechanism conflict, preserve external behavior and replace the mechanism safely.

---

### 1.4 Performance Constraint Principle

The Rust migration must not impose unjustified performance regressions, especially on hot I/O paths.

#### Required interpretation

This project is performance-sensitive in the following areas:

- plain copy fast path corresponding to `simple_cat`
- decorated buffered path corresponding to `cat` and `write_pending`
- low-level transfer loops corresponding to `safe_rw` and `full_rw`
- quoting core rendering corresponding to `quotearg_buffer_restyled`
- allocation and resizing helpers used in repeated operations
- any alignment-sensitive buffering required for throughput

#### Mandatory performance rules

1. The plain copy path must remain a fast path, not a degenerate case of a slower general engine unless benchmarking proves equivalence.
2. Decorated output must remain buffered and chunked.
3. The migration must avoid per-byte overhead increases on hot paths without strong justification.
4. Allocation-heavy subsystems must avoid unnecessary cloning and transient heap churn.
5. Convenience abstractions must not obscure hot loops if they materially degrade throughput.
6. Safety checks are required, but avoid redundant checks inside established hot paths where structure can prove correctness.
7. Platform-specific optimizations may be retained or reintroduced if they preserve semantics and remain maintainable.

#### Acceptance rule

Performance may improve.
Performance may remain roughly equivalent.
Performance may not regress materially on representative workloads without explicit approval and documentation.

---

## 2. Migration Guidelines

### 2.1 C-to-Rust Mapping Rules

The migration must map C concepts into Rust according to the following rules.

#### 2.1.1 Function mapping

- Each significant C behavioral unit should map to a Rust function, method, or small cohesive module with the same semantic responsibility.
- Large C files may be split in Rust for clarity, but externally meaningful behavior families must remain identifiable.
- Wrapper families in C should remain wrapper families in Rust where that preserves testability and semantics.

#### 2.1.2 State mapping

- C mutable state machines must become explicit Rust state structures where possible.
- Hidden coupling through globals must be replaced with encapsulated state unless process-wide global semantics are externally required.
- Distinct state machines must remain distinct:
  - copy engine state
  - line numbering state
  - quoting options state
  - locale query state
  - stdout finalization state

#### 2.1.3 Error mapping

- C status-return and fatal-exit patterns must be translated deliberately.
- Use `Result` for internal propagation where helpful, but preserve original external outcomes.
- Fatal C helpers must map to Rust paths that remain fatal at the same semantic boundary.
- Recoverable versus unrecoverable behavior must not be blurred.

#### 2.1.4 Memory and buffer mapping

- Raw pointer arithmetic should become slice-based or iterator-based logic where possible.
- Explicit-size APIs in C must remain explicit-size aware in Rust.
- `_mem` semantics must not be replaced by implicit NUL-terminated string assumptions.
- Embedded NUL handling must remain correct.
- Output growth behavior must be preserved in transformation and quoting paths.

#### 2.1.5 I/O mapping

- Descriptor-level semantics should map to Rust I/O primitives or low-level OS bindings as needed.
- Stream-finalization behavior must be preserved even if Rust standard library abstractions differ from C stdio.
- Retry, partial-progress, and flush/close semantics must be implemented intentionally, not assumed from library defaults.

#### 2.1.6 Locale and text mapping

- Locale-sensitive behavior must not be silently simplified into byte-only logic where the C version is locale-aware.
- Multibyte conversion behavior must remain explicit.
- Text classification behavior must be validated under locale-sensitive scenarios where relevant.

#### 2.1.7 Allocation mapping

- C `x*alloc` fail-fast helpers should map to Rust utilities with equivalent fatal or controlled-failure semantics.
- Aligned allocation requirements must be implemented with explicit invariants.
- Overflow-sensitive growth logic must be checked in Rust.

#### 2.1.8 Build mapping

- The build system is Makefile-based. The Rust migration must integrate cleanly with the existing or transitional build flow.
- The build must support incremental migration and comparative verification where practical.
- Build changes must not obscure reproducible testing against the C baseline.

---

### 2.2 Principles for Handling Uncertain Behavior

Some aspects of the source behavior are explicitly marked as incompletely evidenced. These areas must be handled conservatively.

#### Governing rule

When behavior is uncertain, the project must prefer evidence, isolation, and verification over assumption.

#### Mandatory uncertainty protocol

1. **Do not guess silently.**
   - Any uncertain behavioral point must be recorded in the relevant spec, plan, or task.

2. **Prefer the narrowest faithful implementation.**
   - Implement only what is supported by evidence.
   - Do not generalize beyond known behavior without need.

3. **Use the C implementation as the oracle.**
   - When summaries are insufficient, inspect source, run the C binary, and derive test evidence.

4. **Create characterization tests before refactoring uncertain logic.**
   - This is especially required for:
     - option behavior in `main`
     - `simple_cat` vs `cat` dispatch conditions
     - retry and partial-transfer behavior in `safe_rw`
     - close/flush failure behavior
     - quoting slot storage lifetime
     - multibyte edge cases
     - descriptor compatibility wrappers

5. **Preserve ambiguity boundaries.**
   - If the original behavior is odd, preserve oddness unless there is explicit approval to change it.

6. **Document evidence level.**
   - Each uncertain migration decision should be tagged in planning artifacts as one of:
     - confirmed from source
     - confirmed from test
     - inferred from module/interface evidence
     - temporary assumption pending verification

7. **No speculative cleanup in uncertain areas.**
   - Uncertain behavior is not the place for architectural simplification.

#### High-risk uncertain domains for this project

These require extra caution:

- exact command-line parsing order inside `main`
- exact path selection criteria between `simple_cat` and `cat`
- exact retry and termination semantics in low-level transfer helpers
- exact exit code mapping in failure cases
- exact locale categories queried
- exact line-number representation
- exact blank-line squeeze state behavior
- exact `copy_file_range` and `fcntl` fallback chains
- exact multibyte invalid/incomplete sequence handling
- exact caching and lifetime rules in `quotearg_n*`

---

### 2.3 Test Verification Requirements

Testing is a first-class migration mechanism, not a later validation step.

#### Required test layers

The migration must maintain and expand tests at the following layers:

1. **Behavioral parity tests**
   - Compare Rust behavior against the C implementation.

2. **Golden CLI tests**
   - Validate stdout, stderr, and exit status for representative invocations.

3. **Module/family tests**
   - Cover quoting, locale handling, transfer helpers, close behavior, and allocation helpers.

4. **Boundary and edge tests**
   - Cover zero-length, embedded NUL, partial reads/writes, multibyte edge cases, blank-line handling, and close-time failures.

5. **Platform-sensitive tests**
   - Cover binary mode, descriptor behavior, and stream finalization where relevant to supported platforms.

6. **Regression tests**
   - Every discovered migration bug must produce a test.

#### Test-oracle rule

When a Rust test and a Rust expectation disagree, the C implementation is the default oracle unless the project has formally approved a behavioral deviation.

#### Minimum behavioral test coverage priorities

At minimum, tests must cover:

- plain concatenation behavior
- decorated output behavior with all major flag combinations
- line numbering and nonblank numbering
- squeeze-blank behavior
- tab, end-marker, and nonprinting transformations
- help and version flows
- quoting behavior across styles and `_mem` variants
- locale-sensitive cases relevant to visible output
- flush/close error behavior
- fatal helper behavior
- buffer-size and explicit-length semantics

---

## 3. Quality Gates

### 3.1 Tests That Must Pass

No migration unit is complete unless all applicable tests pass.

#### Mandatory passing gates

1. **Build gate**
   - The project must build successfully through the agreed Makefile-based flow.

2. **Rust unit and integration test gate**
   - All Rust tests must pass.

3. **Behavioral parity gate**
   - Relevant parity tests against the C implementation must pass.

4. **CLI golden-output gate**
   - Approved golden tests for stdout, stderr, and exit status must pass.

5. **Regression gate**
   - All previously fixed migration regressions must remain fixed.

6. **Module-family gate**
   - If a change touches one of these families, its dedicated tests must pass:
     - copy engine
     - quoting subsystem
     - locale subsystem
     - close/flush subsystem
     - transfer subsystem
     - allocation subsystem

7. **Edge-case gate**
   - If a change affects explicit-length, buffering, or error handling, associated edge tests must pass.

#### Completion standard

A module is not considered migrated merely because it compiles.
It is migrated only when its behavior is verified to the level appropriate for its risk and visibility.

---

### 3.2 Code Review Standards

All substantive changes must be reviewed against this constitution.

#### Reviewers must verify

1. **Behavioral fidelity**
   - Does the change preserve observable behavior?
   - If not, is the deviation explicitly approved and documented?

2. **Interface preservation**
   - Does the change preserve the semantic contract and family structure of the original interface?

3. **Safety**
   - Is the solution safe by default?
   - Is any `unsafe` justified, documented, and minimal?

4. **Error semantics**
   - Are fatal, recoverable, and status-returning paths preserved correctly?

5. **Performance awareness**
   - Does the change affect a hot path?
   - If yes, is there evidence it does not cause unjustified regression?

6. **Test adequacy**
   - Are there sufficient tests for the changed behavior, especially for uncertain or edge-case logic?

7. **Documentation of uncertainty**
   - Are assumptions clearly marked and tracked?

#### Review rejection criteria

A change must be rejected if it:

- alters observable behavior without approval
- collapses meaningful interface distinctions without justification
- replaces controlled failures with panics in user-visible paths
- introduces broad or undocumented `unsafe`
- removes or weakens parity coverage
- changes hot-path structure without benchmark evidence
- resolves uncertainty through assumption instead of verification

---

### 3.3 Performance Benchmark Requirements

Performance validation is mandatory for changes affecting hot paths or architectural structure.

#### Benchmark-required areas

Benchmarks are required for changes affecting:

- `simple_cat` equivalent path
- decorated `cat` equivalent path
- `write_pending` buffering strategy
- `safe_rw` / `full_rw` equivalent logic
- quoting core renderer and reusable slot logic
- allocation and resizing behavior in repeated operations
- any large-scale refactor of I/O or buffering

#### Benchmark expectations

Benchmarks must include representative workloads such as:

- large plain file concatenation
- multiple-file concatenation
- decorated output with numbering and visibility flags enabled
- workloads stressing blank-line squeezing
- workloads stressing quoting of many arguments or names
- cases where output expansion occurs
- cases with small and large buffer sizes where relevant

#### Performance acceptance standard

A change is acceptable when:

- it matches baseline performance within agreed tolerance, or
- it improves performance, or
- it introduces a justified regression that has been explicitly approved for a stronger reason such as correctness or safety

#### Performance review rule

“No obvious issue” is not sufficient for hot-path changes.
If the structure of a hot path changes, benchmark evidence is required.

---

## Amendment and Precedence Rules

### Amendment rule

This constitution may be changed only through an explicit project decision. Any amendment must:

- state the prior rule
- state the new rule
- justify the change
- identify affected migration artifacts
- be reviewed with the same rigor as a major architectural change

### Precedence rule

If any later document conflicts with this constitution, this constitution wins.

The order of precedence is:

1. `constitution.md`
2. approved project-wide migration decisions
3. module/spec documents
4. implementation plans
5. task breakdowns
6. local code comments and ad hoc decisions

---

## Operational Summary

The law of this migration is simple:

- preserve behavior
- preserve interface meaning
- improve safety without changing semantics
- protect performance on hot paths
- treat uncertainty as a verification problem
- require tests and evidence before calling work complete