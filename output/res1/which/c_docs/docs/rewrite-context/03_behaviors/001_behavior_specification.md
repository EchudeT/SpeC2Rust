# 03_behaviors

## Scope and evidence basis

This document describes runtime behavior only to the extent supported by the provided module analysis results. It does not promote missing source-body evidence into implementation claims. Where call order, state mutation details, or error semantics are not visible from the summary, the text states that the current module summary is insufficient to support a more detailed behavior judgment.

Covered modules:

- `main_root`
  - files: `bash.c`, `getopt.c`, `getopt1.c`, `which.c`
- `module_tilde`
  - files: `tilde/shell.c`, `tilde/tilde.c`

---

## 1. Initialization flow and startup order

### 1.1 Process entry points visible from the summaries

The only explicit process entry point exposed in the module summaries is:

- `main` in `getopt.c` (`getopt.c:992-1055`)

No `main` function is listed for `bash.c`, `which.c`, or the `tilde` module within the supplied summaries.

### 1.2 Startup sequence supported by the summaries

From the available evidence, the startup behavior that can be stated is:

1. Program execution enters `main` in `getopt.c`.
2. That entry point exists in the same module as:
   - `getopt`
   - `_getopt_internal`
   - `exchange`
   - `store_args_and_env`
3. The presence of these parsing-related functions in the same file strongly establishes that argument-processing behavior is part of startup and/or the first user-visible operation phase.

A more detailed startup order between `main`, `store_args_and_env`, `getopt`, and `_getopt_internal` cannot be stated from the module summary alone. The current module summary is insufficient to support a more detailed behavior judgment.

### 1.3 Initialization-related helper behavior in `bash.c`

The following functions indicate explicit initialization or environment discovery phases within `bash.c`:

- `initialize_group_array`
- `get_current_user_info`
- `uidget`
- `getmaxgroups`

These names and signatures show that the module includes startup-time or on-demand initialization around:

- current user identity
- group-related state
- environment lookup

However, the exact startup order, whether these are eager or lazy initializers, and which top-level path invokes them are not visible in the provided summaries. The current module summary is insufficient to support a more detailed behavior judgment.

### 1.4 Initialization-related helper behavior in the tilde module

The `module_tilde` summaries expose:

- `get_home_dir` in `tilde/shell.c`
- `tilde_find_prefix`
- `tilde_find_suffix`
- `memory_error_and_abort`

This establishes that the tilde module participates in a path/user-home related runtime phase. It also shows that memory allocation failure is treated as a terminating condition in at least one internal path (`memory_error_and_abort`).

No top-level startup sequencing between the tilde module and the main-root module is shown. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 2. Main user operation flows

## 2.1 Command-line option processing flow

The clearest complete operation family in the summaries is command-line parsing, centered on:

- `main`
- `getopt`
- `getopt_long`
- `getopt_long_only`
- `_getopt_internal`
- `exchange`

### Behavior flow that is directly supported

A supported high-level runtime flow is:

1. `main` receives `argc` and `argv`.
2. Option parsing is performed through `getopt` or one of the long-option wrappers.
3. `getopt_long` and `getopt_long_only` act as front ends to the internal parser family.
4. `_getopt_internal` performs the core parsing work.
5. `exchange` exists as an internal array-reordering helper within the same parsing implementation, indicating that runtime argument permutation is part of at least one parsing mode.

This is enough to state that user-supplied command-line arguments are consumed incrementally, with an internal parser maintaining progress across the argument vector and, in some modes, reordering elements.

### Runtime state changes in this flow

The argument-processing path visibly changes runtime state in these ways:

- the current parse position advances through `argv`
- the active option interpretation changes based on the current token
- long-option wrappers select internal parsing behavior flags
- argument order may be rearranged through `exchange`

The exact variables used for parser state, the exact triggers for reordering, and the precise return progression are not available in the module summary. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 2.2 Path and executable-resolution style flow

The `bash.c` interface set shows a second operation family organized around path scanning and candidate resolution:

- `file_status`
- `absolute_program`
- `substring`
- `extract_colon_unit`
- `get_next_path_element`
- `make_full_pathname`

### Supported behavioral pattern

The available function set supports the following concrete runtime description:

1. A program name or file name is received.
2. The code distinguishes absolute paths from non-absolute names using `absolute_program`.
3. For list-like path strings, colon-delimited units are processed using:
   - `extract_colon_unit`
   - `get_next_path_element`
4. Candidate pathnames are constructed using `make_full_pathname`.
5. Candidate files are checked using `file_status`.

This establishes a repeated scan-and-check runtime flow over path elements.

### Observable state changes in this flow

Within this operation family, runtime state changes include:

- a path-list index advances through a colon-separated string
- temporary substrings or path elements are extracted
- candidate full pathnames are generated
- file status evaluation determines whether scanning continues or stops

The summaries do not show the exact stop conditions, whether the first success terminates the search, or how status values are interpreted by callers. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 2.3 User identity and environment lookup flow

Another visible runtime behavior cluster in `bash.c` is user/environment lookup:

- `get_current_user_info`
- `sh_get_env_value`
- `sh_get_home_dir`
- `uidget`
- `getmaxgroups`
- `initialize_group_array`
- `group_member`

### Supported behavioral pattern

A supported behavior description is:

1. Runtime code needs current-user or environment-derived information.
2. Environment access is performed through `sh_get_env_value`.
3. Home-directory access is exposed through `sh_get_home_dir`.
4. User/group-related information is obtained through:
   - `uidget`
   - `getmaxgroups`
   - `initialize_group_array`
   - `group_member`
   - `get_current_user_info`

This demonstrates that the program includes a runtime path where user identity and membership information influence behavior or are made available for later decisions.

### State changes in this flow

The visible state-changing operations include:

- current-user information becomes initialized or refreshed through `get_current_user_info`
- group-related storage is initialized through `initialize_group_array`
- membership checks are performed through `group_member`

Whether this state is global, cached, one-time initialized, or recomputed per request is not shown. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 2.4 Tilde-related string handling flow

The `module_tilde` function set shows a string-processing flow around shell-style home expansion support:

- `get_home_dir`
- `tilde_find_prefix`
- `tilde_find_suffix`

### Supported behavioral pattern

The behavior that can be stated from the summaries is:

1. An input string is examined for a tilde-related prefix region using `tilde_find_prefix`.
2. A suffix boundary is identified using `tilde_find_suffix`.
3. Home-directory data is obtained through `get_home_dir`.
4. These operations together support runtime handling of strings that contain tilde-marked path segments.

This establishes scanning and boundary-detection behavior over input strings, with dependence on current home-directory information.

### State changes in this flow

Visible state change is limited from the summaries. The functions show string scanning and retrieval of home-directory information, but the exact mutation site, output rewriting sequence, or ownership/lifetime of produced strings is not present. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 3. State machines and state transitions

## 3.1 Command-line parser state machine

The command-line parser is the clearest state-machine-like behavior in the provided data.

### Observable states

Based on the parser interface family, the runtime parser moves among these operational states:

- **initial parse state**
  - parser begins with `argc`, `argv`, and option-spec inputs
- **current token examination**
  - a current argument is checked for option syntax
- **short-option processing**
  - selected when using `getopt`
- **long-option processing**
  - selected through `getopt_long`
- **long-only processing**
  - selected through `getopt_long_only`
- **argument permutation state**
  - entered when `exchange` is used to reorder `argv`
- **completion state**
  - parser has no further option tokens to process

### Supported transitions

The summaries support these transitions:

- `main` or another caller enters parsing through `getopt`/`getopt_long`/`getopt_long_only`
- wrapper functions transfer control to `_getopt_internal`
- `_getopt_internal` advances through tokens
- when argument reordering is needed, control enters `exchange`
- parsing resumes in `_getopt_internal`
- parsing eventually terminates and control returns to the caller

The exact token-classification rules and exact completion criteria are not visible. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 3.2 Path-list scanning state machine

The path-handling helpers imply an iterative scanner over colon-separated path lists.

### Observable states

- **start of path list**
- **extract next path element**
- **construct candidate pathname**
- **check file status**
- **continue scan**
- **scan complete**

### Supported transitions

- A path list and index are provided to `get_next_path_element` or `extract_colon_unit`
- an element is extracted
- `make_full_pathname` combines the element with a target name
- `file_status` evaluates the result
- control either proceeds to the next element or exits the scan

The exact success/failure branching criteria between `file_status` and the scanning loop are not available in the summary. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 3.3 User/group information state transitions

The user/group helper set implies a smaller initialization/use state machine.

### Observable states

- **user info not yet available**
- **group capacity known**
- **group array initialized**
- **membership query active**
- **user info available**

### Supported transitions

- `getmaxgroups` obtains a group-capacity-related value
- `initialize_group_array` prepares group-related state
- `group_member` evaluates membership against that prepared state
- `get_current_user_info` makes current-user information available

The summary does not establish whether `get_current_user_info` internally drives the other helper calls or whether callers orchestrate them separately. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 3.4 Tilde scanner state transitions

The tilde module implies a compact scanning state machine.

### Observable states

- **input string received**
- **prefix search**
- **suffix search**
- **home directory retrieval**
- **expanded or resolved string handling**

### Supported transitions

- `tilde_find_prefix` identifies whether a tilde-relevant region begins
- `tilde_find_suffix` locates the end of the region
- `get_home_dir` supplies the substitution base

The actual expansion step is not listed among the visible functions, so the current module summary is insufficient to support a more detailed behavior judgment about the full state machine.

---

## 4. Error-handling flows

## 4.1 Explicit fatal error path in the tilde module

The strongest explicit error-handling evidence is:

- `memory_error_and_abort` in `tilde/tilde.c`

### Behavior that is directly supported

The module contains a dedicated internal path for memory-related failure that aborts execution rather than returning a recoverable status through the same function.

This establishes:

- at least one allocation-related failure in the tilde module is treated as fatal
- the error path is centralized enough to have its own helper

The exact trigger points, message emission behavior, and termination mechanism are not visible in the summary.

---

## 4.2 Parsing error handling

The presence of `_getopt_internal`, `getopt`, `getopt_long`, and `getopt_long_only` establishes that malformed or unsupported options must be distinguishable during runtime parsing. However, the exact error returns, diagnostics, and continuation rules are not shown in the module summary.

The current module summary is insufficient to support a more detailed behavior judgment.

---

## 4.3 File/path error handling

The path-resolution helpers establish that candidate pathnames are checked through `file_status`. This implies that non-usable candidates are filtered during the search process. However:

- the exact status categories
- whether inaccessible, missing, or non-executable cases are distinguished
- how callers react to each class

are not available from the summaries.

The current module summary is insufficient to support a more detailed behavior judgment.

---

## 4.4 User/group/environment error handling

Functions such as:

- `sh_get_env_value`
- `sh_get_home_dir`
- `get_current_user_info`
- `uidget`
- `getmaxgroups`
- `group_member`

indicate runtime dependence on external process/user context. The summaries do not reveal how absent environment values, missing home-directory information, or group-query failures are represented or propagated.

The current module summary is insufficient to support a more detailed behavior judgment.

---

## 5. Boundary conditions and special-case handling

## 5.1 Absolute versus non-absolute program names

`absolute_program` explicitly marks a boundary split in behavior:

- **absolute input**
  - path search behavior can be bypassed or altered
- **non-absolute input**
  - colon-separated path search helpers become relevant

This distinction is directly supported by the visible function set and should be preserved as a top-level control-flow fork.

---

## 5.2 Colon-delimited path edge cases

The functions:

- `extract_colon_unit`
- `get_next_path_element`

show that path strings are processed incrementally using an index pointer. This means the runtime must handle boundary conditions around:

- start of string
- element boundaries
- end of string

The exact semantics for empty elements, repeated delimiters, or trailing delimiters are not available in the summary. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 5.3 String slicing boundaries

`substring (char const* string, int start, int end)` exposes explicit boundary-based string extraction.

This establishes that some runtime behavior depends on inclusive/exclusive positional slicing decisions. Those index-based boundaries are behaviorally significant and must remain consistent with the C implementation.

The summary does not reveal how invalid ranges are treated. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 5.4 Tilde prefix/suffix detection boundaries

`tilde_find_prefix` and `tilde_find_suffix` demonstrate that tilde handling is not just a blanket replace operation; it depends on locating valid beginning and ending bounds inside a larger string.

Behaviorally significant boundaries include:

- where a tilde-eligible region may begin
- where that region is considered to end

The exact accepted character patterns and delimiter rules are not shown. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 5.5 Group membership and capacity boundaries

`getmaxgroups` and `initialize_group_array` show that group-related behavior has an upper-bound or capacity-sensitive component. `group_member` then operates against that context.

This means the implementation has behavior dependent on group-array sizing and prepared membership state. Exact limits and overflow/failure handling are not visible from the summary.

The current module summary is insufficient to support a more detailed behavior judgment.

---

## 6. Behaviors that must remain consistent with the C version

The following dynamic behaviors are directly evidenced by the summaries and should remain consistent in any reimplementation or migration.

## 6.1 Preserve startup entry and parser-driven initial operation

- Execution begins at `main` in `getopt.c`.
- Early runtime behavior includes command-line parsing using the getopt family.
- Long-option and long-only entry paths must continue to route into the same core parsing behavior represented by `_getopt_internal`.

---

## 6.2 Preserve parser progress and argument reordering behavior

- Option parsing is stateful across the argument list.
- `_getopt_internal` is the central parser engine.
- `exchange` is part of the dynamic behavior and indicates that some parsing modes reorder `argv`.

Any replacement that parses without preserving the same effective token-consumption order and permutation behavior would risk diverging from the C version.

---

## 6.3 Preserve absolute-path branching and path-list scanning behavior

- `absolute_program` creates a control-flow distinction between absolute and non-absolute names.
- Non-absolute handling involves iterative extraction of path elements from a colon-separated list.
- Candidate names are assembled through `make_full_pathname`.
- Candidate evaluation passes through `file_status`.

This search-and-check loop is core behavior and must remain structurally equivalent.

---

## 6.4 Preserve string-boundary-based processing

The following functions make explicit that the C implementation is boundary-sensitive:

- `substring`
- `extract_colon_unit`
- `get_next_path_element`
- `tilde_find_prefix`
- `tilde_find_suffix`

Equivalent behavior requires preserving the same boundary interpretation, especially at starts, ends, and delimiters.

---

## 6.5 Preserve current-user and home-directory lookup flows

- `get_current_user_info`
- `sh_get_env_value`
- `sh_get_home_dir`
- `get_home_dir`

These routines show that user-context and environment-derived values are part of runtime behavior. Any port must preserve when such information is consulted and how that consultation influences subsequent path or expansion logic.

The exact precedence rules between shell/environment/home lookup helpers are not visible here, so only the existence of these lookup flows can be required from the current evidence.

---

## 6.6 Preserve fatal memory-error behavior in the tilde path

The existence of `memory_error_and_abort` means at least one memory-related failure path terminates execution rather than remaining local and recoverable. That fatal behavior must not silently become non-fatal unless the underlying source body shows otherwise.

---

## 7. Performance-sensitive paths

## 7.1 Repeated command-line token scanning

`_getopt_internal` spans a large source range (`getopt.c:518-970`), indicating that the parser core contains the bulk of argument-processing logic. Combined with `exchange`, this identifies a performance-relevant path when:

- argument counts are large
- many tokens require classification
- permutation is triggered repeatedly

The summary does not support quantitative claims beyond that.

---

## 7.2 Repeated path-element iteration

The combination of:

- `extract_colon_unit`
- `get_next_path_element`
- `make_full_pathname`
- `file_status`

identifies a loop-shaped hot path for executable or file lookup through colon-separated search lists. Performance sensitivity arises from repeated:

- delimiter scanning
- substring extraction
- pathname construction
- file checks

This path is especially important because each unsuccessful candidate causes another full iteration step.

---

## 7.3 Repeated boundary scanning in tilde processing

`tilde_find_prefix` and `tilde_find_suffix` indicate string scans over input text to detect replaceable tilde regions. If invoked on many path-like strings, this becomes a repeated linear-scan path.

The current module summary is insufficient to support a more detailed behavior judgment about caching, reuse, or amortization.

---

## 7.4 User/group initialization versus repeated membership checks

The separation of:

- `initialize_group_array`
- `group_member`

suggests a design where setup work and repeated checks are distinct operations. That division is often performance-relevant because initialization may be amortized across many membership queries. However, the summary does not show actual call relationships or reuse guarantees.

The current module summary is insufficient to support a more detailed behavior judgment.

---

## 8. Consolidated runtime behavior picture

From the supplied module summaries, the program behavior can be described at a high level as follows:

1. The visible entry point is `main` in `getopt.c`.
2. A major early runtime activity is command-line option parsing, implemented by a wrapper/core parser structure:
   - `getopt`
   - `getopt_long`
   - `getopt_long_only`
   - `_getopt_internal`
3. Parsing behavior includes a stateful walk over `argv` and may reorder arguments through `exchange`.
4. Another major runtime activity is path-oriented name handling:
   - determine whether a name is absolute
   - iterate through colon-separated path elements
   - build candidate full names
   - evaluate each candidate through file-status checking
5. User and environment context are consulted through user/group/home helpers.
6. Tilde-related string handling uses explicit prefix/suffix boundary detection and home-directory lookup.
7. At least one tilde-module memory-failure path is fatal and aborts execution.

Beyond this level, especially for exact sequencing between helper functions, exact state values, detailed return-driven branching, and exact error semantics, the current module summary is insufficient to support a more detailed behavior judgment.