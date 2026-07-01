# 03_behaviors

## 1. Scope and evidence basis

This document describes runtime behavior that is directly supportable from the provided module summaries for the C project `cflow-new`. It focuses on observed execution flow, state changes, error paths, special handling, and consistency constraints.

Because the input is module-partition metadata rather than full source bodies, several fine-grained behaviors cannot be reconstructed exactly. In such cases, this document explicitly states that **the current module summary is insufficient to support a more detailed behavior judgment**.

The project behavior divides naturally into these areas:

- `src/`: the main `cflow` program, including startup, option parsing, lexical scanning, parsing, symbol/call graph construction, and output generation.
- `src/parseopt/`: command-line and help/usage formatting subsystem.
- `src/wordsplit/`: shell-like word splitting used by configuration/profile style inputs.
- `gnu/`: portability, utility, memory, formatting, hashing, and wrapper support.
- `doc/`: small standalone example or helper programs, each with its own `main`.
- `test/`: small test/example programs with independent entry points.

---

## 2. System-wide runtime model

At the highest level, the main `src` program behaves like a staged pipeline:

1. **Program startup**
   - establish global program state,
   - initialize configuration defaults,
   - initialize option parsing and output driver registry.

2. **Configuration loading**
   - read environment-derived options,
   - read rc/profile files,
   - normalize startup symbols and filters.

3. **Argument parsing**
   - parse command-line options and option arguments,
   - update global behavior flags and output mode.

4. **Lexical and parser initialization**
   - initialize scanner token tables and parser stacks,
   - configure preprocessor command/options if preprocessing is enabled.

5. **Input processing**
   - open each source input,
   - obtain tokens through the lexer,
   - parse declarations, definitions, references, and calls,
   - build/update symbol tables and call/reference relations.

6. **Selection/filter phase**
   - remove or mark symbols according to target/main/global rules,
   - collect starters/targets/functions for output.

7. **Output phase**
   - choose one output driver,
   - emit cross-reference or tree-style results,
   - finalize output.

8. **Cleanup**
   - release parseopt/wordsplit/scanner/symbol temporary state as needed,
   - return from `main`.

This staging is strongly supported by the names and clustering of:
- `src/main.c`
- `src/parser.c`
- `src/c.c`
- `src/symbol.c`
- `src/output.c`
- `src/parseopt/*.c`

---

## 3. Initialization flow and startup order

## 3.1 Main program startup (`src/main.c`)

The primary executable entry point is:

- `main` in `src/main.c`
- helper initialization functions: `init`, `parse_rc`, `include_symbol`, `globals_only`, `xalloc_die`
- option/config helpers: `symbol_override`, `parse_level_string`, `set_level_indent`, `tildexpand`

### Observed startup sequence

From symbol roles and grouping, the runtime startup order is:

1. **Program name and process-level support setup**
   - `gnu/progname.c:set_program_name`
   - `gnu/getprogname.c:getprogname`
   - `src/main.c:init`
   - `src/main.c:xalloc_die`
   - `gnu/xalloc-die.c:xalloc_die`

   The exact ordering between `set_program_name`, `init`, and allocation-failure hook installation is not derivable from the module summary, but these are all startup responsibilities.

2. **Output subsystem registration**
   - `src/output.c:output_init`
   - `src/output.c:register_output`
   - driver handlers:
     - `src/gnu.c:gnu_output_handler`
     - `src/posix.c:posix_output_handler`
     - `src/dot.c:dot_output_handler`

   This indicates the program starts by preparing an internal registry of output drivers and later selects one by name.

3. **Configuration/default symbol state initialization**
   - `src/symbol.c:set_default_starter`
   - `src/symbol.c:clear_starters`
   - starter/target installation functions exist:
     - `install_starter`
     - `install_target`

4. **Option parser initialization**
   - `src/parseopt/parseopt.c:parseopt_init0`
   - `parseopt_init`
   - `parseopt_parse` or repeated `parseopt_next`

5. **Environment and rc file expansion**
   - `src/main.c:parseopt_from_env`
   - `src/main.c:parseopt_from_rc`
   - `src/main.c:parse_rc`
   - profile handling through:
     - `optset_profile`
     - `fromfile`
     - `optfile_lookup`
     - `optfile_register`

6. **Scanner/parser initialization**
   - `src/c.c:init_tokens`
   - `src/c.c:init_lex`
   - `src/parser.c:init_parse`

7. **Per-input processing**
   - `src/c.c:source`
   - `src/c.c:pp_open`
   - parser loop via `yyparse`, token fetching, declaration/call analysis

8. **Output generation**
   - `src/output.c:output`
   - or `xref_output` depending on mode

9. **Cleanup/finalization**
   - `src/c.c:pp_finalize`
   - parseopt/wordsplit/file cleanup helpers where used
   - `parseopt_free`

### Startup configuration sources

The presence of:
- `parseopt_from_env`
- `parseopt_from_rc`
- `optset_profile`
- `fromfile`

shows that startup is not driven only by CLI arguments. Runtime behavior includes a layered configuration load:
1. built-in defaults,
2. environment options,
3. rc/profile file options,
4. direct command-line options.

The exact precedence rules are not fully reconstructible from the summaries, but this layered startup behavior is directly indicated.

---

## 3.2 Parseopt subsystem startup (`src/parseopt`)

The parseopt subsystem has a clear initialization lifecycle:

1. `parseopt_init0`
2. `prepare_optdef`
3. `collect_optdef`
4. lookup support prepared by:
   - `option_find_short`
   - `option_find_long`
   - `find_short_name`
   - `optidx_slot`

Then runtime parsing proceeds through:
- `parseopt_next_internal`
- wrapped by `parseopt_next`, `parseopt_parse`, or `parseopt_getopt`

This indicates a startup pass that preprocesses option definitions before user arguments are consumed.

### Help/usage startup behavior

Help/version output has dedicated hooks:
- `set_help`
- `set_usage`
- `set_version`
- `parseopt_help_fd`
- `parseopt_usage_fd`
- `parseopt_version_fd`

This means startup can terminate early into a reporting mode rather than continuing into normal source parsing.

The current module summary is insufficient to support a more detailed behavior judgment about whether help/version exits immediately from parseopt or returns control to `main` for explicit termination.

---

## 3.3 Lexical scanner initialization (`src/c.c`)

The scanner is Flex-generated or Flex-derived, with these observable initialization behaviors:

- `yy_init_globals`
- `yy_create_buffer`
- `yy_init_buffer`
- `yy_switch_to_buffer`
- `yy_load_buffer_state`
- `yyrestart`
- `yyensure_buffer_stack`

Program-specific scanner initialization:
- `init_tokens`
- `init_lex`
- `set_preprocessor`
- `pp_option`

### Startup effect

At runtime, scanner startup establishes:
- token tables or token-class maps,
- initial input buffer state,
- preprocessor command/options,
- buffer stack bookkeeping.

The presence of `yy_scan_string`, `yy_scan_bytes`, `yy_scan_buffer`, and push/pop buffer APIs indicates the scanner supports both file-backed and memory-backed input streams, although the main program path is file-oriented.

---

## 3.4 Parser initialization (`src/parser.c`)

Parser startup includes:
- `init_parse`
- stack/token cleanup setup:
  - `cleanup_stack`
  - `clearstack`
- token stream state control:
  - `mark`
  - `restore`

This indicates parser state is mutable and reusable per file or per parse segment.

The parser also has a save-stack subsystem:
- `save_token`
- `save_stack`
- `undo_save_stack`
- `finish_save_stack`
- `save_stack_is_empty`

This supports deferred reconstruction of token text or postponed parse decisions during declarations.

---

## 3.5 Symbol table initialization (`src/symbol.c`)

The symbol subsystem startup behavior includes:
- hash table support via `lookup`, `install`, `hash_symbol_hasher`, `hash_symbol_compare`
- starter/target installation and reset:
  - `install_starter`
  - `set_default_starter`
  - `clear_starters`
  - `install_target`

This shows that symbol graph state begins empty or resettable, then is populated during parsing and filtered again before output.

Because `gnu/hash.c` is present with `hash_initialize`, `hash_lookup`, `hash_insert`, `hash_delete`, `hash_remove`, symbol storage is hash-backed at runtime.

---

## 4. Main user operation flows

## 4.1 Normal end-user flow of the `cflow` executable

The main user-visible flow is:

1. User launches `cflow` with options and source paths.
2. Program initializes global services and option definitions.
3. Program applies environment/rc/profile options.
4. Program parses CLI options and selects mode:
   - output driver,
   - symbol inclusion policy,
   - xref/tree mode,
   - preprocess settings,
   - main/target symbols,
   - indentation formatting.
5. Program opens each source input, optionally through a preprocessor.
6. Scanner tokenizes input.
7. Parser classifies declarations, definitions, references, and calls.
8. Symbol table accumulates graph and metadata.
9. Filtering/selection removes irrelevant nodes or retains only requested roots/targets.
10. Output subsystem emits either:
    - tree output,
    - inverted tree output,
    - xref output,
    - gnu/posix/dot formatted output through selected driver.
11. Program exits.

This is the dominant runtime behavior of the project.

---

## 4.2 Option handling flow

Option processing is not just a decode step; it mutates later runtime behavior.

### Option scan flow

The parseopt path is:

1. `parseopt_init` / `parseopt_init0`
2. For each argument:
   - `parseopt_lookahead`
   - `option_find_short` or `option_find_long`
   - negative-form matching via `negmatch`
   - optional `permute` of argv order
3. Dispatch to opt setters such as:
   - `optset_include_classes`
   - `optset_output_driver`
   - `optset_xref`
   - `optset_symbol`
   - `optset_preproc_option`
   - `optset_preprocess`
   - `optset_level_indent`
   - `optset_main_symbol`
   - `optset_clear_main_symbol`
   - `optset_install_target`
   - `optset_int_1`
   - `optset_prepend_path`
   - `optset_profile`

### Runtime state changes caused by options

These setter functions indicate concrete state transitions:
- **include-class state** changes what symbols survive filtering.
- **output-driver state** changes which handler receives output commands.
- **xref mode** changes output algorithm from call-tree style to reference listing.
- **symbol/main/target state** changes root selection and symbol inclusion.
- **preprocessor option state** changes how input files are opened and scanned.
- **indentation state** changes tree formatting.
- **profile/fromfile state** injects more options into current parse context.

The current module summary is insufficient to support a more detailed behavior judgment on the exact order in which conflicting repeated options overwrite each other.

---

## 4.3 Source parsing flow

### File ingestion

Per file, runtime likely passes through:
- `source(name)` in `src/c.c`
- optional preprocessing open:
  - `pp_open`
  - `pp_close`
  - `pp_finalize`
- scanner restart/buffer switch:
  - `yyrestart`
  - `yy_switch_to_buffer`

### Token acquisition

Token stream management includes:
- `get_token`
- `nexttoken`
- `putback`
- `update_loc`
- lexical helpers like `ident`, `backslash`, `getnum`

This shows token acquisition is not purely single-pass; it supports pushback and location updates.

### Parsing and declaration handling

The parser runtime branches across C syntactic forms:
- `yyparse`
- `is_function`
- `parse_declaration`
- `skip_declaration`
- `expression`
- `parse_function_declaration`
- `parse_variable_declaration`
- `initializer_list`
- `parse_knr_dcl`
- `parse_typedef`
- `parse_dcl`
- `dcl`
- `dirdcl`
- `parmdcl`
- `maybe_parm_list`
- `func_body`
- `get_knr_args`
- `declare`
- `declare_type`

This shows the parser does dynamic classification of identifiers and declarators, not just lexical collection.

### Graph construction during parse

When functions or references are recognized:
- `get_symbol`
- `add_reference`
- `call`
- `reference`

These functions mutate symbol relations during traversal.

The parser therefore acts as the core runtime engine that transforms token input into symbol graph state.

---

## 4.4 Symbol lifecycle flow

The symbol subsystem has a clear dynamic lifecycle:

1. **lookup or creation**
   - `lookup`
   - `install`
   - `install_ident`

2. **storage/class adjustment**
   - `ident_change_storage`
   - `init_ident`

3. **relation attachment**
   - references and calls added from parser
   - linked-list append/prepend behavior suggests per-symbol adjacency lists

4. **collection/filtering**
   - `collect_symbols`
   - `collect_functions`
   - `collect_processor`
   - `collect_list_entry`

5. **starter/target marking**
   - `install_starter`
   - `install_target`
   - `mark_callers`
   - `eliminate_non_targets`

6. **scope cleanup**
   - `delete_autos`
   - `delete_statics`
   - `delete_parms`
   - `move_parms`

7. **unlink/delete**
   - `unlink_symbol`
   - `delete_symbol`
   - `symbol_unlink_from_list`

This shows symbols are not immutable records; they change across parse scope boundaries and later selection phases.

---

## 4.5 Output flow

The output subsystem has two layers:

### Driver-independent orchestration
- `output_init`
- `register_output`
- `select_output_driver`
- `begin`
- `separator`
- `print_text`
- `end`
- `newline`
- `output`
- `xref_output`

### Tree/xref traversal
- `print_level`
- `print_refs`
- `direct_tree`
- `inverted_tree`
- `tree_output`
- `print_function`
- `print_type`
- `symbol_is_function`
- `set_active`
- `clear_active`

### Driver-specific rendering
- GNU style:
  - `print_function_name`
  - `gnu_output_handler`
- POSIX style:
  - `print_symbol_type`
  - `posix_output_handler`
- DOT style:
  - `dot_begin`
  - `declare_node`
  - `dot_print_symbol`
  - `dot_output_handler`

### Runtime behavior

The output runtime appears command-driven:
- drivers receive `cflow_output_command`
- common layer issues begin/end/separator/line-oriented commands
- driver-specific code turns symbol events into text or graph syntax

This architecture implies that the same symbol graph can be emitted through multiple dynamic output backends without reparsing input.

---

## 4.6 Wordsplit-based configuration flow

The wordsplit subsystem supports shell-like splitting for profile/env processing.

### Observed runtime path
1. initialization:
   - `wordsplit_init0`
   - `wordsplit_init`
2. segment/node construction:
   - `alloc_space`
   - `wsnode_new`
   - `wsnode_append`
   - `wordsplit_add_segm`
3. transformation passes:
   - variable expansion (`wordsplit_varexp`, `expvar`)
   - command expansion (`wordsplit_cmdexp`, `expcmd`)
   - tilde expansion (`wordsplit_tildexpand`)
   - path expansion (`wordsplit_pathexpand`)
   - whitespace trim (`wordsplit_trimws`)
   - quote removal/coalescing (`wsnode_quoteremoval`, `wsnode_coalesce`)
4. finalization:
   - `wordsplit_finish`
   - `wordsplit_get_words`
5. cleanup:
   - `wordsplit_free_words`
   - `wordsplit_free_envbuf`
   - `wordsplit_free_parambuf`
   - `wordsplit_free`

This is a multi-pass transformation engine, not a single tokenizer.

---

## 5. State machines and state transitions

## 5.1 Parseopt state machine

The parseopt subsystem behaves like a classic option parser state machine.

### States
- **uninitialized**
- **definition-prepared**
- **scanning options**
- **emitting operand/non-option**
- **help/version terminal mode**
- **finished**

### Transitions
- `parseopt_init0` / `parseopt_init`: uninitialized -> definition-prepared
- `parseopt_next_internal`: definition-prepared/scanning -> scanning
- `permute`: scanning -> scanning with reordered argv state
- `set_help` / `set_usage` / `set_version`: scanning -> terminal reporting mode
- `parseopt_parse` or iteration exhaustion: scanning -> finished
- `parseopt_free`: finished -> released

Negative option matching via `negmatch` indicates an explicit branch in option recognition state.

---

## 5.2 Scanner buffer state machine

The Flex-derived scanner exposes a concrete buffer-state runtime model.

### States
- **no buffer / globals not initialized**
- **buffer created**
- **buffer active**
- **buffer pushed**
- **buffer exhausted**
- **buffer deleted**
- **scanner destroyed**

### Transitions
- `yy_init_globals`: no buffer -> initialized globals
- `yy_create_buffer`: initialized -> buffer created
- `yy_init_buffer`: buffer created -> ready
- `yy_switch_to_buffer`: ready -> active
- `yypush_buffer_state`: active -> pushed stack with new active buffer
- `yypop_buffer_state`: pushed -> previous active buffer restored
- `yy_get_next_buffer`: active -> active/exhausted depending on refill result
- `yyrestart`: active/exhausted -> restarted active buffer
- `yy_delete_buffer`: any live buffer -> deleted
- `yylex_destroy`: scanner state -> destroyed

`yy_scan_string`, `yy_scan_bytes`, and `yy_scan_buffer` create alternate entry points into this state machine for in-memory inputs.

---

## 5.3 Parser token-stack state machine

The parser keeps an explicit token stack and restoration model.

### States
- **empty token stack**
- **token accumulation**
- **marked checkpoint**
- **restored checkpoint**
- **saved-token sequence pending**
- **balanced-scan mode**
- **cleared**

### Transitions
- `tokpush` / `tokins`: empty or accumulation -> accumulation
- `mark`: accumulation -> marked checkpoint
- `restore`: marked checkpoint -> restored checkpoint
- `tokdel`: accumulation -> accumulation with deletion
- `save_token` / `save_stack`: accumulation -> saved-token sequence pending
- `undo_save_stack`: pending -> accumulation with rollback
- `finish_save_stack`: pending -> finalized saved string
- `clearstack` / `cleanup_stack`: any parse stack state -> cleared

This supports nontrivial parse backtracking and declarator reconstruction.

---

## 5.4 Balance-state machine for nested syntax

`src/parser.c` contains explicit balance tracking:
- `push_balance_state`
- `pop_balance_state`
- `free_balance_stack`
- `find_closing_paren`

### Runtime model
The parser enters a **nested delimiter tracking** mode when it encounters constructs that require matching parentheses or similar delimiters.

### States
- **no active balance context**
- **nested context stack active**
- **closing delimiter found**
- **stack released**

### Transitions
- `push_balance_state`: no active/active -> active deeper nesting
- `pop_balance_state`: active -> shallower nesting or none
- `find_closing_paren`: active scan until closing condition or termination
- `free_balance_stack`: active -> released

This balance stack is a specialized state machine used inside parse routines such as declaration and expression handling.

---

## 5.5 Symbol graph state machine

Each symbol participates in a lifecycle with observable state transitions.

### States
- **absent**
- **installed**
- **storage-class assigned**
- **referenced**
- **called / caller-linked**
- **starter/target-marked**
- **scope-expired**
- **filtered-out or retained for output**
- **deleted/unlinked**

### Transitions
- `lookup` miss + `install`: absent -> installed
- `init_ident` / `ident_change_storage`: installed -> storage-class assigned
- `reference` / `add_reference`: installed -> referenced
- `call`: referenced/installed -> called/caller-linked
- `install_starter` / `install_target`: installed -> marked
- `delete_autos` / `delete_statics` / `delete_parms`: active -> scope-expired
- `mark_callers`: retained graph expansion upstream
- `eliminate_non_targets`: mixed graph -> filtered retained subset
- `unlink_symbol` / `delete_symbol`: any live state -> deleted

This state machine is central to preserving semantic behavior.

---

## 5.6 Output-driver selection state machine

The output layer behaves as a registered-driver finite state process.

### States
- **no drivers registered**
- **drivers registered**
- **driver selected**
- **output session active**
- **output session complete**

### Transitions
- `output_init` + `register_output`: no drivers -> drivers registered
- `select_output_driver`: drivers registered -> driver selected
- `begin`: driver selected -> session active
- repeated render commands: session active -> session active
- `end`: session active -> session complete

The current module summary is insufficient to support a more detailed behavior judgment about recovery if a requested output driver name is not registered.

---

## 5.7 Wordsplit transformation state machine

The wordsplit engine is explicitly multi-stage.

### States
- **uninitialized**
- **input loaded**
- **node list constructed**
- **expansion in progress**
- **quote/whitespace normalization**
- **finished words available**
- **error state**
- **freed**

### Transitions
- `wordsplit_init0` -> uninitialized baseline
- `wordsplit_init` -> input loaded
- scanning/node creation (`scan_word`, `scan_qstring`, `wsnode_new`, `wordsplit_add_segm`) -> node list constructed
- `wordsplit_process_list` -> expansion in progress
- `wordsplit_varexp`, `wordsplit_cmdexp`, `wordsplit_tildexpand`, `wordsplit_pathexpand` keep engine in transformation state
- `wordsplit_trimws`, `wsnode_quoteremoval`, `wsnode_coalesce` -> normalization
- `wordsplit_finish` -> finished words available
- `_wsplt_seterr`, `_wsplt_setctxerr`, `_wsplt_nomem` -> error state
- `wordsplit_free` -> freed