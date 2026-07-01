# Implementation Plan

## Summary

Port `src/c.c` into a single Rust module that preserves the existing scanner buffer-management behavior and public function surface as closely as practical within Rust idioms. The implementation should focus on migrating the current file’s stateful lexer runtime pieces: buffer switching, buffer stack push/pop, buffer flushing/deletion, stream/state accessors, NUL transition handling, character pushback, restart logic, and fatal-error/reporting paths.

The Rust approach should keep the module narrowly scoped:

- represent the C scanner state explicitly in Rust structs,
- convert implicit global/static state into owned module state gathered in a single scanner/runtime struct,
- preserve call ordering and side effects of the original functions,
- use standard-library I/O and memory management instead of manual allocation,
- model fallible operations with `Result` where internal helpers need it, while keeping outward behavior aligned with the C implementation.

The migration should not add new scanner capabilities, abstractions, or additional subsystem layers beyond what is needed to replace the contents of `src/c.c`.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates required based on the available module evidence
- **Testing**:
  - `cargo test`
  - unit tests focused on buffer state transitions and accessor behavior
- **Performance Goals**:
  - maintain linear-time buffer operations equivalent to the C implementation
  - avoid unnecessary string copying during buffer switch/load/flush paths
  - keep push/pop and active-buffer replacement operations amortized constant time where the C code uses direct pointer/state replacement
  - preserve low-overhead state access for line/input/output/length getters

## Module Mapping

### C to Rust File Mapping

- `src/c.c` → `src/module_src_c_c_20.rs`

If the current crate already centralizes modules through `src/lib.rs` or `src/main.rs`, expose only this migrated module there without introducing extra architectural layers.

### Function Mapping

The Rust module should migrate the existing C functions into methods on a scanner-state struct, except for fatal/internal helpers where a private free function is acceptable if simpler.

- `yy_try_NUL_trans` → `ScannerState::yy_try_nul_trans(...)`
- `yyunput` → `ScannerState::yyunput(...)`
- `yyrestart` → `ScannerState::yyrestart(...)`
- `yy_switch_to_buffer` → `ScannerState::yy_switch_to_buffer(...)`
- `yy_load_buffer_state` → `ScannerState::yy_load_buffer_state()`
- `yy_delete_buffer` → `ScannerState::yy_delete_buffer(...)`
- `yy_flush_buffer` → `ScannerState::yy_flush_buffer(...)`
- `yypush_buffer_state` → `ScannerState::yypush_buffer_state(...)`
- `yypop_buffer_state` → `ScannerState::yypop_buffer_state()`
- `yy_fatal_error` → `ScannerState::yy_fatal_error(...)` or private `fn yy_fatal_error(...) -> !`
- `yyget_lineno` → `ScannerState::yyget_lineno()`
- `yyget_in` → `ScannerState::yyget_in()`
- `yyget_out` → `ScannerState::yyget_out()`
- `yyget_leng` → `ScannerState::yyget_leng()`

### Special Note on `if`

The listed `if` entry is not a valid standalone C function name and should be treated as analysis noise unless the source confirms a macro-generated symbol or parser artifact. Do not create a Rust item for it unless direct inspection of `src/c.c` shows a real exported routine needing migration.

## Data Model

The source analysis only reports multiple anonymous data structures. For the port, replace these anonymous C structs with named Rust types based on their actual roles in `src/c.c`.

### Core Struct Mapping

Because this module contains flex-like runtime functions, the likely migration target is:

- anonymous scanner/buffer state structs → named Rust structs with explicit fields
- C pointers to active buffer/global state → owned fields plus `Option` indices/references inside a scanner-state owner
- C FILE pointers for input/output handles → standard-library reader/writer handles or narrow placeholders retained inside scanner state, depending on actual use in this file

### Recommended Rust Structures

#### `BufferState`

Represents the C buffer object manipulated by switch/load/delete/flush operations.

Expected field categories to migrate directly from C:

- backing byte buffer
- current position/index
- current character count
- buffer capacity
- ownership flag if the C code distinguishes externally supplied vs internally allocated buffers
- interactive flag / beginning-of-line flag if present
- status flags used by restart/flush/load logic

Suggested shape:

```rust
struct BufferState {
    buf: Vec<u8>,
    pos: usize,
    n_chars: usize,
    capacity: usize,
    is_our_buffer: bool,
    is_interactive: bool,
    at_bol: bool,
    fill_buffer: bool,
    buffer_status: BufferStatus,
}
```

#### `BufferStatus`

Maps C integer/status constants to a Rust enum.

```rust
enum BufferStatus {
    New,
    Normal,
    EofPending,
}
```

Use exact variants only if the source confirms them; otherwise define only the statuses required by `src/c.c`.

#### `ScannerState`

Collects the global/static lexer runtime variables into one owner.

Expected field categories:

- current buffer reference/index
- buffer stack
- current token length
- line number
- input handle metadata
- output handle metadata
- character-hold / cursor state used by `yyunput` and load logic
- state integer used by `yy_try_NUL_trans`

Suggested shape:

```rust
struct ScannerState {
    current_buffer: Option<usize>,
    buffers: Vec<BufferState>,
    line_no: i32,
    yy_leng: usize,
    hold_char: Option<u8>,
    c_buf_p: usize,
    start_state: i32,
    input: Option<InputSource>,
    output: Option<OutputSink>,
}
```

The final field set should be derived strictly from the actual globals/statics in `src/c.c`.

#### `InputSource` / `OutputSink`

Only introduce these as minimal wrappers if the C file actually stores stream handles and the getters must return them through stable Rust types. If the source merely tracks placeholders, keep raw state minimal rather than inventing general-purpose I/O abstractions.

Possible minimal forms:

```rust
enum InputSource {
    Stdin,
    File(std::fs::File),
}
```

```rust
enum OutputSink {
    Stdout,
    File(std::fs::File),
}
```

If borrowing/lifetime complexity outweighs value and the handles are not actively used in this module, store only the file identity needed for parity with getters.

### C-to-Rust Type Guidance

- `char *` buffer memory → `Vec<u8>` plus indices
- pointer-to-current-position → `usize` index into owned buffer
- nullable pointer → `Option<T>`
- integer state constants → `enum` or `i32` when exact numeric compatibility matters
- manually allocated nested structs → owned Rust structs dropped automatically
- global mutable state → fields on `ScannerState`

## Implementation Phases

## Phase 1: Inventory and State Extraction

- Inspect `src/c.c` and identify:
  - all file-scope globals/statics used by the listed functions
  - all anonymous structs and their concrete fields
  - all buffer status constants and sentinel values
  - any assumptions about stream ownership and nullability
- Create `src/module_src_c_c_20.rs`.
- Define named Rust equivalents for:
  - buffer state structure
  - scanner/global runtime structure
  - any status enums/constants still needed
- Convert C global mutable state into `ScannerState` fields without changing behavior.
- Add module skeleton methods matching the migrated function names.

**Exit criteria**:
- all structures and constants from `src/c.c` are represented in Rust,
- every listed function has a Rust stub with settled signature,
- there is no remaining need for anonymous data layouts.

## Phase 2: Buffer Lifecycle and State-Transition Port

Implement the functions that mutate buffer ownership and active-buffer selection first, because the remaining routines depend on them.

Functions in this phase:

- `yy_load_buffer_state`
- `yy_switch_to_buffer`
- `yy_flush_buffer`
- `yy_delete_buffer`
- `yypush_buffer_state`
- `yypop_buffer_state`
- `yyrestart`

Technical decisions:

- replace C allocation/free with `Vec<u8>` and owned structs
- replace pointer swapping with `Option<usize>` or direct owned replacement inside `ScannerState`
- preserve buffer-stack ordering and current-buffer update semantics
- handle deletion safely:
  - do not leave stale active-buffer references
  - clear or reassign current buffer exactly as the C control flow requires
- preserve restart/flush semantics for resetting position, counts, and status flags

Tests for this phase:

- switching to a new buffer updates active state correctly
- pushing and popping buffers restores previous buffer state
- flushing resets position/count/status as expected
- deleting active and inactive buffers does not corrupt stack/current selection
- restart reinitializes current input buffer state correctly

**Exit criteria**:
- all buffer lifecycle methods are implemented and tested,
- ownership and drop behavior are fully handled by Rust without manual free logic.

## Phase 3: Character/Scanner Runtime Port

Implement runtime functions that depend on active buffer internals.

Functions in this phase:

- `yy_try_NUL_trans`
- `yyunput`

Technical decisions:

- map pointer arithmetic to explicit index math
- guard against underflow/overflow when moving within buffer contents
- preserve sentinel/NUL handling exactly, especially around end-of-buffer transitions
- keep any scanner-state integer transitions unchanged unless the source allows a stronger enum mapping without semantic drift

Tests for this phase:

- unput reinserts a byte/character and updates cursor state correctly
- NUL transition follows original state machine behavior for representative states
- boundary cases at start/end of buffer match C behavior

**Exit criteria**:
- no raw pointer-style logic remains unresolved,
- index-based Rust implementation matches original state transitions.

## Phase 4: Accessors, Fatal Path, and Verification

Implement the remaining low-level public interface and finalize behavioral checks.

Functions in this phase:

- `yy_fatal_error`
- `yyget_lineno`
- `yyget_in`
- `yyget_out`
- `yyget_leng`

Technical decisions:

- `yy_fatal_error` should terminate in a Rust-appropriate way consistent with original non-returning behavior, typically `panic!` for this internal runtime path unless direct process exit is required by the surrounding crate conventions
- getters should return stored state without introducing synchronization or wrapper layers
- where C returned raw pointers, expose the narrowest Rust return type compatible with actual internal representation and crate usage

Tests for this phase:

- getter values reflect scanner state updates
- fatal path is covered with `#[should_panic]` if implemented as panic
- regression tests execute realistic sequences:
  - create/load buffer
  - switch/push/pop
  - flush/restart
  - query line/input/output/length

**Exit criteria**:
- all listed functions are implemented,
- `cargo test` passes,
- module behavior is stable without adding capabilities beyond the original file.