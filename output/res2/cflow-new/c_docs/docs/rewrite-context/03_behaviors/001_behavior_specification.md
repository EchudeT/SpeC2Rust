# 03_behaviors

## 1. Overview of runtime behavior scope

This document describes observable runtime behavior based only on the provided module analysis summaries for the C project `cflow-new`. It focuses on execution flow, state changes, special handling, and behavior preservation points.

Where function bodies were not provided and summaries expose only names, signatures, file groupings, and coarse dependency counts, the description remains conservative. In such cases, **the current module summary is insufficient to support a more detailed behavior judgment**.

The project behavior divides into several major runtime areas:

- `src/main.c` startup, option intake, initialization, parsing, and final output dispatch.
- lexical and syntactic analysis in `src/c.c` and `src/parser.c`.
- symbol and call graph state maintenance in `src/symbol.c`, `src/linked-list.c`, and `src/depmap.c`.
- output rendering in `src/output.c`, `src/gnu.c`, `src/posix.c`, and `src/dot.c`.
- command-line option engine and formatted help output in `src/parseopt/*`.
- shell-like tokenization and expansion support in `src/wordsplit/wordsplit.c`.
- bundled `gnu/*` support code for memory, error reporting, portability wrappers, hash tables, formatting, and word-wrapping.
- `doc/*` example or auxiliary programs with their own `main` entry points.
- `test/*` standalone test behavior.

---

## 2. Initialization flow and startup order

## 2.1 Top-level application startup (`src/main.c`)

The central runtime entry is:

- `main(int argc, char **argv)` in `src/main.c`
- helper initialization functions include `init()`, `parse_rc()`, `parseopt_from_env()`, `parseopt_from_rc()`, `init_hook()`, and option-setting callbacks.

Observed startup-related behavior:

1. **Process entry**
   - `main` receives raw argument count and vector.
   - `init` exists as a separate initialization routine, indicating a distinct startup phase before normal work proceeds.

2. **Base initialization**
   - `init()` is explicitly present and should be treated as the core initializer for process-global runtime state.
   - `set_program_name` exists in `gnu/progname.c`, and `getprogname` exists in `gnu/getprogname.c`; these support startup identity state.
   - `output_init()` in `src/output.c` suggests output-driver registration or output subsystem preparation happens early.

3. **Configuration intake**
   - Runtime configuration can come from:
     - environment (`parseopt_from_env`)
     - rc files (`parse_rc`, `parseopt_from_rc`, `optfile_lookup`, `fromfile`, `tildexpand`)
     - command-line parsing (`parseopt_*` modules)
   - `optfile_register` indicates duplicate tracking or registration of option files by device/inode identity, which implies startup includes stateful control over already-processed config files.

4. **Option parser initialization**
   - `parseopt_init0`, `parseopt_init`, `parseopt_parse`, `parseopt_next`, `parseopt_getopt`, and `parseopt_free` indicate a staged option-parser lifecycle:
     - parser object initialization
     - option definition preparation
     - iterative consumption of options
     - final cleanup

5. **Help/version dispatch path**
   - `set_help`, `set_usage`, `set_version`, `help_hook`, and `version_hook` indicate startup may terminate into documentation output instead of normal parsing/output production.

6. **Output driver selection**
   - `register_output`, `select_output_driver`, and option setter `optset_output_driver` show that startup resolves one active output backend before final reporting.

7. **Lexer/parser startup**
   - Lexical subsystem:
     - `init_tokens()`
     - `init_lex(int debug_level)`
     - scanner buffer/global setup functions in `src/c.c`
   - Parser subsystem:
     - `init_parse()`
   - These indicate a distinct frontend bootstrapping phase before source analysis begins.

8. **Starter/target selection state**
   - Symbol filtering behavior is configured during startup using:
     - `install_starter`
     - `set_default_starter`
     - `clear_starters`
     - `install_target`
     - `eliminate_non_targets`
   - This means the eventual traversal/output set is shaped before final rendering.

## 2.2 Lexical subsystem initialization (`src/c.c`)

The generated scanner has explicit lifecycle support:

- `yy_init_globals`
- `yy_create_buffer`
- `yy_init_buffer`
- `yyensure_buffer_stack`
- `yyrestart`
- `yy_switch_to_buffer`
- `yy_load_buffer_state`
- `yy_flush_buffer`
- `yypush_buffer_state`
- `yypop_buffer_state`
- `yylex_destroy`

Dynamic startup behavior visible from naming and grouping:

1. Global scanner state is initialized.
2. Input buffer objects are created and initialized.
3. Active buffer state is loaded before tokenization.
4. Preprocessor-facing setup also exists:
   - `set_preprocessor`
   - `pp_option`
   - `pp_open`
   - `pp_finalize`

This indicates source scanning startup includes both scanner-state activation and preprocessor command configuration.

## 2.3 Parser initialization (`src/parser.c`)

The parser side has an explicit setup sequence:

- `init_parse`
- token stack maintenance functions
- save-stack support
- mark/restore mechanisms

Observed startup behavior:

1. token-stack state is initialized.
2. parser can begin consuming lexical tokens using `nexttoken`.
3. save/restore support is prepared before complex declaration parsing.

## 2.4 Output subsystem initialization (`src/output.c`)

The output system has a clear registration/selection lifecycle:

- `register_output`
- `select_output_driver`
- `output_init`

Observed behavior:

1. output drivers are registered under names.
2. one driver is selected at runtime.
3. later output commands are funneled through the selected handler.

Registered handlers are present in:

- `gnu_output_handler`
- `posix_output_handler`
- `dot_output_handler`

This shows output formatting is late-bound but initialized early.

## 2.5 Parseopt help/wordwrap startup (`src/parseopt/*`)

Help and usage output has its own initialization path:

- `wordwrap_open`
- `wordwrap_fdopen`
- margin setup functions
- help-context preparation via `init_usage_vars`, `sort_options`, `print_option_group`

This indicates that when help/version/usage output is requested, a formatted text engine is initialized with explicit line-state and margin state.

## 2.6 Standalone `doc/*` startup behavior

The `doc` directory contains separate entry points:

- `doc/d.c: main`
- `doc/foo.c: main`
- `doc/wc.c: main`
- `doc/whoami.c: main`

These are independent executable flows, not part of the main `src/main.c` startup path.

Observed startup roles:

- `doc/d.c` starts directory processing behavior.
- `doc/wc.c` starts counting/report behavior.
- `doc/whoami.c` starts identity lookup behavior.
- `doc/foo.c` starts a minimal flow around `f()`.

The current module summary is insufficient to support a more detailed behavior judgment about exact startup ordering inside each example beyond those file-local entry points.

---

## 3. Main user operation flows

## 3.1 Primary cflow execution flow

The main program’s operational path is organized around these phases:

1. initialize runtime state
2. collect configuration from environment and rc files
3. parse command-line options
4. configure preprocessing, symbol filters, and output mode
5. open and scan source input
6. parse declarations, functions, references, and calls
7. populate symbol/call structures
8. optionally eliminate non-targets or move/delete scoped symbols
9. produce output in chosen format

This flow is supported by the following runtime subsystems.

## 3.2 Option and configuration flow

### 3.2.1 Command-line option consumption

`src/parseopt/parseopt.c` exposes a real iterative parser:

- `parseopt_next_internal`
- `parseopt_next`
- `parseopt_parse`
- `parseopt_getopt`
- lookup helpers for short and long options
- `permute`
- `parseopt_skip`
- `parseopt_lookahead`

Behaviorally, user arguments are processed in a loop-like progression:

1. current argv element is inspected.
2. parser decides whether it is:
   - a short option
   - a long option
   - an argument requiring permutation
   - a non-option
3. matching option definition is found.
4. option-setting callback updates process state.
5. parser advances to the next token.

`permute` indicates the parser supports stateful rearrangement of option/non-option order during processing.

### 3.2.2 Option-setting callbacks

Runtime state is updated through many specific setters in `src/main.c` and `src/parseopt/optset.c`, including:

- include class selection
- output driver selection
- xref mode
- symbol selection
- preprocessor options
- preprocess enablement
- level indentation
- main symbol selection and clearing
- install target
- path prepending
- profile loading

Operationally, each recognized option routes into a callback that mutates configuration state before scanning begins.

### 3.2.3 Environment and rc-file flow

The application can absorb configuration from non-argv sources:

- `parseopt_from_env`
- `parse_rc`
- `parseopt_from_rc`
- `optfile_lookup`
- `fromfile`
- `optset_profile`

This yields the following behavior:

1. environment-based options are parsed.
2. rc file locations are resolved.
3. rc file content is fed back into the same option framework.
4. profile options can recursively influence later behavior.

`optfile_lookup` and `optfile_register` indicate the runtime tracks seen config files, which is a stateful protection/registry behavior during configuration loading.

## 3.3 Source scanning and token flow

### 3.3.1 Input opening and lexical feed

The lexer-side operational chain includes:

- `source(char *name)`
- `pp_open`
- `yyrestart`
- `get_token`
- `update_loc`

This supports the following user-visible behavior:

1. an input source is chosen.
2. preprocessor-backed stream is opened if configured.
3. scanner buffer is reset to that stream.
4. repeated token acquisition drives parsing.
5. source location state is updated as scanning progresses.

### 3.3.2 Token stack operations

The parser uses explicit token-stack mechanics:

- `tokpush`
- `tokins`
- `tokdel`
- `mark`
- `restore`
- `nexttoken`
- `putback`
- `save_token`
- `save_stack`
- `undo_save_stack`
- `finish_save_stack`
- `cleanup_stack`
- `clearstack`

Behaviorally, this means parsing is not a single-pass consume-only flow. It includes:

- speculative reads
- temporary token preservation
- rollback/restore
- delayed assembly of declaration text or names
- stack cleanup between parse contexts

This is central to declaration parsing and function/variable distinction.

## 3.4 Declaration and function analysis flow

The parser modules show a detailed C-front-end execution path.

### 3.4.1 Top-level parse loop

- `yyparse`
- `declare`
- `parse_dcl`
- `dcl`
- `dirdcl`
- `parmdcl`
- `maybe_parm_list`

This supports a repeated parse cycle:

1. read token stream
2. recognize declaration structure
3. descend into declarator parsing
4. distinguish direct declarators, parameter lists, and function forms
5. build or update symbol state

### 3.4.2 Function handling path

Functions specifically involve:

- `is_function`
- `parse_function_declaration`
- `func_body`
- `get_knr_args`
- `parse_knr_dcl`
- `call`
- `add_reference`

Operationally:

1. parser determines whether current declaration is a function.
2. function declaration details are parsed.
3. body parsing begins.
4. references and calls found in the body are attached to symbol graph state.
5. K&R-style argument declarations receive dedicated handling.

### 3.4.3 Variable/type handling path

Non-function declarations use:

- `parse_variable_declaration`
- `initializer_list`
- `parse_typedef`
- `declare_type`
- `skip_declaration`
- `skip_struct`
- `fake_struct`

Behaviorally:

1. declarations identified as variable/type-related follow a different path from functions.
2. initializer constructs can trigger nested parsing.
3. struct and typedef syntax have special branches.
4. some declarations may be skipped rather than fully modeled.

### 3.4.4 Expression and balanced-syntax handling

- `expression`
- `skip_balanced`
- `find_closing_paren`
- balance-state push/pop/free helpers

This indicates nested constructs are managed with an explicit balance stack:

1. opening delimiter increases nesting state.
2. nested delimiter states are pushed.
3. matching close restores prior state.
4. parser can skip entire balanced regions when needed.

## 3.5 Symbol and graph construction flow

### 3.5.1 Symbol lookup/install path

`src/symbol.c` provides the core symbol-lifecycle behavior:

- `lookup`
- `install`
- `install_ident`
- `install_starter`
- `install_target`
- `get_symbol`
- `add_reference`
- `call`
- `reference`

Operational flow:

1. a name is encountered during parse.
2. runtime looks up an existing symbol record.
3. if absent, a symbol is installed.
4. storage and identity metadata may be updated.
5. references and calls connect symbols into graph/list relationships.

### 3.5.2 Symbol scope cleanup flow

The parser and symbol manager support post-scope cleanup:

- `delete_statics`
- `delete_autos`
- `delete_parms`
- `move_parms`
- `delete_level_autos`
- `delete_level_statics`
- `delete_parms_itr`

This shows that symbol tables are stateful across scope levels and then trimmed or migrated when leaving scopes.

### 3.5.3 Starter/target filtering flow

- `first_starter`
- `next_starter`
- `mark_callers`
- `eliminate_non_targets`
- `include_symbol`
- `globals_only`

Behaviorally:

1. designated starter symbols form traversal roots or inclusion anchors.
2. caller chains can be marked from selected points.
3. non-target symbols can be removed from the eventual result set.
4. output filtering is a later graph-selection pass, not only a parse-time action.

## 3.6 Output generation flow

### 3.6.1 Generic output driver dispatch

The output engine uses command-style driver callbacks:

- `begin`
- `separator`
- `print_text`
- `end`
- `output`
- `xref_output`

Dynamic flow:

1. output session begins.
2. symbols or edges are emitted through selected handler.
3. separators/newlines are injected by output subsystem.
4. output session ends.

### 3.6.2 Tree output flow

The main textual call-tree renderer includes:

- `tree_output`
- `direct_tree`
- `inverted_tree`
- `print_function`
- `print_type`
- `print_refs`
- `print_level`
- `set_level_mark`
- `set_active`
- `clear_active`

This indicates a recursive traversal behavior:

1. select tree direction.
2. recursively visit symbol nodes.
3. maintain indentation/level markers.
4. track active symbols to control traversal state.
5. emit references and type/function displays according to mode.

### 3.6.3 Alternate output backends

Backends are explicit:

- `gnu_output_handler`
- `posix_output_handler`
- `dot_output_handler`

Observed runtime pattern:

1. output core dispatches command enum plus current data.
2. backend interprets begin/item/separator/end-style commands.
3. DOT backend additionally declares nodes and prints symbol edges.
4. GNU/POSIX backends format symbol lines differently.

The current module summary is insufficient to support a more detailed behavior judgment about exact command values or textual layouts.

## 3.7 Standalone utility flows in `doc/*`

### 3.7.1 Directory traversal utility (`doc/d.c`)

Observed runtime path:

- `main`
- `isdir`
- `ignorent`
- `printdir`

Behaviorally:

1. entry point receives argv.
2. names are checked for directory status.
3. some entries are ignored by `ignorent`.
4. directory content is printed recursively or hierarchically using `printdir(level, name)`.

The `level` parameter indicates depth-sensitive traversal state.

### 3.7.2 Word-count utility (`doc/wc.c`)

Observed path:

- `main`
- `counter`
- `getword`
- `isword`
- `report`
- error-report helpers `errf`, `perrf`, `error_print`

Behaviorally:

1. entry point selects file or stream targets.
2. `counter(file)` performs counting.
3. token boundaries are determined through `getword` and `isword`.
4. counts are reported by `report`.
5. formatted and perror-style errors route through dedicated helpers.

### 3.7.3 Identity utility (`doc/whoami.c`)

Observed path:

- `main`
- `who_am_i`

Behaviorally, the program enters through `main` and invokes identity resolution logic through `who_am_i`. The current module summary is insufficient to support a more detailed behavior judgment.

### 3.7.4 Ackermann-style sample (`doc/ack.c`)

Observed path:

- `ack(u_long a, u_long b)`

The module has one internal self-call, which indicates recursive execution. The dynamic behavior is recursive evaluation over two unsigned-long arguments.

### 3.7.5 Minimal sample (`doc/foo.c`)

Observed path:

- `main`
- `f`

This is a very small standalone flow. The current module summary is insufficient to support a more detailed behavior judgment.

## 3.8 Standalone test flows (`test/*`)

The `test` directory contains independent execution flows.

### `test/multi.c`
- `run` calls `helper` and `twice`, then prints a result.

### `test/recursion.c`
- `main` exercises `fib` and `fact`.
- both are recursive numerical flows.

### `test/simple.c`
- `compute` uses `add`, `mul`, and there is an `orphan` function not implied to be in the main path.
- `main` drives the simple compute path.

These are self-contained test/runtime samples, not part of the main cflow application startup.

---

## 4. State machines and state transitions

## 4.1 Option parser state machine

The parseopt subsystem is the clearest explicit state machine in the project.

State-bearing functions and transitions:

- initialization: `parseopt_init0` -> `parseopt_init`
- scan/consume: `parseopt_next_internal` / `parseopt_next`
- observation without consumption: `parseopt_lookahead`
- forced advancement: `parseopt_skip`
- completion: `parseopt_free`

Transition behavior:

1. **Uninitialized parser state**
   - no option definitions prepared yet.

2. **Prepared parser state**
   - option groups collected and normalized.
   - help/version handlers may be inserted.
   - short-name indexes may be prepared.

3. **Active scan state**
   - current argv cursor moves item by item.
   - each token causes a branch:
     - short option matching
     - long option matching
     - negated long-option handling via `negmatch`
     - positional handling
     - permutation handling

4. **Terminal/help state**
   - if help/usage/version options are hit, runtime diverts to printing flows.

5. **Freed parser state**
   - parser-local resources are released.

## 4.2 Scanner buffer state machine

The lexer in `src/c.c` exposes a classic buffered scanner state model.

Important transitions:

- buffer creation
- buffer initialization
- switch active buffer
- load buffer state
- get next buffer
- flush buffer
- push/pop nested buffers
- destroy scanner

Behavioral states:

1. **No active buffer / globals init state**
2. **Active buffer loaded**
3. **Scanning state**
4. **Need refill / next-buffer state**
5. **Nested include or pushed-buffer state**
6. **Flushed or deleted buffer state**
7. **Destroyed scanner state**

`yy_get_previous_state`, `yy_try_NUL_trans`, and `yy_get_next_buffer` indicate internal DFA transitions are maintained across refill boundaries.

## 4.3 Parser token/save-stack state machine

The parser has explicit reversible token processing.

States include:

1. **Normal consume mode**
   - tokens arrive from `nexttoken`.

2. **Marked state**
   - `mark` stores a restorable position.

3. **Saved-token accumulation**
   - `save_token`, `save_stack`, and `finish_save_stack` collect deferred text/state.

4. **Rollback state**
   - `restore` or `undo_save_stack` returns to a prior parse point.

5. **Committed state**
   - `finish_save_stack` finalizes accumulated token text.

6. **Cleanup state**
   - `cleanup_stack` or `clearstack` discards temporary parse state.

This reversible parser-state machine is central to distinguishing declaration forms in C syntax.

## 4.4 Parenthesis/balance tracking state machine

`src/parser.c` has an explicit delimiter balance stack:

- `push_balance_state`
- `pop_balance_state`
- `free_balance_stack`
- `find_closing_paren`

Behavioral states:

1. no outstanding open delimiters
2. open delimiter encountered -> push state
3. nested delimiter encountered -> deeper push
4. closing delimiter -> pop
5. mismatch or termination -> free/reset path

This state machine governs skipping and parsing nested declarators and expressions.

## 4.5 Symbol lifetime/state transitions

A symbol’s runtime behavior includes multiple stages:

1. **Absent**
   - no symbol record yet.

2. **Installed**
   - created by `install`, `install_ident`, `install_starter`, or `install_target`.

3. **Storage-adjusted**
   - `ident_change_storage` modifies storage classification.

4. **Linked into lists / hash tables**
   - via symbol table and linked-list helpers.

5. **Referenced/called**
   - `reference`, `add_reference`, `call` add graph relations.

6. **Active during traversal**
   - `set_active` / `clear_active` reflect output-traversal state.

7. **Scope-deleted**
   - `delete_autos`, `delete_statics`, `delete_parms`.

8. **Unlinked/deleted**
   - `unlink_symbol`, `delete_symbol`, `symbol_unlink_from_list`.

9. **Selected or eliminated for reporting**
   - `mark_callers`, `eliminate_non_targets`.

## 4.6 Output rendering state machine

The output subsystem behaves like a command-driven renderer.

States:

1. **Driver registry populated**
2. **One driver selected**
3. **Begin output session**
4. **Emit symbol/ref/tree elements**
5. **Emit separators/newlines**
6. **End output session**

Backend handlers react to commands rather than directly owning the whole traversal. This separation is a runtime dispatch state machine.

## 4.7 Word-wrap formatter state machine

`src/parseopt/wordwrap.c` maintains explicit line and margin state.

States visible from functions:

1. formatter open
2. line initialized
3. text written into buffer
4. rescan/find-last-whitespace
5. flush line
6. margin adjustment
7. close/flush
8. error state observable via `wordwrap_error`

This is a streaming formatter state machine with left/right margin and line-buffer transitions.

## 4.8 Wordsplit processing pipeline/state machine

The `wordsplit` subsystem is highly stateful.

Major transitions:

1. **Initialization**
   - `wordsplit_init0`
   - `wordsplit_init`

2. **Segmentation/node creation**
   - `wordsplit_add_segm`
   - `wsnode_new`
   - `wsnode_append`
   - `wsnode_insert`

3. **Expansion phases**
   - variable expansion
   - command expansion
   - tilde expansion
   - path expansion

4. **Normalization phases**
   - quote removal
   - coalescing
   - null elimination
   - whitespace trimming

5. **Finalization**
   - `wordsplit_finish`
   - `wordsplit_get_words`

6. **Cleanup/error clear**
   - `wordsplit_free_*`
   - `wordsplit_clearerr`
   - `wordsplit_free`

This is a clear multi-stage transformation pipeline from raw command text to final word vector.

---

## 5. Error-handling flows

## 5.1 Central application-side error reporting

The main program and parseopt subsystem have multiple dedicated error emitters:

- `po_env_error`
- `fromfile_error`
- `parseopt_error`
- `file_error`
- `xalloc_die`

Behavioral flow:

1. an error is detected in a specific subsystem.
2. subsystem-specific formatter emits contextual diagnostics.
3. control either returns to caller or terminates, depending on the emitter’s design.

The current module summary is insufficient to support a more detailed behavior judgment about exact termination conditions for each path.

## 5.2 GNU generic error-reporting flow

`gnu/error.c` provides:

- `error`
- `error_at_line`
- `print_errno_message`
- `flush_stdout`
- `is_open`

Observed behavior:

1. diagnostics are formatted centrally.
2. stdout may be flushed before error emission.
3. errno-based text may be appended.
4. line/file-aware reporting is available.
5. file descriptor openness may be checked before output.

This is a reusable runtime error pipeline for user-facing diagnostics.

## 5.3 Word-count utility error flow (`doc/wc.c`)

`doc/wc.c` uses a specific error funnel:

- `errf`
- `perrf`
- `error_print`

Behavior:

1. formatting helper collects variable arguments.
2. `perr`-style path includes system-error text.
3. diagnostics are centralized instead of open-coded at call sites.

## 5.4 Memory-allocation failure flow

The project contains multiple allocation-related wrappers:

- `xmalloc`, `xrealloc`, `xcalloc`, `xpalloc`, etc.
- `xalloc_die`
- `_wsplt_alloc_die`
- `_wsplt_nomem`
- `rpl_malloc`, `rpl_calloc`, `rpl_realloc`

Behaviorally:

1. low-level allocation wrappers normalize allocation calls.
2. higher-level wrappers can route failure into a dedicated fatal path (`xalloc_die`) or module-specific error state (`_wsplt_nomem`).
3. wordsplit preserves error context via `_wsplt_store_errctx` and `_wsplt_setctxerr`.

Exact failure return conventions must not be over-specified from the summary alone.

## 5.5 Parser and lexer recovery/error flow

- `file_error` reports parse/file-specific issues.
- scanner has `yy_fatal_error`.
- parser has many skip helpers (`skip_to`, `skip_balanced`, `skip_declaration`, `skip_struct`) which represent structured recovery or controlled bypass behavior.

Observed error-handling pattern:

1. malformed or unsupported syntax can trigger a skip path instead of immediate collapse.
2. nested structures can be skipped using balance-aware logic.
3. fatal scanner-level failures use a dedicated non-returning path.

## 5.6 Wordsplit error-state flow

`wordsplit` has a rich internal error model:

- `_wsplt_seterr`
- `_wsplt_setctxerr`
- `_wsplt_store_errctx`
- `_wsplt_seterr_sub`
- `wordsplit_clearerr`
- `wordsplit_strerror`
- `wordsplit_perror`

Behavior:

1. an internal failure sets an error code in the active `wordsplit` object.
2. context text may be saved with the error.
3. nested/subsplit errors can be propagated upward.
4. user-facing translation or printing is available.
5. error state can later be cleared for reuse.

## 5.7 Portability-wrapper error flows in `gnu/*`

Several wrappers exist around system calls and CRT functions:

- `rpl_close`, `close_nothrow`
- `rpl_dup2`, `dup2_nothrow`, `ms_windows_dup2`, `klibc_dup2dirfd`
- `open`, `orig_open`
- `rpl_fstat`, `orig_fstat`
- `rpl_stat`, `orig_stat`
- `klibc_fcntl`, `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`
- `gl_msvc_invalid_parameter_handler`, `_gl_nothrow_get_osfhandle`

The consistent dynamic pattern is:

1. wrapper receives request.
2. wrapper applies platform-specific or safety-specific preprocessing.
3. underlying primitive is invoked through an original/nothrow/helper path.
4. wrapper normalizes the result for the rest of the project.

The current module summary is insufficient to support a more detailed behavior judgment about exact errno manipulation or platform branch precedence.

---

## 6. Boundary conditions and special-case handling

## 6.1 C syntax ambiguity handling

The parser includes several dedicated ambiguity-management helpers:

- `is_function`
- `fake_struct`
- `maybe_parm_list`
- `get_knr_args`
- `parse_knr_dcl`
- `skip_balanced`
- `find_closing_paren`

This shows deliberate handling of boundary cases such as:

- declaration vs function ambiguity
- K&R-style declarations
- nested balanced tokens
- struct-related special syntax
- initializer nesting

These paths must remain behaviorally distinct from ordinary declaration parsing.

## 6.2 Scope and storage boundary handling

Symbol management explicitly distinguishes storage/scope classes:

- `ident_change_storage`
- `delete_autos`
- `delete_statics`
- `delete_parms`
- `move_parms`

Behavioral edge cases include:

- leaving a lexical level
- promoting or moving parameter symbols
- clearing only certain storage classes
- preserving others across scope changes

## 6.3 Output traversal boundary cases

Tree output helpers include:

- `is_printable`
- `is_last`
- `set_active`
- `clear_active`
- `direct_tree`
- `inverted_tree`

This indicates handling for:

- whether a node is eligible for display
- whether a node is the last sibling, affecting branch glyph/indent state
- active-flag transitions during traversal

The active flag is especially important for recursive graph traversal boundaries.

## 6.4 Option parsing edge cases

The parseopt system includes explicit support for:

- short options
- long options
- negative-option matching (`negmatch`)
- permutation of argv elements
- lookahead without consumption
- aliased option handling (`opt_unalias`)
- help/version/usage injections

These are boundary behaviors beyond a simple linear option parser.

## 6.5 Rc/profile file boundary handling

Configuration file paths include:

- file lookup
- tilde expansion
- duplicate registration by device/inode
- file-sourced parse option reading

This indicates careful treatment of:
- user path shorthand
- repeated file inclusion
- nested profile loading

## 6.6 Wordsplit quoting and expansion special handling

`wordsplit` contains many special-case behaviors:

- quoted-string scanning
- quote removal
- C-style quote/unquote helpers
- variable expansion
- command expansion
- tilde expansion
- path/glob expansion
- sed-expression skipping
- parameter assignment and parameter-vector expansion
- null elimination
- whitespace trimming

This is the richest special-case area in the project. It is not a plain whitespace splitter; it is a staged shell-like processor with multiple exceptional transformation rules.

## 6.7 Scanner boundary cases

The generated lexer handles:

- NUL transitions (`yy_try_NUL_trans`)
- next-buffer transitions
- push/pop nested buffers
- scan from string/bytes/buffer
- explicit line/input/output/debug setters and getters

Boundary handling includes:
- end-of-buffer transitions
- string-backed input
- byte-buffer scanning
- nested scanning sources

## 6.8 Hash-table boundary handling

The `gnu/hash.c` modules show explicit support for:

- safe hashing (`safe_hasher`)
- raw fallback hashing/comparison
- bucket-size computation
- prime sizing
- rehashing
- insert-if-absent
- remove/delete
- clear/free
- table verification (`hash_table_ok`)

This indicates boundary attention around:
- empty/non-empty table iteration
- collision buckets
- growth/rehash thresholds
- safe vs raw access paths

The current module summary is insufficient to support a more detailed behavior judgment about exact load-factor policy.

## 6.9 Word-wrap boundary handling

The formatter has explicit support for:

- beginning/end of line queries
- left/right margin changes
- deferred next-left-margin
- paragraph separation
- multibyte-safe scanning (`safe_mbrtowc`)
- whitespace prefix handling
- rescan and last-whitespace logic

These indicate runtime care around:
- hard line width boundaries
- multibyte character boundaries
- paragraph breaks
- flush-on-overflow behavior

## 6.10 Standalone utility boundary cases

### `doc/d.c`
- directory-ignore filtering via `ignorent`
- directory-vs-nondirectory distinction via `isdir`
- recursion depth via `level`

### `doc/wc.c`
- token classification via `isword`
- file-specific reporting via `report`
- error/non-error printing split

### `doc/ack.c`
- recursive numeric boundary behavior is implied by self-call structure

The current module summary is insufficient to support a more detailed behavior judgment on exact stopping conditions in these utilities.

---

## 7. Behaviors that must remain consistent with the C version

## 7.1 Entry-point and return-flow preservation

All `main` functions and public entry functions identified in the summaries must preserve:

- control-flow order
- call sequencing intent
- return-path structure

This applies especially to:

- `src/main.c: main`
- `doc/*: main`
- `test/*: main`

## 7.2 Parser reversibility behavior

The following behavior is essential to preserve:

- mark/restore token positions
- putback capability
- save-stack accumulation and finalization
- balanced-skip traversal

Any reimplementation must preserve the fact that parsing is stateful and reversible in these places, not purely streaming.

## 7.3 Scope-based symbol cleanup semantics

These runtime transitions must remain consistent:

- symbol installation before use
- storage updates after installation
- deletion of autos/statics/parms at scope boundaries
- parameter movement where `move_parms` is used
- target/starter filtering after graph construction

## 7.4 Output traversal semantics

The C implementation distinguishes:

- direct vs inverted tree traversal
- xref output vs tree output
- driver-selected formatting backends
- level/last-sibling-sensitive indentation

These must remain behaviorally separate.

## 7.5 Config precedence pipeline

The runtime accepts configuration from multiple sources:

- environment
- rc/profile files
- argv options

The existence of dedicated loaders for each means source-specific processing order is part of the program behavior and must remain consistent.

The exact precedence cannot be fully ordered from the summary alone; **the current module summary is insufficient to support a more detailed behavior judgment**.

## 7.6 Wordsplit staged transformation model

The following staged behaviors must remain distinct:

1. initialization
2. segmentation
3. variable/command/tilde/path expansion
4. quote removal
5. coalescing/null elimination/trim
6. final word-vector extraction
7. explicit error-state retention and cleanup

This staged model is a runtime behavior requirement, not just an implementation detail.

## 7.7 Hash-table lifecycle behavior

Hash subsystem behavior that must remain intact includes:

- initialize
- lookup/insert
- insert-if-absent distinction
- delete/remove distinction
- iteration (`hash_get_first`, `hash_get_next`, `hash_get_entries`)
- rehashing
- clear/free

## 7.8 Word-wrap streaming behavior

Preserve:

- open/write/flush/close lifecycle
- line buffering and rescan behavior
- left/right margin state
- at-bol / at-eol queries
- paragraph and printf-style write paths

## 7.9 Error routing separation

Different subsystems maintain their own error channels:

- parseopt errors
- file/source parse errors
- generic GNU `error` reporting
- wordsplit object-local errors
- utility-specific error wrappers in `doc/wc.c`

These channels should not be collapsed into a single undifferentiated path if behavior parity is required.

---

## 8. Performance-sensitive paths

## 8.1 Symbol lookup and insertion

The symbol layer depends on hash-based access:

- `hash_symbol_hasher`
- `hash_symbol_compare`
- `lookup`
- `install`

This is a likely performance-critical path because every encountered identifier may traverse it.

## 8.2 Parser token processing hot path

Frequent operations include:

- `nexttoken`
- `putback`
- `tokpush`
- `tokins`
- `tokdel`
- `save_token`

These occur inside declaration and expression parsing loops, so they form a hot path.

## 8.3 Scanner buffer transitions

Likely high-frequency scanner operations include:

- `yy_get_next_buffer`
- `yy_get_previous_state`
- `yy_try_NUL_trans`
- `yy_load_buffer_state`

These are central to token stream throughput.

## 8.4 Call/reference graph construction

Frequent graph update paths include:

- `get_symbol`
- `add_reference`
- `call`
- linked-list append/prepend/iterate
- hash table operations underneath symbol storage

This area combines parsing hot paths with dynamic graph-state updates.

## 8.5 Output traversal over symbol graph

For large graphs, runtime-sensitive paths include:

- `tree_output`
- `direct_tree`
- `inverted_tree`
- `print_symbol`
- sorting/comparison used in output
- `xref_output`

The recursive traversals and cross-reference generation are likely scale-sensitive.

## 8.6 Hash-table maintenance and rehashing

`gnu/hash.c` contains several operations that matter under load:

- bucket-size computation
- safe hashing
- lookup
- insert-if-absent- remove/delete
- rehash
- iteration

These operations are performance-sensitive both in the main symbol table path and in any auxiliary registries such as option-file tracking.

## 8.7 Wordsplit expansion pipeline

Potentially expensive runtime behaviors in `wordsplit` include:

- command expansion
- variable expansion
- pathname expansion
- segment coalescing
- recursive/subsplit processing
- quote/unquote transformations

These are likely not dominant in normal source parsing, but they are among the most behaviorally complex and potentially costly configuration-related paths.

---

## 9. Observable side effects

## 9.1 Standard output and formatted report emission

The project emits user-visible output through several paths:

- main report generation via selected output drivers
- help/usage/version text from parseopt support
- word-wrap-managed formatted output
- standalone utility report functions such as `doc/wc.c: report`

Observable side effects include:

- call graph tree text
- cross-reference listings
- DOT graph text
- GNU/POSIX formatted output variants
- usage/help/version text
- utility-specific textual reports

## 9.2 Standard error diagnostic emission

Diagnostics are emitted through:

- `error`
- `error_at_line`
- `parseopt_error`
- `po_env_error`
- `fromfile_error`
- `file_error`
- `errf`
- `perrf`
- `wordsplit_perror`

Observable side effects include:

- contextual parse/configuration errors
- system-error-augmented diagnostics
- file/line-aware diagnostics
- subsystem-specific failure reports

## 9.3 File and stream interaction

Runtime interacts with files/streams through:

- source input opening
- preprocessor stream opening/finalization
- rc/profile file reading
- word-count utility file processing
- directory utility traversal
- output file descriptor or stream wrapping in wordwrap
- portability wrappers around open/stat/fstat/close/dup2/fcntl

Observable side effects include:

- reading source/configuration files
- traversing filesystem directories
- stat-like metadata queries
- possible descriptor duplication/normalization on some platforms

## 9.4 Process-environment interaction

The runtime inspects environment configuration through:

- `parseopt_from_env`
- program-name helpers
- wordsplit/environment-related expansion support

Observable side effects include dependence on process environment variables for configuration and expansion results.

## 9.5 Memory and object lifecycle side effects

Across modules, dynamic memory is allocated, grown, and freed using:

- `xmalloc`, `xcalloc`, `xrealloc`, `xpalloc`
- wordsplit allocation wrappers
- linked-list and node constructors
- hash table alloc/free helpers
- scanner buffer creation/deletion
- parseopt and wordwrap object lifecycle functions

Observable behavior includes heap growth during parsing/graph construction and cleanup at teardown or object-finalization boundaries.

---

## 10. Inter-module runtime coordination

## 10.1 `main` -> option/configuration modules

`src/main.c` coordinates with:

- `src/parseopt/parseopt.c`
- `src/parseopt/optset.c`
- `src/parseopt/usage.c`
- `src/parseopt/help.c`
- `src/parseopt/version.c`
- `src/wordsplit/wordsplit.c`

Behaviorally:

1. main delegates parsing of options and config sources.
2. parseopt resolves tokens into callbacks.
3. callbacks mutate main-program state.
4. help/version/usage paths may short-circuit normal source-analysis flow.

## 10.2 `main` -> lexer/parser frontend

`src/main.c` coordinates with:

- `src/c.c`
- `src/parser.c`

Behaviorally:

1. source/preprocessor setup is established.
2. scanner is initialized or restarted for input.
3. parser consumes tokens and drives semantic symbol creation.

## 10.3 parser -> symbol table / graph modules

`src/parser.c` coordinates with:

- `src/symbol.c`
- `src/linked-list.c`
- `src/depmap.c`
- hash support

Behaviorally:

1. parsed identifiers are looked up/installed.
2. declaration metadata updates symbol state.
3. function-body references/calls add graph edges.
4. list/hash structures preserve adjacency and membership.

## 10.4 main/symbol -> output modules

Output generation coordinates among:

- `src/output.c`
- `src/gnu.c`
- `src/posix.c`
- `src/dot.c`

Behaviorally:

1. output driver is selected earlier.
2. traversal logic queries symbol graph state.
3. backend-specific handlers turn generic output events into concrete text.

## 10.5 parseopt/help -> wordwrap

Formatted help text coordinates with:

- `src/parseopt/wordwrap.c`

Behaviorally:

1. help/usage builds text fragments and option-group sections.
2. wordwrap enforces margins and line splitting.
3. final formatted help is emitted to a target stream.

## 10.6 configuration/profile handling -> wordsplit

Where option text or profiles require shell-like splitting/expansion, behavior coordinates with:

- `src/wordsplit/wordsplit.c`

This permits nontrivial expansion and tokenization behavior before options re-enter the parseopt pipeline.

---

## 11. Behavior notes by major file group

## 11.1 `src/main.c`

Primary runtime responsibilities visible from function inventory:

- global initialization
- source/configuration intake
- option callback handling
- starter/target state setup
- preprocess/path settings
- output mode selection
- main execution control

The module appears to be the orchestration layer rather than the deep parser itself.

## 11.2 `src/c.c`

Primary behaviors:

- scanner runtime
- token acquisition
- source location updates
- preprocessor stream integration
- scanner buffer lifecycle
- lexical/global debug and buffer state

This is the token-source engine for the parser.

## 11.3 `src/parser.c`

Primary behaviors:

- declaration parsing
- function/body parsing
- rollback-capable token processing
- balanced-delimiter handling
- scope cleanup actions
- parse-time semantic classification

This is the core syntax-analysis state machine.

## 11.4 `src/symbol.c`

Primary behaviors:

- symbol creation/lookup
- graph edge insertion
- traversal roots/targets
- scope-aware deletion
- output-selection filtering
- symbol lifecycle and list unlinking

This is the semantic graph/state manager.

## 11.5 `src/output.c`, `src/gnu.c`, `src/posix.c`, `src/dot.c`

Primary behaviors:

- backend registration and selection
- generic output dispatch
- tree/xref traversal formatting
- backend-specific rendering styles

This is the report emission layer.

## 11.6 `src/parseopt/*`

Primary behaviors:

- token-by-token option scanning
- matching short/long/negated options
- option permutation
- callback-driven state mutation
- formatted usage/help/version output
- auxiliary word wrapping

This is a reusable CLI state machine plus formatted documentation subsystem.

## 11.7 `src/wordsplit/wordsplit.c`

Primary behaviors:

- shell-like lexical segmentation
- staged expansions
- quote handling
- error-state management
- final argv-style word extraction

This is the most transformation-heavy text-processing subsystem.

## 11.8 `gnu/*`

Primary behaviors:

- allocation wrappers
- error-reporting framework
- hash table implementation
- portability wrappers
- program-name handling
- safe formatting and string utilities

These modules support but generally do not define the main application flow.

---

## 12. Conservative conclusions

From the available summaries, the project’s runtime behavior is best characterized as:

1. a **configuration-driven C source analysis pipeline**
2. with a **stateful option/config parser**
3. a **buffered lexical scanner**
4. a **reversible declaration parser**
5. a **scope-aware symbol/call graph builder**
6. a **filtering phase for starters/targets**
7. and a **late-bound output renderer** supporting multiple textual formats

In addition, the repository includes:

- independent documentation/example executables in `doc/*`
- standalone tests in `test/*`
- bundled infrastructure libraries for portability and formatting

Several details—especially exact precedence rules, exact emitted text formats, specific parser recovery choices, and exact termination semantics for some errors—cannot be asserted more strongly from summaries alone. In those places, **the current module summary is insufficient to support a more detailed behavior judgment**.