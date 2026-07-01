# 03_behaviors

## 1. Overall runtime behavior view

From the module summaries, this project is centered on a `cat` executable whose runtime starts in `main` (`cat.c:535-813`) and then uses a set of support modules for:

- command startup and program identity setup
- locale and character-set dependent behavior
- argument quoting for diagnostics and user-facing text
- stream closing and stdout finalization
- low-level file descriptor operations
- buffered and unbuffered copy paths
- safe/full read-write loops
- memory allocation helpers
- version/help reporting

The available evidence is enough to describe the broad dynamic execution layers, but not enough to reconstruct every branch in `main` or every command-line option path. Where detailed branch logic is not visible from the module summary, this document states that explicitly.

---

## 2. Initialization flow and startup order

## 2.1 Process entry

The process entry point is:

- `main` in `cat.c`

This is the top-level runtime coordinator. Based on the surrounding module set, startup behavior includes coordination of:

- program naming (`set_program_name`)
- locale setup/query support (`setlocale_null*`, `hard_locale`, `locale_charset`)
- output finalization support (`close_stdout*`, `close_stream`)
- option/help/version reporting (`usage`, `version_etc*`, `emit_bug_reporting_address`)
- the actual copy path (`copy_cat`, `simple_cat`, `cat`)

The current module summary is insufficient to support a more detailed behavior judgment about exact statement order inside `main`, but `main` is clearly the stage where global runtime mode is selected.

## 2.2 Early program identity state

`set_program_name` (`progname.c`) is a startup-oriented function. Its runtime role is to establish the process's canonical program name from `argv[0]`. Dynamic effects:

- converts process entry metadata into normalized internal name state
- prepares later diagnostics and usage/version text to print a consistent command name

This name-setting behavior is important because later user-visible flows such as `usage`, `version_etc*`, and error-reporting helpers conventionally depend on stable program-name state.

## 2.3 Locale-related initialization state

Several modules form a locale state layer:

- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `setlocale_null`
- `hard_locale`
- `locale_charset`
- `mbrtoc32`

These imply a startup/runtime phase where the program queries locale category settings and derives encoding-dependent behavior. Dynamic roles:

- retrieve locale names without changing locale state
- copy locale names into caller buffers
- provide locked vs unlocked access paths
- determine whether the active locale is "hard"
- expose current locale character set
- convert multibyte input to 32-bit character values

These functions indicate that the program's text-processing behavior is not purely byte-agnostic in all modes. When display-oriented options are active, locale state affects character classification or rendering paths.

The current module summary is insufficient to support a more detailed behavior judgment about exactly which locale categories are queried during startup.

## 2.4 Stdout finalization registration state

The presence of:

- `close_stdout_set_file_name`
- `close_stdout_set_ignore_EPIPE`
- `close_stdout`

shows a startup-to-shutdown control path where process-wide output-close behavior is configured before main work begins. Dynamic meaning:

- a target file name can be recorded for later close-time diagnostics
- EPIPE handling policy can be configured
- final stdout closure is centralized

This creates a small global runtime state machine for final output handling:

1. unconfigured
2. configured with file name and/or EPIPE policy
3. normal output execution
4. final flush/close
5. success or close-time failure handling

## 2.5 Binary/text mode initialization

`set_binary_mode` and `xset_binary_mode_error` indicate a platform-sensitive startup step for file descriptors. Dynamic role:

- switch standard streams or target descriptors into binary mode when required by runtime environment
- abort through a dedicated fatal path if that mode change cannot be established in contexts that require it

The current module summary is insufficient to support a more detailed behavior judgment about which descriptors are switched and at exactly what point in `main`.

## 2.6 Memory helper availability during startup

Allocation support modules are ready for use from process start:

- `alignalloc` / `alignfree`
- `xalignalloc`
- `xmalloc`, `xrealloc`, `xpalloc`, `xcalloc`, `xstrdup`, etc.
- `xalloc_die`

Dynamic meaning:

- normal execution may allocate copy buffers, quoted strings, or temporary text state
- allocation failures do not remain local if xalloc wrappers are used; they route into a fatal termination path

This gives the whole program a fail-fast allocation behavior for code paths that use the `x*` family.

---

## 3. Main user operation flows

## 3.1 Top-level operational split

The core operational functions in `cat.c` are:

- `usage`
- `next_line_num`
- `simple_cat`
- `write_pending`
- `cat`
- `copy_cat`
- `main`

This supports a runtime structure with at least these major user-visible flows:

1. startup and argument processing in `main`
2. help/version/usage exit flow
3. fast copy flow through `simple_cat`
4. decorated/transforming copy flow through `cat`
5. orchestration flow through `copy_cat`
6. final stdout close/cleanup

## 3.2 Help/version flow

`usage` in `cat.c` and `version_etc*` / `emit_bug_reporting_address` in `version-etc.c` define a reporting branch that does not perform normal file concatenation work.

Dynamic flow shape:

1. `main` determines that help/version text should be emitted
2. `usage` or one of the `version_etc*` family prints formatted output to a `FILE *stream`
3. `emit_bug_reporting_address` may print bug-report destination text
4. control exits through status-oriented process termination

Because `version_etc` has layered entry points (`version_etc`, `version_etc_va`, `version_etc_ar`, `version_etc_arn`), the reporting flow supports:
- variadic author lists
- `va_list` forwarding
- counted author arrays

Behaviorally, all of these are alternate entry forms into one reporting task: emitting version/author information to a stream.

## 3.3 Main data-copy flow

### 3.3.1 Copy orchestrator

`copy_cat` is the immediate operational dispatcher for file-copy behavior. Its return type is `int`, so it acts as a status-producing wrapper around lower-level copy functions.

Dynamic responsibilities likely include:
- selecting between the simple and decorated paths
- preparing buffers
- invoking one of the actual copy engines
- returning an operation status back toward `main`

The current module summary is insufficient to support a more detailed behavior judgment about exact selection criteria.

### 3.3.2 Fast path: `simple_cat`

`simple_cat (char *buf, idx_t bufsize)` is the compact copy engine. Its runtime behavior is characterized by:

- repeated input-to-output transfer using a caller-supplied buffer
- minimal transformation state
- boolean success/failure outcome

This is the performance-oriented path for plain concatenation behavior. Since `simple_cat` has an external call count and is separate from the feature-rich `cat` routine, the runtime design clearly distinguishes:
- a lower-overhead transfer mode
- a richer formatting/inspection mode

### 3.3.3 Feature path: `cat`

`cat (...)` has explicit control flags:

- `show_nonprinting`
- `show_tabs`
- `number`
- `number_nonblank`
- `show_ends`
- `squeeze_blank`

This is the main transformation state engine. Dynamic behavior includes reading input, examining content, updating line-format state, and emitting transformed output according to active flags.

Important runtime properties visible from the signature alone:

- behavior changes based on a set of mode flags rather than separate functions
- numbered output and blank-line squeezing require persistent state across input bytes/lines
- visible end markers and nonprinting expansion imply output can grow relative to input
- separate input and output buffers are used, so transformation is staged rather than in-place

### 3.3.4 Pending-output draining

`write_pending (char *outbuf, char **bpout)` is an inline helper that flushes the currently accumulated output region.

Dynamic role:
- treat `outbuf..*bpout` as the current produced-but-not-yet-written segment
- write that segment when the transformation engine decides the output buffer is full or needs to be committed
- reset or advance the output pointer state

This makes the decorated copy path a buffered state machine rather than a write-per-character path.

## 3.4 Line numbering flow

`next_line_num` is a dedicated state-transition function. Dynamic effect:

- mutate the current line-number state to the next displayable value
- support numbering modes inside `cat`

Since `cat` supports both `number` and `number_nonblank`, runtime transitions depend on input classification:

- if all lines are numbered, every logical line transition can advance line-number state
- if only nonblank lines are numbered, advancing depends on whether a line is blank

The current module summary is insufficient to support a more detailed behavior judgment about the exact internal representation of the line counter.

## 3.5 Low-level transfer flow

The project includes two low-level transfer helpers:

- `safe_rw`
- `full_rw`

Dynamic relationship:

- `safe_rw` is the low-level guarded read/write primitive
- `full_rw` is the loop-oriented wrapper that continues until a requested byte count has been fully processed or an end/error condition stops progress

This strongly suggests a layered runtime transfer model:

1. a single syscall-facing operation that handles fragile system-call behavior
2. a retry/accumulation loop that aims to complete larger transfers

The current module summary is insufficient to support a more detailed behavior judgment about whether `safe_rw` is bound to `read` or `write` in this build variant, because only one generalized signature is shown.

## 3.6 File-descriptor and stream support flows

The runtime includes both descriptor-level and stream-level support:

- descriptor level: `dupfd`, `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`, `klibc_fcntl`, `set_binary_mode`, `copy_file_range`
- stream level: `rpl_fflush`, `fpurge`, `rpl_fclose`, `close_stream`, `close_stdout`

This indicates that the program crosses abstraction boundaries depending on need:

- high-volume copy uses raw descriptors or unbuffered operations
- reporting/help/version and finalization use `FILE *` streams
- compatibility wrappers normalize behavior across platforms

## 3.7 Quoting flow for diagnostics and user-facing text

The `quotearg` family is a substantial runtime subsystem. It exposes several layers:

### Configuration state
- `clone_quoting_options`
- `get_quoting_style`
- `set_quoting_style`
- `set_char_quoting`
- `set_quoting_flags`
- `set_custom_quoting`
- `quoting_options_from_style`

### Conversion/output production
- `quotearg_buffer_restyled`
- `quotearg_buffer`
- `quotearg_alloc`
- `quotearg_alloc_mem`
- `quotearg_n_options`
- `quotearg`, `quotearg_mem`
- `quotearg_n`, `quotearg_n_mem`
- `quotearg_style`, `quotearg_style_mem`
- `quotearg_char`, `quotearg_char_mem`
- `quotearg_colon`, `quotearg_colon_mem`
- `quotearg_n_style`, `quotearg_n_style_mem`
- `quotearg_n_style_colon`
- `quotearg_custom`, `quotearg_custom_mem`
- `quotearg_n_custom`, `quotearg_n_custom_mem`
- `quote`, `quote_mem`
- `quote_n`, `quote_n_mem`
- `quotearg_free`

Dynamic behavior:

1. a quoting style/options object is selected or built
2. specialized wrappers adapt simple caller intent into that options object
3. argument text is transformed into a quoted representation
4. result is stored into caller buffer, allocated memory, or a slot-indexed reusable region
5. reusable quoting storage can later be released by `quotearg_free`

This subsystem is important for diagnostics involving filenames, option text, or locale-sensitive quotation marks.

---

## 4. State machines and state transitions

## 4.1 Main execution state machine

A high-level runtime state model, directly supported by the module set, is:

1. **process entry**
   - control enters `main`

2. **startup configuration**
   - program name state established
   - locale/charset state queried or made available
   - stdout-close behavior configured
   - binary mode may be configured

3. **command mode selection**
   - help/version flow or copy flow is selected

4. **copy preparation**
   - copy engine selected
   - buffers allocated or reused
   - input/output formatting mode established

5. **active transfer**
   - input is read
   - optional transformation state is updated
   - output is written

6. **completion**
   - remaining buffered output is flushed
   - streams/stdout are closed
   - status returned

7. **error termination**
   - any fatal allocation/mode/output error diverts to an error path

The current module summary is insufficient to support a more detailed behavior judgment about all intermediate substates in `main`.

## 4.2 Decorated `cat` line-processing state machine

Because `cat` accepts multiple transformation flags, its runtime is best understood as a line/byte processing state machine with persistent flags. Observable state categories include:

- current line-number state
- whether current line is blank or nonblank
- whether blank-line squeezing is active
- output-buffer fill state
- current input scan position
- current mode set for visual transformations

Likely transitions visible from the function split:

- **line start** -> decide numbering prefix
- **character scan** -> classify byte/character
- **special rendering** -> expand tabs, ends, or nonprinting forms
- **buffer threshold** -> call `write_pending`
- **newline reached** -> update blank/nonblank and squeeze state
- **line transition** -> call `next_line_num` when numbering rules require it

The current module summary is insufficient to support a more detailed behavior judgment about the exact blank-line memory representation.

## 4.3 Quoting options state machine

The quoting subsystem clearly has mutable options state. Dynamic transitions include:

1. **default or inherited options**
2. **cloned options**
3. **style update via `set_quoting_style`**
4. **character-specific update via `set_char_quoting`**
5. **flag update via `set_quoting_flags`**
6. **custom quote delimiter update via `set_custom_quoting`**
7. **rendering through one of the `quotearg*` entry points**
8. **cleanup through `quotearg_free`**

Important consistency point:
- wrappers like `quotearg_style`, `quotearg_colon`, and `quotearg_custom` are not independent state machines; they are preset transitions into the same underlying quoting engine.

## 4.4 Locale query state machine

The `setlocale_null` family defines a query-oriented state machine rather than a mutation-oriented one:

1. caller requests locale text for a category
2. unlocked helper or locked wrapper is chosen
3. result is returned directly or copied into caller buffer
4. caller uses returned locale name to decide behavior

Because both locked and unlocked variants exist, the module preserves two execution contexts:
- direct/unlocked fast access
- serialized access via lock-protected wrapper

## 4.5 Output finalization state machine

The `close_stdout` subsystem exposes a small but important machine:

- **initial defaults**
- **file name configured**
- **EPIPE ignore policy configured**
- **close invoked**
- **close success**
- **close failure with policy-sensitive handling**

This state must remain stable because it governs the last observable phase of execution.

## 4.6 Allocation state machine

The `xmalloc` family and `xalloc_die` implement a common allocation lifecycle:

1. caller requests allocation/reallocation/duplication
2. operation succeeds and pointer is returned
3. or operation fails and control transfers to fatal allocator exit path

For growth-oriented helpers such as `x2realloc`, `x2nrealloc`, and `xpalloc`, there is an additional growth-state transition:
- existing buffer state -> resized buffer state

The current module summary is insufficient to support a more detailed behavior judgment about exact growth formulas.

---

## 5. Error-handling flows

## 5.1 Fatal allocation errors

Modules:
- `xmalloc.c`
- `xalloc-die.c`
- `xalignalloc.c`

Dynamic behavior:
- any `x*alloc` family member that cannot satisfy memory needs routes into `xalloc_die`
- `xalignalloc` adds aligned allocation on top of this fatal-on-failure model

This means many internal callers do not need local recovery logic; failure exits the program through a central fatal path.

## 5.2 Binary-mode setup failure

Modules:
- `binary-io.c`
- `xbinary-io.c`

Dynamic behavior:
- `set_binary_mode` performs the mode switch and returns status
- `xset_binary_mode_error` is a no-return fatal branch for failure cases that must terminate execution

This separates recoverable status production from enforced-fatal policy.

## 5.3 Stream close and flush errors

Modules:
- `close-stream.c`
- `closeout.c`
- `fflush.c`
- `fclose.c`

Dynamic behavior:
- `rpl_fflush` manages flush-time state carefully
- `close_stream` provides a consolidated close result for `FILE *`
- `close_stdout` centralizes final output stream completion
- `rpl_fclose` wraps stream close behavior, with `fclose_nothrow` as an internal helper

Error handling here matters at program end, where write failures may not surface until flush/close time.

## 5.4 Copy/write failures

Modules:
- `safe-read.c`
- `full-write.c`
- `cat.c`

Dynamic behavior:
- low-level transfer helpers produce partial-progress or stop conditions
- higher-level copy code returns boolean or integer status
- `main` ultimately converts operational success/failure into process exit behavior

The current module summary is insufficient to support a more detailed behavior judgment about which exact errno cases are retried, preserved, or transformed.

## 5.5 Compatibility-path failures in descriptor operations

Modules:
- `fcntl.c`
- `copy-file-range.c`

Dynamic behavior:
- replacement or wrapper functions normalize platform behavior
- failures in descriptor duplication or range-copy operations are handled inside compatibility layers before result is returned to callers

The current module summary is insufficient to support a more detailed behavior judgment about exact fallback chains for each platform-specific action.

---

## 6. Boundary conditions and special-case handling

## 6.1 Zero-length and bounded-buffer conditions

Several signatures explicitly expose size-aware behavior:

- `safe_rw(fd, buf, count)`
- `full_rw(fd, buf, count)`
- `quotearg_buffer(buffer, buffersize, arg, argsize, o)`
- `quotearg_alloc_mem(arg, argsize, size, o)`
- `quotearg_n_options(n, arg, argsize, options)`
- `quote_mem`, `quotearg_mem`, `quotearg_style_mem`, `quotearg_custom_mem`
- `setlocale_null_r(..., buf, bufsize)`
- `mbrtoc32(..., s, n, ps)`

Dynamic implication:
- these paths are designed to operate on explicit byte counts, not only NUL-terminated strings
- behavior for embedded NULs or non-string byte regions is part of normal operation for the `_mem` variants
- buffer-size limits are runtime control inputs, not just static API details

## 6.2 Blank-line special handling

The main `cat` engine includes:
- `number_nonblank`
- `squeeze_blank`

This establishes two important special-case domains:
- blank lines are distinguished from nonblank lines
- repeated blank lines trigger different transitions than isolated blank lines

Behaviorally, blank lines are not treated as ordinary content in all modes.

## 6.3 Tab, end-of-line, and nonprinting special handling

Flags:
- `show_tabs`
- `show_ends`
- `show_nonprinting`

These define explicit content-classification branches in the copy loop. Runtime effect:
- tabs may be emitted in transformed form
- end-of-line may receive visible markers
- nonprinting characters may be rewritten into displayable sequences

This means output size can exceed input size and output-buffer pressure is mode-dependent.

## 6.4 Locale and multibyte special handling

Modules:
- `hard_locale`
- `locale_charset`
- `mbrtoc32`

Dynamic implication:
- text classification and display-oriented transformations must account for locale-dependent character encoding
- multibyte boundary conditions matter when reading/displaying character data

The current module summary is insufficient to support a more detailed behavior judgment about exact handling of invalid or incomplete multibyte sequences.

## 6.5 Quoting specializations

The quoting subsystem has many special-entry wrappers, each representing a distinct runtime preset:

- colon-oriented quoting
- style-specific quoting
- custom left/right delimiter quoting
- slot-indexed quoting (`quotearg_n*`)
- default public quoting (`quote`, `quote_n`)

Boundary-sensitive behavior here includes:
- explicit byte-length handling for `_mem` forms
- caller-supplied delimiters for custom forms
- indexed storage selection for `_n` forms

## 6.6 Alignment-sensitive allocation

`alignalloc` / `alignfree` and `xalignalloc` show that some runtime buffers may require alignment constraints. Dynamic special handling includes:

- over-allocation or pointer adjustment behavior inside aligned allocation support
- separate free logic for aligned blocks

The current module summary is insufficient to support a more detailed behavior judgment about exact alignment arithmetic invariants beyond the existence of helper functions such as `align_down` and `address_of_pointer_to_malloced`.

---

## 7. Behaviors that must remain consistent with the C version

## 7.1 Entry and termination structure

The following must remain behaviorally stable:

- control begins in `main`
- help/version/reporting remain alternate top-level flows
- normal concatenation work returns status through `copy_cat`/`main`
- final output close handling occurs through the dedicated close/flush path

## 7.2 Dual copy-engine design

The distinction between:
- `simple_cat`
- `cat`

must remain intact. This is a core behavioral split between:
- a simpler transfer mode
- a feature-processing mode with formatting state

Any reimplementation must preserve that there are different runtime paths for plain and decorated copying.

## 7.3 Flag-driven transformation semantics

The `cat` function signature exposes a stable behavioral contract around these runtime switches:

- `show_nonprinting`
- `show_tabs`
- `number`
- `number_nonblank`
- `show_ends`
- `squeeze_blank`

These must continue to act as live control-state inputs to the copy loop, not as compile-time-only distinctions.

## 7.4 Line-number progression behavior

Because `next_line_num` is separated as a dedicated helper, line-number state advancement is a distinct transition in the original C logic. Any port must preserve:
- explicit line-number state
- advancement at line boundaries under numbering rules
- interaction with `number` and `number_nonblank`

## 7.5 Buffered output commit behavior

`write_pending` exists as a dedicated flush step inside the main transform path. Required invariant:
- transformed output accumulates in an output buffer and is flushed in chunks
- the copy engine does not collapse into an unrelated write pattern that changes chunking/state semantics without reason

## 7.6 Low-level transfer layering

The two-level transfer model must remain consistent:

- guarded primitive transfer via `safe_rw`
- completion-oriented looping via `full_rw`

Even if implementation details differ, the runtime layering and stop/progress behavior must remain faithful.

## 7.7 Quoting subsystem behavior families

The C version exposes a rich family of behaviorally related quoting entry points. A consistent implementation must preserve:

- mutable quoting-options objects
- style-based rendering
- character-specific quoting control
- flag-based quoting control
- custom delimiter support
- slot-indexed reusable quoting results
- buffer-based and allocation-based output forms
- explicit-length `_mem` handling
- cleanup via `quotearg_free`

## 7.8 Locale query and character conversion separation

The distinction between:
- locale name retrieval (`setlocale_null*`)
- locale hardness test (`hard_locale`)
- charset retrieval (`locale_charset`)
- multibyte conversion (`mbrtoc32`)

must remain visible in behavior. These are not one interchangeable function; they represent separate runtime decisions and transformations.

## 7.9 Final-output error surfacing

The close/flush family must preserve the C version's end-of-program error behavior structure:

- pending output is flushed
- stdout close is centralized
- EPIPE policy can be configured
- close-time failures remain part of the observable runtime model

## 7.10 Fatal helper behavior

Functions marked by role or type as fatal must remain fatal in behavior:

- `xalloc_die`
- `xset_binary_mode_error`

Callers rely on these as no-return termination branches.

---

## 8. Performance-sensitive paths

## 8.1 Plain copy fast path

`simple_cat` is the clearest performance-sensitive path in the project. Reasons:

- dedicated separate function for simple copying
- caller-supplied reusable buffer
- boolean status return instead of richer formatting logic
- likely selected when transformation flags are inactive

This path should minimize per-byte decision overhead.

## 8.2 Buffered decorated path

Even in feature mode, performance sensitivity is visible through:
- separate input and output buffers in `cat`
- `write_pending` as a chunked output helper

This indicates the implementation is designed to avoid emitting output one byte at a time when transformations enlarge or rewrite content.

## 8.3 Low-level full transfer loop

`full_rw` is performance-relevant because it amortizes system-call overhead across larger logical transfers while still coping with short operations. It sits on a hot I/O path.

## 8.4 Quoting engine core renderer

Within the quoting subsystem, the performance-critical core is:

- `quotearg_buffer_restyled`

Many convenience wrappers funnel toward this behavior. Performance considerations here affect:
- diagnostics that quote many filenames
- repeated quote operations across command execution
- indexed reusable output via `quotearg_n_options`

## 8.5 Reusable quoting slots

The `quotearg_n*` family implies reuse of quote-result storage keyed by slot index `n`. Dynamic benefit:
- repeated formatting calls can avoid forcing every caller to manage storage externally

The current module summary is insufficient to support a more detailed behavior judgment about exact storage lifetime rules beyond the existence of `quotearg_free`.

## 8.6 Allocation growth helpers

`x2realloc`, `x2nrealloc`, and `xpalloc` are performance-sensitive support routines because they manage resizing and growth of dynamic buffers. They matter for:
- amortized growth behavior
- reducing repeated small reallocations
- maintaining throughput in variable-size text or quoting flows

The current module summary is insufficient to support a more detailed behavior judgment about exact growth thresholds.

## 8.7 Aligned allocation

`alignalloc` and `xalignalloc` indicate that some buffers may be alignment-sensitive for runtime efficiency or subsystem requirements. The presence of alignment-specific helpers means preserving aligned-buffer behavior is performance-relevant, not merely cosmetic.

---

## 9. Areas where evidence is limited

The following details cannot be stated more strongly from the supplied module summaries alone:

- exact command-line option parsing order inside `main`
- exact conditions that choose `simple_cat` versus `cat`
- precise retry/termination semantics inside `safe_rw`
- exact output and exit-code mapping in failure cases
- exact locale categories queried during startup
- exact representation of line numbers and blank-line squeeze state
- exact fallback logic inside `copy_file_range` and `fcntl` compatibility wrappers
- exact multibyte error-path details in `mbrtoc32`
- exact caching and storage lifetime rules in the `quotearg_n*` family

For each of these, the current module summary is insufficient to support a more detailed behavior judgment.