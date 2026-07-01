# 03_behaviors

## 1. Overall runtime behavior

The `pwd` project runtime behavior is centered on a command-line entry path in `main` (`pwd.c:326-394`), supported by:

- path acquisition logic in `pwd.c`
- root device/inode acquisition in `root-dev-ino.c`
- output finalization helpers in `closeout.c`, `close-stream.c`, `fclose.c`, and `fflush.c`
- diagnostics / usage / version text helpers in `version-etc.c`
- allocation helpers in `xmalloc.c` and `xalloc-die.c`
- quoting helpers in `quotearg.c`
- locale and charset helpers in `setlocale_null*.c`, `hard-locale.c`, `localcharset.c`, `propername-lite.c`, `c-strcasecmp.c`, and `mbrtoc32.c`
- program-name initialization in `progname.c`

From the module summaries, the runtime is not a single straight-line call sequence only. It includes a startup phase, one of several user-visible operational paths, and a cleanup/output-finalization phase.

---

## 2. Initialization flow and startup order

## 2.1 Process entry and early setup

The visible process entry is:

- `main` (`pwd.c:326-394`)

Supporting startup-related helpers are:

- `set_program_name` (`progname.c:38-92`)
- `usage` (`pwd.c:48-75`)
- `version_etc*` and `emit_bug_reporting_address` (`version-etc.c`)
- `close_stdout*` (`closeout.c`)

The current module summary supports the following startup-order judgment:

1. **Program control enters `main`.**
2. **Program identity is initialized** through `set_program_name`, which exists specifically to establish the running program name state used by later diagnostics/help/version output.
3. **Argument-driven dispatch occurs inside `main`.**
   - This includes normal execution selection and non-normal informational flows such as usage and version reporting.
4. **Output-close behavior is configured** through the `close_stdout` support module when the program wants output failures to be handled in a centralized way.
5. **Working-directory resolution logic is selected and executed.**
6. **The resulting path is emitted to standard output.**
7. **Final output close/flush handling occurs.**

The exact instruction-by-instruction order of these startup calls is not fully recoverable from the summaries alone; the current module summary is insufficient to support a more detailed behavior judgment.

## 2.2 Locale and presentation-related initialization

The project includes several modules that participate in locale-sensitive behavior:

- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null`
- `hard_locale`
- `locale_charset`
- `proper_name_lite`
- `c_strcasecmp`
- `mbrtoc32`

These indicate that runtime behavior includes locale/charset-dependent text handling for diagnostics, quoting, author names, or output formatting. The summaries do not establish exactly which of these are reached on every run of `pwd`; however, they clearly form part of the presentation/runtime environment support.

Observed behavior supported by the summaries:

- locale state can be queried through `setlocale_null*`
- locale hardness can be tested through `hard_locale`
- character set name can be determined through `locale_charset`
- multibyte-to-`char32_t` conversion exists through `mbrtoc32`
- case-insensitive text comparison exists through `c_strcasecmp`

The current module summary is insufficient to support a more detailed behavior judgment about when each is executed during `pwd` startup.

## 2.3 Quoting subsystem initialization behavior

The quoting subsystem has explicit configuration/state functions:

- `clone_quoting_options`
- `get_quoting_style`
- `set_quoting_style`
- `set_char_quoting`
- `set_quoting_flags`
- `set_custom_quoting`
- `quoting_options_from_style`

This establishes that quoting behavior is stateful at the level of a `struct quoting_options`, not just stateless string transformation.

Initialization behaviors visible from summaries:

- a quoting configuration can be created from a style
- a configuration can be cloned
- style and flags can be changed after creation
- special per-character quoting can be toggled
- custom left/right quote delimiters can be installed

This configuration state is then consumed by runtime quoting operations such as:

- `quotearg_buffer`
- `quotearg_alloc`
- `quotearg_alloc_mem`
- `quotearg_n_options`
- style/colon/custom wrappers
- `quote*` wrappers

The summaries do not prove that `pwd` always initializes custom quoting during startup; they only show that the project contains this runtime machinery.

---

## 3. Main user operation flows

## 3.1 Normal command execution flow

The main user-visible purpose is obtaining and printing the current working directory. Based on the available functions, the normal operation flow is:

1. **User invokes the program.**
2. **`main` parses command-line state** and selects a path-resolution strategy.
3. **One of the working-directory acquisition paths is executed:**
   - `logical_getcwd` (`pwd.c:299-323`)
   - `robust_getcwd` (`pwd.c:267-294`)
4. **If reconstructive traversal is needed, helper functions manipulate a path buffer object:**
   - `file_name_init`
   - `file_name_prepend`
   - `find_dir_entry`
   - `file_name_free`
   - `nth_parent`
5. **The final directory name is written to output.**
6. **`close_stdout`-style finalization ensures output errors are handled consistently.**

This is the dominant operational flow exposed by the summaries.

## 3.2 Logical path retrieval flow

`logical_getcwd` is a dedicated path strategy. Because it is separate from `robust_getcwd`, the program behavior includes a strategy distinction between:

- a logical current-directory interpretation, and
- a more reconstruction-oriented robust method

The current module summary supports these runtime characteristics for `logical_getcwd`:

- it returns a `char *`
- it is a self-contained path acquisition step
- it is selected by `main` as one branch of user operation

The exact decision rule used in `main` for choosing `logical_getcwd` versus `robust_getcwd` is not fully described by the summaries; the current module summary is insufficient to support a more detailed behavior judgment.

## 3.3 Robust/reconstructive path retrieval flow

`robust_getcwd` operates on a mutable `struct file_name *`, showing a different execution model from `logical_getcwd`.

Observed runtime flow pieces:

- `file_name_init` creates/initializes a path accumulator object.
- `find_dir_entry` participates in upward or parent-based resolution, because it receives:
  - current directory stat information (`struct stat *dot_sb`)
  - the mutable path accumulator
  - `parent_height`
- `file_name_prepend` prepends path segments, showing that path construction proceeds from leaf toward root.
- `nth_parent` supplies parent-directory path strings based on depth.
- `file_name_free` releases the temporary path structure after completion or failure.

This yields a concrete behavioral sequence:

1. initialize a mutable path object
2. inspect directory identity/state
3. move through parent-height stages
4. prepend discovered directory entries
5. stop when root-related termination is reached
6. emit accumulated path
7. free temporary state

The summaries do not expose the exact loop termination conditions in source terms, but the presence of `parent_height`, `find_dir_entry`, and `get_root_dev_ino` supports this reconstruction-oriented runtime behavior.

## 3.4 Root detection flow

The project includes:

- `get_root_dev_ino` (`root-dev-ino.c:28-37`)

This establishes a runtime operation that acquires device/inode identity for the filesystem root. In behavior terms, this supports:

- determining the stopping condition for upward directory traversal
- comparing current/parent directory identity against root identity

Because `find_dir_entry` and `robust_getcwd` exist alongside `get_root_dev_ino`, root identity is part of the traversal-control state. The exact comparison mechanics are not described in the summaries; the current module summary is insufficient to support a more detailed behavior judgment.

## 3.5 Informational flows: usage and version

The program contains explicit informational execution paths:

- `usage`
- `version_etc_arn`
- `version_etc_ar`
- `version_etc_va`
- `version_etc`
- `emit_bug_reporting_address`

This means `main` can divert from normal path printing into a text-reporting path.

Behaviorally this looks like:

### Usage flow
1. `main` detects an argument pattern that requires help or reports invalid invocation.
2. `usage(status)` runs.
3. The function emits usage/help text and returns or terminates according to `status`.
4. No normal working-directory resolution occurs after the terminal usage path.

### Version flow
1. `main` detects a version-reporting request.
2. One of the `version_etc*` functions writes version/package/author information.
3. `emit_bug_reporting_address` may contribute follow-up reporting text.
4. The normal directory-resolution flow is bypassed.

The summaries confirm these are active execution alternatives, but not the exact option syntax or output wording.

---

## 4. State machines and state transitions

## 4.1 Top-level program control state machine

A behavior-oriented state model, directly supported by the function set, is:

```text
Start
  -> Program-name initialized
  -> Argument dispatch
      -> Usage state
      -> Version state
      -> Logical path state
      -> Robust path state
  -> Output state
  -> Output close/finalization
  -> Exit
```

### State descriptions

- **Start**: process enters `main`
- **Program-name initialized**: diagnostic identity becomes available
- **Argument dispatch**: execution mode is selected
- **Usage state**: help/error usage text path
- **Version state**: version/authorship reporting path
- **Logical path state**: `logical_getcwd`
- **Robust path state**: `robust_getcwd` and helpers
- **Output state**: final path or informational text is written
- **Output close/finalization**: `close_stdout` and underlying stream-close support
- **Exit**: process returns success/failure status

## 4.2 File-name builder state machine

The `struct file_name` helper family supports an internal dynamic state machine:

```text
Uninitialized
  -> Initialized
  -> Repeated prepend/update
  -> Completed path
  -> Freed
```

Transitions:

- `file_name_init`: `Uninitialized -> Initialized`
- `file_name_prepend`: `Initialized/Updating -> Updating`
- `robust_getcwd` completion: `Updating -> Completed path`
- `file_name_free`: `Initialized/Updating/Completed -> Freed`

This is one of the clearest mutable-state subsystems in the project.

## 4.3 Parent traversal state machine

The presence of `parent_height` and `nth_parent` supports a traversal state model:

```text
Current directory known
  -> Parent level 1 checked
  -> Parent level 2 checked
  -> ...
  -> Root reached
  -> Traversal complete
```

For each parent-height step:

- a parent path can be materialized via `nth_parent`
- directory entry identification can be performed via `find_dir_entry`
- the discovered component is prepended into the accumulating path

The exact loop body and transition guard expressions are not available from the summaries.

## 4.4 Quoting-options state machine

`struct quoting_options` has explicit mutable transitions:

```text
Default/style-created
  -> Style changed
  -> Flags changed
  -> Per-character quoting changed
  -> Custom delimiters installed
  -> Cloned
  -> Used for quoting
```

Transitions are driven by:

- `quoting_options_from_style`
- `set_quoting_style`
- `set_quoting_flags`
- `set_char_quoting`
- `set_custom_quoting`
- `clone_quoting_options`

Operational consumption then occurs through:

- `quotearg_buffer`
- `quotearg_alloc*`
- `quotearg_n_options`
- wrapper APIs

## 4.5 Quoted-argument slot state machine

The `quotearg_n*` family shows another runtime state model: quoting by numbered slot.

Relevant functions:

- `quotearg_n_options`
- `quotearg_n`
- `quotearg_n_mem`
- `quotearg_n_style`
- `quotearg_n_style_mem`
- `quotearg_n_style_colon`
- `quotearg_n_custom`
- `quotearg_n_custom_mem`
- `quote_n`
- `quote_n_mem`

Behaviorally this means:

```text
Slot n unused
  -> Slot n quoted with a specific style/options
  -> Slot n content reused or replaced by later calls
  -> Global quote storage released by quotearg_free
```

The summary for `quotearg_free` confirms that some quote-related retained state exists and can be globally released. The exact storage model is not fully derivable from the summaries, so the current module summary is insufficient to support a more detailed behavior judgment.

## 4.6 Stream finalization state machine

Output handling modules support a stream lifecycle model:

```text
Output stream active
  -> Buffered output pending
  -> Flush attempted
  -> Close attempted
  -> Success or failure handling
```

Involved functions:

- `rpl_fflush`
- `close_stream`
- `rpl_fclose`
- `close_stdout`

Additional internal transitions exist around:

- `clear_ungetc_buffer_preserving_position`
- `clear_ungetc_buffer`
- `disable_seek_optimization`
- `restore_seek_optimization`
- `update_fpos_cache`

This shows stream state is actively normalized before or during flushing/closing, not merely delegated to a single libc call.

---

## 5. Error-handling flows

## 5.1 Command-line misuse and help/error reporting

`usage(int status)` is the explicit error/help path for invocation-level problems.

Behavior:

1. `main` detects a command-line condition requiring non-normal handling.
2. `usage(status)` is entered.
3. Usage text is emitted.
4. Control ends consistently with the provided `status`.

The summaries do not expose whether `usage` always exits internally or can return to `main`; the current module summary is insufficient to support a more detailed behavior judgment.

## 5.2 Working-directory retrieval failure handling

The project contains both:

- `logical_getcwd`
- `robust_getcwd`

This separation itself is evidence of differentiated error-handling behavior: one path obtains a logical path, while another path reconstructs a path more defensively.

Runtime error-handling characteristics supported by the summaries:

- there is not only one retrieval path
- a robust path uses filesystem identity (`stat`, root device/inode, parent traversal)
- temporary mutable path state can be initialized and freed around failures

The exact fallback ordering between logical and robust strategies is not fully exposed by the module summaries.

## 5.3 Allocation failure handling

The allocation subsystem is large and centralized:

- `xmalloc`, `xrealloc`, `xcalloc`, `xstrdup`, etc.
- `xalloc_die`

This shows a uniform runtime behavior for memory-demanding operations:

1. operational code requests dynamic memory through `x*` wrappers
2. failure does not remain a silent local event
3. failure is escalated through the shared allocation-failure path `xalloc_die`

The summaries support the existence of centralized allocation-failure behavior. They do not expose exact printed diagnostics or exit codes.

## 5.4 Output flush/close failure handling

Output error handling is explicitly centralized:

- `close_stdout_set_file_name`
- `close_stdout_set_ignore_EPIPE`
- `close_stdout`
- `close_stream`
- `rpl_fclose`
- `rpl_fflush`

Behaviorally:

1. output is written during normal operation
2. finalization is not trusted to raw implicit process exit only
3. an explicit close/flush phase runs
4. failures are handled consistently in one place

The presence of `close_stdout_set_ignore_EPIPE(bool ignore)` additionally shows that broken-pipe behavior is configurable as runtime state before final close handling.

## 5.5 Locale-query error handling

`setlocale_null_r`, `setlocale_null`, and their unlocked/with-lock support functions establish that locale retrieval has dedicated safe-access code paths, including buffer-based APIs.

The current module summary supports:

- direct locale retrieval API
- buffer-filling locale retrieval API
- locked vs unlocked variants

This is runtime error containment around locale state acquisition, but the exact failure return values and fallback strings are not safely derivable here.

## 5.6 Quoting-related failure handling

The quoting subsystem provides both buffer-based and allocation-based APIs:

- `quotearg_buffer`
- `quotearg_alloc`
- `quotearg_alloc_mem`

This indicates two error-handling styles:

- caller-managed output buffer flow
- library-allocated output flow

Global cleanup is available through `quotearg_free`, showing retained quote state is cleaned explicitly when needed.

The exact failure outcomes for buffer exhaustion or allocation failure are not described in the summaries; the current module summary is insufficient to support a more detailed behavior judgment.

---

## 6. Boundary conditions and special-case handling

## 6.1 Root-directory boundary during path reconstruction

The most important path-resolution boundary is reaching the filesystem root.

Evidence:

- `get_root_dev_ino`
- `find_dir_entry`
- `parent_height`
- `nth_parent`

Behaviorally, upward traversal must stop at a root condition rather than continue indefinitely. This root condition is tied to root device/inode identity. This must remain consistent with the C version.

## 6.2 Zero-or-more parent depth handling

`nth_parent(size_t n)` explicitly accepts a parent depth count. This means parent traversal logic handles discrete depth values and can materialize parent references by level.

Boundary-sensitive aspects:

- initial parent level
- repeated larger `n`
- termination at a maximum meaningful traversal depth determined by root reachability

The summaries do not define exact string results for every `n`; the current module summary is insufficient to support a more detailed behavior judgment.

## 6.3 Variable-length path growth

The `file_name` object and `file_name_prepend` indicate that path strings are built incrementally and may grow repeatedly.

This means boundary behavior includes:

- empty or initial path-builder state
- repeated prepend operations
- eventual completed path state
- cleanup after use

The allocation-growth policy of the `file_name` object is not available from the summaries and should not be added.

## 6.4 Quoting special cases

The quoting subsystem has explicit support for special cases:

- style-specific quoting
- quoting arbitrary memory with explicit size (`*_mem`)
- quoting by numbered slot (`quotearg_n*`, `quote_n*`)
- quoting with one extra protected character (`quotearg_char`, `quotearg_char_mem`)
- colon-specialized quoting (`quotearg_colon*`, `quotearg_n_style_colon`)
- custom delimiter pairs (`set_custom_quoting`, `quotearg_custom*`, `quotearg_n_custom*`)

This demonstrates that runtime behavior distinguishes:
- null-terminated strings vs sized memory blocks
- generic style vs colon-sensitive style
- default quote delimiters vs caller-supplied delimiters
- one-shot quoting vs slot-based reusable quoting state

## 6.5 Multibyte and character-set boundaries

The presence of:

- `mbrtoc32`
- `locale_charset`
- `hard_locale`

shows explicit handling of character-encoding boundaries.

Behaviorally relevant special cases include:

- locale-dependent text interpretation
- multibyte decoding
- differences between simple and hard locale environments

The summaries do not specify which user-facing paths in `pwd` directly depend on these functions, but they clearly belong to boundary-sensitive text behavior in the program support stack.

## 6.6 Stream-buffer corner cases

`fflush.c` contains special handling for:

- `clear_ungetc_buffer_preserving_position`
- `clear_ungetc_buffer`
- `disable_seek_optimization`
- `restore_seek_optimization`
- `update_fpos_cache`

This means output/input stream state is not treated as trivial. Boundary cases around stream buffer contents, pushback state, seek optimization flags, and cached file position are explicitly handled during flush behavior.

## 6.7 Broken pipe special handling

`close_stdout_set_ignore_EPIPE` shows a dedicated broken-pipe policy switch. Therefore, `EPIPE` is treated as a first-class special case in final output handling, not merely as an ordinary generic output error.

---

## 7. Behaviors that must remain consistent with the C version

## 7.1 Startup and dispatch consistency

The following top-level behavior must remain unchanged:

- program execution starts in `main`
- command-line dispatch selects between normal path printing and informational/error flows
- program-name setup occurs before diagnostics that depend on it
- final output is explicitly finalized

## 7.2 Path acquisition consistency

The C-version behavior clearly distinguishes at least two path-resolution modes:

- `logical_getcwd`
- `robust_getcwd`

That distinction must remain intact. A reimplementation must not collapse them into a single undifferentiated operation when preserving behavior.

## 7.3 Reconstruction consistency

For the robust path:

- a mutable `file_name` state object is used
- path components are prepended
- parent traversal is depth-tracked
- root detection participates in stopping traversal
- temporary path state is freed

Those dynamic steps must remain consistent.

## 7.4 Quoting subsystem consistency

The quoting subsystem must preserve these behavioral categories:

- options objects are mutable
- style/flags/per-character/custom delimiters are distinct configuration dimensions
- both buffer-based and allocating APIs exist
- both string and sized-memory quoting exist
- numbered-slot quoting state exists
- global quote-state cleanup exists via `quotearg_free`

## 7.5 Output finalization consistency

The C version does not rely only on naïve stream closing. It has layered behavior through:

- `rpl_fflush`
- `close_stream`
- `rpl_fclose`
- `close_stdout`

That explicit flush/close/error path must remain consistent.

## 7.6 Allocation-failure consistency

The `x*alloc` family and `xalloc_die` implement a shared failure policy. Any behavior-preserving reimplementation must keep allocation failure handling centralized rather than returning unchecked nulls into unrelated runtime paths.

## 7.7 Locale/text consistency

The project includes locale-sensitive support functions and character-set helpers. Any preserved behavior should continue to respect:

- locale retrieval APIs
- locale hardness checks
- charset lookup
- multibyte conversion support
- case-insensitive text comparison support where used

The exact invocation sites for each are not fully visible, so the current module summary is insufficient to support a more detailed behavior judgment.

---

## 8. Performance-sensitive paths

## 8.1 Path reconstruction loop

The most obvious performance-sensitive runtime path is the robust current-directory reconstruction flow:

- repeated parent traversal
- repeated directory entry lookup through `find_dir_entry`
- repeated string/path prepending through `file_name_prepend`

This path is sensitive because it is iterative and stateful, unlike a single direct getter call.

## 8.2 Buffer-based quoting path

`quotearg_buffer` and especially `quotearg_buffer_restyled` represent a core formatting path that avoids forcing every call through heap allocation.

This is performance-sensitive because:

- it operates on caller-supplied buffers
- it is the central worker behind multiple wrappers
- many convenience APIs likely converge on it or related shared logic

## 8.3 Shared quote-wrapper fan-out

A large number of front-end quoting functions exist:

- `quotearg`
- `quotearg_mem`
- `quotearg_n`
- `quotearg_n_mem`
- `quotearg_style*`
- `quotearg_colon*`
- `quotearg_custom*`
- `quote*`

This indicates a wrapper-heavy design around a smaller set of core quoting engines. Performance-sensitive behavior therefore depends on preserving the shared-core execution path rather than duplicating logic in each wrapper.

## 8.4 Memory growth helpers

The allocation subsystem contains multiple growth-oriented functions:

- `x2realloc`
- `x2nrealloc`
- `xpalloc`
- `xreallocarray`
- `xnmalloc`

These are performance-sensitive because they support resizing and bulk allocation in repeated-growth scenarios. The summaries do not identify all call sites in `pwd`, but they clearly support hot-path dynamic storage management across the project.

## 8.5 Stream finalization path

`rpl_fflush` includes several internal helper steps around stream state normalization. Since output finalization runs on every successful output path, this is a small but important runtime path where state handling has been specialized.

## 8.6 Locale/charset query caching or repeated lookup

`locale_charset` is a large standalone function body (`830-1159`), which suggests nontrivial runtime work. Whether it is invoked once or repeatedly by `pwd` is not established by the summaries. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 9. Module-by-module dynamic behavior summary

## 9.1 `pwd.c` behavior cluster

Primary dynamic roles:

- parse invocation
- choose execution mode
- retrieve working directory logically or robustly
- build path incrementally when reconstruction is needed
- print usage when requested or needed

Key mutable runtime data:

- `struct file_name`
- traversal depth (`parent_height`)
- directory identity (`struct stat`)

## 9.2 `root-dev-ino.c` behavior cluster

Primary dynamic role:

- obtain root device/inode identity used as a traversal boundary marker

## 9.3 `quotearg.c` behavior cluster

Primary dynamic roles:

- manage quoting configuration state
- quote text into buffers or newly allocated memory
- support reusable slot-based quoted-string state
- support custom, style-based, colon-based, and per-character quoting
- release retained quoting state

## 9.4 `closeout.c`, `close-stream.c`, `fclose.c`, `fflush.c` behavior cluster

Primary dynamic roles:

- normalize stream state
- flush buffers
- close streams
- centralize final stdout error handling
- apply special policy for broken pipes

## 9.5 `version-etc.c` behavior cluster

Primary dynamic roles:

- emit version, package, authorship, and bug-reporting text
- serve informational execution branches instead of normal `pwd` output

## 9.6 Locale/text support cluster

Included files:

- `setlocale_null-unlocked.c`
- `setlocale_null.c`
- `hard-locale.c`
- `localcharset.c`
- `propername-lite.c`
- `c-strcasecmp.c`
- `mbrtoc32.c`

Primary dynamic roles:

- retrieve locale state
- perform locale-sensitive decisions
- support charset-sensitive and multibyte-sensitive text processing

## 9.7 Allocation cluster

Included files:

- `xmalloc.c`
- `xalloc-die.c`
- `xgetcwd.c`

Primary dynamic roles:

- perform checked allocation/reallocation/duplication
- terminate or escalate on allocation failure through centralized logic
- acquire current directory with allocation support through `xgetcwd`

---

## 10. Insufficiency notes

The following details are not justified by the current summaries and should remain unspecified:

- exact command-line option syntax and precedence
- exact exit-status values in each branch
- exact fallback ordering between `logical_getcwd`, `xgetcwd`, and `robust_getcwd`
- exact output wording for usage, errors, and version text
- exact internal storage layout and reuse policy for `quotearg_n*`
- exact buffer sizing, growth factors, or allocation strategies
- exact errno mapping and specific return-value semantics for stream/locale/quoting helpers

For these points, the current module summary is insufficient to support a more detailed behavior judgment.