# 03_behaviors

## Scope and evidence basis

This document describes runtime behavior only to the extent supported by the provided module summary for `main_root` / `yank.c`.

The available evidence is limited to:
- the function list and source ranges,
- the fact that `main` is the entry point,
- the existence of terminal-oriented helpers (`tsetup`, `tend`, `tgetc`, `tmain`, `twrite`, `tputs`),
- the existence of input and pattern/field helpers (`input`, `strtopat`, `fcmp`, `yank`, `xwrite`, `usage`),
- the internal-call existence count.

Where exact control flow, data mutation rules, or failure paths are not directly visible from the summary, the current module summary is insufficient to support a more detailed behavior judgment.

---

## 1. Initialization flow and startup order

### 1.1 Process entry
- Runtime begins in `main(int argc, char *argv[])` in `yank.c:418-492`.
- `main` is the only non-static function listed, so it is the externally visible process entry and the coordinator of the module lifecycle.

### 1.2 Startup responsibilities present in the module
The function set indicates that startup behavior includes some combination of:
- command-line handling in `main`,
- optional or required usage display through `usage`,
- terminal/session setup through `tsetup`,
- data acquisition through `input`,
- transition into the main interactive or selection loop through `tmain`.

This ordering is grounded in the naming and presence of lifecycle-style functions, but the exact sequence among them cannot be fully established from the summary alone. The current module summary is insufficient to support a more detailed behavior judgment.

### 1.3 Initialization phases suggested by function roles
The visible function inventory suggests these startup phases:

1. **Argument reception**
   - `main` receives `argc` and `argv`.
   - `usage` exists as a dedicated helper, which indicates a startup branch for argument validation or explicit help handling.

2. **Program data preparation**
   - `input(void)` indicates a pre-operation stage that gathers or ingests source data before user interaction or output.
   - `strtopat(const char *s)` indicates a transformation step from string form into an internal pattern representation.
   - `fcmp(const struct field *f1, const struct field *f2)` indicates field comparison support, implying an internal collection of `struct field` objects is established before or during main operation.

3. **Terminal/session preparation**
   - `tsetup(void)` strongly indicates explicit terminal state initialization before interactive key reading or screen output.
   - `twrite`, `tputs`, and `tgetc` form a consistent terminal I/O cluster that depends on successful setup.

4. **Interactive run phase**
   - `tmain(void)` is the main terminal-driven operation loop or controller.
   - Its return type, `const struct field *`, indicates it computes or selects a field as its terminal result.

5. **Termination and cleanup**
   - `tend(void)` indicates an explicit shutdown/restore phase for terminal state after the main interaction completes or aborts.

### 1.4 Observable startup ordering constraints
Even without full bodies, some ordering constraints are strongly implied by function purpose:
- `tsetup` must precede terminal-oriented reading/writing through `tgetc`, `twrite`, `tputs`, or `tmain`.
- `tend` belongs after terminal-driven work is finished.
- `usage` is a short-circuit path relative to normal operation.
- `main` governs all of these phases.

Beyond these constraints, the current module summary is insufficient to support a more detailed behavior judgment.

---

## 2. Main user operation flows

### 2.1 High-level operational model
The module appears to implement a flow with three major operational stages:

1. **Acquire and prepare data**
2. **Run a terminal-driven selection/navigation loop**
3. **Emit the selected or yanked result**

This is supported by the coexistence of:
- `input`,
- field comparison (`fcmp`),
- terminal setup/read/write helpers,
- `tmain` returning `const struct field *`,
- `yank(const char *s, size_t nmemb)` for data export.

### 2.2 Data ingestion flow
- `input(void)` is a self-contained phase with no explicit parameters and no return value.
- This indicates that it operates against module-level state rather than caller-provided buffers.
- Because `input` is named separately from `main`, ingestion/preparation is treated as its own runtime step rather than inline setup.

The exact source of the data, ingestion termination rule, buffering strategy, and whether it is one-shot or incremental are not visible from the summary. The current module summary is insufficient to support a more detailed behavior judgment.

### 2.3 Pattern conversion flow
- `strtopat(const char *s)` transforms a string into another `char *` representation.
- This indicates a user-provided or internally formed textual query/pattern is normalized or converted before being used in matching or display logic.

The summary does not expose when this is called, whether the conversion is reversible, or how the result is stored. The current module summary is insufficient to support a more detailed behavior judgment.

### 2.4 Field-oriented operation flow
- The presence of `struct field` and `fcmp(const struct field *f1, const struct field *f2)` indicates that the program manipulates structured items called fields.
- `tmain(void)` returning `const struct field *` indicates the terminal interaction phase resolves to one chosen field, or no field, depending on runtime conditions.

A likely runtime organization is:
- `input` constructs or populates fields,
- `fcmp` supports ordering/equality decisions among them,
- `tmain` exposes them to user interaction,
- the chosen field is then used for output through `yank`.

However, the exact relation among these functions is not visible in the summary. The current module summary is insufficient to support a more detailed behavior judgment.

### 2.5 Interactive terminal flow
The terminal cluster strongly suggests an interactive loop:

- `tsetup(void)` initializes terminal mode/state.
- `tmain(void)` runs the core interaction.
- `tgetc(void)` fetches user input events or key codes.
- `twrite`/`tputs` update the terminal display.
- `tend(void)` restores terminal state at completion.

This is the clearest dynamic subsystem visible in the summary.

Within this flow:
- `tgetc` represents the input edge of the loop.
- `twrite` and `tputs` represent the output edge of the loop.
- `tmain` coordinates repeated state transitions based on received input and current field/query state.

The exact command vocabulary, redraw policy, and loop exit conditions are not available from the summary.

### 2.6 Output/yank flow
- `yank(const char *s, size_t nmemb)` is a dedicated emission/export routine.
- `xwrite(int fd, const char *s, size_t nmemb)` is a lower-level write helper with explicit file descriptor and byte count.
- The function naming indicates `yank` performs the higher-level user-visible output action, while `xwrite` performs the concrete byte transmission.

This suggests the normal success path ends with:
1. selecting or deriving a string,
2. passing that string and length to `yank`,
3. `yank` using `xwrite` to ensure the intended byte sequence is emitted.

The summary does not expose whether the destination is standard output, another descriptor, or a terminal-adjacent channel. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 3. State machines and state transitions

## 3.1 Program lifecycle state machine
A conservative lifecycle state model supported by the function set is:

1. **Start**
   - Enter `main`.

2. **Argument/usage decision**
   - Either continue into normal operation or branch to `usage`.

3. **Preparation**
   - Input collection and internal state construction through `input` and related helpers.

4. **Terminal setup**
   - Transition into terminal-capable mode through `tsetup`.

5. **Interactive control**
   - Repeated input/output processing inside `tmain`, using `tgetc`, `twrite`, and `tputs`.

6. **Selection/result resolution**
   - `tmain` returns `const struct field *`.

7. **Output/yank**
   - The chosen content is emitted through `yank`, which relies on `xwrite`.

8. **Terminal teardown**
   - `tend` restores or ends terminal state.

9. **Exit**
   - `main` returns.

This lifecycle is structurally supported by the available function names and signatures. Exact branch conditions between states are not visible.

## 3.2 Terminal interaction state machine
The terminal subsystem suggests a nested state machine:

### States
- **Terminal inactive**
  - Before `tsetup` or after `tend`.

- **Terminal active**
  - After `tsetup`, before leaving `tmain`.

- **Waiting for input**
  - `tmain` calls or depends on `tgetc`.

- **Processing input**
  - A received terminal event/key is interpreted.

- **Updating display**
  - `twrite` and/or `tputs` send refreshed text to the terminal.

- **Interaction complete**
  - `tmain` returns a `const struct field *`.

### Transitions
- `tsetup` moves terminal state from inactive to active.
- `tgetc` drives transitions from waiting to processing.
- processing leads either to:
  - another wait cycle,
  - a display update,
  - completion/selection,
  - an abort/error path.
- `tend` moves terminal state from active back to inactive.

The exact substate meanings, key decoding rules, and whether there are modal submodes are not available from the summary.

## 3.3 Data-selection state machine
There is evidence for a selection-oriented internal state model centered on `struct field`:

### Candidate states
- **No data yet**
  - before `input`.

- **Data prepared**
  - after ingestion/building of field structures.

- **Query/pattern transformed**
  - after `strtopat` is used on a string.

- **Current field focus/selection changing**
  - during `tmain`, driven by terminal input.

- **Final selection available**
  - when `tmain` returns a `const struct field *`.

- **Selection emitted**
  - after `yank`.

This is a behavior-oriented abstraction consistent with the interface set, but the current module summary is insufficient to support a more detailed behavior judgment about exact transitions or persistence rules.

---

## 4. Error-handling flows

### 4.1 Usage/error short-circuit
- `usage(void)` indicates a dedicated early-exit or message path from `main`.
- This path is distinct from the normal operational flow and likely bypasses the main interaction path.

The exact trigger conditions are not visible.

### 4.2 Write-path robustness
- `xwrite(int fd, const char *s, size_t nmemb)` exists as a wrapper around byte-oriented output.
- The presence of a custom write helper indicates that raw write behavior is important enough to centralize.

A behavior document can safely preserve this invariant:
- all higher-level output that depends on `xwrite` must continue to route through the helper rather than duplicating write behavior elsewhere, because the helper exists specifically to control runtime write handling.

The exact retry rules, partial-write handling, and failure reaction are not visible from the summary. The current module summary is insufficient to support a more detailed behavior judgment.

### 4.3 Terminal cleanup on failure or interruption
- `tend(void)` exists as an explicit terminal-ending function.
- In any implementation-consistent behavior description, terminal state restoration is a required concern whenever terminal mode has been entered.

What cannot be stated from the summary:
- whether `tend` is always called on every error path,
- whether signals or nonlocal exits are handled,
- whether cleanup is conditional.

The current module summary is insufficient to support a more detailed behavior judgment.

### 4.4 Input and pattern conversion failures
- `input(void)` and `strtopat(const char *s)` represent phases that can fail in many C programs, but the summary provides no visible failure protocol.
- No error return type is exposed for `input`, while `strtopat` returns `char *`.
- Because body semantics are unavailable, no stronger statement should be made than this:
  - these functions participate in runtime preparation,
  - the current module summary is insufficient to support a more detailed behavior judgment about their error signaling or recovery behavior.

---

## 5. Boundary conditions and special-case handling

### 5.1 Command-line boundaries
- `main(int argc, char *argv[])` and `usage(void)` establish that the program has command-line boundary handling.
- At minimum, there is a behavior distinction between accepted invocation flow and invocation paths that lead to usage output.

The exact accepted option set and arity constraints are not visible.

### 5.2 Empty or absent data cases
- Because `tmain` returns `const struct field *`, the runtime must accommodate a result-resolution condition.
- A pointer return type creates a boundary condition around whether a valid field is available.
- The current module summary is insufficient to support a more detailed behavior judgment about how empty input, no matches, or canceled selection are represented.

### 5.3 String and byte-count boundaries
- `yank(const char *s, size_t nmemb)` and `xwrite(int fd, const char *s, size_t nmemb)` both operate on explicit byte counts rather than relying only on null termination.
- This means behavior is defined in terms of counted data movement, not just C strings.
- Special cases that the C version must preserve include handling data according to the provided `nmemb` value.

The summary does not reveal how zero-length writes are treated or whether embedded null bytes are expected. The current module summary is insufficient to support a more detailed behavior judgment.

### 5.4 Terminal input boundaries
- `tgetc(void)` returns `int`, which is a conventional shape for key/event retrieval.
- This implies boundary handling around special input values, multi-byte encodings, control keys, or end-of-input conditions may exist.
- The current module summary is insufficient to support a more detailed behavior judgment about those exact cases.

### 5.5 Comparison boundaries
- `fcmp(const struct field *f1, const struct field *f2)` introduces ordering/equality behavior over field objects.
- Comparison functions are often central to stable behavior when duplicates, equal keys, or ordering boundaries exist.
- The exact comparison criteria are not visible, so the current module summary is insufficient to support a more detailed behavior judgment.

---

## 6. Behaviors that must remain consistent with the C version

The following runtime properties are directly supported by the module summary and should remain unchanged in any faithful reproduction:

### 6.1 Single-entry orchestration through `main`
- Program execution begins in `main`.
- `main` remains the coordinator for startup, operation, and exit.

### 6.2 Dedicated usage path
- Invocation handling must preserve a distinct `usage` behavior path rather than folding all startup failures into generic runtime handling.

### 6.3 Distinct preparation, interaction, and teardown phases
- The lifecycle separation implied by `input`, `tsetup`, `tmain`, and `tend` must remain intact.
- Terminal setup and terminal teardown must remain explicit phases, not incidental side effects buried in unrelated routines.

### 6.4 Terminal I/O abstraction boundaries
- Terminal output must continue to flow through `twrite` and/or `tputs`.
- Terminal input must continue to flow through `tgetc`.
- The main interactive controller role must remain centered in `tmain`.

### 6.5 Result production centered on field selection
- `tmain` must continue to resolve to `const struct field *`.
- Downstream behavior must continue to treat the terminal interaction phase as producing a field-oriented result, not an unrelated scalar or opaque code.

### 6.6 Counted-byte output behavior
- Output through `yank` and `xwrite` must continue to use explicit byte counts (`size_t nmemb`).
- Any reimplementation that silently converts this to null-terminated-only behavior would change the observable contract implied by the signatures.

### 6.7 Centralized low-level write path
- The existence of `xwrite` indicates low-level output behavior is intentionally centralized.
- Preserving this centralization is part of preserving runtime behavior consistency.

### 6.8 Field comparison as a distinct operation
- `fcmp` must remain a separate comparison behavior over `struct field`.
- If ordering or equality decisions are needed elsewhere in the runtime, they should continue to reflect this same comparison logic.

### 6.9 Pattern/string conversion remains explicit
- `strtopat` represents a named transformation step from string input to pattern-form output.
- This conversion step should remain explicit in the behavior model.

---

## 7. Performance-sensitive paths

### 7.1 Terminal interaction loop
- The most likely hot path is the `tmain` loop, because it coordinates repeated input and output.
- `tgetc`, `twrite`, and `tputs` are likely exercised frequently during active user interaction.
- Behavior changes that add unnecessary work per input event would affect responsiveness.

### 7.2 Output path
- `xwrite` is a low-level byte transfer helper and therefore a likely performance-sensitive utility.
- `yank` depends on it for final emission behavior.
- Preserving efficient counted-byte handling in this path matters to keep the C version’s runtime character.

### 7.3 Data comparison path
- `fcmp` may sit on a repeated path if field ordering, filtering, or selection maintenance requires frequent comparisons.
- Because comparison helpers are often called many times relative to the number of fields, preserving its semantics and avoiding added overhead is important.

### 7.4 Input/preparation phase
- `input` may process the full source data set and therefore can dominate runtime for large inputs.
- The summary does not reveal complexity drivers, buffering model, or whether `strtopat` is repeatedly applied inside this phase. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 8. Consolidated end-to-end behavior narrative

From the available evidence, the runtime behavior of this module can be described conservatively as follows:

1. The program starts in `main`.
2. `main` handles invocation-level decisions, including a dedicated `usage` path.
3. Normal execution prepares internal data through `input` and related helpers such as `strtopat` and `fcmp`.
4. Before interactive operation, terminal state is explicitly initialized by `tsetup`.
5. The program enters a terminal-driven main loop in `tmain`.
6. During that loop, terminal input is received through `tgetc`, and terminal output is performed through `twrite` and `tputs`.
7. The interactive phase resolves to a `const struct field *` result.
8. A string/byte sequence associated with the resolved result is emitted through `yank`, which uses `xwrite` for counted-byte output.
9. Terminal state is explicitly ended through `tend`.
10. `main` exits.

All finer-grained judgments about exact ordering inside `main`, query semantics, match rules, field storage, selection rules, or detailed recovery logic require direct inspection of the implementation body. The current module summary is insufficient to support a more detailed behavior judgment.