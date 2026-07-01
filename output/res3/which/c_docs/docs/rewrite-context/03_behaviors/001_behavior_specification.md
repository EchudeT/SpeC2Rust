# 03_behaviors

## Scope and evidence note

This document describes dynamic behavior only from the supplied module analysis summaries.

The available evidence is limited to:
- source-file grouping,
- function names,
- function signatures,
- source ranges,
- internal/external call counts at module level,
- module categorization.

The available evidence does **not** include function bodies, explicit call graphs, global-variable definitions, concrete branching logic, or observed runtime traces. Therefore, whenever a behavior cannot be grounded from the summaries alone, this document states:

**the current module summary is insufficient to support a more detailed behavior judgment**

---

## 1. Initialization flow and startup order

### 1.1 Top-level startup entry points

The root module contains a visible program entry point:

- `main` in `getopt.c`

This establishes that at least one executable startup path begins in the argument-parsing related source slice rather than in `bash.c` or `which.c`.

The root module also contains support routines that appear startup-relevant by name:

- `uidget`
- `getmaxgroups`
- `initialize_group_array`
- `get_current_user_info`
- `sh_get_env_value`
- `sh_get_home_dir`

From the naming and co-location, these functions participate in environment/user-context preparation, but the exact startup order among them is not available from the module summary. The current module summary is insufficient to support a more detailed behavior judgment.

### 1.2 Observable startup phases in the root module

Based on the function inventory, startup behavior in the root module is organized around these phases:

1. **Program entry**
   - `main` receives `argc` and `argv`.

2. **Option-processing subsystem availability**
   - `getopt`
   - `_getopt_internal`
   - `getopt_long`
   - `getopt_long_only`
   - `exchange`
   - `store_args_and_env`

   These functions indicate that command-line option parsing is a first-class startup concern.

3. **User/group context preparation**
   - `uidget`
   - `getmaxgroups`
   - `initialize_group_array`
   - `group_member`
   - `get_current_user_info`

   These names indicate an initialization path that prepares process identity or group membership information before later decision-making.

4. **Environment lookup support**
   - `sh_get_env_value`
   - `sh_get_home_dir`

   These expose runtime state derived from environment variables or home-directory lookup.

5. **Path-processing support**
   - `absolute_program`
   - `substring`
   - `extract_colon_unit`
   - `get_next_path_element`
   - `make_full_pathname`
   - `file_status`

   These indicate later execution phases that examine file paths and candidate program locations.

This gives a behavioral picture of a startup sequence in which command-line handling and execution-context setup are both prepared early, followed by path iteration and file-status testing. Exact invocation order remains unconfirmed from the supplied evidence.

### 1.3 Tilde module initialization

The tilde-related module exposes:

- `get_home_dir`
- `tilde_find_prefix`
- `tilde_find_suffix`
- `memory_error_and_abort`

This indicates an auxiliary runtime path for home-directory lookup and tilde-delimited string scanning.

No explicit entry point is present in this module summary. Its behavior is therefore service-oriented rather than startup-owned. The current module summary is insufficient to support a more detailed behavior judgment about when this module is initialized or first used.

---

## 2. Main user operation flows

### 2.1 Command-line option processing flow

The strongest observed runtime flow is command-line parsing, centered on:

- `main`
- `getopt`
- `_getopt_internal`
- `getopt_long`
- `getopt_long_only`
- `exchange`
- `store_args_and_env`

#### Behavior sequence
1. `main` starts with `argc`/`argv`.
2. One of the public option APIs is used:
   - `getopt` for short options,
   - `getopt_long` for long options,
   - `getopt_long_only` for long-option-oriented parsing.
3. These interfaces converge on `_getopt_internal`, which appears to be the common execution engine.
4. During parsing, `exchange` indicates active reordering or normalization of argument positions.
5. `store_args_and_env` indicates that the parser can retain references to startup argument/environment state for later parsing logic.

#### Dynamic behavior that is directly supported by the summary
- The parser has both short and long option handling paths.
- There is an internal parsing engine behind public wrappers.
- Runtime processing is not a single linear scan only; it includes an explicit `exchange` operation, which indicates stateful handling of argument ordering.

The exact user-visible option syntax, parsing modes, and termination conditions are not available from the summary. The current module summary is insufficient to support a more detailed behavior judgment.

### 2.2 Program/path lookup flow in the root module

The root module includes a coherent set of path and file inspection helpers:

- `absolute_program`
- `extract_colon_unit`
- `get_next_path_element`
- `make_full_pathname`
- `file_status`
- `substring`

These names support the following dynamic operation pattern:

1. A target program name or path string is received.
2. `absolute_program` distinguishes direct path forms from names requiring search.
3. Search-list strings are processed incrementally:
4. Candidate full paths are constructed:
5. Candidate filesystem objects are examined:
6. Intermediate string slicing is handled through:

This forms a runtime flow of **search-list iteration -> candidate path creation -> candidate status test**.

The module summary does not confirm whether the search list is `PATH`, another colon-separated variable, or a local string source. The current module summary is insufficient to support a more detailed behavior judgment.

### 2.3 User/group membership based operation flow

The root module also contains a user/group-related cluster:

- `uidget`
- `getmaxgroups`
- `initialize_group_array`
- `group_member`
- `get_current_user_info`

This supports the following dynamic flow:

1. Current user identity data is obtained.
2. Group capacity/count information is determined.
3. A group array is initialized.
4. Membership checks can then be performed with `group_member`.
5. `get_current_user_info` appears to coordinate or expose the prepared user context.

The analysis summary does not expose whether this flow happens lazily on first membership query or eagerly during startup. The current module summary is insufficient to support a more detailed behavior judgment.

### 2.4 Home-directory retrieval and tilde-related operation flow

Across both modules, home-directory behavior is represented by:

- `sh_get_home_dir` in the root module
- `get_home_dir` in `tilde/shell.c`
- `tilde_find_prefix`
- `tilde_find_suffix`

A supported dynamic description is:

1. A string is scanned for tilde expansion boundaries:
2. Home-directory data is requested:
   - `get_home_dir`
   - or root-level `sh_get_home_dir`
3. Resulting text processing proceeds with resolved home-directory content.

The summaries do not show the full expansion routine itself, only the boundary-finding and home lookup support. Therefore, the exact replacement flow is not available. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 3. State machines and state transitions

### 3.1 Option parser state machine

The option parser is the clearest stateful subsystem in the supplied data.

#### Observable states
A minimal state machine supported by the function set is:

1. **Uninitialized parser state**
   - Entry before option scanning begins.

2. **Stored startup context**
   - `store_args_and_env` indicates retention of input context.

3. **Active scan state**
   - `_getopt_internal` processes current argument and option position.

4. **Argument exchange/reordering state**
   - `exchange` indicates a transition where argument layout is rearranged.

5. **Wrapper return state**
   - Control returns through `getopt`, `getopt_long`, or `getopt_long_only`.

6. **Parsing complete state**
   - Reached when no more options remain or parsing stops according to internal logic.

#### State transitions
- `main` -> parser wrapper entry
- wrapper entry -> `_getopt_internal`
- `_getopt_internal` -> `exchange` when reordering is needed
- `exchange` -> `_getopt_internal` to continue scanning
- `_getopt_internal` -> wrapper return after each parse step or on termination
- wrapper return -> `main` loop or post-parse execution

The exact parser flags, restart conditions, and handling of non-option arguments are not fully available from the summary.

### 3.2 Path search iteration state machine

The path-handling helpers indicate a separate iterative state machine.

#### Observable states
1. **Input name received**
2. **Absolute/direct-path classification**
   - `absolute_program`
3. **Search-list iteration active**
   - `extract_colon_unit`
   - `get_next_path_element`
4. **Candidate assembled**
   - `make_full_pathname`
5. **Candidate evaluated**
   - `file_status`
6. **Search continuation or termination**

#### Transition pattern
- name input -> direct/absolute check
- if not direct -> begin path-list iteration
- path element extracted -> full candidate built
- candidate built -> file status checked
- status checked -> either continue iteration or terminate search

The module summary does not state the exact conditions that terminate the search or what status categories are distinguished by `file_status`.

### 3.3 User/group context state machine

A limited but grounded state model can be stated for identity/group handling.

#### Observable states
1. **No prepared user/group context**
2. **User identity acquired**
   - `uidget`
   - `get_current_user_info`
3. **Group limit known**
   - `getmaxgroups`
4. **Group array initialized**
   - `initialize_group_array`
5. **Membership query ready**
   - `group_member`

#### Transition pattern
- startup or first query -> obtain user data
- obtain group sizing info -> initialize group storage
- initialized storage -> perform membership checks

The persistence of this state across multiple operations is not documented in the summary, but the existence of an initialization function suggests that later checks depend on prior setup.

### 3.4 Tilde scanning state machine

The tilde module exposes boundary-detection behavior rather than a complete expansion engine.

#### Observable states
1. **Input string pending scan**
2. **Prefix detection**
   - `tilde_find_prefix`
3. **Suffix detection**
   - `tilde_find_suffix`
4. **Expansion boundary established**
5. **Home-directory lookup support invoked**
   - `get_home_dir`
6. **Terminal failure path**
   - `memory_error_and_abort`

The exact transitions from boundary detection to substitution are not directly present in the supplied interface list.

---

## 4. Error-handling flows

### 4.1 Explicit fatal error path in tilde module

The only directly named hard-failure routine is:

- `memory_error_and_abort`

This supports one concrete behavioral conclusion:

- The tilde module contains at least one failure path that terminates processing rather than reporting a recoverable status through an ordinary return path.

Because no body is available, the exact trigger conditions, emitted diagnostics, and termination mechanism cannot be described further.

### 4.2 Return-based error signaling in root module helpers

Several root functions return `int` and therefore expose a runtime decision result:

- `file_status`
- `absolute_program`
- `uidget`
- `getmaxgroups`
- `group_member`
- `_getopt_internal`
- `getopt`
- `getopt_long`
- `getopt_long_only`
- `main`

This supports the behavior statement that many root-module operations communicate success/failure/classification through returned integer state.

The current module summary is insufficient to support a more detailed behavior judgment about:
- which values indicate failure,
- whether failures are recoverable,
- whether errors are propagated or converted to messages,
- whether `main` exits immediately after specific failures.

### 4.3 Parsing error flow

Because an internal option parser exists, the runtime must include one or more parser-decision branches for invalid, unsupported, or terminating inputs. However, no parser-body facts are provided.

Therefore, the only grounded behavior statement is:

- command-line processing includes explicit internal decision points and wrapper-mediated returns;
- the current module summary is insufficient to support a more detailed behavior judgment about invalid-option handling.

### 4.4 Path/file evaluation error flow

`file_status` indicates an explicit filesystem classification or accessibility test stage.

Grounded behavioral statement:
- candidate paths are not treated as immediately valid; they are examined before acceptance.

The current module summary is insufficient to support a more detailed behavior judgment about:
- nonexistent files,
- permission-denied cases,
- directory-vs-regular-file distinctions,
- how failures affect continued path iteration.

---

## 5. Boundary conditions and special-case handling

### 5.1 Absolute vs non-absolute program names

`absolute_program` establishes a special-case branch at the beginning of path handling.

Behavior that must exist:
- one path-handling branch distinguishes direct path inputs from names that require search-list processing.

This is a key boundary split in runtime behavior.

### 5.2 Colon-delimited path element boundaries

The presence of:
- `extract_colon_unit`
- `get_next_path_element`

shows that colon-separated text is processed incrementally rather than as an undifferentiated string.

Boundary handling that is directly supported:
- path element extraction depends on a mutable index or scan position,
- path traversal proceeds element by element.

The current module summary is insufficient to support a more detailed behavior judgment about:
- empty path elements,
- leading/trailing colons,
- repeated delimiters,
- end-of-string behavior.

### 5.3 String slice boundaries

`substring(const char *string, int start, int end)` indicates explicit boundary-based slicing.

Grounded dynamic significance:
- some runtime operations depend on extracting bounded segments from larger strings,
- segment start/end values are part of control flow and therefore boundary-sensitive.

The current module summary is insufficient to support a more detailed behavior judgment about how out-of-range indices are handled.

### 5.4 Tilde prefix/suffix delimiters

`tilde_find_prefix` and `tilde_find_suffix` establish explicit special-case detection around tilde syntax.

Grounded behavior:
- tilde-related processing is not applied globally to every string; there is a boundary-finding phase that determines where expansion-relevant text begins and ends.

The current module summary is insufficient to support a more detailed behavior judgment about:
- strings without tilde markers,
- multiple tilde segments,
- quoting or escaping effects,
- user-name-qualified tilde forms.

### 5.5 Group membership preparation boundaries

The split between:
- `getmaxgroups`
- `initialize_group_array`
- `group_member`

shows that membership checking is boundary-sensitive to available group-set size and setup completion.

Grounded behavior:
- membership checks depend on prior initialization context,
- there is a runtime distinction between setup and query phases.

The current module summary is insufficient to support a more detailed behavior judgment about:
- zero groups,
- oversized group sets,
- repeated initialization.

### 5.6 Wrapper/API mode boundaries in option parsing

The existence of three public parser entries:
- `getopt`
- `getopt_long`
- `getopt_long_only`

shows mode-specific behavior at the API boundary.

Grounded behavior:
- user input can be processed under at least three parsing modes,
- these modes share a common engine but preserve distinct entry semantics.

The current module summary is insufficient to support a more detailed behavior judgment about the exact behavioral differences among the modes.

---

## 6. Behaviors that must remain consistent with the C version

### 6.1 Preserve entry-point driven execution order

The C version contains a real `main` in `getopt.c`. Any reimplementation or refactor must preserve:
- startup through `main`,
- processing of `argc`/`argv`,
- delegation into the option parsing subsystem.

### 6.2 Preserve the parser layering

The public parser APIs and the internal parser engine are distinct in the C version:

- `getopt`
- `getopt_long`
- `getopt_long_only`
- `_getopt_internal`

This layering must remain behaviorally consistent:
- wrapper entry points remain distinct,
- internal shared parsing logic remains centralized,
- argument reordering behavior through `exchange` remains part of the runtime path where applicable.

### 6.3 Preserve stateful argument-order handling

Because `exchange` exists as a dedicated helper, argument ordering is part of runtime semantics, not just static representation.

This means a compatible implementation must preserve:
- the existence of a parsing path that reorders or exchanges argument positions,
- continuation of parsing after such reordering.

### 6.4 Preserve stepwise path search behavior

The C version separates:
- path-element extraction,
- candidate-path construction,
- candidate-status testing.

Therefore the following runtime decomposition must be preserved:
1. iterate over path units,
2. build candidate full pathnames,
3. test each candidate,
4. continue or stop based on test results.

Collapsing this into a different behavior that skips intermediate distinctions would risk changing observable runtime decisions.

### 6.5 Preserve direct-path discrimination

`absolute_program` is an explicit behavioral gate. Consistency requires:
- maintaining a direct-input classification step before path-list searching.

### 6.6 Preserve user/group setup before membership testing

The C version separates identity/group retrieval from membership query operations.

A consistent implementation must preserve:
- a preparatory phase for user/group context,
- membership checks that depend on that prepared context.

### 6.7 Preserve home-directory lookup support and tilde boundary scanning

Across modules, the C version includes:
- home directory retrieval helpers,
- tilde prefix/suffix boundary detection.

Consistency requires preserving:
- explicit detection of tilde-relevant regions,
- dependence on home-directory data for that behavior.

### 6.8 Preserve fatal memory-error termination path in tilde logic

`memory_error_and_abort` is an explicit dedicated failure path and should remain behaviorally distinct from ordinary returns.

A consistent version must preserve:
- at least one non-recoverable failure branch in the tilde-related runtime flow where the original C code uses this function.

The exact termination mechanism cannot be specified from the summary alone.

---

## 7. Performance-sensitive paths

### 7.1 Option parsing core loop

`_getopt_internal` spans the largest source region among the listed parser functions and acts as the common engine behind multiple public APIs.

This makes it a performance-sensitive path because:
- it is executed during startup/user command processing,
- it is shared by multiple parser modes,
- it likely performs repeated scanning over `argv` state.

A behavior-preserving implementation should avoid introducing extra passes over argument state beyond what the C version requires. The current module summary is insufficient to support a more detailed behavior judgment about exact complexity.

### 7.2 Argument exchange operations

`exchange` is a dedicated routine, indicating that argument reordering is substantial enough to be separated from the main parser body.

This path is performance-sensitive because:
- it operates on the argument vector itself,
- it can be invoked during active parsing,
- repeated exchanges would compound parser cost.

The current module summary is insufficient to support a more detailed behavior judgment about frequency or worst-case cost.

### 7.3 Path list scanning and candidate generation

The combination of:
- `extract_colon_unit`
- `get_next_path_element`
- `make_full_pathname`
- `file_status`

forms an iterative search loop over path entries.

This is performance-sensitive because:
- each candidate requires string processing,
- each candidate triggers status evaluation,
- unsuccessful searches may traverse many elements.

Behavior-preserving work should keep this as an incremental search pipeline rather than a needlessly duplicated or repeated evaluation process.

### 7.4 Repeated string boundary processing

`substring`, `tilde_find_prefix`, and `tilde_find_suffix` indicate repeated bounded string scans.

These are performance-sensitive when used in loops over long path strings or repeated expansion targets.

The current module summary is insufficient to support a more detailed behavior judgment about caching, reuse, or scan optimization in the original C version.

### 7.5 User/group context setup

`getmaxgroups`, `initialize_group_array`, and `group_member` suggest that user/group context may be prepared once and then queried.

This is performance-sensitive in the sense that:
- repeated membership checks should continue to reflect the setup/query split present in the C organization,
- repeated reinitialization could alter runtime cost and behavior.

The current module summary is insufficient to support a more detailed behavior judgment about whether the original implementation caches this context across queries.

---

## 8. Cross-module runtime picture

Considering both modules together, the observed runtime picture is:

1. A root executable path begins in `main`.
2. Command-line parsing is performed through shared parser logic.
3. User/environment context helpers are available to support later decisions.
4. Program/path processing uses iterative search and file-status checking.
5. Tilde-related support provides home-directory retrieval and tilde-boundary scanning.
6. At least one fatal memory-related failure path exists in tilde logic.

This is the strongest integrated behavior description justified by the supplied summaries. Any finer-grained sequencing between these subsystems would exceed the available evidence.