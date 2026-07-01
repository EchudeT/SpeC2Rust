# constitution.md

## Purpose

This document is the governing constitution for the Rust migration of `cflow-new`.
It defines non-optional project-level principles, migration rules, and quality gates.

All later artifacts, including plans, specs, task lists, design notes, reviews, and implementation decisions, **must** conform to this document. If a lower-level document conflicts with this constitution, **this constitution wins**.

---

## Project Scope and Context

The migration target is the C project `cflow-new`, characterized by:

- Build system: `Makefile.in`
- C source files: 68
- Header files: 81
- Entry point: `src/main.c`
- Module units: 120
- Cluster units: 120

The project includes several major behavioral domains:

- `src/`: main program pipeline, parser, scanner, symbol graph, output drivers
- `src/parseopt/`: option parsing and help/usage subsystem
- `src/wordsplit/`: shell-like configuration splitting and expansion
- `gnu/`: portability, allocation, formatting, hashing, wrappers
- `doc/`: example/helper standalone programs
- `test/`: independent test/example logic

The migration is therefore not merely a syntax rewrite. It is a behavioral preservation effort over a multi-stage toolchain with parsing, graph-building, formatting, and portability concerns.

---

# 1. Core Principles

## 1.1 Behavioral Equivalence Principle

### Law
The Rust implementation must preserve the externally observable behavior of the original C project unless a deviation is:

1. explicitly documented,
2. justified,
3. reviewed, and
4. accepted as an intentional compatibility change.

### Required interpretation
"Behavior" includes, but is not limited to:

- CLI option semantics
- configuration precedence and loading behavior
- accepted input forms
- parser decisions and symbol graph construction
- output structure and formatting
- traversal order where user-visible
- error messages, exit behavior, and failure modes
- file handling behavior
- environment and rc/profile interaction
- output driver selection and results
- wordsplit and parseopt semantics
- scanner and parser state transitions that affect results

### Rules
- The Rust version must preserve the staged runtime model of the original program:
  1. startup
  2. config loading
  3. argument parsing
  4. scanner/parser setup
  5. input processing
  6. selection/filtering
  7. output
  8. cleanup/exit
- Publicly visible behavior takes precedence over internal architectural elegance.
- If the C implementation contains quirks relied on by users, those quirks are considered behavior unless clearly proven accidental and unobservable.
- Output compatibility must be treated as a first-class behavior requirement, especially for:
  - GNU output mode
  - POSIX output mode
  - DOT output mode
  - tree and xref variants
- In ambiguous cases, the migration must prefer:
  1. measured behavior from the C binary,
  2. behavior evidenced by source and tests,
  3. conservative emulation over speculative cleanup.

### Non-goals under this principle
- Reinterpreting user intent
- Redesigning command semantics
- Silent normalization of legacy behavior
- "Fixing" behavior without evidence and approval

---

## 1.2 Interface Compatibility First Principle

### Law
All externally consumed interfaces must remain compatible before internal refactoring freedom is exercised.

### Scope of interfaces
Interfaces include:

- command-line interface
- input file expectations
- configuration file and env parsing behavior
- output formats
- error and exit conventions
- module boundaries that are required for integration or staged replacement
- build and invocation expectations needed for existing workflows

### Rules
- The Rust binary must preserve the invocation contract of the C binary.
- Option names, meanings, precedence, and interactions must not change unless explicitly approved.
- Output intended for downstream tools must remain stable.
- File and stream behavior must remain compatible, including stdin/stdout/stderr usage patterns where applicable.
- Rust module APIs may be idiomatic internally, but compatibility shims must be added where needed to preserve system-level behavior.
- Compatibility is more important than purity when replacing:
  - parseopt
  - wordsplit
  - symbol table behavior
  - output drivers
  - scanner/parser pipeline

### Build compatibility expectations
- The migration must continue to support project orchestration expectations derived from `Makefile.in`.
- Transitional build arrangements are acceptable, but they must not break repeatable project builds or test automation.
- Any temporary dual-language or mixed-link arrangement must be deterministic and documented.

---

## 1.3 Safety First Principle

### Law
The Rust migration must reduce memory unsafety and undefined behavior exposure without changing intended program semantics.

### Required interpretation
Safety improvements are a primary reason for migration, but they do not authorize semantic drift.

### Rules
- Safe Rust is the default.
- `unsafe` is allowed only when:
  1. necessary,
  2. narrowly scoped,
  3. documented with invariants,
  4. covered by tests.
- `unsafe` must never be used as a convenience substitute for design effort.
- Pointer-based C structures must be translated into ownership-safe Rust abstractions wherever possible.
- Global mutable state must be minimized and encapsulated.
- Fallible operations must use explicit error handling; panics must not replace normal C error paths unless the original behavior is process termination.
- No new undefined behavior classes may be introduced through FFI, indexing, integer casts, aliasing, or lifetime mistakes.
- Stateful subsystems such as scanner buffers, parser save stacks, balance stacks, linked lists, hash tables, and wordsplit transformations must have explicit invariants in Rust.

### Specific safety expectations by subsystem
- **Parser/scanner**: state machines must be explicit; buffer transitions must be checked.
- **Hash/symbol graph**: ownership of nodes and edges must be unambiguous.
- **Formatting/output**: bounded formatting and allocation behavior must be deliberate.
- **Configuration/wordsplit**: expansion and tokenization must not rely on unchecked buffer mutation.
- **Portability wrappers**: platform-specific behavior must not force undefined or unchecked Rust assumptions.

---

## 1.4 Performance Constraint Principle

### Law
The Rust migration must not introduce material regressions in runtime or memory behavior for normal workloads without explicit approval.

### Required interpretation
This is a compatibility-constrained migration, not a rewrite that trades away performance for abstraction by default.

### Rules
- The Rust version must target performance within an acceptable envelope of the C version.
- Hot paths must be identified and treated carefully, especially in:
  - lexical scanning
  - token handling
  - parser backtracking/save-stack behavior
  - symbol lookup/install
  - call graph traversal
  - output emission
  - wordsplit expansion
  - parseopt scanning
- Avoid premature heap allocation in tight loops when the C code used stack or reusable buffers.
- Avoid unnecessary cloning of strings, tokens, symbol records, and graph edges.
- Preserve asymptotic complexity of major operations unless an alternative is proven equivalent or better.
- Performance regressions are acceptable only when:
  1. they are small,
  2. they buy meaningful safety or maintainability,
  3. they are measured,
  4. they are documented and approved.

### Performance priority order
1. Correctness and compatibility
2. Safety
3. Performance preservation
4. Internal elegance

---

# 2. Migration Guidelines

## 2.1 C-to-Rust Mapping Rules

### 2.1.1 General mapping rule
The migration must map C concepts to Rust in ways that preserve semantics first and idiomatic style second.

### 2.1.2 Functions
- C functions become Rust functions with explicit input/output contracts.
- Functions with hidden global side effects in C must have those effects made explicit where feasible.
- Large C procedures may be decomposed internally, but externally significant control flow must remain traceable.

### 2.1.3 Structs and data records
- C structs should map to Rust structs.
- Anonymous or loosely structured state in C should become named Rust types when doing so improves invariant clarity.
- Layout compatibility is required only where FFI or binary compatibility demands it.

### 2.1.4 Enums and tagged states
- Integer mode flags, parser states, token classes, output commands, and wordsplit modes should become Rust enums where semantics permit.
- Bitflags may remain bitflags when the original behavior depends on combinability.

### 2.1.5 Pointers and ownership
- Raw pointers in C should map to:
  - references/borrows when lifetime is clear,
  - owned values when transfer is intended,
  - indices/handles/IDs when graph identity matters,
  - `Box`, `Rc`, `Arc`, `RefCell`, or interior mutability only when justified.
- Shared mutable aliasing from C must not be reproduced naively; it must be redesigned into a checked ownership model.

### 2.1.6 Linked structures
- Linked lists, symbol chains, and adjacency structures may be reimplemented using vectors, maps, arenas, or stable indices if observable behavior is preserved.
- If traversal order is user-visible, order preservation is mandatory.

### 2.1.7 Hash-backed storage
- C hash table behavior need not be reimplemented literally, but lookup semantics, equality semantics, and visible traversal implications must be preserved where observable.
- If hash iteration order matters to output, the Rust design must stabilize that order.

### 2.1.8 Strings and text
- C strings must be translated carefully with respect to:
  - ownership
  - encoding assumptions
  - truncation behavior
  - escaping
  - embedded null handling when relevant
- Do not assume UTF-8 semantics unless supported by project behavior.
- Path, token, and symbol text handling must preserve existing accepted byte patterns as far as Rust and platform APIs allow.

### 2.1.9 Error handling
- C sentinel returns, errno-like behavior, and fatal exits must map to Rust `Result`, structured errors, or explicit termination paths.
- User-visible error behavior must remain compatible even if internal representation changes.
- Panics are forbidden for expected user-triggered failure paths.

### 2.1.10 Memory allocation behavior
- C allocation helper families from `gnu/` may map to Rust allocation plus explicit failure handling patterns.
- Fatal allocation semantics must remain consistent where user-visible behavior depends on them.

### 2.1.11 Globals
- C globals should be reduced, but behaviorally important global state may be preserved inside explicit context objects.
- Parser, scanner, output, and configuration state should preferentially move into owned contexts.

### 2.1.12 Macros and compile-time configuration
- C macro logic should become:
  - constants,
  - helper functions,
  - trait/platform modules,
  - cfg-gated implementations,
  as appropriate.
- Macro-heavy portability behavior from `gnu/` must be translated carefully rather than discarded.

### 2.1.13 FFI and staged migration
- FFI may be used during transition, but only as an intermediate state.
- FFI boundaries must be minimized, documented, and tested.
- No permanent FFI dependency should remain unless explicitly approved.

---

## 2.2 Subsystem-Specific Migration Guidance

### 2.2.1 `src/main.c` and startup pipeline
- Preserve startup ordering semantics, including:
  - program name setup
  - output driver registration
  - default configuration initialization
  - env/rc/profile loading
  - CLI parsing
  - parser/scanner setup
  - per-input processing
  - output finalization
- Configuration precedence must be preserved unless disproven by executable testing.

### 2.2.2 `src/parseopt`
- The parseopt subsystem must preserve option recognition, lookahead, grouping, permutation, and reporting behavior.
- Help/usage/version output is part of the interface and must be byte- or line-compatible where practical.
- Negative option matching and repeated option semantics must be tested, not inferred.

### 2.2.3 `src/wordsplit`
- Treat wordsplit as a behaviorally rich subsystem, not a utility to replace casually with shell-like libraries.
- Preserve:
  - quoting semantics
  - variable expansion behavior
  - command expansion behavior
  - tilde/path expansion behavior
  - whitespace handling
  - error context behavior
- Multi-pass transformation behavior must be modeled explicitly.

### 2.2.4 Scanner and parser
- The Flex-derived scanner and parser logic must be migrated as explicit state machines.
- Hidden parser behaviors like pushback, save/restore, balance tracking, declaration disambiguation, and K&R handling must not be simplified without evidence.
- Parser rewrites must be proven against fixtures and differential testing.

### 2.2.5 Symbol graph and filtering
- The symbol lifecycle is core semantics.
- Preserve:
  - install/lookup behavior
  - storage-class/state transitions
  - references and caller relationships
  - scope cleanup
  - starter/target marking
  - filtering/elimination rules
- Any internal change to data representation must preserve visible graph output.

### 2.2.6 Output subsystem
- Preserve the driver-oriented architecture conceptually even if implementation changes.
- Common command orchestration and driver-specific rendering must remain separable.
- Output mode selection and formatting compatibility are mandatory.

### 2.2.7 `gnu/` portability layer
- Do not blindly port all `gnu/` code 1:1 if Rust/std/platform APIs already safely provide equivalent behavior.
- However, when those modules embody compatibility behavior rather than mere utility, that behavior must be preserved.
- Portability wrappers around file descriptors, stat/open/fcntl behavior, formatting, allocation, and error reporting require explicit compatibility review.

---

## 2.3 Principles for Handling Uncertain Behavior

### Law
Where source summaries or code do not fully establish behavior, uncertainty must be resolved systematically, never by guesswork.

### Rules
- If behavior is uncertain, the team must:
  1. mark the uncertainty,
  2. identify affected interfaces,
  3. gather evidence,
  4. record the decision.
- Acceptable evidence sources, in order:
  1. executable behavior from the C version
  2. existing tests
  3. source code inspection
  4. module/interface/behavior documentation
  5. historical project docs or user-facing manuals
- If evidence remains inconclusive, the default choice is the most compatibility-preserving conservative interpretation.
- Any intentional divergence must be tracked in a compatibility-deviation record.

### Required uncertainty process
Each uncertain behavior must produce a short decision note containing:

- subsystem
- question
- evidence reviewed
- chosen behavior
- rationale
- test added to lock it in

### Forbidden responses to uncertainty
- "Rust idiom says so"
- "This is probably what they meant"
- "The old behavior looks ugly"
- "No one will depend on this"

---

## 2.4 Test Verification Requirements

### Law
No migrated behavior is considered complete until it is verified.

### Required test layers

#### 1. Baseline characterization tests
Tests capturing current C behavior before or during migration, including:

- CLI invocation behavior
- output snapshots
- configuration precedence
- parser and symbol graph cases
- wordsplit and parseopt edge cases
- error paths

#### 2. Differential tests
Where feasible, the Rust binary must be run against the same inputs as the C binary and compared for:

- exit code
- stdout
- stderr
- generated graph/text output

#### 3. Unit tests
Focused tests for migrated Rust modules, especially:

- parser helpers
- scanner state transitions
- symbol graph operations
- option parsing
- wordsplit transformations
- output rendering components

#### 4. Integration tests
End-to-end tests covering the full program pipeline.

#### 5. Regression tests
Every bug found during migration must produce a regression test before or with the fix.

### Test design requirements
- Prefer deterministic fixtures.
- Preserve minimal reproductions for tricky parsing cases.
- Include representative coverage for:
  - K&R-style declarations
  - nested declarators
  - balance stack behavior
  - configuration file parsing
  - target/starter filtering
  - output mode selection
  - graph traversal edge cases
  - malformed input and error handling

### Equivalence rule
A module is not "done" because it compiles in Rust.
It is done when tests demonstrate behavioral equivalence at the appropriate level.

---

# 3. Quality Gates

## 3.1 Tests That Must Pass

No migration milestone may be merged unless all applicable gates pass.

### Mandatory gates for every merged change
- Rust project builds successfully in the supported build flow.
- All affected unit tests pass.
- All affected integration tests pass.
- No previously passing regression tests fail.
- Formatting and lint checks pass.
- Any differential test suite relevant to the changed subsystem passes or has documented approved exceptions.

### Mandatory gates for subsystem completion
A subsystem is considered migrated only when:

- its Rust implementation is the active implementation for that subsystem,
- behaviorally relevant tests exist,
- differential results against the C version are acceptable,
- compatibility deviations, if any, are explicitly approved,
- remaining technical debt is documented and bounded.

### Minimum subsystem-specific required test expectations

#### Main/startup pipeline
- startup with no args
- startup with env-derived options
- startup with rc/profile options
- startup with conflicting config sources
- normal source-processing flow
- early help/version termination behavior

#### Parseopt
- short options
- long options
- negated options
- repeated options
- option arguments
- operand handling
- permutation behavior
- help/usage/version output cases

#### Wordsplit
- quoting
- escaping
- variable expansion
- command expansion behavior where supported
- tilde expansion
- whitespace trimming/coalescing
- error and context propagation

#### Scanner/parser
- declaration parsing
- function detection
- typedef handling
- nested delimiter balancing
- token save/restore paths
- malformed syntax handling

#### Symbol graph
- symbol installation/lookup
- scope cleanup
- reference/call edge creation
- starter/target marking
- elimination/filtering
- deterministic output-affecting traversal

#### Output
- GNU mode snapshots
- POSIX mode snapshots
- DOT mode snapshots
- tree and xref mode verification
- separators/newlines/session framing behavior

---

## 3.2 Code Review Standards

### Law
Every change must be reviewed against this constitution, not only for code style.

### Required review questions
Reviewers must verify:

1. **Behavior**
   - Does this preserve observable behavior?
   - If behavior changed, is it documented and approved?

2. **Interface**
   - Does this preserve CLI, config, output, and error compatibility?

3. **Safety**
   - Is ownership clear?
   - Is `unsafe` necessary and justified?
   - Are invariants documented?

4. **Testing**
   - Are tests sufficient for the risk level?
   - Has uncertain behavior been characterized?

5. **Performance**
   - Does this introduce allocation, copying, or algorithmic regressions?
   - Was benchmarking done if needed?

6. **Maintainability**
   - Does the design clarify state and invariants?
   - Does it avoid cargo-culting C structure where a safer equivalent exists?

### Required standards
- No unreviewed `unsafe`.
- No unexplained compatibility deviations.
- No merging on "works for my case" evidence.
- No TODO/FIXME in place of required compatibility decisions in core paths.
- No silent replacement of behaviorally rich subsystems with generic crates unless equivalence is demonstrated.

### Documentation requirements in review
Changes must include, as appropriate:

- migration notes
- invariants for unsafe/stateful code
- compatibility notes
- benchmark notes
- added tests and rationale

---

## 3.3 Performance Benchmark Requirements

### Law
Performance-sensitive changes must be measured, and the migrated program must remain within an approved performance envelope.

### Benchmarking triggers
Benchmarking is required when a change affects:

- scanner/tokenization
- parser control flow
- symbol table or graph structures
- output traversal/rendering
- wordsplit processing
- parseopt scanning
- memory allocation strategy
- string/path handling in hot paths

### Benchmark expectations
Benchmarks must compare, where practical:

- C baseline vs Rust implementation
- previous Rust vs new Rust change
- representative small, medium, and large inputs

### Required metrics
At minimum, collect:

- wall-clock runtime
- peak or approximate memory usage
- allocation-sensitive indicators where available
- output equivalence confirmation for benchmark runs

### Acceptance rule
Absent explicit approval, changes must not introduce:

- significant runtime regressions on representative workloads
- significant memory growth for common cases
- algorithmic degradation in core operations

### When regressions are acceptable
A regression may be accepted only if:

- it is measured,
- it is localized,
- it buys clear safety/correctness/maintainability value,
- no practical lower-cost alternative is available,
- approval is recorded.

---

# 4. Enforcement and Decision Rules

## 4.1 Hierarchy of authority
Authority order for project decisions:

1. this constitution
2. approved compatibility decisions
3. subsystem specs
4. implementation plans
5. task-level notes

Lower levels may refine but may not contradict higher ones.

## 4.2 Burden of proof
The burden of proof lies with the proposer of any change that:

- alters behavior,
- changes interface semantics,
- adds unsafe code,
- worsens performance,
- removes tests,
- replaces a subsystem with a new design.

## 4.3 Default tie-breakers
When principles appear in tension, resolve in this order:

1. behavioral equivalence
2. interface compatibility
3. safety
4. performance
5. internal elegance

## 4.4 Definition of done
A migration item is done only when:

- implementation is complete,
- tests pass,
- compatibility is verified,
- review standards are met,
- performance constraints are satisfied,
- documentation is updated where needed.

---

# 5. Non-Optional Project Commitments

The project commits to the following:

- We will not treat the migration as a redesign exercise.
- We will preserve user-visible behavior unless change is intentional and approved.
- We will prefer explicit state and invariants over implicit C-era behavior.
- We will use Rust safety to remove classes of failure, not to excuse semantic drift.
- We will verify behavior with tests, not confidence.
- We will measure performance-sensitive changes.
- We will document uncertainty and resolve it with evidence.