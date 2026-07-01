# main_root Functional Specification

- **Project**: `yank`
- **Module**: `main_root`
- **Category**: `main`
- **Source basis**: `yank.c`
- **Rust branch target**: `001-main_root-rust-port`
- **Generation date**: `2026-06-16`

## 1. Feature Specification

### 1.1 Purpose

The `main_root` module is the program entry module for `yank`. It reads newline-delimited input records from standard input, presents them in an interactive terminal interface, lets the user choose one record, and writes the chosen record to standard output.

This module also supports a non-interactive mode in which records are read, ordered, and emitted without terminal interaction.

### 1.2 In-Scope Functionality

The Rust rewrite must implement the following observed behaviors from `yank.c`:

- Read input items from standard input as text records separated by newlines.
- Store input as selectable fields/items.
- Optionally transform an input string into a search or matching pattern representation before interactive selection logic uses it.
- Order stored items using the module’s field comparison behavior.
- In interactive mode:
  - configure terminal state for direct key-driven interaction,
  - render visible text to the terminal,
  - read key input from the terminal,
  - allow navigation/selection through the available items,
  - restore terminal state when interaction ends.
- When a selection is finalized, emit the selected item to standard output.
- Provide command-line entry behavior, including usage output for invalid invocation.

### 1.3 Out-of-Scope Functionality

The Rust rewrite must not assume or introduce functionality not evidenced by this module analysis, including:

- new public APIs beyond program entry behavior,
- network, IPC, or daemon behavior,
- persistence or history storage,
- concurrency guarantees,
- structured output formats,
- recovery workflows beyond observed terminal cleanup and process termination behavior.

## 2. User Scenarios & Testing

### 2.1 Scenario: Select one item from piped input

A user pipes newline-separated text into the program from standard input. The program opens an interactive terminal UI, displays available items, the user navigates and confirms one item, and the program writes the selected item to standard output.

**Expected result**
- Input is accepted from stdin.
- One item can be selected interactively.
- The exact selected item text is written to stdout.

**Traceability**
- `input`
- `tsetup`
- `tgetc`
- `tmain`
- `yank`
- `main`

### 2.2 Scenario: Terminal interaction cleans up after exit

A user runs the program in interactive mode and exits after selection or termination. Terminal behavior must return to its prior usable state.

**Expected result**
- Terminal mode is changed only for the interaction period.
- Terminal state is restored before process exit.

**Traceability**
- `tsetup`
- `tend`
- `main`
- `struct termios` usage

### 2.3 Scenario: Invalid invocation prints usage

A user runs the program with unsupported or malformed arguments.

**Expected result**
- Usage text is emitted.
- Program exits without entering normal selection flow.

**Traceability**
- `usage`
- `main`

### 2.4 Scenario: Output writing handles full item emission

A selected item is written to stdout, including cases where writing may require repeated write attempts.

**Expected result**
- Full selected item content is emitted.
- Partial writes do not truncate successful output.

**Traceability**
- `xwrite`
- `yank`
- `twrite`
- `tputs`

### 2.5 Scenario: Non-interactive or direct output path

A user invokes the program in a mode supported by `main` that does not require the interactive terminal loop, but still consumes stdin records and emits resulting item text.

**Expected result**
- Supported invocation path completes without terminal UI.
- Output is written to stdout according to the selected/direct path in `main`.

**Traceability**
- `input`
- `fcmp`
- `yank`
- `main`

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Program entry and argument handling
The module shall provide the executable entry behavior for `yank`, parse command-line options supported by the original `main`, and invoke usage output when invocation is invalid.

**Traceability**
- `main`
- `usage`

#### FR-2: Input ingestion
The module shall read records from standard input and retain them as item/field data for later ordering, display, and selection.

**Traceability**
- `input`
- `struct field`

#### FR-3: Pattern conversion
The module shall support conversion of a text string into the module’s internal pattern form when needed by the selection flow.

**Traceability**
- `strtopat`

#### FR-4: Item ordering
The module shall support ordering/comparison of stored items using the original field comparison semantics.

**Traceability**
- `fcmp`
- `struct field`

#### FR-5: Reliable output emission
The module shall write output data to file descriptors in a way that continues until the requested content has been written or an unrecoverable write failure occurs.

**Traceability**
- `xwrite`

#### FR-6: Selected item emission
The module shall emit the selected item text to standard output when an item is chosen or when a direct-output path in program flow requires emission.

**Traceability**
- `yank`
- `main`

#### FR-7: Terminal output support
The module shall support writing raw text to the terminal during interactive operation.

**Traceability**
- `twrite`
- `tputs`

#### FR-8: Terminal session setup
The module shall initialize terminal state required for interactive operation, including obtaining relevant terminal characteristics needed by the UI flow.

**Traceability**
- `tsetup`
- `struct termios`
- `struct winsize`

#### FR-9: Terminal session teardown
The module shall restore terminal state at the end of interactive operation.

**Traceability**
- `tend`
- `struct termios`

#### FR-10: Interactive key input
The module shall read terminal key input and interpret it for use by the interactive selection loop.

**Traceability**
- `tgetc`

#### FR-11: Interactive selection loop
The module shall run an interactive loop that returns the chosen field/item from the available input set.

**Traceability**
- `tmain`
- `struct field`

### 3.2 Key Entities

#### Entity: Field
A `field` represents one input item managed by the module. It is the core unit for storage, ordering, display, and final selection.

**Relationships**
- Created/populated during input ingestion.
- Compared by the field comparison behavior.
- Returned by the interactive main loop as the selected result.
- Emitted by the final output path.

**Traceability**
- `struct field`
- `input`
- `fcmp`
- `tmain`
- `yank`

#### Entity: Terminal session state
Terminal session state represents the process terminal configuration preserved before interaction and restored afterward.

**Relationships**
- Established during terminal setup.
- Used during interactive input/output.
- Restored during terminal teardown.

**Traceability**
- `struct termios`
- `tsetup`
- `tend`

#### Entity: Terminal size/state metadata
Terminal metadata includes display-related terminal properties needed to drive the interactive interface.

**Relationships**
- Read during terminal setup.
- Used by the interactive selection UI to determine display behavior.

**Traceability**
- `struct winsize`
- `tsetup`

#### Entity: Pattern representation
Pattern representation is the internal form produced from a source string for matching or selection-related logic.

**Relationships**
- Derived from input text or user-provided text.
- Consumed by selection logic.

**Traceability**
- `strtopat`

## 4. Success Criteria

### 4.1 Behavioral Success Criteria

1. Given newline-delimited stdin input and a valid interactive terminal, the Rust module can present selectable items and return one selected item to stdout.
   - **Traceability**: `input`, `tsetup`, `tgetc`, `tmain`, `yank`, `main`

2. On every interactive execution path that reaches terminal setup, terminal state is restored before program exit.
   - **Traceability**: `tsetup`, `tend`, `main`

3. When stdout or terminal writes complete in partial segments, the Rust module continues writing until the intended text payload has been fully emitted or an unrecoverable error is reached.
   - **Traceability**: `xwrite`, `yank`, `twrite`, `tputs`

4. Invalid invocation causes usage output and does not proceed into normal selection behavior.
   - **Traceability**: `usage`, `main`

5. The item ordering behavior used by the Rust rewrite is driven by the same field comparison rule as the source module.
   - **Traceability**: `fcmp`, `struct field`

6. The interactive selection loop returns a field/item object that corresponds to one of the ingested input records.
   - **Traceability**: `input`, `tmain`, `struct field`

### 4.2 Port Completion Criteria

1. All source-observed functional responsibilities in `yank.c` are represented in the Rust rewrite: input ingestion, item comparison, output emission, terminal setup/teardown, key input handling, interactive selection, usage handling, and program entry flow.
2. No additional unsupported features are introduced beyond the behaviors evidenced by this module.
3. The Rust implementation preserves the module’s role as the executable root for the program flow.