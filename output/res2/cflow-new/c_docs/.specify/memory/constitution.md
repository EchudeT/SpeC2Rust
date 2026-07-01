# constitution.md

# cflow-new Rust Migration Constitution

## 0. Status and Authority

This document is the governing constitution for the Rust migration of `cflow-new`. It defines non-optional project-level principles, migration rules, and quality gates.

All later documents, including specifications, plans, task lists, review checklists, and implementation notes, **must conform to this constitution**. If any later document conflicts with this constitution, this constitution prevails.

This constitution applies to the full project scope, including:

- the main application entry path rooted at `src/main.c`
- parser, lexer, symbol, graph, output, parseopt, and wordsplit subsystems
- bundled `gnu/*` support code as needed for preserved behavior
- build and test integration through the existing `Makefile.in`-based project layout
- standalone `doc/*` utilities and `test/*` programs when included in migration scope

---

## 1. Core Principles

## 1.1 Behavioral Equivalence Principle

### Law
The Rust implementation must preserve the **observable behavior** of the C project unless a deviation is explicitly approved and documented.

### Required interpretation
Observable behavior includes, at minimum:

- process entry and exit behavior
- option parsing behavior
- configuration loading behavior from argv, environment, and rc/profile sources
- parsing behavior and parser state transitions
- symbol discovery, storage, filtering, and traversal behavior
- output mode selection and emitted output structure
- error routing, termination behavior, and diagnostics class
- filesystem and stream interaction behavior
- test and example program behavior where migrated

### Specific application to `cflow-new`
Because `cflow-new` is a stateful analysis pipeline with:

- reversible parser behavior
- scanner buffer state behavior
- scope-based symbol lifecycle behavior
- late-bound output driver selection
- staged wordsplit transformation behavior
- hash-backed symbol and registry behavior

the migration must preserve not just end results, but also the semantics that produce those results where they affect observable outputs or error handling.

### Rules
1. Rust code must preserve:
   - direct vs inverted tree behavior
   - xref vs tree output distinction
   - parser mark/restore and token putback semantics
   - starter/target filtering semantics
   - wordsplit staged expansion semantics
   - parseopt precedence and iteration semantics to the fullest verified extent

2. Where the C behavior is known only conservatively, Rust must preserve:
   - the behavior demonstrated by tests
   - the behavior evidenced by runtime output comparison
   - the behavior implied by stable public interfaces and integration paths

3. A rewrite that is “cleaner” but behaviorally different is not acceptable by default.

### Forbidden shortcuts
- Replacing a stateful subsystem with a simpler one if it changes edge-case behavior
- Collapsing distinct error paths into a single generic error path
- Removing reversible parser behavior in favor of a purely streaming parser without proof of equivalence
- Changing configuration precedence or option semantics for convenience

---

## 1.2 Interface Compatibility First Principle

### Law
Compatibility with the existing project interface takes priority over internal redesign.

### Scope of compatibility
This includes:

- command-line interface and option behavior
- output formats and mode selection
- entry points and executable expectations
- file and environment based configuration behavior
- module boundaries where they matter for migration staging
- build integration expectations under the existing `Makefile.in` regime

### Required interpretation
The first duty of each migrated Rust component is to behave as a compatible replacement for the C component it supersedes.

### Rules
1. The Rust binary must preserve user-facing invocation semantics unless a change is explicitly approved.
2. Output drivers must preserve externally visible distinctions among GNU, POSIX, DOT, xref, and tree-oriented behaviors.
3. The Rust implementation must support staged migration and coexistence where necessary.
4. Public behavior must not be broken to achieve internal abstraction purity.
5. Internal Rust module structure may differ from C file layout, but traceability to C modules must be maintained.

### Traceability requirement
Every migrated unit must identify:

- the source C module(s) it replaces
- the preserved interface surface
- any intentionally changed behavior
- the evidence supporting compatibility

### Special note for this codebase
Given the large module inventory and the concentration of behavior in:

- `src/main.c`
- `src/c.c`
- `src/parser.c`
- `src/symbol.c`
- `src/output.c`
- `src/parseopt/*`
- `src/wordsplit/*`
- `gnu/hash*`, `gnu/error*`, allocation and portability wrappers

compatibility decisions must be made subsystem-by-subsystem, not only at final application level.

---

## 1.3 Safety First Principle

### Law
The migration must maximize Rust safety guarantees while preserving required behavior.

### Required interpretation
The project exists to replace a C implementation with a safer Rust implementation. Therefore:

- safe Rust is the default
- unsafe Rust is the exception
- FFI and low-level escape hatches are temporary tools, not the default architecture

### Rules
1. Prefer safe Rust abstractions for:
   - ownership
   - lifetime management
   - collections
   - string handling
   - I/O
   - error propagation
   - parser state containers

2. Any `unsafe` usage must be:
   - necessary
   - minimal
   - encapsulated
   - documented with a safety comment
   - covered by targeted tests where practical

3. Memory lifecycle behavior formerly implemented through:
   - manual allocation wrappers
   - linked structures
   - custom hash management
   - scanner buffer allocation
   - wordsplit node allocation

   must be translated into Rust ownership models whenever behavior can be preserved.

4. Silent undefined behavior from the C version must **not** be preserved if a safer behavior can be implemented without breaking required observable behavior.

5. Panics must not replace normal C-style recoverable error paths unless explicitly justified and approved.

### Unsafe policy
Unsafe Rust is allowed only when one of the following is true:

- FFI with existing C code is required during staged migration
- exact low-level layout or interoperability is necessary
- performance-critical low-level code requires it and no safe equivalent is acceptable
- a parser or scanner bridge requires narrow unsafe containment

For each unsafe block, the code review must answer:

- Why is unsafe necessary?
- What invariant is being relied upon?
- How is the unsafe boundary minimized?
- What tests or proofs support correctness?

### Safety over bug-compatibility
If the C version contains behavior that is memory-unsafe, data-racy, or UB-prone, the Rust version must not intentionally recreate that unsafety unless doing otherwise would break a required compatibility guarantee and no safer compatible method exists.

---

## 1.4 Performance Constraint Principle

### Law
The Rust migration must not introduce unacceptable regressions in runtime performance, memory usage, or scaling behavior on core workloads.

### Required interpretation
This project contains performance-sensitive paths, especially in:

- symbol lookup and insertion
- parser token processing
- scanner buffer transitions
- call/reference graph construction
- output traversal
- hash table operations
- wordsplit expansion
- help/wordwrap streaming in large output contexts

### Rules
1. Behavioral correctness comes first, but major regressions are not acceptable.
2. Rust implementations of hot paths must be designed with algorithmic parity or better.
3. Replacing custom C structures with Rust standard structures is encouraged when:
   - behavior remains compatible
   - performance is acceptable or improved
4. Performance must be evaluated on representative workloads, not guessed.
5. Avoid premature micro-optimization, but do not accept obviously inefficient replacements in hot paths.

### Baseline expectations
Unless explicitly waived, the Rust implementation should aim for:

- no material algorithmic regression in parsing and graph construction
- no severe increase in memory footprint on representative input sets
- startup, option parsing, and output generation that remain practical for existing workflows

### Forbidden regressions
- O(n²) replacements for hot-path operations that were effectively O(1) or O(log n)
- excessive cloning or allocation in parser and symbol hot paths
- unbounded buffering where streaming behavior is required
- converting a lazy or iterative behavior into full-materialization behavior without justification

---

## 2. Migration Guidelines

## 2.1 C-to-Rust Mapping Rules

### 2.1.1 General mapping policy
Migration must preserve behavior while translating implementation style into idiomatic Rust where practical.

### Mapping rules
1. **C structs -> Rust structs**
   - Preserve semantic fields and invariants.
   - Reorganize layout only when ABI/layout compatibility is not required.
   - Use enums where tagged-state semantics are clearer and behaviorally equivalent.

2. **C enums / integer flags -> Rust enums / bitflags**
   - Prefer typed enums for closed state sets.
   - Prefer bitflags-style representations where flags are compositional and externally meaningful.

3. **Raw pointers -> references / smart pointers / indices**
   - Prefer `&`, `&mut`, owned values, `Box`, `Rc`, `Arc`, `Vec`, `HashMap`, or arena/index-based models.
   - Use raw pointers only at FFI or tightly-contained low-level boundaries.

4. **Manual linked lists -> Rust collections**
   - Replace with `Vec`, `VecDeque`, maps, sets, or arena-backed node graphs where behavior is preserved.
   - If insertion/traversal/unlink semantics matter observably, document how the Rust structure preserves them.

5. **Custom hash tables -> Rust maps only with behavior review**
   - Standard Rust maps may replace custom hash structures only if required behavior is preserved.
   - If iteration order, insert-if-absent semantics, collision behavior visibility, or tuning-sensitive behavior matters, this must be assessed explicitly.
   - If necessary, implement a dedicated compatible structure.

6. **C strings / char buffers -> Rust string types**
   - Use `String` and `&str` for textual data.
   - Use `Vec<u8>` / byte slices when byte-level or non-UTF-8 behavior must be preserved.
   - Do not force UTF-8 semantics onto data that was byte-oriented in C.

7. **Error codes and global error channels**
   - Preserve subsystem separation.
   - Prefer structured Rust errors internally.
   - Externally, preserve expected diagnostics and return/termination behavior.

8. **Global mutable state**
   - Minimize it.
   - Where global process state is part of behavior, encapsulate it in explicit runtime state structures.
   - Hidden global state should be reduced, not expanded.

9. **Recursive C control flow**
   - Preserve recursion where behaviorally natural and safe.
   - Convert to iterative forms only when equivalent and beneficial.

10. **Macros / preprocessor behavior**
    - Replace C macros with functions, consts, enums, traits, or helper methods where possible.
    - Macro-origin behavior that affects semantics must remain traceable.

---

## 2.1.2 Scanner and parser subsystem rules

Because `src/c.c` and `src/parser.c` implement a buffered scanner plus reversible parser, the following are mandatory:

1. Token acquisition must preserve rollback-capable semantics.
2. Mark/restore, putback, save-stack, and balanced-skip behavior must remain explicit in design.
3. If generator-based replacements are considered, they must first demonstrate edge-case parity.
4. Parser ambiguity handling for declaration/function distinctions must remain behaviorally faithful.
5. K&R and special declaration handling must not be removed solely because modern Rust code would prefer a simplified grammar.

---

## 2.1.3 Symbol and graph subsystem rules

Because symbol management is central to `cflow-new`:

1. Symbol identity rules must remain explicit.
2. Lookup/install behavior must preserve “existing vs create” distinctions.
3. Scope cleanup semantics for autos, statics, and parameters must remain distinct.
4. Starter/target/filtering flows must remain separate from plain symbol existence.
5. Traversal-active state used by output generation must be preserved without introducing hidden behavior changes.

---

## 2.1.4 Output subsystem rules

1. Output driver registration and selection must remain explicit.
2. Backend-specific rendering logic must remain separable.
3. Tree, inverted tree, xref, GNU, POSIX, and DOT output modes must not be conflated.
4. Formatting differences that are user-visible must be treated as compatibility-sensitive.
5. Output streaming behavior should remain streaming where practical.

---

## 2.1.5 parseopt and wordsplit subsystem rules

These are high-risk migration areas due to statefulness and edge cases.

1. `parseopt` must preserve:
   - short and long option behavior
   - negated option behavior where supported
   - lookahead/skip semantics
   - permutation behavior
   - help/version/usage dispatch paths

2. `wordsplit` must preserve:
   - staged pipeline semantics
   - quoting and unquoting behavior
   - variable/command/tilde/path expansion stages
   - context-rich error reporting
   - cleanup and reuse semantics where applicable

3. These subsystems must not be “simplified” by swapping in generic crates unless behavior parity is verified.

---

## 2.1.6 GNU support code rules

Bundled `gnu/*` code should not be rewritten blindly.

1. If a Rust standard library feature fully replaces a `gnu/*` helper with preserved behavior, prefer the Rust feature.
2. If the `gnu/*` code encodes portability or behavior normalization relied upon by the app, that behavior must be reproduced.
3. Memory wrappers, error reporting, hash support, path handling, and formatting helpers must be reviewed for:
   - observable semantics
   - platform dependencies
   - error behavior
   - iteration or formatting differences

---

## 2.2 Principles for Handling Uncertain Behavior

## 2.2.1 Conservatism rule

When C behavior is uncertain, the project must act conservatively.

### Required actions
1. Do not invent behavior.
2. Do not assume modernized semantics are acceptable.
3. Do not “clean up” ambiguous behavior without evidence.

### Evidence hierarchy
When behavior is unclear, decisions must be based on the strongest available evidence in this order:

1. passing existing tests
2. differential behavior against the C binary
3. observable CLI/output behavior
4. source-level call flow and state analysis
5. comments or docs from the original project
6. implementation inference

If evidence remains insufficient, preserve flexibility and document the uncertainty.

---

## 2.2.2 Documented uncertainty rule

Every uncertain migration decision must record:

- what behavior is uncertain
- why it is uncertain
- what evidence was available
- what compatibility choice was made
- what future test or validation could reduce uncertainty

Undocumented guessing is prohibited.

---

## 2.2.3 Differential validation rule

For uncertain subsystems, the Rust version must be validated against the C implementation through differential testing whenever practical.

Priority areas include:

- parseopt behavior
- parser ambiguity behavior
- wordsplit transformations
- output formatting
- error diagnostics
- filesystem and config loading edge cases

---

## 2.2.4 No silent semantic drift rule

If a migration choice knowingly changes behavior, that change must be:

- explicit
- justified
- reviewed
- documented
- approved before merge

Silent drift is a constitutional violation.

---

## 2.3 Test Verification Requirements

## 2.3.1 General test law

No migration step is complete until it is verified by tests appropriate to the risk of the changed behavior.

### Minimum categories
The project must maintain and expand:

- unit tests
- integration tests
- golden/output comparison tests
- differential tests against the C implementation where useful
- regression tests for discovered mismatches

---

## 2.3.2 Behavioral test requirements by subsystem

### Main / startup / configuration
Tests must cover:

- process startup
- environment option ingestion
- rc/profile loading
- option precedence where known
- help/version/usage flows
- output driver selection

### Scanner and parser
Tests must cover:

- token flow
- balanced construct handling
- reversible parsing behavior
- declaration/function distinction
- scope transitions
- representative source inputs

### Symbol and graph behavior
Tests must cover:

- lookup/install distinctions
- reference/call graph construction
- starter/target filtering
- scope cleanup behavior
- traversal state correctness

### Output
Tests must cover:

- GNU output mode
- POSIX output mode
- DOT output mode
- xref-related behavior
- direct and inverted tree behavior
- indentation / separator / formatting-sensitive output

### parseopt
Tests must cover:

- short options
- long options
- permutation behavior
- non-option handling
- help/version/usage paths
- invalid option behavior

### wordsplit
Tests must cover:

- quoting
- variable expansion
- tilde expansion
- command/path-related transformations as applicable
- error propagation
- final word extraction

### GNU helper replacements
Where Rust replaces `gnu/*` functionality, tests must cover the preserved observable contract.

---

## 2.3.3 Regression test rule

Every bug found during migration must produce a regression test unless impossible or clearly unjustified.

The regression test should be added:

- at the smallest meaningful level if local
- at integration level if externally visible
- at differential level if parity with C was the failing concern

---

## 2.3.4 Golden test rule

For user-visible outputs, golden tests are required where formatting matters.

Golden tests must be used especially for:

- tree output
- xref output
- DOT output
- help/usage/version output
- diagnostics where formatting is stable and important

Golden updates require reviewer scrutiny and must not be treated as routine noise.

---

## 3. Quality Gates

## 3.1 Tests That Must Pass

A change may not merge unless all required applicable gates pass.

## 3.1.1 Baseline mandatory gates

1. The project must build successfully in its supported migration configuration.
2. All unit tests must pass.
3. All integration tests must pass.
4. All regression tests must pass.
5. All golden tests must pass or be intentionally updated with review justification.
6. Linting and formatting checks must pass.
7. No new unchecked panic path may be introduced in normal operation-sensitive code without approval.

---

## 3.1.2 Differential parity gates

For behaviorally sensitive changes, the following must pass where infrastructure exists:

1. Rust vs C output comparison on representative fixtures
2. Rust vs C CLI behavior comparison on representative option sets
3. Rust vs C parser/wordsplit edge-case comparisons where applicable

These gates are mandatory for high-risk subsystems and strongly preferred elsewhere.

---

## 3.1.3 Scope-based gate policy

### Low-risk internal refactor
Must pass:
- build
- unit tests
- relevant integration tests
- lint/format

### Medium-risk subsystem migration
Must pass:
- all above
- targeted golden tests
- new regression tests if applicable
- reviewer-confirmed traceability to C behavior

### High-risk behavior-affecting migration
Must pass:
- all above
- differential tests
- representative performance benchmark
- explicit approval from reviewer(s) on compatibility evidence

High-risk includes changes to:

- parser
- scanner
- wordsplit
- parseopt
- symbol graph lifecycle
- output formatting logic
- startup/config precedence logic

---

## 3.2 Code Review Standards

## 3.2.1 Review law

No substantive migration code may merge without review by a maintainer or designated reviewer applying this constitution.

## 3.2.2 Required review questions

Every review must verify, at minimum:

1. **Behavior**
   - Does this preserve C-visible behavior?
   - If not, is the change explicit and approved?

2. **Traceability**
   - Is the replaced C functionality identified?
   - Is the migration mapping clear?

3. **Safety**
   - Is safe Rust used by default?
   - Is any unsafe justified and minimal?

4. **Testing**
   - Are tests sufficient for the risk level?
   - Was a regression test added for any bug fix?

5. **Performance**
   - Does the change risk hot-path regression?
   - If yes, was it measured?

6. **Error semantics**
   - Are error channels preserved appropriately?
   - Were panics introduced where recoverable behavior existed?

7. **Maintainability**
   - Is the Rust design understandable and modular?
   - Does it avoid unnecessary complexity without sacrificing compatibility?

---

## 3.2.3 Unsafe review standard

Any PR containing `unsafe` must include:

- a clear safety explanation at the unsafe site
- reviewer confirmation that a safe alternative was not reasonable
- tests or argumentation covering the relevant invariants

Unsafe without explanation must not be merged.

---

## 3.2.4 Golden-output review standard

Any change to expected output must answer:

- Is this an intentional behavior change or a correction?
- What evidence shows the new output is more correct?
- Does it match C behavior?
- Which fixtures changed and why?

Bulk unexplained golden churn must be rejected.

---

## 3.3 Performance Benchmark Requirements

## 3.3.1 Benchmark law

Performance-sensitive changes must be backed by benchmarks or other direct measurement.

## 3.3.2 Required benchmark areas

Benchmarks should cover representative workloads in:

- startup and option parsing
- source scanning
- parser throughput
- symbol table operations
- graph construction
- output generation
- wordsplit-heavy configuration handling where relevant

---

## 3.3.3 Benchmark trigger conditions

Benchmarking is required when a change:

- alters parser data structures
- changes scanner buffering behavior
- changes symbol storage or hash strategy
- changes traversal or output-generation algorithms
- changes wordsplit internals materially
- replaces a custom C structure with a Rust standard structure in a hot path
- introduces additional allocations, cloning, or buffering in a hot path

---

## 3.3.4 Benchmark evaluation standard

A benchmark result is acceptable if it shows one of:

- no meaningful regression
- a justified tradeoff with clear benefit and accepted cost
- an improvement

A change with significant regression and no approved justification must not merge.

---

## 4. Project-Wide Enforcement Rules

## 4.1 Staged migration is allowed, but hidden incompleteness is not

The project may migrate incrementally. However:

- incomplete areas must be clearly identified
- temporary bridges must be explicit
- FFI boundaries must be documented
- TODO-based silent gaps are not acceptable substitutes for tracked migration status

---

## 4.2 The existing build ecosystem must remain operable during migration

Because the current project uses `Makefile.in`, migration work must preserve practical buildability throughout the project unless an approved transitional build strategy says otherwise.

A better future build system may be introduced, but not at the expense of losing controlled migration and verification.

---

## 4.3 Law of subsystem priority

The following subsystems are constitutionally high-priority and high-scrutiny due to behavior density and migration risk:

1. `src/main.c`
2. `src/c.c`
3. `src/parser.c`
4. `src/symbol.c`
5. `src/output.c`
6. `src/parseopt/*`
7. `src/wordsplit/*`
8. `gnu/hash*`
9. `gnu/error*`
10. portability wrappers with externally visible behavior

Changes here require stronger evidence than simple leaf-module rewrites.

---

## 4.4 Documentation obligation

Every significant migration milestone must leave behind enough documentation to answer:

- what C behavior is covered
- what remains unmigrated
- what tests prove parity
- what uncertainties remain
- what temporary compromises exist

Undocumented migration is incomplete migration.

---

## 4.5 Constitution change policy

This constitution may be amended only by explicit project decision. Amendments must:

- be written
- identify the prior rule being changed
- justify the change
- avoid weakening compatibility, safety, or verification without strong cause

Until amended, this constitution remains binding.

---

## 5. Non-Negotiable Summary

The Rust migration of `cflow-new` shall:

- preserve observable behavior first
- preserve interface compatibility first
- use Rust safety as the default implementation model
- respect performance constraints on hot paths
- treat uncertain behavior conservatively
- require tests proportional to risk
- require code review proportional to risk
- require performance evidence for hot-path changes

No later plan or implementation may overrule these laws implicitly.