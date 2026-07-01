# 03_behaviors

## 1. Overview of runtime behavior

The analyzed project is organized into several behavior-bearing areas:

- `src/`: the primary cflow program logic
- `src/parseopt/`: command-line and configuration parsing plus help/usage text formatting
- `src/wordsplit/`: shell-like string splitting and expansion engine
- `gnu/`: portability/runtime support, memory helpers, I/O wrappers, hash table support, formatting support
- `doc/`: standalone sample/demo programs
- `test/`: standalone test/demo programs

From the symbol distribution and function names, the main executable behavior is centered in:

- `src/main.c`
- `src/parser.c`
- `src/c.c`
- `src/symbol.c`
- `src/output.c`
- `src/parseopt/*.c`

The `doc/` and `test/` trees contain independent entry points and should be treated as separate runnable programs, not as startup stages of the main cflow executable.

Where implementation bodies are not visible from the summary, behavior is described only at the level supported by the available function names, call density, and grouping.

---

## 2. Initialization flow and startup order

## 2.1 Primary cflow executable startup

The main startup entry is:

- `src/main.c: main(int argc, char **argv)`

The visible surrounding functions indicate the following startup sequence exists as a dynamic flow:

1. **Program entry receives raw argv**
   - `main` starts with the process command line.
   - It is supported by `parse_rc`, `init`, `include_symbol`, `globals_only`, and option/config helpers in the same file.

2. **Program-global initialization**
   - `init()` in `src/main.c` appears to perform initial program setup before parsing input or producing output.
   - The exact field-by-field initialization is not available from the summary.
   - The current module summary is insufficient to support a more detailed behavior judgment.

3. **Configuration loading**
   - `parse_rc()` indicates a startup phase that reads runtime configuration files.
   - Supporting flow:
     - `parseopt_from_rc`
     - `optfile_lookup`
     - `tildexpand`
     - `parseopt_from_env`
     - `fromfile`
     - `optset_profile`
   - This shows startup configuration can come from:
     - rc files
     - environment
     - explicit profile/file-driven option sources

4. **Option parser initialization**
   - `src/parseopt/parseopt.c` contains:
     - `parseopt_init0`
     - `parseopt_init`
     - `parseopt_parse`
     - `parseopt_getopt`
   - This indicates a staged parser setup:
     - zero/base initialization
     - argv binding
     - scan/parse execution
     - query/access of parse results

5. **Application-specific option setters run**
   - `src/main.c` contains many `optset_*` handlers:
     - include classes
     - output driver
     - xref mode
     - symbol selection
     - preprocessor options
     - indentation setup
     - main symbol setup
     - install target
     - path prepend
     - profile load
   - This shows startup is not a single parse step; it is an iterative parse-and-apply cycle where each recognized option mutates program state.

6. **Lexer/parser initialization before source processing**
   - `src/c.c`:
     - `init_tokens`
     - `init_lex`
   - `src/parser.c`:
     - `init_parse`
   - This indicates a second-stage initialization after configuration:
     - token subsystem setup
     - lexical analyzer setup
     - parser stack/state setup

7. **Output subsystem initialization**
   - `src/output.c`:
     - `output_init`
     - `register_output`
     - `select_output_driver`
   - `src/gnu.c`, `src/posix.c`, `src/dot.c` each expose output handlers.
   - This indicates startup includes selecting and binding one output backend before final reporting.

8. **Source processing and result production**
   - Input files are scanned via lexer/parser flows.
   - Symbols and references are accumulated.
   - Output is emitted through selected driver and/or xref/tree output functions.

9. **Termination**
   - No explicit shutdown summary is available for the main program.
   - Some subsystems provide cleanup:
     - `parseopt_free`
     - `yylex_destroy`
     - linked-list destruction
     - wordsplit freeing APIs
   - Whether all are always executed on normal exit is not visible here.

---

## 2.2 Startup order inside option/config parsing

The parse-option subsystem has an internal startup structure:

1. `parseopt_init0(struct parseopt *po)`
   - establishes parser base state
2. `parseopt_init(struct parseopt *po, int argc, char **argv)`
   - attaches runtime argv and prepares option tables
3. `prepare_optdef`
   - option definitions are normalized/prepared
4. `collect_optdef`
   - option definitions are collected
5. `parseopt_parse` / `parseopt_getopt`
   - iterative option consumption begins

Additional built-in help/version behavior is wired by:
- `set_help`
- `set_usage`
- `set_version`

This means help/version are not external hacks; they are part of the parser’s stateful option handling path.

---

## 2.3 Startup order inside lexical analysis

The generated scanner in `src/c.c` shows a classic staged runtime:

1. `init_tokens()`
2. `init_lex(int debug_level)`
3. scanner buffer creation or reset
   - `yy_create_buffer`
   - `yy_init_buffer`
   - `yyrestart`
   - `yy_switch_to_buffer`
4. token acquisition
   - `get_token`
   - internal flex engine:
     - `yy_get_next_buffer`
     - `yy_get_previous_state`
     - `yy_try_NUL_trans`
5. end-of-input handling
   - `yywrap`
   - buffer pop/delete
6. scanner teardown
   - `yylex_destroy`

This indicates startup of lexical processing is buffer-oriented and stateful, not a stateless line-by-line parser.

---

## 2.4 Startup order inside parsing

`src/parser.c` suggests the parser runs after lexical setup:

1. `init_parse()`
2. token stack management begins:
   - `tokpush`
   - `nexttoken`
   - `putback`
3. optional save/restore support:
   - `mark`
   - `restore`
   - `save_token`
   - `finish_save_stack`
4. top-level parse loop:
   - `yyparse()`
5. declaration/expression/function-body specific handlers run.

This indicates parsing is a mutable-token-stream workflow with backtracking-like save/restore capability.

---

## 2.5 Startup order for standalone demo/test executables

Separate `main` functions exist in:

### `doc/`
- `doc/d.c`
- `doc/foo.c`
- `doc/wc.c`
- `doc/whoami.c`

### `test/`
- `test/recursion.c`
- `test/simple.c`

These are independent programs with their own startup order. They do not participate in the primary `src/main.c` runtime.

---

## 3. Main user operation flows

## 3.1 User flow: run cflow on source input

The dominant interactive/runtime flow supported by the summaries is:

1. launch `main(argc, argv)`
2. initialize global state
3. load rc/environment/profile-derived options
4. parse command-line arguments
5. select output style and symbol filtering rules
6. initialize lexical and parse subsystems
7. open/process source input
8. build symbol/reference/call relationships
9. emit output in selected representation
10. exit

The available modules show several major branches inside this flow.

---

## 3.2 User flow: option processing

This is a dynamic, iterative operation rather than a one-shot parse.

### Observed operation sequence
- parser looks ahead: `parseopt_lookahead`
- parser consumes/skips current token: `parseopt_skip`
- parser chooses short or long option matching:
  - `option_find_short`
  - `option_find_long`
- parser resolves negation rules:
  - `negmatch`
- parser may reorder positional arguments:
  - `permute`
- parser dispatches to option setter callbacks

### State changes caused by option handlers
The option handlers in `src/main.c` indicate runtime state is updated for:

- symbol inclusion classes
- selected output driver
- xref mode
- main symbol selection/clearing
- preprocessor enablement and options
- indentation formatting
- install target registration
- integer-valued settings
- include path prepending
- profile loading

This means user options are not just stored; they immediately mutate the program’s active configuration.

---

## 3.3 User flow: reading options from environment and files

The presence of:

- `parseopt_from_env`
- `fromfile`
- `parseopt_from_rc`
- `optfile_lookup`
- `optfile_register`

shows a multi-source configuration flow:

1. locate candidate config source
2. detect file identity / prevent repeated registration (`optfile_register`)
3. parse the file contents (`fromfile`)
4. route errors through dedicated reporters (`fromfile_error`, `po_env_error`)
5. apply resulting option settings through normal option setter paths

This means startup configuration is cumulative and source-aware.

---

## 3.4 User flow: lexical scanning and token production

The lexer flow is stateful and buffer-driven.

### Main runtime stages
1. set up preprocessor behavior
   - `set_preprocessor`
   - `pp_option`
2. open preprocessed source or direct source stream
   - `pp_open`
   - `source`
3. scanner reads from current buffer
   - `yy_get_next_buffer`
   - `yy_get_previous_state`
4. client obtains next token
   - `get_token`
5. token location may be updated
   - `update_loc`
6. escape and numeric lexemes are specialized:
   - `backslash`
   - `getnum`
7. end-of-input and nested/buffer transitions:
   - `yywrap`
   - `yyrestart`
   - push/pop/switch/delete buffer functions

This indicates the lexer is built to handle nontrivial source forms, nested/buffered input, and location tracking.

---

## 3.5 User flow: parsing declarations, functions, and references

The parser summaries show a broad runtime split among declarations, definitions, and expressions.

### Top-level parse flow
- `yyparse`
- `is_function`
- `parse_declaration`
- `parse_function_declaration`
- `parse_variable_declaration`
- `parse_typedef`
- `parse_dcl`
- `declare`
- `declare_type`

### Expression/body/reference flow
- `expression`
- `func_body`
- `call`
- `reference`
- `add_reference`
- `get_symbol`

### K&R and structural special paths
- `parse_knr_dcl`
- `get_knr_args`
- `fake_struct`
- `skip_struct`

This indicates the runtime parser distinguishes:
- type declarations
- variable declarations
- function declarations/definitions
- old-style K&R declarations
- structure-related syntax
- call/reference recording from executable bodies

The parser does not merely parse syntax; it also updates symbol graph state while parsing.

---

## 3.6 User flow: symbol-table construction and mutation

The symbol subsystem is dynamically active throughout parsing.

### Core operations
- `lookup`
- `install`
- `install_ident`
- `install_starter`
- `install_target`
- `ident_change_storage`
- `init_ident`

### Deletion and lifecycle operations
- `delete_statics`
- `delete_autos`
- `delete_parms`
- `unlink_symbol`
- `delete_symbol`
- `static_free`

### Collection/filtering operations
- `collect_symbols`
- `collect_functions`
- `eliminate_non_targets`
- `mark_callers`

This shows the symbol table is not append-only. It supports:
- insertion
- storage-class mutation
- scoped deletion
- starter/target selection
- graph pruning

That is a key dynamic property of the program.

---

## 3.7 User flow: output generation

The output layer has multiple modes.

### Shared output-control flow
- `output_init`
- `register_output`
- `select_output_driver`
- `begin`
- `separator`
- `print_text`
- `end`

### Tree and reference-oriented output
- `output`
- `tree_output`
- `direct_tree`
- `inverted_tree`
- `print_refs`
- `xref_output`

### Driver-specific backends
- `gnu_output_handler`
- `posix_output_handler`
- `dot_output_handler`

This indicates output generation is command/event driven:
- initialize driver
- emit begin marker
- emit symbols and edges/references
- emit separators/newlines
- emit end marker

`dot_output_handler` and node declaration helpers show one output mode emits graph structure rather than plain text.

---

## 3.8 User flow: wordsplit processing

The `src/wordsplit` tree exposes a separate shell-like tokenization engine.

### High-level flow
1. initialize wordsplit state
   - `wordsplit_init0`
   - `wordsplit_init`
2. scan command/string into nodes
   - `scan_word`
   - `scan_qstring`
   - delimiter skipping helpers
3. expand content
   - variable expansion
   - command expansion
   - tilde expansion
   - path expansion
4. postprocess nodes
   - quote removal
   - node coalescing
   - whitespace trimming
5. finalize result
   - `wordsplit_finish`
   - `wordsplit_get_words`
6. cleanup
   - `wordsplit_free_*`

### Expansion-specific paths
- `wordsplit_varexp`
- `wordsplit_cmdexp`
- `wordsplit_tildexpand`
- `wordsplit_pathexpand`

This is an explicitly staged transformation engine over an internal node list.

---

## 3.9 User flow: word wrapping for help/usage output

`src/parseopt/wordwrap.c` shows a runtime text-formatting flow:

1. `wordwrap_open` / `wordwrap_fdopen`
2. line state initialization: `wordwrap_line_init`
3. text write path:
   - `wordwrap_write`
   - `wordwrap_puts`
   - `wordwrap_putc`
   - `wordwrap_printf` / `wordwrap_vprintf`
4. wrapping decisions:
   - whitespace scan
   - last whitespace tracking
   - line flush
5. margin control:
   - set left margin
   - next left margin
   - set right margin
6. finalization:
   - `wordwrap_flush`
   - `wordwrap_close`

This means help/usage generation is dynamically width-managed, not emitted as fixed raw strings.

---

## 4. State machines and state transitions

## 4.1 Parseopt state machine

The parse option subsystem behaves like a command-line parsing state machine with these observed states:

1. **Uninitialized**
   - before `parseopt_init0`

2. **Base initialized**
   - after `parseopt_init0`
   - option tables and parser fields are prepared

3. **Bound to argv**
   - after `parseopt_init`
   - parser has argc/argv context

4. **Scanning**
   - `parseopt_next_internal`
   - current token is examined as:
     - short option
     - long option
     - non-option argument
     - help/usage/version trigger

5. **Option dispatch**
   - matching option setter is called
   - parser state mutates as option values are stored/applied

6. **Permutation/skip handling**
   - `permute`
   - `parseopt_skip`
   - parser advances over or reorders pending arguments

7. **Finished**
   - parse loop completes
   - `parseopt_is_set`, `parseopt_optdef_by_code`, and related queries can be used

8. **Freed**
   - `parseopt_free`

The exact field transitions are not shown, but the state progression is clearly staged.

---

## 4.2 Flex scanner state machine

The generated scanner in `src/c.c` is the clearest explicit state machine in the project.

### Main scanner states
- active buffer present / absent
- current buffer new / normal / flushed
- scanning input
- end-of-buffer transition
- pushed nested buffer stack
- destroyed/reset state

### Transition functions
- `yy_create_buffer`
- `yy_init_buffer`
- `yy_scan_buffer`
- `yy_scan_string`
- `yy_scan_bytes`
- `yy_switch_to_buffer`
- `yypush_buffer_state`
- `yypop_buffer_state`
- `yy_flush_buffer`
- `yy_delete_buffer`
- `yyrestart`
- `yy_get_next_buffer`
- `yy_get_previous_state`
- `yy_try_NUL_trans`
- `yy_load_buffer_state`
- `yy_init_globals`
- `yylex_destroy`

This is a concrete runtime state machine whose transitions are driven by:
- initial source binding
- end-of-buffer detection
- nested source switching
- scanner reset/destruction

---

## 4.3 Parser token-stack state machine

`src/parser.c` maintains its own mutable parse-stack/token-save machinery.

### Main states
1. **Empty stack / clean parse state**
   - after `init_parse` or `clearstack`

2. **Token accumulation**
   - `tokpush`
   - `tokins`

3. **Consumption**
   - `nexttoken`

4. **Backtrack-ready**
   - `mark`
   - `save_token`

5. **Rollback / restore**
   - `restore`
   - `undo_save_stack`

6. **Commit saved sequence**
   - `finish_save_stack`

7. **Cleanup**
   - `cleanup_stack`
   - `tokdel`

This indicates the parser runtime includes reversible transitions, not just one-way token consumption.

---

## 4.4 Balance-state stack transitions

The parser also has an explicit balance-state stack:

- `push_balance_state`
- `pop_balance_state`
- `free_balance_stack`
- `find_closing_paren`

This state machine tracks nested balanced constructs such as parentheses/brackets/braces at parse time.

Behaviorally, the transitions are:

- push on entering a nested region
- pop on closing match
- free all on end/error/cleanup
- search for matching close while respecting nesting level

---

## 4.5 Symbol lifecycle states

Symbols appear to move through several states during runtime:

1. **Absent**
   - before `lookup` succeeds

2. **Installed**
   - via `install` / `install_ident`

3. **Storage-class-adjusted**
   - via `ident_change_storage`

4. **Referenced or called**
   - via `reference`, `add_reference`, `call`

5. **Starter/target-marked**
   - via `install_starter`, `install_target`, `set_default_starter`

6. **Collected/selected**
   - via `collect_symbols`, `collect_functions`

7. **Pruned**
   - via `eliminate_non_targets`, `mark_callers`

8. **Scope-deleted**
   - via `delete_autos`, `delete_parms`, `delete_statics`

9. **Unlinked/deleted**
   - via `unlink_symbol`, `delete_symbol`

The exact bit flags or field values are not visible, but the behavioral lifecycle is clear.

---

## 4.6 Output driver state transitions

Output generation follows a driver state model:

1. **No driver / uninitialized**
2. **Registered drivers available**
   - `register_output`
3. **Selected active driver**
   - `select_output_driver`
4. **Begin output**
   - `begin`
5. **Emit items**
   - symbol printing / refs / tree edges / xref
6. **Emit separators/newlines as needed**
7. **End output**
   - `end`

Driver handlers (`gnu_output_handler`, `posix_output_handler`, `dot_output_handler`) act as transition responders for output commands.

---

## 4.7 Wordsplit node-processing state machine

The wordsplit engine exposes a rich transformation pipeline over node lists.

### Node/list states
1. **Input not initialized**
2. **Wordsplit context initialized**
3. **Raw segments created**
   - `wordsplit_add_segm`, `wsnode_new`, `wsnode_append`
4. **Node list structurally edited**
   - insert/remove/split/prefix-split
5. **Expansion stage**
   - variable expansion
   - command expansion
   - tilde expansion
   - path expansion
6. **Quote-removal and coalescing**
   - `wsnode_quoteremoval`
   - `wsnode_coalesce`
   - tail coalescing
7. **Null-elimination and trim**
   - `wsnode_nullelim`
   - `wordsplit_trimws`
8. **Finish/final word vector**
   - `wordsplit_finish`
   - `wordsplit_get_words`
9. **Freed**
   - `wordsplit_free*`

This is one of the strongest explicit transformation-state subsystems in the project.

---

## 5. Error-handling flows

## 5.1 Main-program error routing

Several dedicated error reporters exist:

- `po_env_error`
- `fromfile_error`
- `file_error`
- `xalloc_die` in `src/main.c`
- GNU support `error` and `error_at_line`

This shows errors are handled through subsystem-specific reporting paths rather than one single generic handler.

### Observed error sources
- environment option parsing
- option file parsing
- parser file/token context
- allocation failure
- generic runtime/library conditions

The exact exit behavior of each reporter is not uniformly visible from the summaries. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 5.2 Parseopt error flow

The parseopt subsystem includes:

- `parseopt_error`
- built-in help/usage/version actions
- option matching helpers that can reject invalid forms

Dynamic error flow appears to be:

1. detect invalid or unsupported option form
2. identify current parser/option context
3. report through `parseopt_error` or an embedding-specific hook
4. stop current parse step or continue according to parser policy

The continuation/termination policy is not fully visible here.

---

## 5.3 Parser error flow

`src/parser.c` contains `file_error`, plus token stack save/restore helpers.

That suggests parser error handling includes:
- reporting the offending token/file location
- using mark/restore or stack cleanup to recover parser state
- skipping forward in some contexts (`skip_to`, `skip_balanced`, `skip_struct`, `skip_declaration`)

This is important: parse errors are not represented only as immediate termination. There are explicit skip/recovery paths.

---

## 5.4 Lexer error flow

The scanner includes:
- `yy_fatal_error`
- buffer management teardown
- `pp_finalize`
- `pp_close`

This indicates fatal scanner errors exist, and the scanner also has explicit cleanup/finalization paths for preprocessor-driven input streams.

---

## 5.5 Allocation/error flow in support libraries

GNU and local support layers include multiple failure handlers:

- `xalloc_die`
- `_wsplt_alloc_die`
- `_wsplt_nomem`
- `_wsplt_seterr`
- `wordwrap_error`
- `error` / `error_at_line`
- `errf` / `perrf` / `error_print` in `doc/wc.c`

Behaviorally this means:
- memory-sensitive paths often centralize failure behavior
- wordsplit stores error context in addition to error status
- help/wordwrap maintains its own output/error state
- sample programs have local reporting helpers

The exact error codes and message formats are outside the summary evidence.

---

## 6. Boundary conditions and special-case handling

## 6.1 Configuration-source boundaries

Observed dedicated logic exists for:
- environment-sourced options
- rc files
- option files / profile files
- duplicate file tracking (`optfile_register` with device/inode parameters)

This indicates the startup system handles repeated or nested configuration sources as a distinct boundary condition.

---

## 6.2 Command-line option edge cases

Special-case handling is explicitly represented by:
- short vs long options
- option negation matching (`negmatch`)
- help/usage/version pseudo-options
- argv permutation (`permute`)
- skipped arguments (`parseopt_skip`)

Therefore command-line behavior must preserve:
- recognition path differences
- non-option/option reordering behavior
- special control options that bypass normal operation flow

---

## 6.3 Parser syntax edge cases

The parser has dedicated paths for:
- balanced delimiters
- K&R declarations
- typedef parsing
- struct skipping/faking
- initializer lists
- function-vs-variable disambiguation

This means the parser behavior is intentionally branchy around ambiguous C syntax, not a flat token consumer.

Preserved boundary handling includes:
- nested delimiter search
- declaration grammar ambiguity resolution
- old-style function syntax handling
- skipping unsupported or irrelevant structured regions

---

## 6.4 Scope and storage boundaries

The symbol subsystem explicitly distinguishes:
- statics
- autos
- parameters
- starter symbols
- target symbols

And supports separate deletion functions:
- `delete_statics`
- `delete_autos`
- `delete_parms`

Thus scope exit and storage-class cleanup are runtime-significant boundaries.

---

## 6.5 Output-mode boundaries

The output subsystem has at least these behavior boundaries:
- GNU style
- POSIX style
- DOT graph style
- xref output vs tree output

Switching output driver is not cosmetic; it changes the runtime emission path.

---

## 6.6 Scanner input boundaries

The scanner explicitly handles:
- file-backed buffers
- memory buffers
- string/bytes scanning
- buffer stack push/pop
- end-of-input wrapping

This means input origin and nesting are important boundary cases.

---

## 6.7 Wordsplit special cases

The wordsplit engine contains explicit handling for:
- quoted strings
- variable expansion
- command substitution
- tilde expansion
- glob/path expansion
- null elimination
- whitespace trimming
- C-style quote/unquote helpers
- parameter assignment and parameter-vector expansion
- parenthesis matching
- sed-expression skipping

This is a large special-case surface. These are not peripheral utilities; they are central to the wordsplit runtime.

---

## 6.8 Platform boundary handling in `gnu/`

The `gnu/` area contains many wrappers whose behavior exists specifically at platform or libc boundaries:

- `open`, `rpl_stat`, `rpl_fstat`, `rpl_dup2`, `rpl_close`, `rpl_fcntl`
- Windows-specific stat and invalid-parameter handling
- close/ioctl hook execution
- cloexec helpers
- `getdtablesize` compatibility
- `getprogname`, `set_program_name`
- stdio wrapper entry points
- `strerror` override logic

These support layers are behaviorally important because they normalize runtime behavior across environments. The exact platform matrix is not fully reconstructible from the summary, but cross-platform boundary mediation is clearly a core concern.

---

## 7. Behaviors that must remain consistent with the C version

## 7.1 Preserve staged startup ordering

The C implementation separates:
- main/global init
- configuration ingestion
- option parse/apply
- lexer/parser init
- source analysis
- output emission

Any reimplementation must preserve this runtime order, because configuration setters and output driver selection are upstream of parsing and reporting.

---

## 7.2 Preserve mutable token-stream parsing behavior

The parser is not purely functional. It relies on:
- push/insert/delete token operations
- mark/restore
- save stack
- skip/recovery functions
- balance tracking

These behaviors must remain mutable and order-sensitive.

---

## 7.3 Preserve symbol lifecycle mutations

The C version supports:
- install
- lookup
- storage mutation
- scope-based deletion
- caller/target marking
- collection and pruning

A replacement must preserve these transitions, not just final symbol existence.

---

## 7.4 Preserve output-driver dispatch behavior

Output is driven through registered handlers and command callbacks. The active driver changes runtime formatting and traversal behavior. This dispatch model must remain consistent.

---

## 7.5 Preserve wordsplit transformation pipeline order

The wordsplit engine is staged:
1. initial scan
2. expansion
3. quote removal
4. coalescing
5. trimming
6. finish/export
7. free

Reordering these stages would change behavior. The C version’s sequence must remain consistent.

---

## 7.6 Preserve scanner buffer-stack semantics

The scanner supports:
- switching current buffer
- pushing nested buffers
- popping back
- scan-from-string/bytes
- explicit flush/delete

These buffer transitions are part of runtime semantics and must remain consistent.

---

## 7.7 Preserve C-specific declaration parsing branches

Dedicated branches exist for:
- K&R declarations
- typedefs
- struct-related syntax
- function/variable disambiguation

These must remain behaviorally distinct.

---

## 7.8 Preserve dedicated error paths

Error handling is localized by subsystem:
- parseopt errors
- parser file errors
- wordsplit contextual errors
- allocation-failure handlers
- GNU generic error routines

These paths should not be collapsed into a single undifferentiated failure mode if behavior parity is required.

---

## 8. Performance-sensitive paths

## 8.1 Lexer hot path

The generated scanner routines are likely on the hottest path during source analysis:

- `yy_get_next_buffer`
- `yy_get_previous_state`
- `get_token`
- `update_loc`
- numeric/escape lexing helpers

These run repeatedly per token and per buffer refill. Their dynamic frequency makes them performance-sensitive.

---

## 8.2 Parser token and declaration path

Repeated parse-time functions likely form another hot region:

- `nexttoken`
- `putback`
- `tokpush`
- `dcl`
- `dirdcl`
- `getident`
- `expression`
- `declare`
- `call`
- `reference`

These functions are deeply involved in normal source traversal.

---

## 8.3 Symbol lookup/insertion path

The symbol subsystem depends on hash-based lookup and collection:

- `lookup`
- `install`
- `hash_symbol_hasher`
- `hash_symbol_compare`

Given repeated identifier encounters, this path is performance-sensitive.

The external GNU hash implementation also contains critical operations:

- `hash_lookup`
- `hash_find_entry`
- `hash_insert`
- `hash_insert_if_absent`
- `hash_remove`
- `hash_rehash`
- bucket size / prime selection helpers

These are central for symbol-table throughput.

---

## 8.4 Output traversal path

For large symbol graphs, these operations are likely hot:

- `tree_output`
- `direct_tree`
- `inverted_tree`
- `xref_output`
- `print_symbol`
- `print_refs`

They traverse and emit potentially large graph structures.

---

## 8.5 Wordsplit node-processing path

The wordsplit subsystem contains several potentially expensive repeated passes:

- scanning words/quoted strings
- variable expansion
- command expansion
- path expansion
- node coalescing
- process-list execution over nodes

This is especially true because behavior is node-list based, which can imply multiple list passes. The exact cost model is not visible from the summary, but repeated transformation passes are evident.

---

## 8.6 Wordwrap formatting path

For help/usage generation, performance is less globally critical than parsing, but internal hot loops exist:

- `wordwrap_write`
- `wordwrap_rescan`
- `wordwrap_last_ws`
- `flush_line`
- width/margin management helpers

These are important when formatting large generated help text.

---

## 8.7 Depmap closure computation

`src/depmap.c` includes:

- `transitive_closure`
- `depmap_tc`

This is a concentrated compute-oriented path, likely performance-sensitive when dependency graph size grows.

---

## 9. Behavior notes for standalone `doc/` programs

## 9.1 `doc/d.c`

Behavior-bearing functions:
- `isdir`
- `ignorent`
- `printdir`
- local `main`

The runtime clearly includes recursive or hierarchical directory-print behavior:
- classify path as directory
- ignore selected entries
- print directory content with level information

Because `printdir(int level, char *name)` carries a nesting level, the behavior includes level-based traversal state. The exact traversal order and skip criteria are not available in the summary.

---

## 9.2 `doc/wc.c`

Behavior-bearing functions:
- error printers
- `report`
- `isword`
- `getword`
- `counter`
- local `main`

This is a staged counting flow:
1. initialize/reporting context
2. classify input characters into word/non-word
3. extract words from a file stream
4. count and report per file
5. main dispatches counting over argv inputs

This module has explicit error-reporting separation and a word-detection/counting hot path.

---

## 9.3 `doc/whoami.c`

Behavior-bearing functions:
- `who_am_i`
- local `main`

The runtime behavior is centered on obtaining identity information and returning/printing it via main. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 9.4 `doc/ack.c`

Single function:
- `ack(u_long a, u_long b)`

This is a standalone recursive computation module. Since internal call count is 1 and external call count is 0, the function is self-recursive. The exact recursion branching behavior is not visible in the summary, but recursive control flow is a behavior that must be preserved.

---

## 10. Behavior notes for standalone `test/` programs

## 10.1 `test/multi.c`

Functions:
- `helper`
- `twice`
- `run`

The visible summary indicates:
- `run` calls `helper`
- computes `twice(42)`
- prints the result

This is a linear demonstration flow with one helper call and one arithmetic call before output.

---

## 10.2 `test/recursion.c`

Functions:
- `fib`
- `fact`
- `main`

This module demonstrates recursive execution paths for Fibonacci and factorial. Since these are central recursion examples, preserving recursive call behavior is the key runtime property.

---

## 10.3 `test/simple.c`

Functions:
- `add`
- `mul`
- `orphan`
- `compute`
- `main`

This module demonstrates a small computation pipeline with:
- basic arithmetic helpers
- one unreferenced/orphan function
- `compute` using helper arithmetic
- `main` invoking the test path

The exact arithmetic composition is not fully visible, but internal call count indicates deliberate intra-file call structure.

---

## 11. Areas where finer dynamic judgment is not supported

The following areas cannot be described more deeply from the supplied summaries alone:

- exact contents of global state initialized by `init()`
- exact exit-code rules in `src/main.c`
- exact file traversal rules in `doc/d.c`
- exact token grammar reductions in `yyparse`
- exact field-level symbol flags and state bits
- exact error-code propagation for parseopt, wordsplit, and GNU wrappers
- exact memory ownership/lifetime discipline beyond visible free/cleanup entry points
- exact output text format emitted by GNU/POSIX/DOT handlers
- exact subprocess behavior, if any, inside wordsplit command expansion

For these topics, the current module summary is insufficient to support a more detailed behavior judgment.