# main_root Functional Specification

## Metadata
- Project: `yank`
- Module: `main_root`
- Category: `main`
- Source basis: `yank.c`
- Target Rust branch: `001-main_root-rust-port`
- Generation date: `2026-06-07`

## 1. Feature Specification

### 1.1 Purpose
`main_root` is the program entry module for the `yank` command. It is responsible for:
- parsing command-line options,
- reading candidate text fields from standard input,
- preparing matching and display state,
- running an interactive terminal selection session,
- emitting the selected field to standard output,
- and managing terminal setup and restoration during interactive use.

This specification covers the behavior evidenced by `yank.c`, especially `main`, `input`, `strtopat`, `fcmp`, `xwrite`, `yank`, `twrite`, `tputs`, `tsetup`, `tend`, `tgetc`, `tmain`, and `usage`.

### 1.2 Functional Scope
The Rust rewrite must implement the following end-user behavior:

1. Accept textual input records from standard input and store them as selectable fields.
2. Support command-line configuration of matching behavior and output behavior as evidenced by the main program flow.
3. Convert a user-supplied search string into an internal pattern form before matching.
4. Present an interactive terminal UI for choosing one field from the loaded input set.
5. Track terminal dimensions and terminal mode needed for interactive character-by-character input.
6. Read interactive key input from the terminal and update selection state accordingly.
7. Write terminal control/output text and final selected content reliably to the appropriate file descriptors.
8. Restore terminal state before program exit, including after the interactive session completes.
9. Print usage information for invalid invocation patterns.

### 1.3 Out of Scope
The Rust rewrite must not claim or introduce functionality not evidenced by this module analysis. In particular, this specification does not require:
- new public library APIs,
- non-interactive machine-readable output modes,
- multi-selection output,
- thread-safe behavior,
- persistence or session recovery,
- plugin or extension hooks,
- network or IPC features.

## 2. User Scenarios & Testing

### 2.1 Scenario: Start program with valid input and select an entry
A user pipes or redirects newline-delimited text into the program, starts it in a terminal, navigates the candidate list, confirms a selection, and receives the selected field on standard output.

The Rust version must support:
- loading all available input fields from standard input,
- interactive selection using terminal input,
- emitting exactly the selected field content through the final output path.

Traceability: `input`, `tsetup`, `tgetc`, `tmain`, `yank`, `main`.

### 2.2 Scenario: Invoke with invalid arguments
A user provides unsupported or malformed command-line arguments.

The Rust version must support:
- detecting invalid invocation,
- printing usage text,
- terminating without entering the interactive selection session.

Traceability: `usage`, `main`.

### 2.3 Scenario: Use search text for matching
A user starts the program with a search or pattern-related option that supplies text used for matching candidates.

The Rust version must support:
- converting the provided string into the internal pattern representation,
- applying that pattern to the candidate set used by the interactive selector.

Traceability: `strtopat`, `main`, `tmain`.

### 2.4 Scenario: Operate in a real terminal session
A user runs the program from a terminal where screen control and raw key input are required.

The Rust version must support:
- switching the terminal into the mode required by the interactive UI,
- writing terminal control/output sequences during the session,
- restoring the original terminal mode when leaving the session.

Traceability: `tsetup`, `twrite`, `tputs`, `tend`, `main`.

### 2.5 Scenario: Terminal resize-aware display session
A user runs the interactive selector in a terminal with finite width and height.

The Rust version must support:
- obtaining terminal size for display layout decisions,
- using available rows/columns to drive interactive presentation behavior.

Traceability: `tsetup`, `tmain`.

### 2.6 Scenario: Reliable output of selected text
A user selects text whose output may require multiple low-level write attempts.

The Rust version must support:
- retrying or otherwise ensuring complete writing of intended output data,
- using the same reliability approach for terminal-directed writes and final selected result writes.

Traceability: `xwrite`, `yank`, `twrite`, `tputs`.

### 2.7 Suggested Test Cases
1. **Usage path**
   - Invoke with invalid option syntax.
   - Verify usage text is emitted and process exits without interactive mode.

2. **Basic selection**
   - Provide a small input set.
   - Select one visible item.
   - Verify output matches the chosen field exactly.

3. **Pattern initialization**
   - Start with a search string option.
   - Verify the initial interactive result set reflects the converted pattern.

4. **Terminal restoration**
   - Enter interactive mode and exit.
   - Verify terminal settings are restored after process termination.

5. **Terminal output path**
   - Exercise rendering updates in the selector.
   - Verify terminal writes complete without truncation.

6. **Empty or minimal input handling**
   - Start with no input or one input field, as supported by the original program behavior.
   - Verify the session behavior matches the source behavior without corruption or partial writes.

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Program startup and argument handling
The module shall serve as the executable entry point and interpret command-line arguments to determine runtime behavior, including recognizing invalid invocation and dispatching to usage output when needed.

Traceability: `main`, `usage`.

#### FR-2: Input field ingestion
The module shall read candidate data from standard input and store it as selectable fields for later matching and display.

Traceability: `input`, `struct field`, `main`.

#### FR-3: Pattern preparation
When a search text is supplied through program invocation, the module shall convert that text into the internal pattern form used by the selection workflow.

Traceability: `strtopat`, `main`.

#### FR-4: Field ordering support
The module shall support comparison of fields for ordering purposes where the program flow requires sorted or rank-based field handling.

Traceability: `fcmp`, `struct field`.

#### FR-5: Reliable low-level writing
The module shall provide a reliable write path that attempts to deliver the full requested byte sequence to a file descriptor.

Traceability: `xwrite`.

#### FR-6: Selected result emission
The module shall output the selected field content through the program’s result-output path once selection is finalized.

Traceability: `yank`, `main`.

#### FR-7: Terminal-directed writing helpers
The module shall provide terminal output helpers for writing byte sequences and C-string-equivalent text used by the interactive interface.

Traceability: `twrite`, `tputs`.

#### FR-8: Terminal session setup
Before interactive selection begins, the module shall initialize terminal state required for interactive operation, including terminal mode and screen-size awareness.

Traceability: `tsetup`, `struct termios`, terminal size structure usage in `tsetup`.

#### FR-9: Terminal session teardown
After interactive operation ends, the module shall restore terminal state altered for the session.

Traceability: `tend`, stored terminal state structures, `main`.

#### FR-10: Interactive character input
During the selection session, the module shall read terminal input events one character or key sequence at a time and translate them into actions consumable by the interactive selector.

Traceability: `tgetc`.

#### FR-11: Interactive selection loop
The module shall run the terminal-driven main selection loop and return the chosen field when a selection is completed.

Traceability: `tmain`, `struct field`.

#### FR-12: End-to-end execution flow
The module shall coordinate initialization, input loading, interactive selection, output emission, and cleanup in a single executable control flow.

Traceability: `main`, `input`, `tsetup`, `tmain`, `yank`, `tend`.

### 3.2 Key Entities

#### Field
A `field` represents one selectable candidate item derived from standard input. It is the central unit used by input ingestion, ordering/comparison, interactive selection, and final output.

Relationships:
- created/populated during input loading,
- compared by field comparison logic,
- returned by the interactive selection loop,
- emitted by the final output path.

Traceability: `struct field`, `input`, `fcmp`, `tmain`, `yank`, `main`.

#### Pattern text representation
An internal pattern representation is derived from a user-provided search string and used to influence matching behavior in the selection workflow.

Relationships:
- produced from invocation-time search text,
- consumed by interactive selection or candidate filtering behavior.

Traceability: `strtopat`, `main`, `tmain`.

#### Terminal session state
Terminal session state includes the original terminal attributes and the active interactive-session configuration, as well as current terminal dimensions.

Relationships:
- captured during terminal setup,
- used during interactive display/input handling,
- restored during teardown.

Traceability: `tsetup`, `tend`, `tgetc`, terminal-related structures in `yank.c`.

## 4. Success Criteria

### 4.1 Behavioral Success Criteria
1. The Rust module can be built as the executable entry module for the `yank` program and performs the full startup-to-exit flow evidenced in `main`.
   - Traceability: `main`.

2. With valid newline-delimited input, the Rust module loads candidate fields and allows one field to be selected through the interactive terminal workflow.
   - Traceability: `input`, `tmain`, `struct field`.

3. After a selection is confirmed, the Rust module writes the selected field content completely to the result output path without truncation.
   - Traceability: `xwrite`, `yank`.

4. During interactive use, the Rust module enters the required terminal mode and restores the original terminal configuration upon exit from the session.
   - Traceability: `tsetup`, `tend`, terminal state structures.

5. Terminal-directed output used by the interactive UI is written through reliable write behavior equivalent in effect to the C module.
   - Traceability: `xwrite`, `twrite`, `tputs`.

6. When invocation is invalid, the Rust module emits usage information and does not proceed into the normal interactive selection path.
   - Traceability: `usage`, `main`.

7. When a search string is provided, the Rust module converts it into the internal pattern form and uses it in the selection workflow.
   - Traceability: `strtopat`, `main`, `tmain`.

8. The interactive session obtains and uses terminal size information needed for display behavior.
   - Traceability: `tsetup`, terminal size usage, `tmain`.

### 4.2 Verification Expectations
The rewrite is considered successful when:
- all scenarios in Section 2 are supported,
- each functional requirement in Section 3.1 is implemented,
- terminal setup/teardown behavior is externally observable as correct,
- selected output matches the chosen input field content exactly,
- no required behavior from `yank.c` listed in this specification is omitted.