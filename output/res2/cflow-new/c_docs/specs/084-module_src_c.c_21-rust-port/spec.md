# spec.md

## Title

Functional Specification for `module_src_c.c_21` Rust Rewrite

## Metadata

- Project: `cflow-new`
- Module: `module_src_c.c_21`
- Category: `module_cluster`
- Source file: `src/c.c`
- Rust branch: `084-module_src_c.c_21-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides the externally visible control and support surface around the C lexical scanner implemented in `src/c.c`. The evidenced responsibilities are:

- exposing scanner state accessors and mutators for current token text, line number, input stream, output stream, and debug flag;
- initializing lexical/token-related state;
- classifying identifier-like input through `ident`;
- configuring preprocessor command and options;
- releasing scanner-owned resources;
- providing scanner memory allocation helpers.

The Rust rewrite must preserve this functional boundary. It must implement equivalent observable behavior for scanner control, initialization, identifier/token support, preprocessor configuration, and teardown, without adding unsupported capabilities.

## Feature Specification

### Scanner State Access and Control

The module must provide behavior equivalent to the scanner accessors and mutators evidenced in `src/c.c`:

- retrieve the current scanner text (`yyget_text`);
- set the current scanner line number (`yyset_lineno`);
- set scanner input stream/source (`yyset_in`);
- set scanner output stream/sink (`yyset_out`);
- retrieve scanner debug state (`yyget_debug`);
- set scanner debug state (`yyset_debug`).

These operations form the control surface used by surrounding code to inspect and alter scanner runtime state.

### Scanner Lifecycle Support

The module must support scanner lifecycle cleanup through behavior equivalent to `yylex_destroy`. Cleanup must release scanner-associated resources and leave the scanner in a destroyed/reset state consistent with the original module’s teardown role.

### Scanner Memory Support

The module must provide scanner memory management behavior corresponding to:

- allocation (`yyalloc`);
- reallocation (`yyrealloc`);
- deallocation (`yyfree`).

The Rust rewrite does not need to mimic C allocation primitives literally, but it must preserve the module’s functional role of supplying memory operations required by scanner resource management.

### Token and Lexical Initialization

The module must support explicit initialization of token/lexical state through:

- `init_tokens`;
- `init_lex(int debug_level)`.

The rewrite must preserve the ability to initialize token-related data before lexical use, and to initialize lexical operation with a caller-provided debug level.

### Identifier Handling

The module must provide the behavior exposed by `ident()`. Based on the available evidence, this function participates in lexical classification/handling of identifier-like input and must remain available with equivalent externally observable results.

### Preprocessor Configuration

The module must support configuring preprocessing behavior through:

- `set_preprocessor(const char *arg)`;
- `pp_option(int opt, const char *arg)`.

The Rust rewrite must preserve the ability to set the preprocessor command/value and to apply option/value pairs affecting preprocessing configuration, limited to the functionality evidenced by these functions.

## User Scenarios & Testing

### Scenario 1: Initialize lexical support before scanning

A caller initializes token state and then initializes lexical behavior with a chosen debug level.

Expected support:
- token initialization completes without requiring prior scanner activity;
- lexical initialization accepts a debug level;
- after initialization, scanner debug state reflects the configured level where applicable.

Traceability:
- `init_tokens`
- `init_lex`
- `yyget_debug`
- `yyset_debug`

### Scenario 2: Configure scanner runtime controls

A caller updates scanner runtime settings before or during lexical processing:
- sets the current input source;
- sets the output sink;
- adjusts the current line number;
- enables or disables debug behavior.

Expected support:
- each setting operation changes subsequent scanner state consistently;
- querying debug state returns the current configured value;
- current text retrieval remains available as scanner text is produced.

Traceability:
- `yyset_in`
- `yyset_out`
- `yyset_lineno`
- `yyset_debug`
- `yyget_debug`
- `yyget_text`

### Scenario 3: Read current token text during lexical processing

During scanning, a caller asks for the current matched text.

Expected support:
- the returned text corresponds to the scanner’s current token/match state;
- access is read-oriented and reflects current scanner progress.

Traceability:
- `yyget_text`

### Scenario 4: Classify or process an identifier token

During lexical analysis, identifier handling is invoked.

Expected support:
- identifier-related processing/classification is performed through the module entry point;
- the function returns a result meaningful to the surrounding lexer/parser flow, consistent with the original module behavior.

Traceability:
- `ident`

### Scenario 5: Configure preprocessor command and options

Before scanning input that depends on preprocessing, a caller:
- sets the preprocessor program/value;
- applies one or more options with associated arguments.

Expected support:
- preprocessor configuration state is updated by these calls;
- later lexical/preprocessing stages observe the configured values.

Traceability:
- `set_preprocessor`
- `pp_option`

### Scenario 6: Destroy scanner state after use

After lexical work is complete, a caller destroys scanner state.

Expected support:
- scanner-owned resources are released;
- repeated use requires reinitialization rather than assuming prior state remains active;
- destroy reports completion status consistent with the original function contract.

Traceability:
- `yylex_destroy`

### Scenario 7: Scanner-managed memory is allocated and released

Internal scanner operations require dynamic storage that may grow or be freed.

Expected support:
- allocation returns usable storage for requested size;
- reallocation preserves the role of resizing previously allocated storage;
- free releases previously allocated storage.

Traceability:
- `yyalloc`
- `yyrealloc`
- `yyfree`

## Requirements

### Functional Requirements

#### FR-1: Scanner text access
The module shall provide access to the scanner’s current text buffer/content through behavior equivalent to `yyget_text`.

Traceability:
- `yyget_text` in `src/c.c:2576-2579`

#### FR-2: Scanner line number control
The module shall allow the scanner’s current line number to be set through behavior equivalent to `yyset_lineno`.

Traceability:
- `yyset_lineno` in `src/c.c:2588-2592`

#### FR-3: Scanner input source control
The module shall allow the scanner input source to be set through behavior equivalent to `yyset_in`.

Traceability:
- `yyset_in` in `src/c.c:2600-2603`

#### FR-4: Scanner output sink control
The module shall allow the scanner output sink to be set through behavior equivalent to `yyset_out`.

Traceability:
- `yyset_out` in `src/c.c:2605-2608`

#### FR-5: Scanner debug state query and update
The module shall support querying and updating scanner debug state through behavior equivalent to `yyget_debug` and `yyset_debug`.

Traceability:
- `yyget_debug` in `src/c.c:2610-2613`
- `yyset_debug` in `src/c.c:2615-2618`

#### FR-6: Scanner destruction
The module shall provide a scanner destruction/cleanup operation equivalent to `yylex_destroy` that releases scanner-associated runtime state.

Traceability:
- `yylex_destroy` in `src/c.c:2659-2680`

#### FR-7: Scanner memory operations
The module shall provide memory allocation support for scanner operations through behavior equivalent to `yyalloc`, `yyrealloc`, and `yyfree`.

Traceability:
- `yyalloc` in `src/c.c:2708-2711`
- `yyrealloc` in `src/c.c:2713-2724`
- `yyfree` in `src/c.c:2726-2729`

#### FR-8: Token initialization
The module shall provide token-related initialization through behavior equivalent to `init_tokens`.

Traceability:
- `init_tokens` in `src/c.c:2777-2812`

#### FR-9: Lexical initialization
The module shall provide lexical initialization with caller-supplied debug level through behavior equivalent to `init_lex(int debug_level)`.

Traceability:
- `init_lex` in `src/c.c:2814-2820`

#### FR-10: Identifier handling
The module shall provide identifier-processing behavior through `ident`, preserving its role in lexical/token classification flow.

Traceability:
- `ident` in `src/c.c:2822-2844`

#### FR-11: Preprocessor command configuration
The module shall allow preprocessor command/value configuration through behavior equivalent to `set_preprocessor`.

Traceability:
- `set_preprocessor` in `src/c.c:2852-2856`

#### FR-12: Preprocessor option configuration
The module shall allow preprocessor option updates using an option selector and associated argument through behavior equivalent to `pp_option`.

Traceability:
- `pp_option` in `src/c.c:2858-2871`

### Key Entities

#### Scanner state
The module operates on scanner runtime state that includes current text, line number, debug flag, input source, output sink, and destroyable runtime resources.

Traceability:
- `yyget_text`
- `yyset_lineno`
- `yyset_in`
- `yyset_out`
- `yyget_debug`
- `yyset_debug`
- `yylex_destroy`

#### Buffer state
`struct yy_buffer_state` appears repeatedly and represents scanner buffer-related state used by the lexer runtime. The Rust rewrite must preserve the functional concept of scanner buffer state as part of scanner operation and teardown, even if represented differently.

Traceability:
- `struct yy_buffer_state` references in `src/c.c`

#### Transition/state-machine support
`struct yy_trans_info` indicates scanner transition metadata used by the generated lexer. The rewrite must preserve equivalent lexical transition behavior as needed to support the public scanner functions in this module.

Traceability:
- `struct yy_trans_info` in `src/c.c:440-444`

#### Token/identifier support state
`init_tokens` and `ident` imply token tables or classification state used for lexical/token recognition. The Rust rewrite must preserve these relationships sufficiently for initialization and identifier handling to behave equivalently.

Traceability:
- `init_tokens`
- `ident`

#### Preprocessor configuration state
Preprocessor settings consist of a selected preprocessor command/value and option/value updates supplied through configuration functions.

Traceability:
- `set_preprocessor`
- `pp_option`

#### Dynamic storage support
`struct obstack` and the allocation helpers indicate dynamic storage used by scanner or token/preprocessor support. The Rust rewrite must preserve the functional availability of dynamic storage for these operations.

Traceability:
- `struct obstack` references in `src/c.c`
- `yyalloc`
- `yyrealloc`
- `yyfree`

## Success Criteria

### SC-1: Initialization behavior is preserved
A test that calls token initialization and lexical initialization with a chosen debug level observes successful setup, and subsequent debug-state query reflects the configured state where the original module does.

Traceability:
- `init_tokens`
- `init_lex`
- `yyget_debug`

### SC-2: Scanner control setters are observable
A test that sets line number, input source, output sink, and debug state can observe that the scanner accepts these updates without loss of subsequent scanner operability.

Traceability:
- `yyset_lineno`
- `yyset_in`
- `yyset_out`
- `yyset_debug`

### SC-3: Current scanner text remains accessible
A test exercising lexical progress can retrieve current scanner text through the Rust equivalent of `yyget_text`, and the returned value corresponds to the active token/match state.

Traceability:
- `yyget_text`

### SC-4: Identifier handling remains functional
A test invoking identifier handling through `ident` on representative identifier input receives results consistent with the original module’s lexical classification role.

Traceability:
- `ident`

### SC-5: Preprocessor configuration is retained
A test that sets a preprocessor value and applies options can observe that later module behavior uses the configured settings rather than defaults.

Traceability:
- `set_preprocessor`
- `pp_option`

### SC-6: Destruction releases scanner runtime state
A test that initializes and uses scanner state, then calls destroy, observes successful completion and no continued dependence on pre-destroy runtime state without reinitialization.

Traceability:
- `yylex_destroy`

### SC-7: Memory helper behavior remains correct
A test covering allocate, reallocate, and free operations confirms that requested storage can be obtained, resized, and released in support of scanner operations.

Traceability:
- `yyalloc`
- `yyrealloc`
- `yyfree`

### SC-8: Functional boundary is preserved without expansion
The Rust rewrite exposes only the evidenced functionality in this specification for this module boundary and does not require unsupported capabilities to satisfy the above scenarios.

Traceability:
- all listed module functions and data structures in `src/c.c`