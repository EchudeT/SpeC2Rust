# constitution.md

## Purpose

This document defines the binding project-level principles for the Rust migration of the `which` C project. It is the governing constitution for all later specification, design, planning, implementation, review, and release work.

Where later documents conflict with this constitution, this constitution takes precedence.

This constitution is intentionally stricter than ordinary project guidance. Its purpose is to preserve externally observable behavior while improving implementation safety and maintainability.

---

## Scope

This constitution applies to the full migration effort across the observed project structure, including:

- Build integration through `Makefile.in`
- Root program behavior represented in:
  - `bash.c`
  - `getopt.c`
  - `getopt1.c`
  - `which.c`
- Tilde-related behavior represented in:
  - `tilde/shell.c`
  - `tilde/tilde.c`

It applies especially to the two analyzed module units:

- `main_root`
- `module_tilde`

It governs all Rust replacements, shims, FFI boundaries, tests, benchmarks, and transitional hybrid states.

---

## Core Principles

### 1. Behavioral Equivalence Principle

The Rust migration must preserve the externally observable behavior of the C program unless a deliberate, documented, and approved deviation is required.

#### Required meaning of behavioral equivalence

Behavioral equivalence includes, at minimum:

- command-line parsing behavior
- option acceptance and rejection behavior
- argument ordering effects during parsing
- path resolution flow and candidate checking order
- absolute vs non-absolute path handling
- user/group lookup behavior as observable from outputs and exit status
- environment lookup behavior
- home-directory lookup behavior
- tilde scanning and expansion behavior
- fatal behavior for memory-failure paths that are explicitly fatal in C
- exit codes
- stdout/stderr content and ordering, where observable
- error/non-error distinction
- return-value conventions at public boundaries

#### Known behaviorally sensitive areas

The following areas are presumed high-risk and must be treated as equivalence-critical:

- `main(argc, argv)` startup flow
- `getopt`, `getopt_long`, `getopt_long_only`, and `_getopt_internal`
- `exchange`-driven argument reordering semantics
- iterative colon-delimited path traversal
- `file_status`
- `absolute_program`
- `substring`
- `extract_colon_unit`
- `get_next_path_element`
- `make_full_pathname`
- `initialize_group_array`
- `group_member`
- `get_current_user_info`
- `sh_get_env_value`
- `sh_get_home_dir`
- `get_home_dir`
- `tilde_find_prefix`
- `tilde_find_suffix`
- `memory_error_and_abort`

#### Rules

1. Do not simplify behavior merely because the Rust version can be cleaner.
2. Do not replace staged flows with different observable semantics.
3. Do not alter ambiguous edge-case behavior without proof from C behavior or approved decision records.
4. If the C behavior is ugly but externally observable, preserve it unless explicitly exempted.
5. If behavior cannot yet be proven, the migration must remain conservative.

---

### 2. Interface Compatibility First Principle

Public and project-consumed interfaces must remain compatible before internal elegance is considered.

#### Required interface priorities

Compatibility must be preserved for:

- process entry behavior
- CLI surface
- option names and parsing modes
- expected argument conventions
- data and call boundaries used by surrounding build and runtime structure
- module-level responsibilities implied by the original architecture

#### Interface rules

1. The Rust migration must continue to provide a `main` entry path compatible with the current program model.
2. Public option parser behavior must preserve the layered model:
   - `getopt`
   - `getopt_long`
   - `getopt_long_only`
   - shared core behavior equivalent to `_getopt_internal`
3. Root-module and tilde-module role boundaries must not be collapsed in ways that alter observable semantics.
4. Build integration must work within the existing `Makefile.in`-driven project structure unless a formally approved build transition plan says otherwise.
5. Transitional wrappers, adapters, or FFI are allowed if they preserve compatibility and reduce migration risk.
6. Interface preservation has priority over internal refactoring convenience.

#### Architectural implication

Rust modules may be reorganized internally, but the migration must preserve the observable contract implied by:

- startup path
- parser layering
- path scanning stages
- user/group preparation and membership query flow
- separate environment/home lookup roles
- tilde scanning phases
- explicit fatal handling path for memory failure in tilde logic

---

### 3. Safety First Principle

Rust safety benefits are a primary goal, but safety improvements must be introduced without breaking required behavior.

#### Safety objectives

The migration must reduce or eliminate:

- unchecked memory access
- invalid pointer usage
- buffer overflows
- use-after-free risks
- double frees
- null dereferences at Rust-managed boundaries
- accidental undefined behavior
- unsound ownership transfer across boundaries

#### Safety rules

1. Safe Rust is the default.
2. `unsafe` is permitted only when necessary and must be minimized, isolated, and justified.
3. Every `unsafe` block must document:
   - why it is needed
   - what invariants it depends on
   - how those invariants are enforced
4. Panics must not replace expected C-style runtime behavior at public boundaries.
5. Fatal behavior required by the original program must be explicit and intentional, not accidental panic leakage.
6. String, path, environment, and argv handling must be implemented with special care for C compatibility cases.
7. Allocation failure handling must respect required behavior for explicitly fatal paths, especially tilde memory failure behavior.
8. FFI boundaries, if used, must be narrow, documented, and covered by tests.

#### Safety vs compatibility

When safety and compatibility appear to conflict:

- first seek a design that satisfies both
- if not possible, preserve externally required behavior while containing risk
- document the tradeoff
- require explicit review approval

---

### 4. Performance Constraint Principle

The Rust migration must not introduce unjustified regressions in the behaviorally significant runtime paths of the C program.

#### Performance-sensitive paths

The following paths are presumed performance-sensitive and must be treated carefully:

- core option parsing through `_getopt_internal`-equivalent logic
- argument reordering behavior corresponding to `exchange`
- colon-delimited path scanning loops
- repeated candidate pathname construction
- repeated `file_status` checks
- repeated user/group membership queries after setup
- repeated tilde scanning over strings

#### Performance rules

1. Do not introduce additional full-pass parsing where the C behavior is incremental.
2. Do not replace iterative path scanning with bulk preprocessing if it changes scaling or candidate evaluation order.
3. Do not move setup work into repeated query paths without justification.
4. Avoid unnecessary allocation in hot loops.
5. Preserve amortized setup/query boundaries where present in C.
6. Any regression must be measured, justified, and approved.
7. A modest constant-factor change may be acceptable only if:
   - behavior is preserved
   - safety or maintainability is materially improved
   - benchmarks remain within approved thresholds

---

## Migration Guidelines

### 1. C-to-Rust Mapping Rules

The migration must translate C structure into Rust in a way that preserves semantics first and improves structure second.

#### General mapping rules

1. Map each analyzed module unit to a clearly named Rust module or submodule.
2. Preserve major responsibility boundaries:
   - `main_root` responsibilities stay coherent
   - `module_tilde` responsibilities stay coherent
3. Preserve function-level behavior before combining or refactoring functions.
4. Avoid premature abstraction that obscures one-to-one traceability to the C behavior.
5. Maintain a migration map from C symbols/functions to Rust implementations.

#### Function mapping rules

- Functions with externally visible behavioral roles must have a traceable Rust counterpart.
- Internal helper consolidation is allowed only after equivalence is demonstrated.
- C functions that represent explicit stages in a flow should remain explicit stages in Rust, especially for:
  - parser core/wrappers
  - path iteration
  - user/group setup vs query
  - tilde prefix/suffix detection

#### Data mapping rules

- Replace anonymous or weakly typed C state with explicit Rust types where possible.
- Preserve layout compatibility only where required by FFI or external ABI expectations.
- Represent parser state, path iteration state, and user/group cached state explicitly.
- Avoid global mutable state unless required for compatibility; when required, isolate and document it.

#### String and path rules

- Treat C string semantics as compatibility-sensitive.
- Be explicit about ownership, lifetime, encoding assumptions, and null-termination at boundaries.
- Do not assume UTF-8 unless proven safe for the interface being migrated.
- Preserve delimiter-sensitive behavior for colon-separated path processing and tilde scanning.

#### Error model mapping rules

- Model recoverable vs fatal behavior intentionally.
- Do not silently convert abort-like behavior into recoverable `Result` returns if the original behavior is observably fatal.
- Do not silently convert recoverable C outcomes into process-terminating Rust behavior.
- Exit behavior, diagnostics, and propagation style must match the C-visible contract.

#### Build and integration rules

- The Rust implementation must remain integrable with the existing build model.
- Cargo may be used internally, but the project-facing build contract must remain compatible with `Makefile.in` unless formally changed.
- Intermediate hybrid builds are acceptable if they improve migration safety and testability.

---

### 2. Principles for Handling Uncertain Behavior

The provided analysis explicitly identifies areas where the current summary is insufficient to prove exact behavior. These areas require disciplined handling.

#### Default rule: preserve uncertainty conservatively

When behavior is uncertain:

1. Do not invent new semantics.
2. Do not choose the most elegant interpretation by default.
3. Do not assume standard-library behavior matches the existing program.
4. Prefer temporary compatibility scaffolding over irreversible redesign.

#### Required evidence sources

Uncertain behavior must be resolved, in order of preference, by:

1. direct inspection of original C source
2. characterization tests against the C build
3. existing project docs or historical behavior evidence
4. controlled differential testing
5. explicit project decision records when exact behavior cannot be recovered

#### Required handling patterns

##### a. If C behavior can be observed
Write tests against the C implementation first, then require the Rust implementation to match.

##### b. If C behavior cannot be confidently observed
Preserve the closest structure implied by the current evidence and mark it as provisional.

##### c. If behavior is underdetermined and user-visible
Escalate for explicit decision and document the chosen compatibility stance.

##### d. If behavior is internal and not observably distinguishable
A safer Rust internal design may be used, but only if public behavior remains unchanged.

#### Areas specifically requiring caution

- exact invalid-option handling details
- parser termination and state conventions
- argument reordering details
- exact file status classification semantics
- empty/trailing/consecutive path element handling
- substring boundary handling
- user/group initialization timing and caching semantics
- home-directory source precedence and fallback rules
- tilde grammar boundaries
- error message text and exit behavior in failure cases

#### Documentation rule

Any unresolved or newly resolved uncertain behavior must be recorded in migration notes or decision records, including:

- what was uncertain
- what evidence was used
- what decision was made
- what tests now lock the behavior

---

### 3. Test Verification Requirements

Testing is mandatory and is the primary mechanism for proving equivalence.

#### Required test layers

The project must maintain, at minimum:

1. **Characterization tests**
   - capture observed C behavior
2. **Rust unit tests**
   - validate isolated logic
3. **Integration tests**
   - validate end-to-end CLI behavior
4. **Differential tests**
   - compare C and Rust outputs/exit behavior
5. **Regression tests**
   - lock in bugs fixed during migration
6. **Performance benchmarks**
   - guard hot paths

#### Minimum coverage expectations by subsystem

##### Option parsing
Tests must cover:

- short options
- long options
- long-only options
- mixed option/non-option argv
- argument reordering effects
- invalid options
- repeated parser calls where applicable
- termination behavior
- long option index behavior if exposed

##### Path resolution
Tests must cover:

- absolute program paths
- non-absolute names requiring path scanning
- multiple path elements
- candidate construction order
- successful and unsuccessful lookup
- boundary conditions in colon-delimited iteration
- repeated delimiters, empty elements, and trailing delimiters where observable

##### User/group behavior
Tests must cover, to the extent observable:

- current user identity retrieval paths
- group membership queries
- initialized vs uninitialized behavior if externally visible
- failure or unavailable-info handling where reproducible

##### Environment and home lookup
Tests must cover:

- environment value retrieval
- home-directory retrieval behavior
- precedence/fallback behavior when determinable
- empty/missing environment cases

##### Tilde behavior
Tests must cover:

- prefix detection
- suffix detection
- eligible and ineligible input strings
- home-directory-backed expansion behavior
- memory-failure fatal path behavior if reproducibly testable or otherwise validated by targeted review

#### Test result rule

No migrated subsystem is considered complete until its behavior is locked by tests appropriate to its risk and observability.

---

## Quality Gates

### 1. Tests That Must Pass

A change may not merge unless all required tests pass for the affected scope.

#### Mandatory gates

1. The Rust code must compile cleanly in the supported project build flow.
2. All existing preserved tests must pass.
3. All new characterization and regression tests for changed behavior must pass.
4. Differential tests between C and Rust must pass for migrated features.
5. Integration tests must confirm matching:
   - exit status
   - stdout
   - stderr
   - observable side effects relevant to the feature
6. Any temporary compatibility shims must themselves be tested.

#### Release-level gate

A release-ready Rust replacement requires:

- no known unapproved behavioral deviations
- passing end-to-end test suite
- passing differential suite for all migrated user-facing behaviors
- documented disposition of every known uncertain behavior area affecting released functionality

---

### 2. Code Review Standards

Every migration change must be reviewed against this constitution, not just for style or correctness.

#### Reviewers must verify

1. **Behavior**
   - Does this preserve the C-observable contract?
2. **Traceability**
   - Is the Rust implementation traceable to the original C behavior or characterization tests?
3. **Safety**
   - Is `unsafe` avoided or properly justified?
4. **Edge cases**
   - Are uncertain or boundary cases covered by tests or documented decisions?
5. **Performance**
   - Does this change affect a hot path, and if so, is it measured?
6. **Integration**
   - Does this remain compatible with the current build and interface expectations?

#### Required review artifacts

For non-trivial changes, the PR or review package must include:

- affected C functions/modules
- Rust mapping summary
- behavior-preservation notes
- tests added or updated
- benchmark impact, if relevant
- decision record links for any deliberate deviation

#### Automatic rejection conditions

A change must be rejected or sent back if it:

- changes observable behavior without documentation and approval
- removes compatibility scaffolding before equivalence is proven
- introduces `unsafe` without invariant documentation
- replaces explicit fatal behavior with implicit panic leakage
- adds unmeasured overhead to hot paths
- weakens tests while claiming preserved behavior
- relies on assumptions where evidence was required

---

### 3. Performance Benchmark Requirements

Performance validation is required for all behaviorally significant hot paths and for any change suspected to affect runtime cost.

#### Required benchmark targets

At minimum, benchmarks must exist for:

- option parsing throughput across varying argv sizes
- argument reordering scenarios
- path scanning across varying PATH lengths
- candidate pathname construction/check loops
- repeated group membership query scenarios where practical
- tilde scanning across repeated string inputs

#### Benchmark rules

1. Benchmarks must compare against the C baseline where feasible.
2. Benchmarks must run on representative inputs, not only trivial cases.
3. Microbenchmarks are allowed, but end-to-end benchmarks are preferred for user-visible paths.
4. Benchmark methodology must be documented well enough to reproduce.
5. Claimed performance improvements or acceptability must be evidence-based.

#### Performance acceptance

A change affecting a benchmarked path is acceptable only if one of the following is true:

- performance is equal or better within noise tolerance
- regression is within approved threshold and justified by major safety or maintainability benefit
- regression occurs only in non-critical paths and is documented

#### Escalation threshold

Any clear regression in:

- parser core behavior
- path iteration/candidate checking
- repeated membership queries
- repeated tilde scanning

must be explicitly reviewed before merge.

---

## Governance

### Amendment Rule

This constitution may be changed only by explicit project decision. Any amendment must be:

- written
- reviewed
- justified
- versioned

No later spec, plan, or task document may silently override this constitution.

### Conflict Resolution Rule

If a lower-level document conflicts with this constitution:

1. the constitution wins
2. the lower-level document must be corrected
3. implementation based on the conflicting rule must not proceed without amendment approval

### Compliance Rule

Every migration plan, task list, and implementation PR must be evaluated for compliance with:

- Behavioral Equivalence Principle
- Interface Compatibility First Principle
- Safety First Principle
- Performance Constraint Principle

Non-compliant work is incomplete by definition.