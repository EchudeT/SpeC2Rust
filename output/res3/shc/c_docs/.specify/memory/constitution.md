# constitution.md

## Purpose

This document is the governing constitution for the Rust migration of `shc`. It defines the non-negotiable project-level principles, migration rules, and quality gates that all later specifications, plans, tasks, implementation decisions, and reviews must follow.

Where the recovered C behavior is only partially known from available analysis, this constitution requires preservation of all behavior that is evidenced, explicit handling of uncertainty, and prevention of accidental semantic drift.

---

## Project Context

- Source project: `shc`
- Current implementation basis: single C module in `src/shc.c`
- Build system context: `Makefile.in`
- Migration scope: project-wide Rust rewrite with compatibility constraints
- Known behavioral centers:
  - command-line parsing
  - stateful reset/key/transform flow
  - script reading
  - shell/text evaluation
  - randomized/noise generation
  - output formatting and C artifact generation
  - build-step orchestration

---

# 1. Core Principles

## 1.1 Behavioral Equivalence Principle

The Rust migration must preserve the observable behavior of the C program before it attempts to improve structure, style, or internals.

### Rules

1. The Rust version must preserve all behavior explicitly evidenced by the available interface and behavior summaries.
2. The migration must preserve the end-to-end workflow shape:
   - entry through `main`
   - top-level orchestration
   - argument processing
   - state reset/key/transform stages
   - script/text handling
   - output generation
   - build step
   - status propagation to process exit
3. Functions that are visible behavioral control points in C must remain visible control points in Rust, even if internal organization changes.
4. Status-bearing operations in C must not be silently converted into infallible operations in Rust unless equivalence is proven and tests demonstrate no externally visible change.
5. Any change that could alter:
   - exit codes,
   - generated file contents,
   - command-line interpretation,
   - ordering of externally visible side effects,
   - error messages or error timing,
   - build invocation behavior,
   - randomization semantics relied on by output generation
   is considered a behavioral change and is prohibited unless explicitly approved by a higher-order migration decision document.

### Required interpretation

- "Equivalent" means externally observable compatibility, not source-level similarity.
- Safer internal Rust structure is encouraged only when it does not change externally visible behavior.
- If exact C semantics are awkward in Rust, compatibility still wins unless doing so would introduce unsoundness or undefined behavior on the Rust side.

---

## 1.2 Interface Compatibility First Principle

The migration must preserve the program's practical interface before redesigning internals.

### Rules

1. Command-line behavior is a compatibility surface and must be treated as stable unless proven otherwise.
2. The Rust binary must preserve expected invocation shape based on the C program:
   - command-line driven execution
   - equivalent argument handling model
   - equivalent top-level workflow
3. The conceptual separation between:
   - per-argument handling (`parse_an_arg`-like behavior),
   - whole-argument orchestration (`parse_args`-like behavior),
   - orchestration (`do_all`-like behavior),
   must remain present in the Rust implementation, even if expressed through Rust modules, methods, or helper types.
4. The transformation subsystem represented by `stte_0`, `key`, `key_with_file`, and `arc4` must remain a distinct subsystem with preserved lifecycle semantics.
5. The generation/output subsystem represented by formatting helpers, `write_C`, and `make` must remain architecturally distinct enough to test and verify compatibility.
6. Generated artifacts are part of the interface when they are consumed by downstream tools or users.
7. The build system integration contract must remain operable within the existing `Makefile.in`-driven project structure unless and until a separately approved build migration supersedes it.

### Required interpretation

- Compatibility applies to inputs, outputs, side effects, and control flow contracts.
- Internal Rust APIs may differ from C, but externally visible behavior must not.

---

## 1.3 Safety First Principle

Rust safety advantages must be fully used, but never as an excuse to change behavior without evidence.

### Rules

1. Safe Rust is the default.
2. `unsafe` is permitted only when strictly necessary and must be:
   - minimal in scope,
   - documented with invariants,
   - reviewed with special scrutiny,
   - covered by tests that exercise the surrounding behavior.
3. The migration must eliminate C-origin memory hazards where possible without changing observable behavior.
4. Pointer-and-length operations from C must be mapped into Rust using explicit, validated representations such as slices, vectors, byte buffers, or structured types.
5. Integer conversions, signed/unsigned boundaries, and length calculations must be explicit and checked.
6. Nullability assumptions from C must be modeled intentionally in Rust using `Option`, `Result`, or validated references/owned values.
7. Panics must not replace ordinary C-style error paths in user-facing behavior.
8. Secrets or sensitive transformation state, where applicable, must not be made less safe during migration.
9. FFI, process invocation, file I/O, and output generation boundaries must validate all assumptions before acting.

### Required interpretation

- Safety improvements are mandatory at the implementation level.
- Behavioral drift introduced under the label of "safety cleanup" is not allowed unless documented and approved.

---

## 1.4 Performance Constraint Principle

The Rust migration must not impose unjustified performance regressions on behaviorally important paths.

### Rules

1. Performance-sensitive paths identified from the C analysis must be treated as constrained:
   - argument parsing loops
   - stateful buffer transformation
   - script reading
   - random/noise generation
   - byte/array formatting
   - C artifact generation
   - final build orchestration
2. The Rust version must avoid obvious regressions such as:
   - unnecessary data copying,
   - repeated allocation inside hot loops,
   - avoidable UTF-8 conversions when byte semantics are sufficient,
   - excessive process spawning,
   - repeated full-buffer reparsing where a single pass suffices.
3. Safety checks are required, but should be structured to avoid pathological overhead in hot paths.
4. Performance optimizations must not reduce clarity in behaviorally sensitive code unless benchmarks justify them.
5. Deterministic correctness takes precedence over micro-optimization, but large regressions are unacceptable.

### Required interpretation

- The goal is parity or better on representative workloads.
- "Fast enough" must be demonstrated, not assumed.

---

# 2. Migration Guidelines

## 2.1 C-to-Rust Mapping Rules

These rules define the default translation policy from the current C design into Rust.

### 2.1.1 Structural mapping

1. The single C module may be split into multiple Rust modules if doing so improves safety, testability, or clarity.
2. Any split must preserve the behavioral boundaries evident in the source analysis:
   - CLI parsing
   - orchestration
   - transformation state
   - script/text acquisition and evaluation
   - random/noise helpers
   - formatting/output helpers
   - generation/build steps
3. Names may become more idiomatic in Rust, but traceability to original responsibilities must remain obvious.

### 2.1.2 Function mapping

1. C functions returning status codes should normally map to `Result<T, E>` internally.
2. At compatibility boundaries, those results must be converted back into the externally expected status behavior, including exit code behavior where applicable.
3. C `void` functions with implicit side effects should be reviewed for:
   - required mutable state,
   - hidden failure conditions,
   - opportunities to make invariants explicit.
4. Functions that operate on buffers and lengths must use byte-oriented Rust types unless evidence shows text semantics are required.

### 2.1.3 Data mapping

1. Raw C buffers should map to:
   - `&[u8]` / `&mut [u8]` for borrowed byte data,
   - `Vec<u8>` for owned mutable buffers,
   - `String` / `&str` only when text semantics are required and valid.
2. C strings from file or shell inputs must not be assumed to be valid UTF-8 without evidence.
3. Global mutable state in C should be replaced with explicit Rust state holders where feasible.
4. Stateful transformation logic must be represented with a dedicated state type whose lifecycle makes reset/key/transform ordering explicit.
5. Anonymous C structs discovered during analysis must be replaced with named Rust types if they persist beyond trivial local use.

### 2.1.4 I/O and process mapping

1. File I/O must preserve C-visible outcomes:
   - success/failure behavior
   - ordering of writes
   - generated content shape
2. Output formatting helpers must preserve textual output semantics byte-for-byte where those outputs are externally consumed or testable.
3. External build-step behavior must be encapsulated behind a Rust interface that is easy to test, but the production path must preserve practical behavior.

### 2.1.5 Randomness mapping

1. Randomized helper behavior must not be casually replaced with different semantics.
2. If the original randomness source is uncertain, the migration must first preserve output behavior under characterization tests before substituting implementations.
3. If exact random sequence equivalence is not required but output constraints are, those constraints must be documented and tested explicitly.

---

## 2.2 Principles for Handling Uncertain Behavior

The analysis explicitly states that some C behavior is not fully recoverable from summary data. This uncertainty must be handled conservatively.

### Rules

1. Absence of evidence is not permission to redesign behavior.
2. When behavior is uncertain, the team must prefer:
   - direct inspection of the original C source,
   - characterization testing of the C executable,
   - fixture generation from the C implementation,
   - side-by-side comparison under representative inputs.
3. Any area marked uncertain in the behavior summary must be tagged in the Rust implementation plan until resolved.
4. If exact behavior cannot be determined, the team must:
   - document the uncertainty,
   - state the chosen compatibility assumption,
   - add tests that lock in the chosen behavior,
   - obtain review approval for that assumption.
5. Uncertain behavior around error handling, exit status, argument parsing, formatting, or output generation is high risk and must be resolved before declaring parity.
6. Uncertain behavior in internal-only structure may be resolved with idiomatic Rust choices if no externally visible effect is introduced.
7. No speculative cleanup is allowed in the same change set as uncertain semantic migration.

### Required escalation cases

The following require explicit design-note level documentation before implementation is accepted:

- unknown return-code meaning
- ambiguous parser branching behavior
- uncertain file/key failure handling
- uncertain generated output syntax
- uncertain build-step trigger conditions
- uncertain random/noise constraints affecting output

---

## 2.3 Test Verification Requirements

Testing is the mechanism by which behavioral equivalence is established.

### Required test layers

1. **Characterization tests against the C implementation**
   - Capture current behavior for representative command-line inputs and file scenarios.
   - Use the C implementation as the baseline whenever feasible.

2. **Rust unit tests**
   - Cover isolated logic for parsing, state transitions, formatting, and helper utilities.
   - Include edge cases around lengths, empty inputs, and status propagation.

3. **Golden-output tests**
   - Compare generated outputs to approved baseline artifacts.
   - Required for formatting helpers and C output generation paths.

4. **Integration tests**
   - Exercise end-to-end CLI behavior.
   - Validate orchestration from invocation to generated artifact and final status.

5. **Error-path tests**
   - Verify failures at parsing, file access, transformation preparation, generation, and build stages.
   - Ensure error handling matches expected externally visible outcomes.

6. **Performance benchmarks**
   - Measure representative workloads on the key constrained paths.

### Mandatory verification policy

1. New Rust functionality is not considered migrated until it is covered by tests appropriate to its risk.
2. Bug fixes must include a regression test.
3. Any compatibility claim must cite test evidence or source-evidence rationale.
4. Any intentionally changed behavior must have:
   - explicit approval,
   - updated tests,
   - migration note explaining why the change is acceptable.

---

# 3. Quality Gates

## 3.1 Tests That Must Pass

No code may be merged unless all applicable gates below pass.

### 3.1.1 Baseline correctness gates

1. The project must compile cleanly in its supported Rust toolchain configuration.
2. All Rust unit tests must pass.
3. All integration tests must pass.
4. All golden-output tests must pass.
5. All regression tests must pass.
6. No newly introduced compiler warnings may be ignored without justification and approval.

### 3.1.2 Compatibility gates

1. For every migrated behaviorally significant path, there must be evidence of parity with the C implementation or an approved documented deviation.
2. At minimum, parity testing must cover:
   - CLI invocation success paths
   - CLI invocation failure paths
   - script file reading behavior
   - transformation lifecycle behavior
   - output formatting behavior
   - generated C artifact production
   - build-step orchestration behavior
3. Exit statuses and error outcomes must be verified where observable.
4. Where byte-for-byte output parity is expected, tests must assert byte-for-byte equality.

### 3.1.3 Safety gates

1. `cargo test` and configured static analysis must pass.
2. Any `unsafe` block must have:
   - inline safety justification,
   - focused test coverage,
   - reviewer sign-off.
3. No unchecked integer narrowing or pointer-like reinterpretation may be merged without explicit review acknowledgment.
4. Panics in ordinary user-facing error paths are release blockers unless expressly justified as impossible states.

### 3.1.4 Build and packaging gates

1. The Rust migration must remain compatible with the project's build integration expectations.
2. Changes affecting `Makefile.in` interaction must be tested in the expected build flow.
3. The project must remain reproducibly buildable in documented supported environments.

---

## 3.2 Code Review Standards

Every merged change must pass review against this constitution.

### Required review criteria

1. **Behavioral preservation**
   - Does the change preserve evidenced C behavior?
   - If not, is the deviation explicitly approved and documented?

2. **Traceability**
   - Can the reviewer map the Rust code to the original C responsibility?
   - Are migrated functions or subsystems still recognizable in role?

3. **Safety**
   - Are ownership, borrowing, mutability, and lifetimes used clearly and correctly?
   - Is `unsafe` absent, or if present, clearly justified?

4. **Error handling**
   - Are fallible operations modeled explicitly?
   - Are user-visible failures handled without unexpected panics?

5. **Boundary correctness**
   - Are pointer/length translations correct?
   - Are empty, zero, null-like, large, and signed/unsigned edge cases considered?

6. **Output compatibility**
   - If output formatting or generation changed, is there parity evidence?

7. **Test adequacy**
   - Are new or changed behaviors covered by unit/integration/golden/regression tests as appropriate?

8. **Performance awareness**
   - Does the change introduce unnecessary allocations, copies, reparsing, or process overhead?

### Reviewer authority

1. Reviewers must reject code that is more idiomatic but less compatible.
2. Reviewers must reject code that hides uncertain behavior behind assumptions without documentation.
3. Reviewers must reject code that collapses distinct behavioral phases into opaque implementations that are difficult to verify.
4. Reviewers may require smaller, more traceable migration steps when a change is too broad to assess confidently.

---

## 3.3 Performance Benchmark Requirements

Performance verification is mandatory for behaviorally important paths.

### Required benchmarks

Benchmarks must exist, as applicable, for:

1. argument parsing over representative argument counts
2. transformation over representative buffer sizes
3. script reading over representative file sizes
4. randomized/noise generation over representative output sizes
5. formatting and output generation over representative artifact sizes
6. end-to-end generation workflow
7. build-step orchestration overhead where measurable

### Benchmark policy

1. The first Rust implementation of a major subsystem must establish a baseline benchmark.
2. Subsequent changes to that subsystem must be checked for regression when performance-sensitive code is touched.
3. Significant regressions require either:
   - remediation before merge, or
   - explicit approval with documented rationale.
4. Benchmark scenarios must reflect realistic usage, not only microbenchmarks.
5. Benchmark conclusions must distinguish:
   - CPU cost,
   - allocation behavior,
   - I/O cost,
   - external process cost.

### Performance acceptance standard

1. No substantial regression is acceptable on the main generation path without approval.
2. If a regression is accepted to preserve correctness or safety, the decision must be documented and tracked.
3. Performance improvements must not compromise behavioral equivalence.

---

# 4. Enforcement

1. This constitution overrides convenience, stylistic preference, and unapproved refactoring goals.
2. All downstream documents must conform to it:
   - specs
   - implementation plans
   - task breakdowns
   - code reviews
   - test plans
3. When a downstream document conflicts with this constitution, this constitution wins.
4. Any amendment to these principles must be explicit, reviewed, and recorded as a project-level decision.

---

# 5. Non-Negotiable Summary

The Rust migration of `shc` must be:

- behaviorally equivalent first,
- interface-compatible first,
- safe by construction,
- performance-conscious,
- evidence-driven where behavior is uncertain,
- test-verified before claiming parity.

Any implementation that is cleaner but less compatible is non-compliant with this constitution.