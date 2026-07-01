# spec.md

## Title

Rust Functional Specification for `module_src_c.c_21`

## Metadata

- Project: `cflow-new`
- Module: `module_src_c.c_21`
- Category: `module_cluster`
- Source file(s): `src/c.c`
- Target branch: `084-module_src_c.c_21-rust-port`
- Generation date: `2026-06-11`

## Overview

This module provides lexer runtime control and initialization support for the C-language scanning subsystem in `src/c.c`. The evidenced functionality covers:

- querying and updating lexer runtime state exposed through `yy*` accessors and mutators,
- destroying lexer-owned runtime state,
- memory allocation wrappers used by the lexer runtime,
- initialization of token-related and lexical-analysis state,
- recognition of identifier tokens through a dedicated helper,
- configuration of an external preprocessor command and its options.

The Rust rewrite must preserve the same functional boundaries and observable behavior implied by these entry points. The specification does not require unrelated parser behavior, undocumented scanner features, or any new API beyond what is evidenced here.

## Feature Specification

### 1. Lexer runtime state access and mutation

The module exposes accessors and mutators for scanner state associated with the active lexer runtime.

The Rust version must implement functionality equivalent to:

- retrieving the current scanner text buffer content through the `yyget_text` behavior,
- setting the current scanner line number through the `yyset_lineno` behavior,
- assigning the scanner input stream through the `yyset_in` behavior,
- assigning the scanner output stream through the `yyset_out` behavior,
- reading and updating the scanner debug flag through `yyget_debug` and `yyset_debug`.

These operations must affect the active lexer context consistently so that later scanning-related operations observe the updated state.

### 2. Lexer runtime teardown

The module supports explicit destruction of lexer runtime resources through `yylex_destroy`.

The Rust version must provide equivalent teardown behavior for lexer-owned state, including cleanup of scanner buffers and reset of runtime state needed to allow clean shutdown of the current lexer session.

### 3. Lexer memory wrapper functions

The module provides allocator wrappers through `yyalloc`, `yyrealloc`, and `yyfree`.

The Rust version must preserve the functional role of these operations inside the rewritten module:

- allocate memory for lexer runtime needs,
- resize previously allocated lexer runtime storage,
- release previously allocated lexer runtime storage.

The rewrite may use Rust-managed memory internally, but module behavior must still support the allocation, growth, and release semantics required by the scanner runtime represented by these functions.

### 4. Token and lexer initialization

The module includes initialization entry points:

- `init_tokens`
- `init_lex(int debug_level)`

The Rust version must implement initialization behavior that prepares token-related state before lexical analysis and initializes lexer state, including application of the supplied debug level.

### 5. Identifier recognition support

The module contains an `ident` function that performs identifier-related lexical handling.

The Rust version must preserve this identifier-recognition functionality as part of the scanning subsystem so that identifier tokens are recognized and handled consistently with the original module’s role.

### 6. Preprocessor command configuration

The module allows configuration of an external preprocessor invocation through:

- `set_preprocessor(const char *arg)`
- `pp_option(int opt, const char *arg)`

The Rust version must support:

- setting or replacing the configured preprocessor program/command string,
- recording preprocessor options identified by an option code and argument.

This requirement is limited to configuration behavior evidenced by these functions. The specification does not require execution semantics beyond what is necessary for the module to retain and use configured preprocessor settings within the lexer-related workflow.

## User Scenarios & Testing

### Scenario 1: Initialize lexical analysis with a selected debug level

A caller starts a lexical-analysis session and initializes module state before scanning.

Expected support:

1. The caller invokes token initialization.
2. The caller invokes lexer initialization with a debug level.
3. Subsequent scanner state reflects the requested debug setting.

Test expectations:

- initialization completes without requiring unrelated module setup,
- the debug level set during initialization is observable through the debug accessor or equivalent internal behavior.

### Scenario 2: Rebind scanner input and output streams

A caller changes the source stream being scanned and the destination stream used by the lexer runtime.

Expected support:

1. The caller assigns a new input stream.
2. The caller assigns a new output stream.
3. The active scanner context uses those streams for subsequent lexer activity.

Test expectations:

- setting input and output succeeds for a valid active lexer context,
- later scanner operations observe the updated stream bindings.

### Scenario 3: Track line numbering during scanning

A caller needs the scanner to continue from a known source line number.

Expected support:

1. The caller sets the current line number explicitly.
2. Identifier or other token scanning continues from that line basis.

Test expectations:

- the configured line number is stored in scanner state,
- subsequent lexer behavior uses the updated line number rather than stale state.

### Scenario 4: Inspect current token text

After scanning activity has produced or selected current text, a caller retrieves the active lexeme text.

Expected support:

1. Scanner activity establishes current token text.
2. The caller invokes the text accessor.
3. The returned text corresponds to the scanner’s current token text.

Test expectations:

- returned text matches current scanner state,
- retrieval does not itself destroy or invalidate scanner state unexpectedly.

### Scenario 5: Recognize an identifier

A caller or scanner path invokes identifier-specific lexical handling.

Expected support:

1. Source input contains a valid identifier sequence.
2. The identifier helper processes the sequence.
3. The function returns an integer result consistent with lexer token handling.

Test expectations:

- valid identifier text is accepted by the identifier path,
- the result integrates with token-based lexer flow.

### Scenario 6: Configure preprocessing

A caller customizes how source preprocessing should be configured before lexical analysis of preprocessed input.

Expected support:

1. The caller sets a preprocessor command.
2. The caller adds one or more options with option code and argument.
3. The configured preprocessing state remains available to the module’s lexical workflow.

Test expectations:

- command configuration is replaceable,
- options are recorded in call order sufficient for later use by the module,
- null or empty handling is consistent with the original function contract as exercised by existing callers.

### Scenario 7: Destroy lexer state after use

A caller finishes with the scanner and explicitly destroys lexer state.

Expected support:

1. The caller invokes lexer destruction.
2. Scanner-owned resources are released.
3. The lexer is left in a non-leaking, reset state.

Test expectations:

- destruction returns the module’s defined success/failure code,
- repeated use-after-destroy is not required unless evidenced by existing module flow,
- resource-owning scanner state is cleaned up once destruction completes.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide access to the current scanner text for the active lexer runtime, corresponding to `yyget_text` in `src/c.c`.
- **FR-2**: The module shall allow the active scanner line number to be updated explicitly, corresponding to `yyset_lineno` in `src/c.c`.
- **FR-3**: The module shall allow the active scanner input stream to be replaced, corresponding to `yyset_in` in `src/c.c`.
- **FR-4**: The module shall allow the active scanner output stream to be replaced, corresponding to `yyset_out` in `src/c.c`.
- **FR-5**: The module shall expose the current scanner debug setting and allow that setting to be updated, corresponding to `yyget_debug` and `yyset_debug` in `src/c.c`.
- **FR-6**: The module shall support explicit destruction of lexer runtime resources and state, corresponding to `yylex_destroy` in `src/c.c`.
- **FR-7**: The module shall provide allocation, reallocation, and free behavior for lexer runtime storage needs, corresponding to `yyalloc`, `yyrealloc`, and `yyfree` in `src/c.c`.
- **FR-8**: The module shall initialize token-related state needed by the lexical subsystem, corresponding to `init_tokens` in `src/c.c`.
- **FR-9**: The module shall initialize lexer state with a caller-supplied debug level, corresponding to `init_lex` in `src/c.c`.
- **FR-10**: The module shall provide identifier-processing behavior as part of lexical analysis, corresponding to `ident` in `src/c.c`.
- **FR-11**: The module shall support setting the configured preprocessor command string, corresponding to `set_preprocessor` in `src/c.c`.
- **FR-12**: The module shall support recording preprocessor options as option-code and argument pairs, corresponding to `pp_option` in `src/c.c`.

### Key Entities

- **Lexer runtime state**: The active scanner state represented by the `yy_buffer_state`-related structures in `src/c.c`. This entity holds or mediates current scanner text, line number, stream bindings, and destruction-relevant runtime state.
- **Scanner transition/runtime support data**: The lexer runtime also references transition information represented by `yy_trans_info`, supporting token scanning behavior within the scanner state machine.
- **Token/preprocessor accumulation storage**: The module references `obstack` and uses it in proximity to token/preprocessor initialization and configuration code. This entity represents expandable storage used by module-managed lexical or preprocessing configuration state.
- **Preprocessor configuration state**: Logical module state configured by `set_preprocessor` and `pp_option`, consisting of the selected preprocessor command and associated option entries.
- **Identifier token handling path**: Logical lexical-processing path implemented by `ident`, operating within the initialized lexer/token state and returning an integer token-related result.

Relationships:

- The lexer runtime state is initialized by `init_lex`, queried and mutated by the `yyget_*`/`yyset_*` functions, and released by `yylex_destroy`.
- Token-related state is established by `init_tokens` and used by identifier handling and broader lexical processing.
- Preprocessor configuration state is built by `set_preprocessor` and `pp_option` and is retained for use by the module’s lexical workflow.
- Allocation wrapper behavior supports storage needs of scanner runtime and associated module state.

## Success Criteria

- **SC-1**: A Rust test corresponding to `init_lex` demonstrates that supplying a debug level causes subsequent debug-state retrieval to report that level.
- **SC-2**: A Rust test corresponding to `yyset_lineno` demonstrates that setting the current line number updates the active lexer state used by later scanning behavior.
- **SC-3**: A Rust test corresponding to `yyset_in` and `yyset_out` demonstrates that input and output bindings on the active lexer runtime can be replaced without breaking subsequent lexer operation.
- **SC-4**: A Rust test corresponding to `yyget_text` demonstrates retrieval of the current lexeme text after scanner activity has established token text.
- **SC-5**: A Rust test corresponding to `ident` demonstrates successful handling of at least one valid identifier input and returns an integer token/result compatible with the module’s lexical flow.
- **SC-6**: A Rust test corresponding to `set_preprocessor` and `pp_option` demonstrates that a preprocessor command and one or more option entries are retained accurately for later module use.
- **SC-7**: A Rust test corresponding to `yylex_destroy` demonstrates that lexer-owned runtime state is cleaned up and the destroy operation completes with the expected return status.
- **SC-8**: Internal module validation or tests corresponding to `yyalloc`, `yyrealloc`, and `yyfree` demonstrate that lexer runtime storage can be allocated, resized, and released in support of scanner state management.
- **SC-9**: The Rust module exposes no required functional regressions relative to the listed source functions in `src/c.c`; each function-mapped requirement FR-1 through FR-12 is implemented or represented by equivalent Rust behavior within the module boundary.