# 03_behaviors

## 1. Initialization flow and startup order

### 1.1 Process entry and early startup
The observable runtime entry is `main` in `pwd.c` (`main_root_file_name_03`). Based on the available module summaries, startup behavior is organized around the following early steps:

1. **Program name setup**
   - `set_program_name` in `progname.c` exists as a dedicated startup-support function.
   - Its presence indicates that executable identity is initialized early so later diagnostics and help/version output can use a normalized program name.
   - The current module summary is insufficient to support a more detailed behavior judgment about exact normalization rules or fallback handling.

2. **Locale-related initialization**
   - Multiple locale support helpers are present:
     - `setlocale_null`, `setlocale_null_r`, and unlocked variants
     - `hard_locale`
     - `locale_charset`
     - `mbrtoc32`
     - `proper_name_lite`
   - This indicates that startup includes locale-aware behavior preparation for message formatting, quoting, author-name display, or multibyte processing.
   - The current module summary is insufficient to support a more detailed behavior judgment about exact startup call order among these helpers.

3. **Output finalization hook configuration**
   - `close_stdout`, `close_stdout_set_file_name`, and `close_stdout_set_ignore_EPIPE` indicate a runtime pattern where output-stream shutdown behavior is configured before normal execution completes.
   - This supports an execution model where the program does work first and final stream error checking is deferred until exit or explicit shutdown.

4. **Argument and option handling entry**
   - `main` is accompanied by `usage`, `version_etc*`, and `emit_bug_reporting_address`.
   - This indicates that command-line parsing in `main` branches early into:
     - usage/help output,
     - version output,
     - or normal working-directory resolution flow.

### 1.2 Working directory resolution path selection
Two distinct directory resolution paths are explicitly visible:

- `logical_getcwd` in `pwd.c`
- `robust_getcwd` in `pwd.c`

This shows that after command-line handling, `main` selects between at least two path-generation behaviors:

1. **Logical path mode**
   - Driven by `logical_getcwd`.
   - This path is separate and returns `char *`, indicating a string-oriented path production flow.

2. **Robust/physical reconstruction mode**
   - Driven by `robust_getcwd`, supported by:
     - `find_dir_entry`
     - `nth_parent`
     - `file_name_init`
     - `file_name_prepend`
     - `file_name_free`
     - `get_root_dev_ino`
   - This path uses an explicit mutable `file_name` state object and directory traversal support.

The current module summary is insufficient to support a more detailed behavior judgment about the exact user option names or exact selection conditions.

### 1.3 Quoting subsystem initialization behavior
The `quotearg` family forms a reusable runtime subsystem rather than the main operational core of `pwd`. Its startup-relevant behavior consists of configurable quoting state:

- `clone_quoting_options`
- `get_quoting_style`
- `set_quoting_style`
- `set_char_quoting`
- `set_quoting_flags`
- `set_custom_quoting`
- `quoting_options_from_style`

This implies the following startup/runtime preparation model:

1. A quoting configuration object exists as mutable state.
2. Callers may clone defaults, inspect style, mutate style, mutate per-character rules, mutate flags, or install custom quote delimiters.
3. String-formatting functions later consume this state to render user-visible text safely or consistently.

The current module summary is insufficient to support a more detailed behavior judgment about whether `pwd` itself uses all these quoting entry points directly during startup.

---

## 2. Main user operation flows

## 2.1 Help/version flow
A user-visible control path exists for informational output:

- `usage(int status)`
- `version_etc`
- `version_etc_va`
- `version_etc_ar`
- `version_etc_arn`
- `emit_bug_reporting_address`

Behaviorally, this flow is:

1. `main` inspects arguments.
2. For help-like conditions, control goes to `usage`.
3. For version-like conditions, control goes to one of the `version_etc*` functions.
4. Bug-reporting text can be emitted as part of version/help presentation.

This path is output-oriented and does not require working-directory reconstruction. The exact emitted text and precise branching conditions are not recoverable from the summaries alone.

## 2.2 Logical current-directory output flow
A direct path-returning behavior is represented by `logical_getcwd`.

Observed runtime shape:

1. `main` enters logical mode.
2. `logical_getcwd` computes or fetches a directory string.
3. `main` outputs the returned path.
4. Normal shutdown follows, including final output-stream closure checks.

This flow is simpler than the robust traversal path because it returns a complete string directly rather than incrementally building a path object.

The current module summary is insufficient to support a more detailed behavior judgment about whether environment variables or only system calls are consulted.

## 2.3 Robust current-directory reconstruction flow
This is the most structurally visible runtime path in `pwd.c`.

### Step-by-step observed behavior structure
1. **Create a path-building state**
   - `file_name_init` allocates or initializes a `struct file_name *`.
   - This object is the mutable accumulator for the final path.

2. **Obtain current-directory identity**
   - `robust_getcwd` drives the process.
   - It works with directory metadata, since `find_dir_entry` consumes a `struct stat *dot_sb`.

3. **Iterative upward traversal**
   - `find_dir_entry(struct stat *dot_sb, struct file_name *file_name, size_t parent_height)` shows a recursive or repeated parent-search behavior.
   - `parent_height` is explicit traversal state.
   - `nth_parent(size_t n)` indicates direct generation of parent path references such as repeated ascents.

4. **Match current directory inside parent**
   - The name `find_dir_entry` and its parameters show the runtime purpose: identify the entry name corresponding to the current directory within a parent directory context.
   - When found, the discovered path component is prepended into the accumulating `file_name` object using `file_name_prepend`.

5. **Root detection and stopping condition**
   - `get_root_dev_ino` from `root-dev-ino.c` indicates explicit acquisition of root device/inode identity.
   - This identity supports a traversal stopping condition when the current point reaches filesystem root.

6. **Finalize and release**
   - After `robust_getcwd` finishes constructing the path in the accumulator, the resulting string is used for output.
   - `file_name_free` releases the path-building object at the end of its lifetime.

### Dynamic character of this flow
This is not a single system query flow; it is a stateful reconstruction flow:

- traversal state changes with `parent_height`,
- path buffer state grows by repeated prepend operations,
- stop condition is tied to root identity,
- each successful step moves one level closer to the final absolute path.

## 2.4 File-name accumulator flow
The `struct file_name` helpers expose a local state machine for path assembly.

### Behavioral sequence
1. `file_name_init` creates an empty or initial accumulator.
2. Repeated calls to `file_name_prepend` add path components in front of the current accumulated content.
3. The accumulator is consumed by higher-level logic.
4. `file_name_free` destroys the accumulator.

This makes path construction efficient for reverse traversal, where components are discovered from leaf to root but must appear root to leaf in the output string.

The current module summary is insufficient to support a more detailed behavior judgment about resizing thresholds or internal buffer layout.

## 2.5 Quoting and user-visible string rendering flow
The quoting subsystem exposes several operational flows depending on caller needs.

### Configuration flow
1. Build or copy options:
   - `clone_quoting_options`
   - `quoting_options_from_style`
2. Inspect or mutate options:
   - `get_quoting_style`
   - `set_quoting_style`
   - `set_char_quoting`
   - `set_quoting_flags`
   - `set_custom_quoting`

### Rendering flow
1. Core formatting:
   - `quotearg_buffer`
   - internal `quotearg_buffer_restyled`
2. Heap-allocated result flow:
   - `quotearg_alloc`
   - `quotearg_alloc_mem`
3. Slot-based reusable result flow:
   - `quotearg_n_options`
   - `quotearg_n`
   - `quotearg_n_mem`
   - `quotearg_n_style`
   - `quotearg_n_style_mem`
   - `quotearg_n_style_colon`
   - `quotearg_n_custom`
   - `quotearg_n_custom_mem`
4. Convenience wrappers:
   - `quotearg`
   - `quotearg_mem`
   - `quotearg_style`
   - `quotearg_style_mem`
   - `quotearg_char`
   - `quotearg_char_mem`
   - `quotearg_colon`
   - `quotearg_colon_mem`
   - `quotearg_custom`
   - `quotearg_custom_mem`
   - `quote`
   - `quote_mem`
   - `quote_n`
   - `quote_n_mem`

### Dynamic behavior
This subsystem supports multiple runtime usage patterns:

- caller-supplied output buffer,
- newly allocated result,
- indexed reusable slots for repeated formatting,
- formatting under standard styles,
- formatting with custom delimiters,
- formatting with extra escaping for selected characters such as colon or caller-specified bytes.

`gettext_quote` indicates style-sensitive and message-sensitive quote delimiter selection, which ties visible output to locale/text configuration.

`quotearg_free` indicates that slot-based or cached quoting state persists across calls and must be reset or released explicitly.

## 2.6 Memory-allocation support flow
The `xmalloc` families provide a common runtime behavior pattern for all dynamic allocation in the program.

### Allocation behavior family
- basic allocation: `xmalloc`, `ximalloc`, `xcharalloc`
- resizing: `xrealloc`, `xirealloc`, `xreallocarray`, `xireallocarray`
- counted allocation: `xnmalloc`, `xinmalloc`
- geometric growth helpers: `x2realloc`, `x2nrealloc`, `xpalloc`
- zeroing allocation: `xzalloc`, `xizalloc`, `xcalloc`, `xicalloc`
- duplication helpers: `xmemdup`, `ximemdup`, `ximemdup0`, `xstrdup`

### Dynamic behavior
These functions centralize memory state transitions:

1. request allocation or resize,
2. on success, return usable storage,
3. on allocation failure, dispatch to shared failure behavior via `xalloc_die`.

This means most higher-level runtime flows can assume successful allocation and avoid repeating local failure code.

The current module summary is insufficient to support a more detailed behavior judgment about exact growth formulas in every helper, except that `x2nrealloc` and `xpalloc` are clearly intended for expansion-oriented paths.

## 2.7 Output shutdown flow
Output completion is not just a raw `fclose(stdout)`.

Observed flow components:

- `close_stdout`
- `close_stdout_set_file_name`
- `close_stdout_set_ignore_EPIPE`
- `close_stream`
- `rpl_fclose`
- `rpl_fflush`

Behaviorally:

1. During setup, the program can register output context such as file name and whether `EPIPE` should be ignored.
2. At shutdown, `close_stdout` validates and closes standard output behavior.
3. Lower-level stream shutdown uses:
   - flush logic (`rpl_fflush`)
   - close logic (`close_stream`, `rpl_fclose`)

This creates a controlled output finalization path where write errors that occur late are still surfaced consistently.

---

## 3. State machines and state transitions

## 3.1 Main execution state machine
The top-level runtime can be described as:

1. **Process start**
2. **Runtime setup**
   - program name initialization
   - locale/output support readiness
3. **Argument interpretation**
4. **Mode selection**
   - usage/help
   - version
   - logical directory output
   - robust directory output
5. **Output emission**
6. **Output finalization**
7. **Exit**

Transition triggers are argument-dependent at the `main` level. Exact option tokens are not available in the summaries.

## 3.2 Directory reconstruction state machine
The robust directory algorithm exposes a clearer multi-state behavior:

1. **Accumulator initialized**
   - via `file_name_init`

2. **Current node identified**
   - stat information prepared for current directory

3. **Parent selection state**
   - `nth_parent(parent_height)` obtains access to a parent level reference

4. **Entry search state**
   - `find_dir_entry` locates the child name of the current directory inside the parent

5. **Path accumulation state**
   - `file_name_prepend` adds the newly found component

6. **Termination check**
   - compare against root identity from `get_root_dev_ino`

7. **Repeat or finish**
   - if not at root, increase traversal height and continue
   - if at root, final path is complete

8. **Cleanup**
   - `file_name_free`

This is the most explicit internal state machine observable from the module partition results.

## 3.3 Quoting-options state machine
`struct quoting_options` behaves like a mutable formatting state object.

### States
- default/initial style state
- cloned state
- style-modified state
- flags-modified state
- per-character override state
- custom-delimiter state

### Transitions
- `clone_quoting_options`: copy existing state into a new independent state
- `set_quoting_style`: change quoting style
- `set_char_quoting`: change one character’s treatment
- `set_quoting_flags`: replace or update option flags
- `set_custom_quoting`: install caller-provided left/right delimiters

### Consumption transitions
- `quotearg_buffer`, `quotearg_alloc`, `quotearg_n_options`, and wrapper functions consume the current state to produce formatted output.

## 3.4 Quoting slot/cache state machine
The presence of `quotearg_n_options` and `quotearg_free` indicates reusable indexed storage.

### States
- uninitialized slot set
- active slot `n` allocated
- slot content replaced on later call for same `n`
- all slot storage released by `quotearg_free`

This state machine matters because returned pointers from indexed quoting helpers are tied to persistent subsystem state rather than necessarily being caller-owned fresh allocations each time.

The current module summary is insufficient to support a more detailed behavior judgment about pointer invalidation timing across every wrapper.

## 3.5 Stream finalization state machine
The output layer shows the following states:

1. **Stream open**
2. **Optional file-name context registered**
3. **Optional EPIPE policy registered**
4. **Flush attempt**
   - `rpl_fflush`
5. **Close attempt**
   - `close_stream` / `rpl_fclose`
6. **Success or failure handling**
   - coordinated by `close_stdout`

## 3.6 Locale-query state machine
The locale support helpers imply a small configuration-query state machine:

1. request locale string (`setlocale_null` or `_r` variant),
2. optionally access unlocked path or locked wrapper path,
3. classify locale hardness with `hard_locale`,
4. obtain charset with `locale_charset`,
5. use this information in formatting, message display, or multibyte conversion.

The current module summary is insufficient to support a more detailed behavior judgment about caching, locking granularity, or reuse duration.

---

## 4. Error-handling flows

## 4.1 Allocation failure flow
A clear centralized failure path exists:

1. Higher-level code calls one of the `x*alloc*` helpers.
2. If allocation cannot be satisfied, control reaches `xalloc_die`.
3. `xalloc_die` terminates allocation-dependent execution through a shared failure mechanism.

This centralization is a consistency property of the runtime.

## 4.2 Working-directory reconstruction failure flow
The robust `pwd` path involves several operations that can fail during traversal or matching:

- building/accessing parent paths,
- gathering directory metadata,
- locating the matching directory entry,
- growing the accumulated path object.

The existence of a dedicated robust traversal stack indicates that failures are handled within this path rather than delegated to a single direct call. However, the current module summary is insufficient to support a more detailed behavior judgment about exact diagnostics, retry rules, or exit statuses.

## 4.3 Output error flow
The output layer is explicitly hardened:

1. Program writes normal output.
2. At termination, `close_stdout` runs.
3. It relies on stream-level helpers such as `rpl_fflush`, `close_stream`, and `rpl_fclose`.
4. Late write/flush/close failures are therefore part of the program’s runtime behavior, not ignored by default.

The presence of `close_stdout_set_ignore_EPIPE` shows a policy branch for one class of output failure. The current module summary is insufficient to support a more detailed behavior judgment about exact suppression scope.

## 4.4 Locale query and conversion failure flow
Locale helpers include locked and unlocked forms and buffer-oriented forms:

- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null`

This shows explicit handling for bounded-buffer and locale-query failure modes. `mbrtoc32` also indicates multibyte conversion error-aware runtime behavior. Exact error return propagation is not recoverable here because the instruction forbids adding unobserved return semantics.

## 4.5 Quoting failure flow
Quoting has both buffer-based and allocating variants:

- buffer-based paths surface size pressure through returned size information,
- allocation-based paths depend on the `xmalloc` family or equivalent allocation support,
- slot-based paths require managed persistent storage and cleanup through `quotearg_free`.

The current module summary is insufficient to support a more detailed behavior judgment about how malformed multibyte data or invalid quoting configuration is rendered.

---

## 5. Boundary conditions and special-case handling

## 5.1 Root-directory boundary
`get_root_dev_ino` and traversal-by-parent behavior show that root is a first-class boundary condition.

Behaviorally:
- traversal must stop at root,
- root identity is based on device/inode data rather than only textual path comparison,
- this prevents endless upward traversal and defines the completion point for robust path reconstruction.

## 5.2 Deep parent traversal boundary
`nth_parent(size_t n)` and `find_dir_entry(..., size_t parent_height)` show explicit handling of arbitrarily increasing parent depth.

This means the implementation is prepared for:
- nested directory depth,
- repeated ascent levels,
- path construction where component count is not fixed in advance.

The current module summary is insufficient to support a more detailed behavior judgment about maximum supported depth beyond normal memory limits.

## 5.3 Variable-length path growth
`file_name_prepend` combined with allocation helpers and growth-oriented allocators (`x2nrealloc`, `xpalloc`) indicates that path assembly handles dynamic size growth.

Special cases covered at the behavior level:
- empty or initial accumulator state,
- repeated prepends,
- buffer growth while preserving existing accumulated content.

## 5.4 Fixed-buffer versus dynamic-buffer quoting
The quoting subsystem supports two important boundaries:

1. **Caller-supplied fixed output**
   - `quotearg_buffer`
2. **Runtime-sized allocation**
   - `quotearg_alloc`
   - `quotearg_alloc_mem`

This means the same quoting logic is expected to operate both under tight buffer limits and under unconstrained dynamic allocation paths.

## 5.5 String length with explicit memory size
Many functions accept both pointer and byte length:

- `quotearg_buffer`
- `quotearg_alloc_mem`
- `quotearg_n_mem`
- `quotearg_n_style_mem`
- `quotearg_char_mem`
- `quotearg_colon_mem`
- `quotearg_custom_mem`
- `quote_mem`
- `quote_n_mem`

This is a strong signal that runtime behavior must support data that is not limited to simple NUL-terminated strings. Special handling exists for explicit-size arguments, which is relevant for embedded NULs or partial buffers.

## 5.6 Custom quoting delimiter boundary
`set_custom_quoting`, `quotearg_n_custom_mem`, `quotearg_custom`, and `quotearg_custom_mem` show that quote delimiters are not fixed. Runtime behavior must support caller-supplied left and right quote strings as a distinct formatting mode.

The current module summary is insufficient to support a more detailed behavior judgment about validation of empty or null delimiters.

## 5.7 Special-character quoting boundary
`set_char_quoting`, `quotearg_char`, `quotearg_char_mem`, `quotearg_colon`, and `quotearg_colon_mem` show targeted handling for specific characters.

This means runtime behavior supports:
- generic style-based quoting,
- plus fine-grained extra protection for selected bytes,
- plus a predefined colon-sensitive path.

## 5.8 Locale-sensitive quote selection boundary
`gettext_quote` and locale helpers indicate that visible quote characters may vary with style and locale. This is a boundary condition because the same logical input may produce different visible delimiters depending on runtime environment state.

## 5.9 Case-insensitive text handling boundary
`c_strcasecmp` provides locale-independent or controlled case-insensitive comparison behavior for C-like text handling. The current module summary is insufficient to support a more detailed behavior judgment about where it is used in `pwd`, but it is part of boundary handling for text comparisons.

## 5.10 Stream state boundary during flush/close
The stream helpers in `fflush.c` reveal explicit handling for nontrivial stdio states:

- `clear_ungetc_buffer_preserving_position`
- `clear_ungetc_buffer`
- `disable_seek_optimization`
- `restore_seek_optimization`
- `update_fpos_cache`
- `rpl_fflush`

This indicates boundary-aware flushing when streams contain pushback state, cached position state, or optimization flags that must be temporarily altered.

---

## 6. Behaviors that must remain consistent with the C version

## 6.1 Mode-selection consistency
The C version clearly distinguishes:
- informational modes (`usage`, `version_etc*`),
- logical cwd generation (`logical_getcwd`),
- robust cwd reconstruction (`robust_getcwd`).

Any reimplementation must preserve this mode split and not collapse all path resolution into a single undifferentiated flow.

## 6.2 Reverse-construction path behavior
The C implementation exposes a specific dynamic strategy for robust path construction:
- identify current directory,
- climb to parents,
- find the child entry name in each parent,
- prepend components,
- stop at root.

This traversal-and-prepend behavior must remain consistent, because it is visible in function structure and state progression.

## 6.3 Root-identity stopping behavior
Root detection must remain based on the dedicated root identity mechanism exposed by `get_root_dev_ino`, not replaced in documentation by a weaker textual shortcut.

## 6.4 Mutable quoting-options behavior
The C version treats quoting behavior as mutable runtime state, not as a fixed formatting rule:
- style can change,
- flags can change,
- per-character exceptions can change,
- custom delimiters can change,
- cloned option objects can diverge.

Any preserved behavior document must keep this as stateful configuration.

## 6.5 Multiple quoting result ownership models
The C version exposes three distinct result models:
1. caller-provided buffer,
2. fresh allocated result,
3. indexed reusable slot result.

These models must remain behaviorally distinct.

## 6.6 Explicit-size string handling
The C version repeatedly distinguishes between NUL-terminated APIs and explicit-size APIs. That distinction must remain behaviorally visible and not be collapsed into plain string-only handling.

## 6.7 Centralized allocation-failure behavior
The allocation helpers are designed around a shared failure path via `xalloc_die`. Consistency requires preserving the existence of a single allocation-failure policy boundary rather than scattered ad hoc handling.

## 6.8 Final output verification behavior
The C version does not treat output completion as complete at the last write call. It has explicit close-time verification through `close_stdout` and lower-level stream wrappers. This late-stage validation behavior must remain consistent.

## 6.9 Locale-sensitive formatting behavior
The combination of `setlocale_null*`, `hard_locale`, `locale_charset`, `gettext_quote`, `proper_name_lite`, and `mbrtoc32` shows that user-visible text behavior depends on runtime locale state. This dependency must remain visible in behavior descriptions.

## 6.10 Stream-state repair around flush
The C version includes special flushing support for stream internals such as ungetc buffers and seek optimization state. A behaviorally faithful implementation must preserve that flushing is not treated as a trivial direct library pass-through.

---

## 7. Performance-sensitive paths

## 7.1 Robust path reconstruction loop
`robust_getcwd` with `find_dir_entry` and `file_name_prepend` is the most likely performance-sensitive user path because it can scale with directory depth and parent-search work.

Performance-relevant characteristics visible from the summaries:
- repeated parent traversal,
- repeated directory-entry lookup,
- repeated prepend operations,
- repeated metadata-based stop checks.

This path should remain efficient for deep directory hierarchies.

## 7.2 Dynamic path buffer growth
Path accumulation is performance-sensitive because repeated prepends can trigger resizing. The presence of growth helpers such as `x2nrealloc` and `xpalloc` indicates that expansion cost management is an explicit concern in the codebase.

## 7.3 Quoting core formatter
`quotearg_buffer_restyled` spans the dominant internal body of `quotearg.c` and is the core engine behind many wrappers. This makes it a hot path for any repeated diagnostic or user-visible formatting workload.

Performance-sensitive traits:
- one core implementation fans out to many public wrappers,
- it handles styles, flags, per-character quoting, custom delimiters, and buffer bounds,
- wrappers likely exist to avoid duplicating logic and to keep common formatting centralized.

## 7.4 Reusable quoting slots
`quotearg_n_options` plus `quotearg_free` indicates a reuse-oriented design. Indexed slots reduce repeated allocation when formatting multiple arguments over time. This is a performance-relevant behavior and should remain distinct from one-shot allocation flows.

## 7.5 Buffer-based quoting versus allocating quoting
`quotearg_buffer` provides a no-extra-result-allocation path for callers that already manage storage. This is performance-sensitive in repeated or tight-loop formatting scenarios.

## 7.6 Locale query helpers with locked/unlocked variants
The presence of unlocked and locked locale query forms indicates performance awareness around synchronization overhead. The current module summary is insufficient to support a more detailed behavior judgment about exact call frequency, but the split itself is performance-relevant.

## 7.7 Stream flush/close wrappers
`rpl_fflush`, `close_stream`, and `rpl_fclose` are on the output finalization path. They are less likely to dominate steady-state runtime than directory traversal, but they are performance-sensitive in the sense that they carefully manage stream state rather than using a minimal wrapper.

## 7.8 Multibyte conversion path
`mbrtoc32` is a low-level conversion routine with substantial implementation size. This suggests that multibyte-to-Unicode conversion is nontrivial and performance-sensitive where locale-aware text processing is active.

---

## 8. Overall dynamic behavior summary

At runtime, the program starts by establishing execution context such as program identity, locale-sensitive support, and controlled output shutdown behavior. `main` then selects one of several top-level flows: help/version output, logical current-directory retrieval, or robust physical directory reconstruction.

The most characteristic operational behavior is the robust reconstruction flow: create a mutable path accumulator, repeatedly ascend toward parent directories, find the current directory’s name within each parent, prepend discovered components, stop when root identity is reached, emit the final path, and then finalize output with explicit stream checking.

Supporting this core behavior are shared subsystems:

- a stateful quoting engine with mutable configuration and multiple result-delivery modes,
- centralized allocation wrappers with a shared fatal-failure path,
- locale and charset helpers that influence user-visible formatting,
- stream wrappers that preserve correctness during flush and close.

Where exact runtime details are not shown by the module summaries, the current module summary is insufficient to support a more detailed behavior judgment.