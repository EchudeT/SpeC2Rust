# spec.md

## Title

Functional Specification for `module_src_c.c_21` Rust Rewrite

## Document Metadata

- Project: `cflow-new`
- Module: `module_src_c.c_21`
- Category: `module_cluster`
- Source file: `src/c.c`
- Target branch: `084-module_src_c.c_21-rust-port`
- Generation date: 2026-06-17

## Overview

This module provides the externally visible control surface around a generated C lexical scanner plus project-specific lexer initialization and preprocessor option handling.

The evidenced functionality falls into four areas:

1. Accessors and mutators for scanner state, including current token text, line number, debug flag, and input/output streams.
2. Scanner lifecycle and memory-management helpers used by the generated lexer runtime.
3. Project-specific lexer setup, including token-table initialization and debug-aware lexer initialization.
4. Preprocessor configuration entry points, including setting the preprocessor program and adding preprocessor options.

The Rust rewrite must preserve the functional behavior exposed by these entry points and the relationships among scanner state, token initialization, and preprocessor configuration.

## Scope

### In Scope

- Behavior represented by the functions:
  - `yyget_text`
  - `yyset_lineno`
  - `yyset_in`
  - `yyset_out`
  - `yyget_debug`
  - `yyset_debug`
  - `yylex_destroy`
  - `yyalloc`
  - `yyrealloc`
  - `yyfree`
  - `init_tokens`
  - `init_lex`
  - `ident`
  - `set_preprocessor`
  - `pp_option`
- Functional use of scanner buffer state as represented by `struct yy_buffer_state`
- Functional use of token/preprocessor storage involving `struct obstack`

### Out of Scope

- Reproducing C implementation details of generated scanner internals unless needed to preserve observable behavior
- Introducing new public APIs not evidenced in the source analysis
- New concurrency, persistence, recovery, or serialization behavior
- Performance goals beyond preserving functional correctness

## Feature Specification

### 1. Scanner State Access and Control

The module exposes operations to inspect and update scanner state used by lexical analysis.

The Rust version must implement behavior equivalent to:

- returning the current matched token text from the scanner state (`yyget_text`)
- updating the scanner's current line number (`yyset_lineno`)
- setting the scanner input stream/source handle (`yyset_in`)
- setting the scanner output stream/sink handle (`yyset_out`)
- reading the current scanner debug flag (`yyget_debug`)
- updating the scanner debug flag (`yyset_debug`)

These operations form the externally controllable state of the lexer and must act on the active scanner instance used by the module.

### 2. Scanner Lifecycle Management

The module provides destruction/reset behavior for the scanner runtime through `yylex_destroy`.

The Rust rewrite must provide equivalent lifecycle cleanup for scanner-owned runtime state so that:

- scanner resources can be released after use
- repeated setup/use/cleanup cycles are supported without stale scanner state remaining active
- cleanup returns a status result equivalent in meaning to the C function's success/failure contract

### 3. Lexer Runtime Memory Hooks

The module includes allocation helpers used by the scanner runtime:

- `yyalloc`
- `yyrealloc`
- `yyfree`

The Rust rewrite must preserve the functional role of these helpers as the scanner runtime's memory management boundary. If these are not exposed directly in Rust, their behavior must still be represented internally so scanner state can be created, resized, and released consistently with the source module's needs.

### 4. Token Initialization and Lexer Initialization

The module provides project-specific setup functions:

- `init_tokens`
- `init_lex`

The Rust version must preserve the distinction between:

- token-related initialization performed by `init_tokens`
- lexer startup/configuration performed by `init_lex`, including use of a debug-level input

`init_lex` must initialize lexer operation in a way that is compatible with subsequent scanner use and scanner debug state.

### 5. Identifier-Related Lexical Operation

The module exposes `ident`, which performs an identifier-related lexer function.

Because the source analysis only evidences the function name and signature, the Rust rewrite must preserve its role as a callable lexer operation returning an integer result, and must keep it behaviorally compatible with the module's token/lexer setup. No broader semantics beyond that should be invented.

### 6. Preprocessor Program and Option Configuration

The module provides configuration entry points for preprocessing:

- `set_preprocessor(const char *arg)`
- `pp_option(int opt, const char *arg)`

The Rust version must support:

- setting the preprocessor command/program reference used by the module
- recording or applying preprocessor options identified by an integer option code and associated argument text

This configuration must remain available to the rest of the lexer/preprocessing flow represented by the module.

## User Scenarios & Testing

### Scenario 1: Initialize lexer with debugging disabled

A caller initializes lexer state for normal operation by invoking token initialization and lexer initialization with a non-debug or zero debug level.

The Rust version must support verifying that:

- token initialization can be invoked before lexer use
- lexer initialization accepts a debug-level argument
- scanner debug state after initialization is consistent with the provided level or later explicit debug setting

### Scenario 2: Enable and inspect scanner debug mode

A caller enables scanner debugging through the module and later queries whether debugging is enabled.

The Rust version must support verifying that:

- calling the debug setter changes scanner debug state
- calling the debug getter returns the current debug state
- repeated set/get sequences are stable and reflect the latest value

### Scenario 3: Update scanner line tracking during lexing setup

A caller sets the current line number before or during lexical processing.

The Rust version must support verifying that:

- the line-setting entry point accepts integer line numbers
- the active scanner state reflects the updated line value for subsequent scanner behavior dependent on line tracking

### Scenario 4: Replace scanner input and output handles

A caller redirects lexer input and output by setting new stream/source and sink handles.

The Rust version must support verifying that:

- input source configuration can be changed through the input setter
- output sink configuration can be changed through the output setter
- the active scanner uses the updated handles for later operation

### Scenario 5: Read current token text

During or immediately after token recognition, a caller retrieves the current token text from the scanner.

The Rust version must support verifying that:

- token text can be retrieved from active scanner state
- the returned text corresponds to the scanner's current match at the time of the call

### Scenario 6: Configure preprocessing command and options

Before processing source input, a caller chooses a preprocessor program and adds options.

The Rust version must support verifying that:

- the preprocessor program reference can be set
- multiple option-setting calls can be made using option code plus argument
- later module operation can access the configured preprocessor state

### Scenario 7: Perform scanner cleanup and reuse

A caller uses the lexer, destroys scanner state, then initializes again for another run.

The Rust version must support verifying that:

- cleanup can be invoked after scanner use
- cleanup releases active scanner runtime state
- a later initialization/use cycle works without depending on stale state from the prior cycle

## Requirements

### Functional Requirements

#### FR-1: Scanner token text access
The module shall provide access to the current matched token text of the active scanner state, corresponding to `yyget_text` in `src/c.c`.

#### FR-2: Scanner line number control
The module shall allow the current scanner line number to be set, corresponding to `yyset_lineno` in `src/c.c`.

#### FR-3: Scanner input source control
The module shall allow the active scanner input source/stream to be replaced, corresponding to `yyset_in` in `src/c.c`.

#### FR-4: Scanner output sink control
The module shall allow the active scanner output sink/stream to be replaced, corresponding to `yyset_out` in `src/c.c`.

#### FR-5: Scanner debug state query
The module shall expose the current scanner debug flag, corresponding to `yyget_debug` in `src/c.c`.

#### FR-6: Scanner debug state update
The module shall allow the scanner debug flag to be updated, corresponding to `yyset_debug` in `src/c.c`.

#### FR-7: Scanner runtime destruction
The module shall provide a scanner-destruction operation that releases scanner runtime state and returns a status result, corresponding to `yylex_destroy` in `src/c.c`.

#### FR-8: Scanner runtime allocation support
The module shall support creation, resizing, and release of scanner runtime storage as required by the lexer runtime, corresponding to `yyalloc`, `yyrealloc`, and `yyfree` in `src/c.c`.

#### FR-9: Token subsystem initialization
The module shall provide token-related initialization prior to lexer use, corresponding to `init_tokens` in `src/c.c`.

#### FR-10: Lexer initialization with debug-level input
The module shall provide lexer initialization that accepts a debug-level parameter, corresponding to `init_lex` in `src/c.c`.

#### FR-11: Identifier-related lexer operation
The module shall provide the `ident` lexer operation as a callable function returning an integer result and compatible with the initialized lexer state, corresponding to `ident` in `src/c.c`.

#### FR-12: Preprocessor program selection
The module shall provide configuration of the preprocessor program/command from string input, corresponding to `set_preprocessor` in `src/c.c`.

#### FR-13: Preprocessor option registration
The module shall provide configuration of preprocessor options using an integer option selector and associated string argument, corresponding to `pp_option` in `src/c.c`.

### Key Entities

#### Scanner State
Represented by `struct yy_buffer_state` in `src/c.c`.

This entity holds the active lexer runtime state needed for input processing and scanner control. The functions for token text access, line-number setting, stream redirection, debug control, and destruction operate on or against this active scanner state.

#### Scanner Transition Metadata
Represented by `struct yy_trans_info` in `src/c.c`.

This entity represents lexer transition data used by the generated scanner runtime. The Rust rewrite must preserve its functional role insofar as it is required for equivalent lexical behavior, but it need not preserve C layout unless observable behavior depends on it.

#### Token/Option Storage
Represented in the analyzed source by `struct obstack` usage in `src/c.c`.

This entity supports dynamic storage associated with token initialization and/or preprocessor configuration. The Rust rewrite must preserve the functional capacity to accumulate and retain the data needed by `init_tokens`, `set_preprocessor`, and `pp_option`.

#### External Referenced Type
A referenced type name `foo` appears in the analysis without local definition.

Because no local structure or behavior is evidenced for this type within the module boundary provided here, the Rust rewrite shall only preserve any required interface compatibility that is directly necessary for the module's evidenced functions. No additional semantics are specified.

## Success Criteria

1. A Rust implementation provides equivalents for all evidenced module entry points listed in this specification, with behavior traceable to `src/c.c`.
2. After debug state is set through the module, querying debug state returns the most recently applied value.
3. After line number is set through the module, subsequent scanner state reflects the updated line value for lexer operation.
4. After input or output handles are changed through the module, later scanner activity uses the updated handle assignments.
5. During active token recognition, the module can return current token text equivalent in role to `yyget_text`.
6. Token initialization and lexer initialization can be invoked in a valid setup sequence prior to lexer use.
7. Preprocessor program configuration and option configuration are both accepted and retained for later module use.
8. Scanner destruction can be invoked after use, returns a status result, and leaves the module able to undergo a fresh initialization/use cycle.
9. Internal runtime storage required for scanner operation can be created, resized, and released without breaking the observable behaviors above.

## Traceability

- `yyget_text` → FR-1, Success Criteria 5
- `yyset_lineno` → FR-2, Success Criteria 3
- `yyset_in` → FR-3, Success Criteria 4
- `yyset_out` → FR-4, Success Criteria 4
- `yyget_debug` → FR-5, Success Criteria 2
- `yyset_debug` → FR-6, Success Criteria 2
- `yylex_destroy` → FR-7, Success Criteria 8
- `yyalloc` / `yyrealloc` / `yyfree` → FR-8, Success Criteria 9
- `init_tokens` → FR-9, Success Criteria 6
- `init_lex` → FR-10, Success Criteria 6
- `ident` → FR-11
- `set_preprocessor` → FR-12, Success Criteria 7
- `pp_option` → FR-13, Success Criteria 7
- `struct yy_buffer_state` → Scanner State entity
- `struct yy_trans_info` → Scanner Transition Metadata entity
- `struct obstack` → Token/Option Storage entity