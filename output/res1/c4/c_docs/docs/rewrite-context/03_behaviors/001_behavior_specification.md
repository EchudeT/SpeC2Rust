# 03_behaviors

## Scope and evidence basis

This document describes runtime behavior only to the extent supported by the provided module analysis results.

Observed implementation-bearing files:
- `c4.c`
- `hello.c`
- `test/c4.c`
- `test/hello.c`

Observed behavior-relevant functions:
- `next()`
- `expr(int lev)`
- `stmt()`
- `main(int argc, char **argv)` in `c4.c` and `test/c4.c`
- `main()` in `hello.c` and `test/hello.c`

The module summaries provide function names, source ranges, and internal call counts, but not function bodies. Therefore, all detailed execution claims must stay within those boundaries. Where body-level behavior is required but not present in the summary, the current module summary is insufficient to support a more detailed behavior judgment.

---

## 1. Initialization flow and startup order

### 1.1 Program entry points

The project contains more than one `main` definition across different source files:
- `c4.c: main(int argc, char **argv)`
- `hello.c: main()`
- `test/c4.c: main(int argc, char **argv)`
- `test/hello.c: main()`

Dynamic startup therefore depends on which translation unit is built into the executable being run.

### 1.2 Startup flow for the `c4.c` executable path

For the executable built from `c4.c`, the observable startup shape is:

1. Process enters `main(int argc, char **argv)`.
2. `main` becomes the controlling routine for the runtime of this program variant.
3. Based on the internal-call presence of this module and the exposed parser-like function set, runtime work is organized around:
   - token advancement through `next()`
   - expression handling through `expr(int lev)`
   - statement handling through `stmt()`

The exact order in which `main` first invokes these routines is not available from the summary. The current module summary is insufficient to support a more detailed behavior judgment.

### 1.3 Startup flow for the `hello.c` executable path

For the executable built from `hello.c`, startup is simpler in structure:

1. Process enters `main()`.
2. Control remains within that single entry routine unless it invokes additional internal or library routines.

No non-`main` functions are listed for `hello.c`. The current module summary is insufficient to support a more detailed behavior judgment about any internal stages inside this `main()`.

### 1.4 Test startup flow

The `test` directory mirrors the same shape:
- `test/c4.c` has `next`, `expr`, `stmt`, and `main(argc, argv)`
- `test/hello.c` has `main()`

This indicates that test runtime startup follows the same entry ordering pattern as the corresponding non-test file:
- `test/c4.c` starts in its own `main(argc, argv)`
- `test/hello.c` starts in its own `main()`

### 1.5 Initialization characteristics that are directly supported

From the summaries alone, the following initialization properties are supported:
- Startup is entry-point driven through `main`.
- The `c4`-related path is multi-stage because it has distinct routines for token progression, expression processing, and statement processing.
- The `hello`-related path is single-entry and no additional internal interface is exposed in the summary.

No explicit global initialization sequence, constructor-like phase, or configuration-loading phase is shown in the module summary.

---

## 2. Main user operation flows

### 2.1 Primary operational flow in `c4.c`

The named functions strongly constrain the shape of runtime behavior even when body detail is unavailable. The operational flow is centered on three repeatedly usable activities:

- `next()` advances the current processing position in some sequential input stream or internal parse stream.
- `expr(int lev)` performs expression-oriented processing parameterized by a level value.
- `stmt()` performs statement-oriented processing.

What can be stated safely about dynamic flow:

1. `main(argc, argv)` is the top-level controller.
2. During execution, `main` participates in a workflow that uses the three internal routines.
3. `stmt()` and `expr(int lev)` represent two distinct processing layers.
4. `next()` is a progression routine used during active processing rather than a standalone endpoint.

The exact loop nesting, dispatch criteria, and termination conditions are not available from the summary. The current module summary is insufficient to support a more detailed behavior judgment.

### 2.2 Expression-processing flow

Observed interface:
- `expr(int lev)`

Behavioral implications directly supported by the signature and module structure:
- Expression handling is not flat; it is parameterized by `lev`.
- Runtime expression processing therefore depends on the current level argument.
- State changes during expression processing are level-sensitive.

A valid high-level dynamic description is:
1. Some controlling code enters `expr` with a specific level value.
2. `expr` performs level-dependent processing.
3. During that processing, internal state advances through further token or input progression activity associated with `next()`, or through nested expression logic.

The exact conditions under which `expr` calls itself, calls `next`, or returns are not shown. The current module summary is insufficient to support a more detailed behavior judgment.

### 2.3 Statement-processing flow

Observed interface:
- `stmt()`

Supported dynamic description:
1. Control enters `stmt()` when the current runtime context requires statement handling.
2. `stmt()` performs statement-level processing.
3. Statement-level processing is part of the same overall execution system as expression handling and token progression.

A statement routine in this structure is operationally a dispatch point or statement executor/parser stage, but the current module summary is insufficient to support a more detailed behavior judgment about:
- how statements are selected,
- whether statements recursively contain expressions,
- whether statement processing loops over multiple statements,
- how statement completion is detected.

### 2.4 Top-level command-line-driven flow

Because `c4.c` and `test/c4.c` expose:
- `main(int argc, char **argv)`

the top-level runtime flow includes command-line entry with argument count and argument vector supplied by the process environment.

Supported behavior description:
1. Program startup receives command-line state through `argc` and `argv`.
2. `main` can alter runtime flow based on these inputs.
3. Once initial top-level decisions are made, execution proceeds into the internal processing functions.

The summary does not disclose how command-line values are interpreted. The current module summary is insufficient to support a more detailed behavior judgment.

### 2.5 Main flow in `hello.c`

Observed interface:
- `main()`

The runtime flow here is minimal in externally visible structure:
1. Process enters `main()`.
2. All behavior for this program variant is controlled from that single function.
3. Program eventually returns from `main()`.

No finer operational stages are available from the summary.

### 2.6 Test flows

The test-side behavior mirrors the same runtime patterns as the main-side files:
- `test/c4.c` follows the same multi-stage flow structure as `c4.c`
- `test/hello.c` follows the same single-entry flow structure as `hello.c`

This means behavioral consistency between main and test variants is part of the observed structure, even though exact test assertions or output checks are not visible.

---

## 3. State machines and state transitions

### 3.1 Explicitly observable state-machine candidates

From function naming and layering, the `c4` runtime exposes a staged processing machine with at least these conceptual states:

- **Startup state**: execution begins in `main`
- **Token/input advancement state**: execution is inside `next()`
- **Expression-processing state**: execution is inside `expr(int lev)`
- **Statement-processing state**: execution is inside `stmt()`
- **Termination state**: execution returns from `main`

This is a behavioral organization supported by the observed function decomposition.

### 3.2 Transition structure for the `c4` path

At the highest level, transitions are:

- `main` -> active processing
- active processing -> `next`
- active processing -> `expr(lev)`
- active processing -> `stmt`
- processing routines -> return to caller
- top-level control -> program termination via return from `main`

The summaries confirm internal calls exist in this module:
- root module internal call count: 18
- test module internal call count: 9

This supports the statement that runtime transitions occur among these internal functions rather than execution staying isolated inside each function.

### 3.3 Level-driven expression state

`expr(int lev)` introduces an explicit state parameter:
- the current expression-processing mode depends on `lev`
- transitions within expression handling are sensitive to that level

The summary does not provide the range of `lev`, threshold conditions, or level transition rules. The current module summary is insufficient to support a more detailed behavior judgment.

### 3.4 Statement versus expression sub-states

The presence of both `stmt()` and `expr(int lev)` indicates at least two processing sub-states within the main runtime:
- a statement-oriented sub-state
- an expression-oriented sub-state

The dynamic system must therefore transition between these sub-states according to runtime context. The rule set for these transitions is not available in the summary.

### 3.5 Test-state equivalence

Because the test module contains the same function set and same file-level organization for `c4`, the same state-machine shape exists there:
- startup
- token/input advancement
- expression processing
- statement processing
- termination

The test summary does not disclose any additional instrumentation states or special test-only transitions.

---

## 4. Error-handling flows

### 4.1 Evidence limits

No explicit error-handling function, error code enumeration, or failure-path summary is present in the provided analysis results.

Therefore:
- the current module summary is insufficient to support a more detailed behavior judgment about explicit error signaling,
- the current module summary is insufficient to support a more detailed behavior judgment about recovery behavior,
- the current module summary is insufficient to support a more detailed behavior judgment about fatal versus non-fatal failures.

### 4.2 Error-handling behavior that can still be stated safely

Only the following general runtime facts are supported:
- Any error behavior, if present, must still flow through the same top-level entry points (`main`).
- In the `c4` path, any error detected during progression, expression handling, or statement handling would have to affect control flow among `next`, `expr`, `stmt`, and `main`.
- Program completion still occurs by returning from `main`, unless body code uses another mechanism not visible in the summary.

### 4.3 Unsupported specifics

The document must not assert any of the following, because the summary does not contain the evidence:
- particular diagnostic messages,
- command-line usage failures,
- parse error formats,
- cleanup sequences,
- return-value meanings,
- retry loops,
- exception-like control transfers,
- sentinel states for invalid input.

---

## 5. Boundary conditions and special-case handling

### 5.1 Command-line boundaries

For `main(int argc, char **argv)`:
- command-line argument presence is part of the runtime input boundary,
- startup behavior can vary with `argc` and `argv`.

The current module summary is insufficient to support a more detailed behavior judgment about:
- minimum required argument count,
- handling of zero or extra arguments,
- treatment of invalid argument strings.

### 5.2 Expression-level boundaries

For `expr(int lev)`:
- `lev` is an explicit runtime boundary variable,
- behavior is conditioned on the supplied level value.

The current module summary is insufficient to support a more detailed behavior judgment about:
- valid `lev` ranges,
- lower or upper bounds,
- how out-of-range values are treated,
- whether equal levels cause different paths than greater/lesser levels.

### 5.3 Empty or terminal processing conditions

Because `next`, `expr`, and `stmt` form a staged processing pipeline, some terminal or boundary condition must exist for control to return and for the program to finish. However, that condition is not described in the summary.

Therefore, the current module summary is insufficient to support a more detailed behavior judgment about:
- end-of-input behavior,
- empty source behavior,
- single-token or single-statement behavior,
- nested-expression depth limits,
- termination after the last statement.

### 5.4 Duplicate source-tree behavior

There are duplicated implementations in:
- root sources
- `test/` sources

This creates an important project-level special case:
- behavior must be understood per build target, not by assuming all `main` functions run together.
- only one selected `main` entry point participates in any single executable startup path.

### 5.5 `hello` special case

`hello.c` and `test/hello.c` are special-case minimal flows because they expose only `main()`. Their runtime behavior does not expose the multi-stage processing shape seen in `c4.c`.

This difference must be preserved when describing or porting behavior:
- `c4` path: staged internal workflow
- `hello` path: single-entry minimal workflow

---

## 6. Behaviors that must remain consistent with the C version

### 6.1 Entry-point behavior must remain unchanged

Any preserved implementation must keep:
- startup at the appropriate `main` function for the selected build target,
- completion through the same top-level control routine.

For `c4.c` and `test/c4.c`, preservation requires:
- `main(int argc, char **argv)` remains the controlling entry,
- command-line parameters remain part of the runtime input state.

For `hello.c` and `test/hello.c`, preservation requires:
- `main()` remains the sole observed entry.

### 6.2 Internal control-flow layering must remain unchanged

For the `c4` path, the C version clearly separates runtime work into:
- `next()`
- `expr(int lev)`
- `stmt()`

This layered behavior must remain consistent:
- token/input advancement remains distinct from expression handling,
- expression handling remains distinct from statement handling,
- top-level coordination remains in `main`.

### 6.3 Expression level parameterization must remain unchanged

The C version exposes `expr(int lev)` rather than a parameterless expression routine. Therefore:
- level-dependent behavior is part of the observed runtime contract,
- any preserved version must continue to model expression processing as dependent on a caller-supplied level.

### 6.4 Internal interaction pattern must remain present

The module summaries report internal calls:
- 18 in the root module
- 9 in the test module

Thus, preservation requires keeping the existence of active runtime interaction among internal routines. A rewritten version must not collapse the behavior into an unrelated control structure that removes the observed layered flow.

### 6.5 Main-versus-test correspondence must remain consistent

Because the test module mirrors the root `c4` and `hello` structure, consistency requires:
- corresponding test variants preserve the same runtime decomposition as their non-test counterparts,
- test-target behavior should continue to reflect the same entry and processing stages visible in the C sources.

### 6.6 Unsupported consistency claims

The summary does not justify stronger consistency requirements about:
- exact output text,
- exact exit statuses,
- exact parsing grammar,
- exact error messages,
- exact recursion depth behavior,
- exact memory lifetime behavior.

The current module summary is insufficient to support a more detailed behavior judgment on those points.

---

## 7. Performance-sensitive paths

### 7.1 Most likely hot paths from observed structure

Without body contents, only structural performance observations are valid.

In the `c4` path, the most performance-sensitive runtime areas are the repeatedly usable processing routines:
- `next()`
- `expr(int lev)`
- `stmt()`

These are the only exposed non-`main` internal functions, and they are linked by internal calls. That makes them the primary candidates for repeated execution during active program work.

### 7.2 Why these paths matter structurally

- `next()` appears to be a progression primitive. If called frequently, its runtime cost accumulates across the whole processing session.
- `expr(int lev)` is parameterized and may participate in nested or repeated processing. Its cost is structurally significant even without body visibility.
- `stmt()` represents statement-level handling and therefore likely sits on a repeated top-level processing path within the `c4` runtime structure.

The exact call frequencies are not provided. The current module summary is insufficient to support a more detailed behavior judgment.

### 7.3 Main routine sensitivity

`main(argc, argv)` is performance-relevant only as the top-level coordinator. In this structure, sustained runtime work is more likely concentrated in the subordinate processing functions than in process entry itself, but the current module summary is insufficient to support a more detailed behavior judgment.

### 7.4 Test-path performance

The test module has the same function shape and internal interaction pattern, so the same routines are structurally performance-sensitive there as well:
- `test/c4.c: next`
- `test/c4.c: expr`
- `test/c4.c: stmt`

### 7.5 Performance constraints that must be preserved

From the module summary alone, the only defensible preservation requirement is:
- do not alter the runtime design so that frequently interacting internal routines become disconnected from the staged control flow observed in the C sources.

No stronger claim about asymptotic complexity, buffering strategy, memory allocation pattern, or I/O throughput is supported by the provided evidence.