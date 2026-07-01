# 03_behaviors

## 1. Overall runtime behavior

This project is centered on a `cat` executable, with `main` in `cat.c` acting as the startup coordinator and the rest of the modules supplying supporting runtime behaviors: content copying, formatted output transformations, locale handling, stream closing, quoting for diagnostics/help text, memory allocation helpers, and platform-compatibility wrappers.

From the module summaries, the observable runtime organization is:

- **Program entry and control**: `main` (`cat.c`)
- **Primary data movement**:
  - `copy_cat`
  - `simple_cat`
  - `cat`
  - `write_pending`
  - `safe_rw`
  - `full_rw`
  - `copy_file_range`
- **User-visible metadata/help/version output**:
  - `usage`
  - `version_etc*`
  - `emit_bug_reporting_address`
- **Output/error finalization**:
  - `close_stdout*`
  - `close_stream`
- **Locale/encoding-dependent behavior**:
  - `setlocale_null*`
  - `hard_locale`
  - `locale_charset`
  - `mbrtoc32`
  - `proper_name_lite`
- **Quoting and formatting of strings for messages**:
  - `quotearg*`
  - `quote*`
- **Memory support and fatal allocation handling**:
  - `alignalloc` / `alignfree`
  - `xalignalloc`
  - `xmalloc` family
  - `xalloc_die`
- **Platform and stdio/file-descriptor compatibility layers**:
  - `fcntl` wrappers
  - `binary-io`
  - `xbinary-io`
  - `fflush` replacement helpers
  - `fclose` replacement
  - `fpurge`
  - `fadvise`

The current module summary is insufficient to support a more detailed whole-program call graph beyond these role groupings.

---

## 2. Initialization flow and startup order

## 2.1 Entry point

The executable starts in:

- `main` (`cat.c:535-813`)

This is the primary startup function and is the only observed non-static public entry point for the executable path.

## 2.2 Early process setup

The module set strongly indicates that early startup includes setup steps commonly required before main user operations begin:

- `set_program_name` initializes the program-name state from `argv[0]`.
- locale-related setup is supported by:
  - `setlocale_null`
  - `setlocale_null_r`
  - `hard_locale`
  - `locale_charset`
- output stream shutdown behavior is configurable through:
  - `close_stdout_set_file_name`
  - `close_stdout_set_ignore_EPIPE`

Because `main` has 13 external calls and the project includes dedicated modules for program-name, locale, quoting, version text, and close-on-exit output handling, these are part of the startup-capable behavior surface. However, the current module summary is insufficient to support a more detailed behavior judgment about exact invocation order inside `main`.

## 2.3 Command dispatch preparation

`main` is also the visible point where the runtime must choose among at least these broad paths:

1. **usage/help or version-style output path**
2. **normal content-copying path**
3. **startup failure path**
4. **normal shutdown path with output close handling**

This follows from the presence of:
- `usage`
- `version_etc*`
- `emit_bug_reporting_address`
- `copy_cat`
- `close_stdout`

The current module summary is insufficient to support a more detailed behavior judgment about option parsing order, specific flags, or exact branch conditions.

## 2.4 Resource preparation before copy operations

The copy path is supported by:
- aligned allocation helpers: `alignalloc`, `xalignalloc`
- general allocation helpers: `xmalloc` family
- binary/text mode control: `set_binary_mode`, `xset_binary_mode_error`
- advisory I/O helper: `fdadvise`, `fadvise`

This indicates startup may include buffer preparation and file-descriptor mode configuration before entering the main copy loop. The current module summary is insufficient to support a more detailed behavior judgment about exact buffer sizes, which descriptors are switched, or when advisory I/O is applied.

---

## 3. Main user operation flows

## 3.1 High-level user-visible execution modes

The program’s runtime behavior separates into two main classes:

### A. Informational execution
Used when the program emits usage/help/version-related text rather than copying file content.

Relevant functions:
- `usage`
- `version_etc`
- `version_etc_va`
- `version_etc_ar`
- `version_etc_arn`
- `emit_bug_reporting_address`

Behaviorally, this path:
- formats program-identification and author/package/version strings,
- emits bug-reporting information,
- terminates without entering the content-copy engine.

The current module summary is insufficient to support a more detailed behavior judgment about exact output text or branch triggers.

### B. Content-copy execution
Used when the program performs cat-like file-to-output transfer.

Relevant functions:
- `copy_cat`
- `simple_cat`
- `cat`
- `write_pending`
- `safe_rw`
- `full_rw`
- `copy_file_range`

This is the dominant operational path.

---

## 3.2 Copy path structure

### 3.2.1 Dispatcher level

`copy_cat` (`cat.c:503-532`) appears to be the direct content-copy coordinator exposed from `cat.c` aside from `main`.

Its role in runtime terms is to:
- enter the file-copying operation,
- choose or invoke the actual copy implementation,
- return a status to its caller.

Since `copy_cat` is separated from `main`, the runtime likely uses it as the main execution unit once startup and option handling are complete. The current module summary is insufficient to support a more detailed behavior judgment about whether `copy_cat` handles one source or a sequence of sources.

### 3.2.2 Fast/simple transfer path

`simple_cat` (`cat.c:155-181`) is explicitly a simpler copy routine.

Behaviorally, this path represents:
- a reduced-transformation transfer mode,
- movement of input bytes into an output buffer,
- completion/failure reported as `bool`.

Because `simple_cat` is distinct from the more complex `cat` routine, the runtime contains a branch between:
- **plain copy behavior**
- **transformed/annotated copy behavior**

The current module summary is insufficient to support a more detailed behavior judgment about the exact selection rules.

### 3.2.3 Transforming transfer path

`cat` (`cat.c:211-497`) is the main transformation-capable copy engine. Its signature shows runtime behavior depends on a collection of boolean controls:

- `show_nonprinting`
- `show_tabs`
- `number`
- `number_nonblank`
- `show_ends`
- `squeeze_blank`

This is strong direct evidence that the program has a transformation pipeline that can, during copying:

- annotate or rewrite nonprinting characters,
- specially treat tab characters,
- number lines,
- number only nonblank lines,
- mark line endings,
- compress repeated blank lines.

This function is therefore the main stateful content-processing engine, not just a pass-through byte copier.

### 3.2.4 Buffered output flushing during transformation

`write_pending` (`cat.c:187-197`) is an inline helper with:
- an output buffer pointer,
- a moving output position pointer.

This indicates that the transforming `cat` routine accumulates output into a buffer and flushes pending data when needed. Runtime behavior includes:
- appending transformed bytes into `outbuf`,
- advancing `bpout`,
- flushing buffered output through `write_pending` at defined thresholds or transition points.

The current module summary is insufficient to support a more detailed behavior judgment about exact flush thresholds.

---

## 3.3 Low-level read/write behavior

## 3.3.1 `safe_rw`

`safe_rw` exists both under `include/safe-read.c` and `safe-read.c` with the same signature:

- `size_t safe_rw (int fd, void const *buf, size_t count);`

The naming and placement indicate it is a low-level I/O primitive used in transfer paths. The exact retry/error semantics cannot be stated from the summary alone. Behavior that can be stated safely:

- it performs a file-descriptor based data transfer operation,
- it accepts an fd, a buffer, and a byte count,
- it returns a `size_t` amount/result used by higher-level copy logic.

The current module summary is insufficient to support a more detailed behavior judgment.

## 3.3.2 `full_rw`

`full_rw` (`full-write.c:57-79`) is another low-level transfer helper and has one external call, indicating it is layered over another primitive rather than being the terminal system interface.

Behaviorally:
- it is part of the output/write path,
- it attempts to carry out a complete transfer over a descriptor-based interface,
- it returns a `size_t` result consumed by callers.

The current module summary is insufficient to support a more detailed behavior judgment about partial-write retry behavior.

## 3.3.3 `copy_file_range`

`copy_file_range` (`copy-file-range.c`) supplies a compatibility wrapper for a kernel/file-descriptor assisted copy primitive.

Behaviorally:
- it offers a direct in-fd to out-fd transfer path,
- supports optional input and output offsets,
- exposes `length` and `flags`,
- returns an `ssize_t` operation result.

This introduces an alternate transfer path that can bypass byte-wise transformation when the runtime can use descriptor-to-descriptor copying. The current module summary is insufficient to support a more detailed behavior judgment about whether `cat` actually dispatches to this path in the analyzed build.

---

## 4. State machines and state transitions

## 4.1 Program-level execution state machine

The observable program-level state machine is:

1. **Process entry**
   - enters `main`

2. **Startup/setup**
   - program name state prepared
   - locale/output behavior prepared
   - user-request mode selected

3. **Dispatch**
   - informational output path, or
   - content-copy path

4. **Copy execution**
   - simple copy mode, or
   - transformation mode

5. **Termination**
   - success completion, or
   - error completion with diagnostics/stream-closing behavior

The current module summary is insufficient to support a more detailed behavior judgment about substate ordering inside `main`.

---

## 4.2 Content transformation state machine in `cat`

The signature of `cat` provides direct evidence of multiple runtime modes that alter per-byte and per-line behavior.

### 4.2.1 Line numbering state

The presence of:
- `next_line_num`
- `number`
- `number_nonblank`

shows a dedicated line-numbering state machine.

Behaviorally, the program maintains line-number state that changes as lines are processed:

- on a line boundary, numbering state may advance,
- `next_line_num` updates internal line-number representation,
- whether the current line receives a number depends on `number` and `number_nonblank`.

The current module summary is insufficient to support a more detailed behavior judgment about internal counters, formatting width, or exact text representation.

### 4.2.2 Blank-line suppression state

The `squeeze_blank` boolean shows a stateful distinction between:
- first blank line in a run,
- subsequent blank lines in the same run.

That means the transforming copy loop must remember whether the immediately preceding processed line(s) were blank, then transition between states such as:

- **nonblank context**
- **blank line emitted**
- **consecutive blank line encountered**

The runtime effect is that repeated blank input lines are not handled identically when this mode is active.

### 4.2.3 Character rendering state

The booleans:
- `show_nonprinting`
- `show_tabs`
- `show_ends`

show a character-rendering state machine that distinguishes at least:

- newline/end-of-line characters,
- tab characters,
- printable/nonprinting characters.

Runtime transitions occur per input character:
- if current character is newline, end-marking behavior may apply,
- if current character is tab, tab-specific rendering may apply,
- if current character is nonprinting, alternate rendering may apply,
- otherwise normal pass-through rendering applies.

### 4.2.4 Buffered output state

`write_pending` and the `bpout` pointer show that output generation is not one-byte immediate output only; there is buffered accumulation state:

- **buffer empty / current write position at start**
- **buffer accumulating transformed output**
- **buffer flush required**
- **buffer reset after flush**

This state is critical because transformation output can expand input representation and requires controlled emission.

---

## 4.3 Quoting subsystem state machine

The `quotearg` family exposes a configurable formatting subsystem. Dynamic state is represented by `struct quoting_options` and modified through:

- `clone_quoting_options`
- `get_quoting_style`
- `set_quoting_style`
- `set_char_quoting`
- `set_quoting_flags`
- `set_custom_quoting`

Behaviorally, quoting proceeds through these states:

1. **options source selection**
   - provided options object, cloned options object, or style-derived options

2. **style configuration**
   - quoting style selected or updated

3. **character-specific adjustment**
   - specific characters marked for special quoting treatment

4. **format execution**
   - one of the `quotearg_*` functions produces a quoted string or writes to a caller-supplied buffer

5. **slot/cache lifecycle**
   - `quotearg_n_options` suggests numbered quote slots
   - `quotearg_free` releases retained quoting state

The current module summary is insufficient to support a more detailed behavior judgment about storage lifetime, slot reuse policy, or exact escaping rules.

---

## 4.4 Locale state machine

The locale-related modules indicate a runtime distinction between:

- **locale query without locking**
  - `setlocale_null_unlocked`
  - `setlocale_null_r_unlocked`

- **locale query with locking**
  - `setlocale_null_r_with_lock`
  - `setlocale_null_r`
  - `setlocale_null`

This implies a locale-state access machine with transitions based on:
- whether lock protection is required,
- whether caller supplies output storage,
- whether the locale string is returned directly or copied into caller buffer.

`hard_locale` adds a binary classification state:
- **default/simple locale**
- **hard locale**

`locale_charset` adds a charset-resolution state:
- runtime determines the active locale character set and returns it for consumers such as quoting or multibyte conversion logic.

The current module summary is insufficient to support a more detailed behavior judgment about cache invalidation or synchronization details.

---

## 4.5 Stream/stdio state machine

Several modules show explicit state transitions around stdio streams:

- `rpl_fflush`
- `clear_ungetc_buffer_preserving_position`
- `clear_ungetc_buffer`
- `disable_seek_optimization`
- `restore_seek_optimization`
- `update_fpos_cache`
- `fpurge`
- `rpl_fclose`
- `close_stream`
- `close_stdout`

This reveals a stream-state machine involving:

1. **normal buffered stream**
2. **stream with ungetc/pushback content**
3. **stream with seek optimization temporarily disabled**
4. **flush in progress**
5. **purge/reset of buffered content**
6. **close in progress**
7. **closed / close failure**

The runtime specifically preserves and updates stream position-related state during flush/purge operations. The current module summary is insufficient to support a more detailed behavior judgment about exact stdio implementation branches.

---

## 5. Error-handling flows

## 5.1 Top-level errors

`main` has a substantial number of external calls and therefore acts as the top-level collector of failures from:
- startup configuration,
- copying,
- output emission,
- shutdown.

Behaviorally, error handling at the top level includes:
- aborting or returning failure after unsuccessful setup,
- returning failure from copy operations,
- performing output close checks before exit.

The current module summary is insufficient to support a more detailed behavior judgment about exit codes or exact diagnostic wording.

---

## 5.2 Output write and transfer errors

Error-sensitive transfer functions include:
- `safe_rw`
- `full_rw`
- `simple_cat`
- `cat`
- `copy_cat`
- `write_pending`
- `copy_file_range`

Runtime error flow here is:

1. low-level transfer function reports failure or short/terminal result,
2. current copy routine stops normal progression,
3. status propagates back through `simple_cat`/`cat`/`copy_cat`,
4. `main` resolves final process outcome.

This propagation pattern is supported by the boolean and integer return types across the copy layers. The current module summary is insufficient to support a more detailed behavior judgment about exact conversion between low-level failure results and final process status.

---

## 5.3 Allocation failures

The project has explicit fatal allocation support:

- `xmalloc` family
- `xalignalloc`
- `xalloc_die`

This establishes a dedicated error path for memory failure:

1. allocation helper is called,
2. null or invalid result is detected internally,
3. fatal handler `xalloc_die` is invoked,
4. control does not return in the fatal path, or returns only through program termination semantics.

`xset_binary_mode_error` is also marked `_Noreturn`, indicating another fatal error path tied to failure in binary mode setup.

The current module summary is insufficient to support a more detailed behavior judgment about emitted messages.

---

## 5.4 Stream closing and finalization errors

Relevant functions:
- `close_stream`
- `close_stdout`
- `close_stdout_set_file_name`
- `close_stdout_set_ignore_EPIPE`
- `rpl_fclose`

These modules show that shutdown is not a trivial `fclose(stdout)` event. Instead, the runtime includes explicit close/error policy state:

- target file name can be recorded for later error context,
- EPIPE handling policy can be configured,
- final output close is centralized through `close_stdout`.

Behaviorally, finalization errors are therefore handled in a specialized shutdown path rather than being left entirely to raw stdio return values.

---

## 5.5 Locale and conversion-related errors

Relevant modules:
- `setlocale_null_r_unlocked`
- `setlocale_null_r`
- `mbrtoc32`

These expose buffer-sized locale queries and multibyte-to-`char32_t` conversion, both of which are inherently error-sensitive interfaces. From the summaries alone, it is valid to state:

- runtime checks and reports operation results through integer or `size_t` returns,
- callers must branch on these results,
- failure or incomplete conversion is part of the supported runtime flow.

The current module summary is insufficient to support a more detailed behavior judgment about specific error codes or encoding-failure policies.

---

## 6. Boundary conditions and special-case handling

## 6.1 Zero-length and bounded-buffer operations

Several interfaces directly show bounded operations:

- `safe_rw(fd, buf, count)`
- `full_rw(fd, buf, count)`
- `quotearg_buffer(buffer, buffersize, arg, argsize, o)`
- `quotearg_alloc_mem(arg, argsize, size, o)`
- `quotearg_n_options(n, arg, argsize, options)`
- `quote_n_mem(n, arg, argsize)`
- `quote_mem(arg, argsize)`
- `setlocale_null_r(category, buf, bufsize)`
- `setlocale_null_r_unlocked(category, buf, bufsize)`
- `copy_file_range(..., length, flags)`
- `mbrtoc32(..., n, ...)`

This means runtime behavior explicitly handles:
- caller-provided size limits,
- finite source-length processing,
- direct memory blocks that are not limited to NUL-terminated strings.

The current module summary is insufficient to support a more detailed behavior judgment about exact zero-length handling rules, but bounded operation support is clearly part of the design.

---

## 6.2 Distinction between string and memory forms

Many formatting interfaces come in paired forms:

- `quotearg` / `quotearg_mem`
- `quotearg_style` / `quotearg_style_mem`
- `quotearg_colon` / `quotearg_colon_mem`
- `quotearg_custom` / `quotearg_custom_mem`
- `quotearg_n` / `quotearg_n_mem`
- `quotearg_n_style` / `quotearg_n_style_mem`
- `quote` / `quote_mem`
- `quote_n` / `quote_n_mem`

Dynamic significance:
- one runtime path operates on conventional terminated strings,
- another operates on explicit byte spans.

This avoids collapsing embedded-NUL or fixed-length input handling into plain string logic.

---

## 6.3 Special rendering modes in content copying

The `cat` transformation engine has explicit special-case branches for:

- tabs
- line endings
- nonprinting characters
- blank lines
- numbered vs non-numbered lines
- nonblank-only numbering

These are not incidental options; they are stateful runtime branches integrated into the main copy loop.

---

## 6.4 Custom quoting delimiters

`set_custom_quoting`, `quotearg_n_custom_mem`, `quotearg_custom`, and `quotearg_custom_mem` show a dedicated special-case quoting mode where left and right quotes are provided by the caller.

Boundary behavior here includes:
- runtime use of caller-supplied delimiter strings instead of built-in styles,
- separate processing path for custom delimiters,
- support for both numbered-slot and direct-call variants.

---

## 6.5 Colon-oriented quoting variants

`quotearg_n_style_colon`, `quotearg_colon`, and `quotearg_colon_mem` represent a dedicated special-case quoting path for colon-sensitive formatting. The dynamic implication is that the quoting subsystem contains a branch where colon treatment differs from general quoting behavior.

The current module summary is insufficient to support a more detailed behavior judgment about the exact rendering rule.

---

## 6.6 File-descriptor duplication and flag edge handling

The `fcntl` compatibility group includes:

- `dupfd`
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`
- `klibc_fcntl`

This indicates explicit handling for descriptor duplication actions and close-on-exec-related actions, rather than relying on one raw system implementation path.

Runtime special-case coverage includes:
- different `fcntl` actions,
- a variadic dispatch entry (`klibc_fcntl`),
- action-specific helper paths.

The current module summary is insufficient to support a more detailed behavior judgment about unsupported-action handling.

---

## 7. Behaviors that must remain consistent with the C version

## 7.1 Entry and dispatch semantics

Any reimplementation must preserve that:
- `main` is the sole top-level program entry in the executable path,
- startup performs setup before content-copy work,
- runtime dispatch separates informational output from file-copy behavior,
- copy status propagates back to the top-level process result.

## 7.2 Copy-engine mode separation

It must remain true that the runtime distinguishes between:
- a simpler copy path (`simple_cat`)
- a transformation-capable copy path (`cat`)

These two paths are structurally distinct in the analyzed C code and should not be collapsed into behavior that loses the distinction.

## 7.3 Transformation option effects

The transformation engine must preserve runtime control over:
- nonprinting-character rendering
- tab rendering
- line numbering
- nonblank-only numbering
- end-of-line marking
- blank-line squeezing

The exact formatting syntax cannot be restated from the summaries alone, but the presence and dynamic effect of these switches must remain consistent.

## 7.4 Stateful line-number advancement

`next_line_num` must remain part of the runtime line-number progression behavior. Any port must preserve:
- an internal line-number state,
- advancement across line processing,
- interaction with numbering-related flags.

## 7.5 Buffered output flushing behavior

The relationship between:
- output buffer accumulation,
- moving output pointer,
- flushing via `write_pending`

must remain intact. A port should not rewrite the behavior as fully unbuffered per-character emission if that would alter observable ordering or failure propagation.

## 7.6 Quoting configuration lifecycle

The quoting subsystem must preserve:
- mutable quoting options,
- style retrieval and modification,
- per-character quoting overrides,
- custom quote delimiters,
- numbered quote entry points,
- memory-length-aware quoting entry points,
- explicit cleanup via `quotearg_free`.

## 7.7 Locale and charset query behavior

The distinction between:
- unlocked locale query,
- locked locale query,
- caller-buffer locale retrieval,
- direct locale string retrieval,
- hard-locale classification,
- locale charset resolution

must remain present as dynamic behavior, even if internals are refactored.

## 7.8 Fatal helper behavior

`xalloc_die` and `xset_binary_mode_error` represent dedicated fatal paths. Their non-returning or termination-oriented role must be preserved.

## 7.9 Stream finalization policy

Shutdown behavior must preserve:
- centralized `close_stdout`,
- configurable file-name context via `close_stdout_set_file_name`,
- configurable EPIPE policy via `close_stdout_set_ignore_EPIPE`,
- explicit stream close checking.

## 7.10 Compatibility wrapper behavior as runtime boundaries

Modules such as:
- `rpl_fflush`
- `rpl_fclose`
- `copy_file_range`
- `klibc_fcntl`
- `rpl_fcntl_DUPFD*`
- `set_binary_mode`

must remain semantically visible as compatibility or normalization points. Even if an implementation target has different primitives, the same higher-level runtime decisions and branching outcomes need to remain consistent.

---

## 8. Performance-sensitive paths

## 8.1 Main copy loops

The most performance-sensitive runtime area is the content-copy path:

- `copy_cat`
- `simple_cat`
- `cat`
- `safe_rw`
- `full_rw`
- `write_pending`

This is where the program spends sustained time on large inputs. The presence of separate simple and transforming paths indicates a deliberate performance split:
- avoid transformation overhead when not needed,
- use a richer per-character state machine only when requested.

## 8.2 Buffered output rather than immediate emission

`write_pending` and explicit in/out buffers show that the implementation is designed to amortize write operations. This buffering is a core performance behavior and should be preserved.

## 8.3 Advisory and direct descriptor operations

The presence of:
- `fdadvise`
- `fadvise`
- `copy_file_range`

shows that the codebase includes performance-oriented I/O support beyond naive byte copying. Even where exact use sites are not visible from the summaries, these modules define performance-sensitive optional paths.

## 8.4 Alignment-aware allocation

- `alignalloc`
- `alignfree`
- `xalignalloc`

indicate that some runtime paths are sensitive to buffer alignment. This matters for large-buffer I/O efficiency or implementation constraints. The current module summary is insufficient to support a more detailed behavior judgment about which buffers require alignment.

## 8.5 Quoting buffer vs allocating variants

The quoting subsystem offers both:
- direct buffer-filling (`quotearg_buffer`)
- heap-allocating (`quotearg_alloc`, `quotearg_alloc_mem`)
- numbered-slot reuse (`quotearg_n_options` and related `quotearg_n*` functions)

This suggests performance-sensitive distinction between:
- caller-managed storage,
- one-shot allocation,
- reusable internal storage.

The current module summary is insufficient to support a more detailed behavior judgment about slot reuse performance strategy, but the existence of multiple APIs is itself a dynamic performance consideration.

## 8.6 Stream-state manipulation around flush

The `fflush` replacement helpers:
- `disable_seek_optimization`
- `restore_seek_optimization`
- `update_fpos_cache`
- `clear_ungetc_buffer*`

show a performance-sensitive stdio path where buffering, file-position coherence, and seek behavior are actively managed during flushing.

## 8.7 Allocation growth helpers

Within `xmalloc.c`, functions such as:
- `x2realloc`
- `x2nrealloc`
- `xpalloc`

show runtime support for growing buffers or arrays in scalable ways rather than only exact-size reallocations. The current module summary is insufficient to support a more detailed behavior judgment about growth formulas, but these are clearly performance-relevant utility paths.

---

## 9. Module-group behavior notes

## 9.1 `cat.c` behavior cluster

Observed runtime responsibilities:
- user help/usage output
- line-number state progression
- plain copy
- transformed copy
- buffered output emission
- top-level copy dispatch

This is the behavioral core of the executable.

## 9.2 `quotearg.c` behavior cluster

Observed runtime responsibilities:
- construct and mutate quoting-option state
- format text into caller buffers
- allocate quoted strings
- manage reusable quoting slots
- expose convenience wrappers by style, colon mode, character mode, custom delimiters, and numbered slots
- release retained quoting state

This is a reusable stateful formatting subsystem, not just a single helper.

## 9.3 locale/charset behavior cluster

Observed runtime responsibilities:
- query current locale as string
- perform locked and unlocked retrieval
- classify whether locale is “hard”
- resolve locale character set
- support multibyte to `char32_t` conversion

This cluster supports runtime behavior that depends on localization or multibyte handling.

## 9.4 stdio/fd compatibility cluster

Observed runtime responsibilities:
- binary mode switching
- `fcntl` compatibility handling
- flush replacement
- close replacement
- stream purge
- output-close policy management

This cluster normalizes low-level runtime behavior across environments.

## 9.5 allocation support cluster

Observed runtime responsibilities:
- aligned allocation/free
- checked/fatal allocation wrappers
- duplicate/copy allocation helpers
- zero-initialized allocation helpers
- growth allocation helpers

This cluster underpins safe and scalable internal buffer management.

---

## 10. Insufficient-evidence areas

The following details are not supportable from the current summaries and should remain explicitly unresolved:

- exact option parsing syntax in `main`
- exact sequence of startup calls inside `main`
- exact file iteration strategy over command-line operands
- precise read/write retry semantics of `safe_rw` and `full_rw`
- exact visible formatting produced by `cat` transformation flags
- exact line-number format and starting value
- exact quoting escaping rules and storage lifetime semantics
- exact locale-locking mechanism
- exact stream error code mapping and exit statuses
- exact memory growth formulas in `xmalloc` helpers
- exact conditions under which optimized kernel copy or advisory I/O are used

For each of these, the current module summary is insufficient to support a more detailed behavior judgment.