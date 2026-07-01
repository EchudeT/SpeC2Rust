# 03_behaviors

## Scope and evidence basis

This document describes runtime behavior only to the extent supported by the provided module analysis results. It is limited to the observed source files, function names, signatures, file groupings, and module-level counts.

Where implementation bodies are not available from the summary, the document explicitly states that the current module summary is insufficient to support a more detailed behavior judgment.

Covered modules:

- `main_root`
  - files: `bash.c`, `getopt.c`, `getopt1.c`, `which.c`
- `module_tilde`
  - files: `tilde/shell.c`, `tilde/tilde.c`

---

## 1. Initialization flow and startup order

### 1.1 Program entry points observed

The only explicit process entry point listed in the summary is:

- `main` in `getopt.c` (`int main (argc, argv)`)

This means the observable startup sequence, from the available evidence, begins in `getopt.c:main`.

No `main` entry point is listed for `bash.c`, `which.c`, or the `tilde` module.

### 1.2 Early argument-processing startup behavior

The module contains a concentrated cluster of option-processing functions:

- `store_args_and_env`
- `exchange`
- `_getopt_internal`
- `getopt`
- `getopt_long`
- `getopt_long_only`

This strongly establishes that one startup path includes command-line option parsing logic, with `main` in `getopt.c` serving as the initiating runtime point for that path.

Behavior that can be stated from the summary:

1. Process starts in `getopt.c:main`.
2. Option parsing behavior is available through:
   - short option path: `getopt`
   - long option path: `getopt_long`
   - long-only option path: `getopt_long_only`
   - shared parser core: `_getopt_internal`
3. There is an internal permutation or reordering routine:
4. There is an internal routine named:
   whose name indicates participation in preserving or recording startup arguments/environment for the option-parsing flow, but the current module summary is insufficient to support a more detailed behavior judgment.

### 1.3 User/environment initialization behavior in the root module

The root module also exposes a set of functions related to user and group context:

- `uidget`
- `getmaxgroups`
- `initialize_group_array`
- `group_member`
- `get_current_user_info`
- `sh_get_env_value`
- `sh_get_home_dir`

From naming and grouping alone, a runtime path exists in which the program retrieves current user identity and environment-backed home-directory information.

The observable initialization-style sequence that is justified by the summary is:

1. User identity/group-related support functions are available.
2. `get_current_user_info` exists as an explicit state-establishing routine.
3. Environment lookup helpers exist:

The current module summary is insufficient to support a more detailed behavior judgment about exact call order among these functions.

### 1.4 Tilde module initialization-related behavior

The `module_tilde` slice contains:

- `get_home_dir` in `tilde/shell.c`
- `tilde_find_prefix`
- `tilde_find_suffix`
- `memory_error_and_abort`

This indicates a second home-directory-related path distinct from `sh_get_home_dir` in the root module.

Observable startup-related facts:

- Tilde expansion support depends on access to a home directory provider (`get_home_dir`).
- Tilde string scanning is split into prefix and suffix detection phases.
- The current module summary is insufficient to support a more detailed behavior judgment about whether this module performs explicit one-time initialization.

---

## 2. Main user operation flows

## 2.1 Command-line option processing flow

The clearest complete operational flow in the evidence is command-line option parsing.

### Runtime flow shape

1. `main` receives `argc` and `argv`.
2. It can invoke one of the public option interfaces:
   - `getopt`
   - `getopt_long`
   - `getopt_long_only`
3. These public interfaces converge on `_getopt_internal`.
4. During parsing, `exchange` may be used to rearrange argument ordering within `argv`.
5. The parser returns control to `main` as options are consumed.

### Dynamic state changes visible from function structure

Even without implementation bodies, these state changes are directly implied by the function family:

- Parsing advances through `argv`.
- The current option interpretation mode changes based on whether the caller chose:
  - short options,
  - long options,
  - long-only options.
- `exchange` indicates that the parser can alter the positional arrangement of arguments during runtime, not just inspect them.
- `longind` in `_getopt_internal` indicates that the parser can track the index of a matched long option when that interface is in use.

The current module summary is insufficient to support a more detailed behavior judgment on exact parser state variables, termination conditions, or emitted diagnostics.

## 2.2 Path scanning and pathname construction flow

The root module exposes a second coherent runtime flow around path handling:

- `absolute_program`
- `substring`
- `extract_colon_unit`
- `get_next_path_element`
- `make_full_pathname`
- `file_status`

This supports a user operation flow in which a program name or file name is processed against path-like input.

### Observable runtime stages

1. Determine whether a program string is already absolute:
   - `absolute_program`
2. If path-list processing is required, scan a colon-delimited list:
   - `extract_colon_unit`
   - `get_next_path_element`
3. Construct candidate full pathnames:
   - `make_full_pathname`
4. Evaluate each candidate file:
   - `file_status`

### Dynamic behavior that is supported by the function set

- A path string is traversed incrementally using an index pointer:
  - `extract_colon_unit(const char* string, int* p_index)`
  - `get_next_path_element(const char* path_list, int* path_index_pointer)`
- This indicates stateful iteration through a delimited string rather than one-shot parsing.
- Candidate strings are generated from path element + program name.
- Candidate files are checked through `file_status`.
- `substring` provides support for extracting pieces of source strings during this flow.

The current module summary is insufficient to support a more detailed behavior judgment about:
- the exact meaning of file status results,
- whether empty path elements are normalized,
- whether current-directory substitution occurs,
- or how iteration terminates.

## 2.3 Current user and group evaluation flow

Another runtime flow centers on evaluating the current process's user/group context:

- `uidget`
- `getmaxgroups`
- `initialize_group_array`
- `group_member`
- `get_current_user_info`

### Observable behavior stages

1. User identity can be obtained through `uidget`.
2. Group capacity or limit can be retrieved through `getmaxgroups`.
3. Internal group-related storage can be initialized:
   - `initialize_group_array`
4. Membership can be tested:
   - `group_member(GID_T gid)`
5. A wrapper or aggregator routine exists:
   - `get_current_user_info`

### Dynamic state characteristics

- `initialize_group_array` being `static void` indicates an internal setup step that prepares module-local state used by later membership queries.
- `group_member` implies a repeated-query operation after initialization.
- `get_current_user_info` likely represents a synchronization step between process identity information and module state, but the current module summary is insufficient to support a more detailed behavior judgment on exact fields or caching semantics.

## 2.4 Environment and home-directory retrieval flow

The summary shows two home/environment helper sets:

### Root module helpers

- `sh_get_env_value(const char* v)`
- `sh_get_home_dir(void)`

### Tilde module helper

- `get_home_dir()`

### Observable runtime behavior

- Runtime behavior includes reading an environment value by name.
- Runtime behavior includes obtaining a home directory string.
- These home-directory flows are relevant to tilde-related path processing and shell-like behavior.

The current module summary is insufficient to support a more detailed behavior judgment about:
- precedence between environment and account data,
- caching,
- string ownership,
- or fallback ordering.

## 2.5 Tilde string processing flow

The `tilde` module contains explicit scanning functions:

- `tilde_find_prefix`
- `tilde_find_suffix`

This supports a string-driven operation flow for identifying the segment of input that should be treated as a tilde expression.

### Observable runtime stages

1. Receive an input string.
2. Detect the start/prefix boundary of a tilde-relevant segment:
   - `tilde_find_prefix(const char *string, int *len)`
3. Detect the ending/suffix boundary:
   - `tilde_find_suffix(const char *string)`
4. Use home-directory information from `get_home_dir`.
5. Abort through `memory_error_and_abort` if a memory-related fatal condition is encountered.

The current module summary is insufficient to support a more detailed behavior judgment about:
- which characters delimit a tilde expression,
- whether user-qualified forms are supported,
- or how output strings are assembled.

---

## 3. State machines and state transitions

## 3.1 Option parser state machine

The option-processing family is the clearest state-machine-like subsystem in the available evidence.

### States directly supported by the function set

1. **Uninitialized parse state**
   - Before any parsing function is called.

2. **Argument/environment capture state**
   - `store_args_and_env` indicates a setup state used before or during parsing.

3. **Active parse state**
   - `_getopt_internal` processes `argc`, `argv`, `optstring`, optional long-option arrays, and optional long-option index output.

4. **Argument reordering state**
   - `exchange` indicates a transitional state where argument positions in `argv` are modified.

5. **Short-option dispatch state**
   - reached via `getopt`.

6. **Long-option dispatch state**
   - reached via `getopt_long`.

7. **Long-only dispatch state**
   - reached via `getopt_long_only`.

8. **Parse completion/return-to-caller state**
   - parser returns to its caller after each parse step or at termination.

### State transitions justified by the summary

- `main` -> `getopt`/`getopt_long`/`getopt_long_only`
- public wrapper -> `_getopt_internal`
- `_getopt_internal` -> `exchange` when argument permutation is needed
- `_getopt_internal` -> caller on completion of current parse step or end of parsing

The current module summary is insufficient to support a more detailed behavior judgment on:
- exact transition conditions,
- whether there are explicit error states for unknown options,
- and whether ordering modes are configurable.

## 3.2 Path-list iteration state machine

The path-processing helpers indicate an iterative scanner over colon-separated text.

### States

1. **Path iteration not started**
   - index pointer initialized externally.

2. **Extract next unit**
   - `extract_colon_unit`

3. **Advance to next path element**
   - `get_next_path_element`

4. **Construct candidate path**
   - `make_full_pathname`

5. **Check candidate**
   - `file_status`

6. **Continue iteration or finish**
   - based on index movement through the path string

### State transitions

- initial path string + index -> extraction
- extracted path unit -> pathname construction
- pathname construction -> file status check
- file status check -> either next path element or completion

The index pointer parameters are direct evidence that iteration state is externalized and updated across calls.

## 3.3 User/group information state machine

### States

1. **User/group info not prepared**
2. **Group array initialized**
   - via `initialize_group_array`
3. **Current user info loaded**
   - via `get_current_user_info`
4. **Membership query active**
   - via `group_member`

### State transitions

- startup/first use -> initialization
- initialization -> current user info ready
- current user info ready -> repeated membership checks

The current module summary is insufficient to support a more detailed behavior judgment on whether initialization is one-time, lazy, or repeatable.

## 3.4 Tilde scanning state machine

### States

1. **Raw input string received**
2. **Prefix search**
   - `tilde_find_prefix`
3. **Suffix search**
   - `tilde_find_suffix`
4. **Home directory lookup**
   - `get_home_dir`
5. **Expansion continuation or failure**
   - fatal memory path through `memory_error_and_abort`

### State transitions

- raw string -> prefix detection
- prefix found -> suffix detection
- suffix found -> home directory lookup / expansion work
- memory failure -> abort

The current module summary is insufficient to support a more detailed behavior judgment on non-fatal failure states for unmatched or ineligible input.

---

## 4. Error-handling flows

## 4.1 Explicit fatal error path in tilde processing

The clearest error-handling behavior in the provided modules is:

- `memory_error_and_abort` in `tilde/tilde.c`

This establishes an explicit fatal flow:

1. Tilde-related processing encounters a memory error condition.
2. Control transfers to `memory_error_and_abort`.
3. The process or active execution path is aborted.

No stronger statement should be made from the summary about messaging, cleanup, or exit code.

## 4.2 Parser error handling

The option parser clearly has to distinguish valid and invalid command-line forms, but the available summary only confirms parser entry points and the shared internal engine.

Accordingly:

- There is an operational distinction between normal parse progression and non-normal parse outcomes within `_getopt_internal`.
- The current module summary is insufficient to support a more detailed behavior judgment on:
  - exact invalid-option handling,
  - whether diagnostics are printed,
  - whether parsing stops immediately,
  - or what return values mark each condition.

## 4.3 File/path checking error paths

`file_status` establishes that pathname-based operations include explicit file evaluation.

Observable error-flow facts:

- Candidate pathnames are not assumed valid; they are checked.
- Path iteration can continue after checking a candidate.

The current module summary is insufficient to support a more detailed behavior judgment on:
- whether missing files, non-executable files, or permission issues are separated,
- whether failures are accumulated or returned immediately,
- or how the final failure state is represented.

## 4.4 User/group lookup error paths

The user/group helper set strongly implies system-query operations. However, the summary does not expose exact error branches.

Therefore:

- User/group-related functions must handle unsuccessful or unavailable identity/group information in some way at runtime.
- The current module summary is insufficient to support a more detailed behavior judgment on fallback flow, cached invalid states, or error return conventions.

---

## 5. Boundary conditions and special-case handling

## 5.1 Absolute versus non-absolute program names

`absolute_program(char const* string)` establishes a special-case split in the program/file resolution flow.

Behavior that must exist:

- One branch handles already-absolute program strings.
- Another branch handles names that require path-list based resolution.

This boundary is central because it changes whether colon-delimited path traversal is needed.

The current module summary is insufficient to support a more detailed behavior judgment on what exact string forms count as absolute.

## 5.2 Colon-delimited path boundary behavior

The presence of:

- `extract_colon_unit`
- `get_next_path_element`

shows that path processing is sensitive to delimiter boundaries and maintains a mutable current index.

Special-case classes that are directly evidenced by the scanner design:

- start of string
- delimiter encounter
- end of string
- repeated extraction across successive calls

The current module summary is insufficient to support a more detailed behavior judgment on:
- empty elements,
- trailing delimiters,
- consecutive delimiters,
- or embedded special syntax.

## 5.3 Substring boundary handling

`substring(char const* string, int start, int end)` indicates explicit start/end slicing behavior.

This creates boundary-sensitive cases around:

- `start`
- `end`
- zero-length slices
- slice endpoints near string boundaries

Because the implementation body is not available in the summary, the current module summary is insufficient to support a more detailed behavior judgment on how invalid or inverted ranges are handled.

## 5.4 Group membership query boundaries

`group_member(GID_T gid)` creates a special-case boundary around:

- queried `gid` values
- initialized versus uninitialized group context

Since `initialize_group_array` exists, correct runtime behavior depends on the relationship between membership checks and prior setup.

The current module summary is insufficient to support a more detailed behavior judgment on whether `group_member` self-initializes, requires prior setup, or treats unknown group IDs specially.

## 5.5 Home-directory retrieval boundaries

Both `sh_get_home_dir` and `get_home_dir` indicate home-directory lookup behavior, which creates boundary cases around unavailable or empty home-directory data.

That boundary clearly matters to tilde processing and shell-related path expansion.

The current module summary is insufficient to support a more detailed behavior judgment on:
- null results,
- empty-string handling,
- or precedence across data sources.

## 5.6 Tilde string boundary detection

`tilde_find_prefix` and `tilde_find_suffix` explicitly divide input strings into eligible and ineligible regions.

Therefore the runtime must distinguish between:

- strings where a tilde-relevant prefix is found,
- strings where no such prefix is found,
- strings where a suffix boundary is found at different positions.

The current module summary is insufficient to support a more detailed behavior judgment on exact recognized grammar.

---

## 6. Behaviors that must remain consistent with the C version

The following consistency requirements are grounded in the observed function structure and module composition.

## 6.1 Preserve startup entry behavior

- Execution must continue to begin at `getopt.c:main` for the path represented by this module slice.
- Any rewritten or ported implementation must preserve that a `main(argc, argv)` entry exists and drives the option-processing flow represented here.

## 6.2 Preserve option-parser layering

The C version has a layered parser structure:

- `getopt`
- `getopt_long`
- `getopt_long_only`
- shared core `_getopt_internal`

This behavioral structure must remain consistent:

- public option APIs must continue to funnel into one shared parsing engine or preserve equivalent centralized behavior,
- long-option and long-only modes must remain distinct operational modes,
- argument reordering behavior represented by `exchange` must remain available where the C version uses it.

## 6.3 Preserve in-place or effective argument-order transitions

Because `exchange(argv)` exists as a dedicated internal routine, the C behavior includes runtime transformation of effective argument ordering.

A compatible implementation must preserve the same observable parsing order effects. The current module summary is insufficient to support a more detailed behavior judgment on the exact reordering algorithm.

## 6.4 Preserve iterative path traversal behavior

Path resolution behavior in the C version is not a single direct lookup; it is staged:

- test absolute path status,
- iterate through colon-separated path units,
- build candidate full pathnames,
- test each candidate.

This sequence and its externally advanced index-based iteration behavior must remain consistent.

## 6.5 Preserve user/group context preparation and query flow

The existence and separation of:

- `initialize_group_array`
- `get_current_user_info`
- `group_member`

mean the C version distinguishes between state preparation and later queries.

Any reimplementation must preserve that membership checks operate against prepared current-user/group context rather than an unrelated or stateless mechanism, unless the same observable behavior is maintained.

## 6.6 Preserve environment/home lookup roles

The C version separates:

- generic environment value lookup (`sh_get_env_value`)
- shell-side home lookup (`sh_get_home_dir`)
- tilde-module home lookup (`get_home_dir`)

A compatible implementation must preserve the same role separation at the observable behavior level, even if internal code organization changes.

## 6.7 Preserve tilde scanning phases

Tilde handling is phase-based in the C version:

- identify prefix,
- identify suffix,
- obtain home directory,
- handle fatal memory failure through a dedicated abort path.

These stages must remain behaviorally consistent.

## 6.8 Preserve fatal memory-error behavior in tilde flow

`memory_error_and_abort` is explicit and dedicated. A compatible implementation must preserve that this class of failure remains fatal in the corresponding path, rather than being silently ignored or converted into normal non-fatal control flow.

---

## 7. Performance-sensitive paths

## 7.1 Repeated option parsing path

`_getopt_internal` is the core parser and is likely on the hot path for all command-line parsing operations represented in this slice.

Performance-sensitive characteristics visible from the summary:

- It is the convergence point for three public APIs.
- It processes `argc`/`argv` directly.
- It may trigger `exchange`, which implies additional data movement or pointer swapping work.

For behavior preservation, implementations should avoid introducing extra parsing passes that would change runtime scaling on large argument lists.

## 7.2 Argument reordering path

`exchange` is a dedicated helper, indicating that argument rearrangement is a notable runtime activity, not a trivial incidental branch.

This path is performance-sensitive because it operates on the `argv` array itself or its effective ordering. The current module summary is insufficient to support a more detailed behavior judgment on complexity characteristics.

## 7.3 Path-list scanning loop

The combination of:

- `extract_colon_unit`
- `get_next_path_element`
- `make_full_pathname`
- `file_status`

forms an iterative resolution loop that can execute once per path element.

This is performance-sensitive when:

- the path list is long,
- many candidate pathnames are tested,
- or file-status checks repeat frequently.

The behavior that must remain consistent is incremental scanning rather than unrelated bulk preprocessing that would alter when candidates are checked.

## 7.4 File-status checking path

`file_status` sits directly on the candidate evaluation path and is therefore sensitive in repeated lookup scenarios.

Because candidate generation and status testing are separated, performance depends on the number of candidate paths examined before success or exhaustion.

The current module summary is insufficient to support a more detailed behavior judgment on caching or early-exit optimizations in the C implementation.

## 7.5 Group membership repeated-query path

`group_member` appears to represent a query function that may be called multiple times after setup. This makes:

- `initialize_group_array`
- `get_current_user_info`

important amortized-cost boundaries.

A behavior-preserving implementation should keep the distinction between setup work and repeated membership checks, since collapsing them could change runtime cost patterns.

## 7.6 Tilde scan and expansion path

`tilde_find_prefix` and `tilde_find_suffix` indicate scanning over input strings. This path becomes performance-sensitive when applied repeatedly across many shell-like tokens or path strings.

The fatal memory path also implies that allocation-related failure handling is part of this runtime path, though the current module summary is insufficient to support a more detailed behavior judgment on allocation frequency.

---

## 8. Cross-module runtime interaction summary

From the available evidence, the modules participate in the following broad dynamic system:

1. **Program startup / argument parsing**
   - starts in `getopt.c:main`
   - uses the layered getopt parser family

2. **Path-based resolution**
   - distinguishes absolute inputs from path-search inputs
   - iterates through colon-delimited path lists
   - constructs and checks candidate full paths

3. **User/environment context**
   - retrieves current user and group-related state
   - supports membership tests
   - retrieves environment and home-directory values

4. **Tilde-oriented shell string handling**
   - identifies tilde-relevant string boundaries
   - relies on home-directory lookup
   - has an explicit fatal memory-error branch

For any finer-grained execution narrative, the current module summary is insufficient to support a more detailed behavior judgment.