# 03_behaviors

## 1. Runtime Overview

From the available module summaries, the `cat` project runtime is organized around a conventional command-line entry flow:

1. Process enters `main` in `cat.c`.
2. Early process-level setup is performed through utility modules such as:
   - program-name setup (`set_program_name`)
   - locale-related helpers (`setlocale_null*`, `hard_locale`, `locale_charset`)
   - output-close policy (`close_stdout_set_*`, `close_stdout`)
   - version/help text emitters (`version_etc*`, `emit_bug_reporting_address`)
3. Command execution then moves into the main data-copy path in `cat.c`, centered on:
   - `copy_cat`
   - `simple_cat`
   - `cat`
   - `write_pending`
4. Runtime I/O support is provided by lower-level helpers such as:
   - `safe_rw`
   - `full_rw`
   - `copy_file_range`
   - advisory/binary-mode wrappers (`fdadvise`, `fadvise`, `set_binary_mode`, `xset_binary_mode_error`)
5. On termination, output finalization is handled by stream-close helpers such as:
   - `close_stdout`
   - `close_stream`
   - `rpl_fclose`

The current module summary is insufficient to support a more detailed behavior judgment about exact startup sequencing inside `main`, exact option parsing order, or exact file iteration order.

---

## 2. Initialization Flow and Startup Order

## 2.1 Entry into `main`

The principal startup point is:

- `main` (`cat.c:535-813`)

Because `main` has high external interaction count, it acts as the runtime coordinator. The summaries establish that `main` depends on many external modules, which is consistent with startup, option handling, and dispatch behavior.

Observed startup-related helpers include:

- `set_program_name`
  Establishes process/program naming state from `argv[0]`. This is startup state initialization because the function takes only the initial program name and returns `void`.

- `setlocale_null`, `setlocale_null_r`, and unlocked/with-lock variants
  These functions manage locale state querying/copying and indicate a startup or environment-setup role where locale-sensitive behaviors are established or inspected.

- `hard_locale`
  Returns a boolean for a locale category and therefore contributes a runtime branch condition for locale-sensitive behavior.

- `locale_charset`
  Returns the current locale character set and therefore supports runtime decisions tied to text/quoting/output representation.

- `close_stdout_set_file_name`, `close_stdout_set_ignore_EPIPE`
  These configure global or module-retained closing behavior before program termination paths are exercised.

- `set_binary_mode` / `xset_binary_mode_error`
  These indicate startup or pre-I/O configuration for file descriptor mode.

## 2.2 Help / Version / Non-copy startup branches

The presence of:

- `usage`
- `version_etc`
- `version_etc_va`
- `version_etc_ar`
- `version_etc_arn`
- `emit_bug_reporting_address`

shows that startup does not always proceed into file-copy execution. Some command-line paths branch into message emission and terminate without entering the main copy loop.

Behaviorally:

- `usage(status)` is a dedicated branch target for usage output and exit-status-based termination flow.
- `version_etc*` functions emit version/author information to a supplied `FILE *stream`.
- `emit_bug_reporting_address` emits post-version/help reporting text.

The current module summary is insufficient to support a more detailed behavior judgment about the exact option tokens that trigger these branches.

## 2.3 Memory-support initialization behavior

There is no evidence of one centralized allocator initialization phase. Instead, memory behavior appears demand-driven through wrappers:

- `xmalloc`, `xrealloc`, `xzalloc`, `xcalloc`, `xstrdup`, etc.
- `alignalloc`, `alignfree`
- `xalignalloc`
- `xalloc_die`

Behaviorally, these form runtime support services rather than an explicit startup stage.

---

## 3. Main User Operation Flows

## 3.1 Top-level operational modes

The primary user-visible runtime has two broad execution modes in `cat.c`:

1. **Simple copy mode**
   - centered on `simple_cat`
2. **Formatted/feature-rich copy mode**
   - centered on `cat`

Both are selected under control of `copy_cat` and `main`.

This distinction is directly supported by the existence of:
- `simple_cat(char *buf, idx_t bufsize)`
- `cat(char *inbuf, idx_t insize, char *outbuf, idx_t outsize, bool show_nonprinting, bool show_tabs, bool number, bool number_nonblank, bool show_ends, bool squeeze_blank)`
- `copy_cat(void)`

The parameter list of `cat` shows that the program has a behaviorally richer path when output transformations are enabled.

## 3.2 Simple copy flow

`simple_cat` has a compact signature: one buffer and one size, returning `bool`. This indicates a straightforward processing loop with a success/failure result.

Dynamic behavior that is directly supported by the summary:

- Runtime reads data into a provided buffer.
- Data is forwarded to output.
- The function reports success/failure as a boolean end-state.

This path is performance-oriented because:
- it avoids the many formatting flags present in `cat`
- it uses a single provided buffer instead of separate input/output transformation buffers

Supporting low-level runtime helpers likely used in this path are:

- `safe_rw`
- `full_rw`
- `copy_file_range`

The exact call chain cannot be stated beyond the recorded external-call counts, but these modules exist specifically for safe or full read/write style operation.

## 3.3 Feature-rich copy flow

`cat(...)` takes explicit booleans for:

- `show_nonprinting`
- `show_tabs`
- `number`
- `number_nonblank`
- `show_ends`
- `squeeze_blank`

This establishes the dynamic behavior of the non-simple path:

1. Input is consumed through an input buffer.
2. Output is accumulated through a distinct output buffer.
3. Each input segment is examined against formatting and transformation rules.
4. Output may be delayed until `write_pending` flushes the currently accumulated transformed data.
5. Function returns `bool`, indicating runtime success/failure.

The presence of both `next_line_num` and `write_pending` shows that this path maintains internal processing state across input characters or lines.

### Observed transformation sub-behaviors

- **Line numbering**
  - managed by `next_line_num`
  - controlled by `number` and `number_nonblank`
- **Blank-line suppression**
  - controlled by `squeeze_blank`
- **Line-end marking**
  - controlled by `show_ends`
- **Tab visibility**
  - controlled by `show_tabs`
- **Nonprinting rendering**
  - controlled by `show_nonprinting`

The current module summary is insufficient to support a more detailed behavior judgment about exact textual encodings used for transformed characters or exact precedence when multiple formatting flags interact.

## 3.4 Buffered output flush flow

`write_pending(char *outbuf, char **bpout)` is a small but important runtime operation:

- `outbuf` is the start of a pending-output region.
- `bpout` is the moving end pointer of buffered output.
- The function flushes accumulated transformed output.

Dynamic meaning:
- output generation and output emission are decoupled
- the transformation path can emit in chunks instead of per character
- flushing is explicitly invoked when the buffer reaches a threshold or a logical boundary

## 3.5 Copy dispatch flow

`copy_cat(void)` serves as a bridge between top-level control and actual data transfer. Since it returns `int` instead of `bool`, it likely converts copy success/failure into a process-level status form.

Behaviorally:
- `main` delegates actual copy execution to `copy_cat`
- `copy_cat` selects or orchestrates `simple_cat` vs `cat`
- `copy_cat` converts operational outcomes into a final status code

The current module summary is insufficient to support a more detailed behavior judgment about whether dispatch is based on parsed options, file type, file descriptor properties, or all of these together.

---

## 4. State Machines and State Transitions

## 4.1 Process-level execution state machine

A conservative behavior state machine supported by the summaries is:

1. **Startup**
   - enter `main`
   - initialize program name / locale / output-close behavior / binary mode as needed
2. **Administrative branch**
   - emit usage or version text
   - terminate
3. **Preparation for copy**
   - allocate buffers / configure file mode / set file name for error-close reporting
4. **Copy execution**
   - run `copy_cat`
   - internally select `simple_cat` or `cat`
5. **Finalization**
   - close/flush output through `close_stdout` and related helpers
6. **Exit**
   - return process status

This is the clearest runtime machine that can be stated from the available summaries.

## 4.2 Formatted copy state machine

The `cat(...)` function implies a per-stream or per-file processing state with at least these components:

- current input position within `inbuf`
- current output fill position within `outbuf`
- line numbering state
- blank-line suppression state
- current active transformation flags

### State transitions directly supported by interface evidence

- **Normal character processing**
  - input consumed
  - output appended
- **Line boundary encountered**
  - line-related state updated
  - may trigger `next_line_num`
  - may trigger blank-line squeeze logic
  - may append end markers if `show_ends`
- **Tab encountered**
  - branch on `show_tabs`
- **Nonprinting character encountered**
  - branch on `show_nonprinting`
- **Output buffer near/full**
  - call `write_pending`
  - reset pending-output pointer state
- **End of input**
  - flush remaining pending output
  - return success/failure

The current module summary is insufficient to support a more detailed behavior judgment about exact internal flags used to remember “previous line blank” or whether line numbering increments before or after emission.

## 4.3 Line numbering state

`next_line_num(void)` has no parameters and no return value, which indicates module-retained numbering state rather than caller-owned state.

Dynamic consequences:

- numbering is persistent across calls during a copy session
- the function advances the current line-number representation
- `cat(...)` uses that persistent state when numbering behavior is enabled

This state must remain consistent across repeated line events during one invocation flow.

## 4.4 Quoting-option state machine

The `quotearg` family exposes a separate configuration-and-render runtime:

### Configuration state
- `clone_quoting_options`
- `get_quoting_style`
- `set_quoting_style`
- `set_char_quoting`
- `set_quoting_flags`
- `set_custom_quoting`
- `quoting_options_from_style`

### Rendering state
- `quotearg_buffer`
- `quotearg_alloc`
- `quotearg_alloc_mem`
- `quotearg_n_options`
- wrapper variants (`quotearg_n`, `quotearg_style`, `quotearg_colon`, `quotearg_custom`, `quote`, etc.)

Behaviorally this forms a two-stage state machine:

1. construct or modify quoting configuration
2. apply configuration to one argument string or memory block
3. return quoted text in caller buffer, newly allocated memory, or slot-indexed storage

The existence of `quotearg_free` shows that some quoted results persist across calls and later require explicit cleanup.

### Indexed quoting state

Functions like:

- `quotearg_n`
- `quotearg_n_mem`
- `quotearg_n_options`
- `quotearg_n_style`
- `quotearg_n_style_mem`
- `quotearg_n_style_colon`
- `quotearg_n_custom`
- `quotearg_n_custom_mem`
- `quote_n`
- `quote_n_mem`

show a slot-based state behavior keyed by integer `n`. Dynamic implication:

- different `n` values address independent retained quoting result slots
- repeated calls can update or reuse per-slot result state
- `quotearg_free` resets or releases retained quoting resources

The current module summary is insufficient to support a more detailed behavior judgment about slot lifetime rules beyond explicit existence of `quotearg_free`.

## 4.5 Locale query state transitions

The locale module presents two related behaviors:

- unlocked retrieval/manipulation (`setlocale_null_unlocked`, `setlocale_null_r_unlocked`)
- locked wrapper retrieval (`setlocale_null_r_with_lock`, `setlocale_null_r`, `setlocale_null`)

This indicates a state machine with:

1. locale category requested
2. locale string retrieved into static or caller buffer
3. lock/no-lock path selected depending on wrapper
4. result returned as string or status integer

Since several internal `setlocale_null_r_with_lock` definitions are recorded in one file, the implementation contains conditionalized variants. The current module summary is insufficient to support a more detailed behavior judgment about platform-specific branch criteria.

## 4.6 Stream-state handling machine

The stream support modules expose stream-state transitions:

- `rpl_fflush`
- `fpurge`
- `close_stream`
- `rpl_fclose`
- `clear_ungetc_buffer*`
- `disable_seek_optimization`
- `restore_seek_optimization`
- `update_fpos_cache`

Dynamic behavior supported by these names/signatures:

1. stream may contain unread pushed-back input or pending output
2. flush/purge helpers normalize stream state
3. seek optimization may be temporarily disabled and later restored
4. file-position cache may be updated after flush operations
5. stream is then safely closed

This is a runtime state-preservation and cleanup machine for `FILE *` objects.

---

## 5. Error-Handling Flows

## 5.1 Copy-path failure reporting

Both `simple_cat` and `cat` return `bool`, showing that copy logic has explicit success/failure signaling. `copy_cat` returns `int`, showing conversion from operation status to process-level status.

Behaviorally:

- low-level I/O or transformation failures propagate upward
- top-level flow consolidates them into final program status in `main`

The current module summary is insufficient to support a more detailed behavior judgment about exact diagnostics emitted at each failure site.

## 5.2 Safe/full I/O error flow

Two low-level modules are clearly dedicated to robust descriptor I/O:

- `safe_rw`
- `full_rw`

Dynamic role:

- `safe_rw` provides a guarded descriptor read/write primitive
- `full_rw` builds on a lower-level routine and continues the transfer logic until a completion condition or failure condition is reached

Because only function signatures and module clustering are available, the current module summary is insufficient to support a more detailed behavior judgment about retry conditions, partial-transfer semantics, or exact error returns.

## 5.3 Output finalization failure flow

The output-close path includes:

- `close_stdout_set_file_name`
- `close_stdout_set_ignore_EPIPE`
- `close_stdout`
- `close_stream`
- `rpl_fclose`

This establishes a dedicated termination-time error-handling path.

Behaviorally:

1. runtime may register the current output file name
2. runtime may configure whether `EPIPE` should be ignored
3. on shutdown, `close_stdout` performs final output close processing
4. lower layers close specific streams and report status

This separation is important because shutdown failures are handled differently from in-loop copy failures.

## 5.4 Allocation failure flow

Allocation wrappers include:

- `xmalloc`, `xrealloc`, `xcalloc`, `xstrdup`, etc.
- `xalignalloc`
- `xalloc_die`

Dynamic behavior:

- callers request memory through x-alloc wrappers
- failure is escalated to a dedicated fatal path via `xalloc_die`

Because `xset_binary_mode_error` is explicitly `_Noreturn`, and `xalloc_die` is a dedicated fatal helper, the runtime clearly distinguishes fatal unrecoverable support failures from ordinary operation status returns.

The current module summary is insufficient to support a more detailed behavior judgment about exit codes or exact messages.

## 5.5 Binary-mode configuration failure flow

`set_binary_mode(int fd, int mode)` returns `int`, while `xset_binary_mode_error(void)` is `_Noreturn`.

Behaviorally:

- mode setting is attempted on a file descriptor
- failure can be escalated to a fatal error path
- fatal escalation does not return to caller

## 5.6 Quoting-related failure flow

Quoting has both buffer-based and allocation-based interfaces:

- `quotearg_buffer` writes into caller-provided buffer
- `quotearg_alloc` / `quotearg_alloc_mem` allocate result storage
- indexed wrappers retain internal storage until `quotearg_free`

Runtime failure handling therefore splits:

- caller-buffer path avoids internal allocation for the immediate result
- allocation paths depend on memory wrappers and therefore can enter the fatal allocation path

The current module summary is insufficient to support a more detailed behavior judgment about whether all quoting allocations use xalloc wrappers in every path.

---

## 6. Boundary Conditions and Special-Case Handling

## 6.1 Empty or minimal input handling

The existence of explicit buffer sizes in:
- `simple_cat(char *buf, idx_t bufsize)`
- `cat(char *inbuf, idx_t insize, char *outbuf, idx_t outsize, ...)`
- `quotearg_buffer(char *buffer, size_t buffersize, ...)`

shows that runtime behavior is size-aware and must handle small or exact-fit buffers.

Dynamic boundary conditions that are definitely present:

- zero or small remaining buffer capacity
- flush-before-overflow behavior via `write_pending`
- end-of-input with partially filled output buffer

The current module summary is insufficient to support a more detailed behavior judgment about whether zero-sized buffers are accepted in all cases.

## 6.2 Blank-line special cases

Because `cat` accepts both:
- `number_nonblank`
- `squeeze_blank`

blank lines are a distinguished runtime case.

Observed behavior classes:

- blank lines can affect numbering decisions
- blank lines can affect emission suppression decisions
- transitions between blank and nonblank lines affect internal state

## 6.3 Line-end special cases

The `show_ends` flag makes line termination an explicit branch condition, not just passive input. Therefore line ends are treated as semantic events in the formatted copy path.

## 6.4 Nonprinting and tab special cases

`show_nonprinting` and `show_tabs` demonstrate that tabs and nonprinting bytes/characters are not always copied transparently. They trigger alternate output transformations.

The current module summary is insufficient to support a more detailed behavior judgment about exact formatting sequences.

## 6.5 Locale special cases

Locale handling modules show explicit branch-sensitive behavior:

- locked vs unlocked locale access
- hard locale vs non-hard locale check
- locale charset lookup

Therefore text-related runtime behavior may differ across locale states. This is especially relevant for:
- quoting text generation
- proper-name display
- character conversion via `mbrtoc32`

## 6.6 Character conversion special cases

The presence of two `mbrtoc32` definitions in one module indicates conditionalized implementations or replacement logic.

Dynamic special cases necessarily include:
- converting multibyte input to `char32_t`
- maintaining/using `mbstate_t`
- handling stateful conversion across calls

The current module summary is insufficient to support a more detailed behavior judgment about invalid sequence handling, incomplete sequence handling, or exact fallback behavior.

## 6.7 Stream buffering special cases

Modules `fflush.c`, `fpurge.c`, and `clear_ungetc_buffer*` establish explicit handling for streams with special internal states:

- pushed-back input (`ungetc` state)
- pending output
- seek optimization state
- cached file position state

These are edge cases in stream maintenance and closing logic.

## 6.8 Descriptor duplication and fcntl special cases

The `fcntl` replacement area includes:

- `dupfd`
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`
- `klibc_fcntl`

This shows explicit handling for descriptor duplication variants and platform/library-specific `fcntl` behavior.

The current module summary is insufficient to support a more detailed behavior judgment about exact platform divergence rules.

---

## 7. Behaviors That Must Remain Consistent with the C Version

## 7.1 Top-level control separation

The C version clearly separates:

- process entry and option dispatch (`main`)
- copy orchestration (`copy_cat`)
- fast path (`simple_cat`)
- feature path (`cat`)
- shutdown (`close_stdout` and stream close helpers)

Any reimplementation must preserve this runtime separation of concerns, especially the fact that not all invocations enter the same data path.

## 7.2 Boolean success/failure propagation in copy routines

`simple_cat` and `cat` both return `bool`, while `copy_cat` and `main` use integer statuses. This layered outcome propagation must be preserved:

- internal copy operations resolve to success/failure
- higher layers translate that into process-visible termination status

## 7.3 Stateful formatted output behavior

The C version contains internal mutable runtime state for formatted copying:

- persistent line numbering via `next_line_num`
- pending-output pointer movement via `write_pending`
- line/blank-sensitive transformations in `cat`

This means a reimplementation must preserve stateful, event-driven processing rather than treating formatting as stateless per-byte substitution.

## 7.4 Slot-based quoting behavior

The `quotearg_n*` and `quote_n*` families show that the quoting subsystem is not only a pure one-shot formatter. It has slot-indexed retained behavior plus explicit cleanup via `quotearg_free`.

That retained runtime behavior must remain consistent:
- slot `n` remains meaningful across calls
- wrappers around style/custom/colon/default quoting continue routing through this indexed model
- cleanup remains explicit

## 7.5 Locale-aware branching

Locale support is not monolithic; it is divided into:
- locale retrieval
- hard-locale predicate
- charset lookup
- multibyte conversion

This means locale-sensitive behavior is distributed across runtime branches and should remain so in any behavior-preserving port.

## 7.6 Fatal vs nonfatal support failures

The C version distinguishes:
- recoverable operational failure paths returning `bool`/`int`
- unrecoverable support failures using dedicated fatal helpers (`xalloc_die`, `xset_binary_mode_error`)

This distinction must remain consistent.

## 7.7 Stream cleanup ordering

Dedicated cleanup helpers exist for:
- flushing
- purging
- close-with-status
- stdout-specific finalization

Therefore the C behavior includes explicit stream-state normalization and finalization steps, not just raw close calls. This consistency must be preserved.

---

## 8. Performance-Sensitive Paths

## 8.1 `simple_cat` as the primary fast path

`simple_cat` is the clearest performance-oriented path:

- single buffer argument
- no transformation flags
- boolean success/failure result
- lower control overhead than the feature-rich `cat`

This function should be treated as the high-throughput copy path.

## 8.2 Buffered transformation in `cat`

Even the feature path is performance-aware because it uses:

- separate input and output buffers
- explicit pending-output management through `write_pending`

This avoids immediate per-character writes and preserves chunked output behavior.

## 8.3 Low-level full-transfer helpers

`full_rw` and `safe_rw` are performance-sensitive because they sit directly on descriptor transfer paths. Their behavior affects:
- syscall frequency
- partial-transfer handling
- main copy throughput

The current module summary is insufficient to support a more detailed behavior judgment about exact loop strategies.

## 8.4 Kernel-assisted copy support

`copy_file_range` is an explicitly separate module, indicating a performance-sensitive specialized transfer path for file-descriptor copying.

The current module summary is insufficient to support a more detailed behavior judgment about where `main` or `copy_cat` selects this path.

## 8.5 Advisory I/O hints

`fdadvise` and `fadvise` exist only to influence file-access behavior. Their presence marks an optimization path rather than functional core logic.

Behaviorally:
- they do not define the copy result
- they tune surrounding I/O behavior when invoked

## 8.6 Binary-mode setup

`set_binary_mode` affects descriptor mode before data transfer. This is performance-relevant in the sense that it configures the data path once, outside the per-byte/per-buffer loop.

## 8.7 Allocation helpers for reusable storage

The quoting subsystem and copy buffers depend on allocator wrappers, and the presence of:
- `x2realloc`
- `x2nrealloc`
- `xpalloc`

shows growth-oriented allocation helpers designed for runtime efficiency during dynamic buffer expansion.

The current module summary is insufficient to support a more detailed behavior judgment about which exact high-level paths use each growth helper.

---

## 9. Module-Specific Dynamic Behavior Notes

## 9.1 `safe_rw`

Observed in both `include/safe-read.c` and `safe-read.c`, `safe_rw` is a low-level descriptor transfer primitive. Its behavior must be preserved as a reusable runtime building block for safe data movement. The current module summary is insufficient to support a more detailed behavior judgment.

## 9.2 `full_rw`

`full_rw` wraps a lower-level transfer primitive and forms a complete-transfer loop behavior. This is a critical runtime path for writing or reading all requested bytes where higher layers expect a fuller completion policy than one raw syscall provides.

## 9.3 `close_stdout`

This module retains shutdown policy state:
- output file name
- whether `EPIPE` is ignored

That means shutdown behavior is configurable during runtime before final close occurs.

## 9.4 `version_etc*`

These functions are pure output-emission branches in command-processing flow. They do not participate in copy-state changes, but they are terminal user-operation paths.

## 9.5 `alignalloc` / `xalignalloc`

These modules support aligned allocation as a runtime service. `xalignalloc` adds fatal-on-failure semantics on top of `alignalloc`.

## 9.6 `rpl_fflush` and `fpurge`

These functions manage stream internals in nontrivial states and therefore are important for correctness around buffered I/O boundaries, especially before close or when switching stream usage phases.

## 9.7 `proper_name_lite`

This helper depends on external modules and likely selects between ASCII and UTF-8 names based on runtime text environment. The current module summary is insufficient to support a more detailed behavior judgment beyond that it returns one of two naming forms as a runtime display choice.

---

## 10. Consolidated Behavioral Picture

The `cat` project runtime, as supported by the module summaries, is a layered execution system:

- `main` coordinates startup, administrative branches, and copy dispatch.
- The core operational path splits into:
  - a fast raw copy path (`simple_cat`)
  - a stateful formatted path (`cat`)
- Formatted copying maintains internal state for:
  - line numbering
  - blank-line handling
  - output-buffer flushing
  - per-character transformation decisions
- Support subsystems provide:
  - robust I/O transfer (`safe_rw`, `full_rw`, `copy_file_range`)
  - stream-state management (`rpl_fflush`, `fpurge`, `close_stream`, `rpl_fclose`)
  - environment sensitivity (locale and charset helpers)
  - fatal and nonfatal error separation (`xalloc_die`, `xset_binary_mode_error`)
  - retained quoting state with explicit cleanup (`quotearg_n*`, `quotearg_free`)
- Termination includes explicit stdout finalization behavior rather than relying only on implicit process teardown.

Where exact branch conditions, error text, or detailed low-level loop logic are not shown in the summaries, the current module summary is insufficient to support a more detailed behavior judgment.