# spec.md

## Title

Rust Port Functional Specification: `main_root` Module

## Status

Draft

## Scope

This specification defines the functional behavior to preserve when rewriting the `main_root` module from `yank.c` into Rust for branch `001-main_root-rust-port`.

The module is the program entry module for the `yank` executable. It is responsible for:

- parsing command-line options,
- acquiring candidate text fields from standard input,
- optionally transforming the input into a field list,
- presenting an interactive terminal selection interface,
- writing the selected result to standard output,
- optionally copying the selected result to the X clipboard,
- managing terminal setup and restoration,
- reporting invalid usage.

This specification covers only behavior evidenced by the analyzed module.

## Feature Specification

### Overview

The Rust version must implement the same end-user behavior as the original main module: read newline-delimited input records, derive selectable fields, allow terminal-based selection of one field, and emit the selected field. Depending on invocation options, the module must either:

- operate in a direct/output-only mode, or
- enter an interactive terminal mode for selection.

The module must also support optional pattern conversion for matching input content and optional clipboard output through the X11 helper path already evidenced by the C module.

### Supported Functional Behavior

#### 1. Command-line driven execution

The module must start from a `main` entry point and interpret command-line arguments to configure runtime behavior. Invalid invocation must produce a usage message and terminate without normal execution.

This behavior is evidenced by:
- `main`
- `usage`

#### 2. Input acquisition from standard input

The module must read text input from standard input and treat it as a sequence of candidate items. The input path must support building the internal list of selectable fields used by later processing.

This behavior is evidenced by:
- `input`
- `struct field`
- `main`

#### 3. Pattern preparation from string input

When the program is invoked with a textual pattern/filter expression, the module must convert that string into the internal pattern form expected by the matching/selection flow.

This behavior is evidenced by:
- `strtopat`
- `main`

The Rust port must preserve the conversion role of this step, even if internal representation differs.

#### 4. Field ordering/comparison support

The module must support comparison of field records for ordering-related behavior used by the program flow.

This behavior is evidenced by:
- `fcmp`
- `struct field`

The Rust port may implement comparison idiomatically, but the comparison semantics used by the module must be preserved.

#### 5. Reliable output writing

The module must write output buffers completely to their destination file descriptor rather than assuming a single write succeeds. This applies to terminal output and normal output paths used by the module.

This behavior is evidenced by:
- `xwrite`
- `twrite`
- `tputs`
- `yank`

#### 6. Selected text emission

The module must emit the selected text to standard output. Output must preserve the selected byte content and length handled by the original module.

This behavior is evidenced by:
- `yank`
- `main`

#### 7. Optional clipboard transfer

The module must support the clipboard/output behavior already implemented by the original `yank` flow, where selected text is transferred through the existing X-oriented path in addition to or as part of normal output behavior.

This behavior is evidenced by:
- `yank`

The Rust version must preserve the user-visible effect of this module behavior, but it must not introduce new clipboard targets or APIs not evidenced here.

#### 8. Terminal UI initialization and restoration

For interactive selection mode, the module must:

- initialize terminal state,
- determine terminal characteristics needed by the UI,
- enter the selection session,
- restore terminal state on exit.

This behavior is evidenced by:
- `tsetup`
- `tend`
- `main`
- `struct termios`

The Rust port must preserve the invariant that terminal state is restored after interactive operation completes or exits through the normal control paths handled by this module.

#### 9. Interactive key-driven selection

The module must support interactive terminal input handling for selection. It must read terminal key input, interpret user actions, update selection/navigation state, and complete when a field is chosen or the interaction otherwise terminates according to the original control flow.

This behavior is evidenced by:
- `tgetc`
- `tmain`
- `struct field`

The Rust version must preserve the same category of behavior: terminal-driven navigation and selection over the field list.

#### 10. Usage/help reporting

When invocation arguments are invalid, the module must print a usage message.

This behavior is evidenced by:
- `usage`
- `main`

## User Scenarios & Testing

### Scenario 1: Select one item from piped input interactively

A user pipes a newline-separated list of candidate strings into the program and runs it in terminal-interactive mode. The program displays the candidates, lets the user navigate through them via keyboard input, and returns the chosen item.

Expected behavior:
- input is read from standard input,
- terminal mode is entered,
- one field is selected,
- the selected field is written to standard output,
- terminal settings are restored afterward.

Traceability:
- `input`
- `tsetup`
- `tgetc`
- `tmain`
- `tend`
- `yank`

### Scenario 2: Invoke with an initial text pattern

A user invokes the program with an initial textual pattern/filter argument and provides candidate input on standard input. The program converts the provided string to its internal pattern form and uses it in the selection flow.

Expected behavior:
- argument parsing accepts the pattern-bearing invocation form,
- the pattern string is transformed for internal use,
- selection/output behavior reflects that configured pattern.

Traceability:
- `main`
- `strtopat`
- `tmain`

### Scenario 3: Invalid arguments

A user runs the executable with unsupported or malformed arguments.

Expected behavior:
- the program does not continue normal selection execution,
- a usage message is written,
- process termination indicates invalid invocation according to the original behavior.

Traceability:
- `main`
- `usage`

### Scenario 4: Output path writes complete data

A user selects or produces an output value whose full contents must be written even if the underlying write operation completes only partially in one system call.

Expected behavior:
- complete selected content is written,
- no truncation occurs due to partial writes.

Traceability:
- `xwrite`
- `twrite`
- `tputs`
- `yank`

### Scenario 5: Terminal session cleanup

A user starts an interactive session and exits through the normal control paths of the interface.

Expected behavior:
- terminal configuration modified for the UI is restored before process exit,
- the shell remains usable after the program ends.

Traceability:
- `tsetup`
- `tend`
- `main`

### Scenario 6: Clipboard-affecting yank behavior

A user performs a selection in a mode where the module’s yank behavior transfers the selected text through its existing X clipboard path.

Expected behavior:
- selected text is passed through the program’s yank operation,
- the standard output effect is preserved,
- the clipboard-related side effect evidenced by the C module is preserved.

Traceability:
- `yank`
- `main`

## Requirements

### Functional Requirements

#### FR-1: Program entry and option handling
The module shall provide the executable entry behavior for `yank`, parse supported command-line options and arguments, and reject invalid invocation with a usage message.

Traceability:
- `main`
- `usage`

#### FR-2: Standard input ingestion
The module shall read candidate text data from standard input and construct the internal field set used by the remainder of execution.

Traceability:
- `input`
- `struct field`
- `main`

#### FR-3: Pattern string conversion
The module shall convert a user-provided string into the internal pattern representation required by module operation when such input is supplied.

Traceability:
- `strtopat`
- `main`

#### FR-4: Field comparison semantics
The module shall provide field comparison behavior used by its ordering-related logic over candidate items.

Traceability:
- `fcmp`
- `struct field`

#### FR-5: Complete-write semantics
The module shall ensure that writes performed through its output helpers continue until the full requested byte sequence has been written or an error path is taken.

Traceability:
- `xwrite`
- `twrite`
- `tputs`
- `yank`

#### FR-6: Selected-result emission
The module shall output the selected or yanked text as bytes corresponding to the chosen field content.

Traceability:
- `yank`
- `main`

#### FR-7: Clipboard-transfer behavior
The module shall preserve the original module’s clipboard-affecting yank behavior through the existing X-oriented path used when text is yanked.

Traceability:
- `yank`

#### FR-8: Interactive terminal setup
The module shall initialize terminal state required for the interactive interface before reading terminal-driven selection input.

Traceability:
- `tsetup`
- `main`
- `struct termios`

#### FR-9: Interactive input processing
The module shall read terminal key input and use it to drive selection interaction over the available fields until completion under the module’s supported control flow.

Traceability:
- `tgetc`
- `tmain`
- `struct field`

#### FR-10: Terminal restoration
The module shall restore terminal state after interactive execution completes through the module’s normal exit paths.

Traceability:
- `tend`
- `main`
- `struct termios`

### Key Entities

#### Field
A field is the module’s core candidate-selection record. It represents one selectable input item and is the unit used for comparison, interactive selection, and final output.

Traceability:
- `struct field`
- `fcmp`
- `tmain`
- `main`

#### Field collection
The module maintains a collection of fields derived from standard input. This collection feeds ordering/comparison behavior and terminal selection behavior.

Traceability:
- `input`
- `struct field`
- `tmain`

#### Pattern representation
The module uses an internal pattern form derived from a source string for matching/filter-related operation.

Traceability:
- `strtopat`
- `main`

#### Terminal session state
The module maintains terminal-related state needed to enter and leave interactive mode, including saved terminal configuration.

Traceability:
- `tsetup`
- `tend`
- `tgetc`
- `struct termios`

## Success Criteria

### SC-1: Valid interactive selection flow
Given newline-delimited input on standard input and a supported interactive invocation, the Rust module can present a terminal selection session, accept keyboard-driven selection, and output the chosen field.

Traceability:
- `input`
- `tsetup`
- `tgetc`
- `tmain`
- `yank`

### SC-2: Usage handling parity
Given unsupported or malformed arguments, the Rust module prints a usage message and does not proceed with normal selection execution.

Traceability:
- `main`
- `usage`

### SC-3: Pattern conversion support
Given an invocation form that supplies a pattern string, the Rust module converts that string into the internal form needed by the selection flow and completes execution without omitting that step.

Traceability:
- `strtopat`
- `main`

### SC-4: Full output delivery
For module-managed output operations, the Rust module writes the full selected byte sequence without truncation from partial-write behavior.

Traceability:
- `xwrite`
- `twrite`
- `tputs`
- `yank`

### SC-5: Terminal restoration after interaction
After the Rust module exits an interactive session through its normal control flow, the user’s terminal remains in a usable restored state.

Traceability:
- `tsetup`
- `tend`
- `main`

### SC-6: Field-based selection preservation
The Rust module preserves field-centered behavior: input is represented as selectable field records, those records participate in comparison/selection flow, and the final emitted text comes from the selected field.

Traceability:
- `struct field`
- `input`
- `fcmp`
- `tmain`
- `yank`

### SC-7: Clipboard side effect preservation
When exercising the module path corresponding to yanking text, the Rust module preserves the original clipboard-affecting behavior evidenced by the C implementation.

Traceability:
- `yank`