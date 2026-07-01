# Implementation Plan: main_root

## Summary

Port `yank.c` into a single Rust main-module implementation that preserves the existing command-line entry flow, input processing, pattern conversion, output writing, and terminal lifecycle behavior without introducing new capabilities or reorganizing the program beyond what Rust requires.

The Rust implementation should keep the migration centered on one executable entrypoint and a small set of internal helper functions corresponding closely to the original C functions:

- input handling
- pattern conversion
- file/content comparison
- buffered and terminal writes
- terminal setup/teardown and character input
- top-level command dispatch and usage output

The technical approach is:

- map each C function to a Rust function with near-identical responsibility
- keep state explicit in Rust structs instead of relying on anonymous C storage
- use `std::io` and `std::fs` for file and terminal-adjacent I/O where possible
- represent fallible operations with `Result`
- use RAII-style ownership to ensure cleanup for terminal/session state, while still preserving explicit setup/end call ordering from the C design

## Technical Context

### Language/Version

- Rust 1.78 or newer

### Primary Dependencies

Prefer the Rust standard library only.

Recommended crates: none by default.

If raw terminal mode or terminal capability behavior in `tsetup`, `tputs`, `tgetc`, and `tend` cannot be implemented adequately with `std` alone, add exactly one minimal terminal crate after validating the C behavior during migration:

- `crossterm` for terminal mode control and character input

Do not introduce additional abstraction crates unless required by verified behavior in `yank.c`.

### Testing

- `cargo test`

Testing focus:

- unit tests for pure helpers such as pattern conversion and comparison logic
- argument/usage behavior tests where practical
- output writer behavior using in-memory buffers
- terminal-specific behavior kept minimal and tested only where deterministic

### Performance Goals

- maintain behavior comparable to the C implementation for normal interactive and file-processing usage
- avoid unnecessary heap allocation during input scanning and write paths
- preserve streaming behavior for file and standard input processing where present
- keep terminal operations direct and lightweight
- do not trade correctness for micro-optimizations during the first port

## Module Mapping

### Source File Mapping

C source:
- `yank.c`

Rust target:
- `src/main.rs`

Keep the first migration in a single file if that best matches the original structure. Only split into additional Rust modules if the file becomes unmanageably large during the port, and only along existing function boundaries from `yank.c`.

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `input` | `fn input(...) -> Result<..., ...>` | Convert pointer-based input handling to slice/string/buffer-based logic. |
| `strtopat` | `fn strtopat(...) -> Result<..., ...>` | Preserve parsing semantics; convert to owned Rust pattern representation. |
| `fcmp` | `fn fcmp(...) -> Result<..., ...>` | Use `std::fs`/`std::io` comparisons; preserve return meaning explicitly. |
| `xwrite` | `fn xwrite<W: Write>(...) -> io::Result<()>` | Map low-level writes to `Write::write_all`. |
| `yank` | `fn yank(...) -> Result<..., ...>` | Central operational routine; retain original control flow. |
| `twrite` | `fn twrite(...) -> io::Result<()>` | Keep as terminal/output-specific wrapper if still distinct in C. |
| `tputs` | `fn tputs(...) -> io::Result<()>` | Preserve terminal capability/output role with Rust string/byte handling. |
| `tsetup` | `fn tsetup(...) -> Result<TerminalState, ...>` | Replace global terminal state with owned Rust state. |
| `tend` | `fn tend(state: &mut TerminalState) -> Result<(), ...>` | Explicit cleanup, potentially also backed by `Drop`. |
| `tgetc` | `fn tgetc(...) -> Result<..., ...>` | Character input from terminal/stdin; preserve blocking semantics. |
| `tmain` | `fn tmain(...) -> Result<..., ...>` | Keep as terminal-mode main loop/helper if distinct from `main`. |
| `usage` | `fn usage(...)` | Print usage text to stderr/stdout per original behavior. |
| `main` | `fn main()` | Parse args, call Rust equivalents, convert errors to exit status. |

## Data Model

The C analysis reports multiple anonymous structures. Because their names and fields are not available, the Rust plan should keep data modeling conservative and derive structures directly from actual field groupings found in `yank.c` during implementation.

### Data-Structure Mapping Strategy

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous terminal-related struct(s) | `struct TerminalState` | Holds terminal mode/configuration previously stored in anonymous/global C state. |
| anonymous pattern-related struct(s) | `struct Pattern` or `enum Pattern` | Use explicit typed representation for parsed pattern data from `strtopat`. |
| anonymous input/output buffer struct(s) | `struct BufferState` | Replace raw pointers, lengths, and capacities with `Vec<u8>` or `String`. |
| anonymous option/config struct(s) | `struct Config` | Collect command-line flags and mode selections passed across functions. |
| anonymous file comparison/input state struct(s) | `struct InputState` / `struct FileState` | Encapsulate file handles, paths, cursors, and flags. |

### C-to-Rust Type Conventions

Use these conversions consistently during the port:

- `char *` used as text -> `String` or `&str`
- `char *` used as byte buffer -> `Vec<u8>` or `&[u8]`
- `FILE *` / file descriptors -> `std::fs::File`, `std::io::Stdin`, `std::io::Stdout`, `std::io::Stderr`
- integer status codes -> `Result<T, E>` internally, mapped to process exit codes at `main`
- flag fields / booleans -> `bool`
- pointer + length pairs -> slices or owned buffers
- nullable pointers -> `Option<T>`

### Memory Management and Error Handling

- eliminate manual allocation/free by using owned Rust containers
- replace pointer aliasing with borrowing and explicit ownership
- preserve operational failure points with `io::Result` or a small module-local error enum
- keep error typing narrow; do not build a broad framework
- ensure terminal cleanup occurs on all exit paths, preferably through owned state plus explicit `tend`

## Implementation Phases

### Phase 1: Skeleton Port and Type Recovery

Goal: establish a compiling Rust executable with the original entry structure intact.

Tasks:
- create `src/main.rs`
- map `main`, `usage`, and `tmain` first
- inspect `yank.c` and recover the actual anonymous-struct roles into minimal Rust structs
- define module-local type aliases, structs, and enums only as required by migrated functions
- define a small error strategy:
  - `io::Result` where sufficient
  - one local error enum only if mixed parse/terminal/I/O failures require it

Exit criteria:
- Rust binary compiles
- command-line entry path exists
- data structures necessary for the rest of the port are identified and declared

### Phase 2: Core Non-Terminal Logic Migration

Goal: migrate the deterministic processing logic before terminal-specific behavior.

Tasks:
- port `strtopat`
- port `input`
- port `fcmp`
- port `xwrite`
- port `yank` core logic, wiring it to the above helpers
- replace C buffer arithmetic with Rust slices, `Vec<u8>`, and `String` as appropriate
- preserve original return semantics while converting internals to `Result`

Testing:
- add unit tests for:
  - pattern parsing edge cases
  - comparison outcomes
  - write helper behavior using in-memory writers
  - input parsing/processing behavior where independent of terminal state

Exit criteria:
- core processing path works without terminal-specific code
- tests cover migrated pure/helper logic
- no unsafe Rust used unless directly forced by unavoidable terminal behavior

### Phase 3: Terminal Behavior Migration

Goal: port terminal setup, output, input, and cleanup with the smallest viable Rust surface.

Tasks:
- port `tsetup`
- port `tputs`
- port `twrite`
- port `tgetc`
- port `tend`
- model terminal lifecycle with `TerminalState`
- first attempt implementation with `std` only where feasible
- if raw mode or single-character input semantics require more than `std`, add `crossterm` and limit its use to these functions only
- preserve explicit setup/teardown ordering from the C flow

Testing:
- keep automated tests focused on deterministic output formatting helpers
- avoid speculative integration tests for terminal state beyond what the migrated behavior clearly requires

Exit criteria:
- terminal path is functionally connected
- cleanup is reliable on normal and error returns
- terminal-specific dependency usage, if any, is minimal and localized

### Phase 4: Integration, Behavior Alignment, and Cleanup

Goal: finish the port by aligning observable behavior and simplifying only where it does not alter structure.

Tasks:
- connect `main` fully to `tmain`/`yank` flow
- verify stderr/stdout usage and exit codes against the C implementation
- normalize function signatures for idiomatic but restrained Rust
- remove leftover C-style sentinel logic that is no longer needed after ownership conversion
- ensure all file and terminal resources are dropped cleanly
- add focused integration-style tests for argument handling and end-to-end non-interactive flows where practical

Exit criteria:
- `yank.c` functionality is represented in `src/main.rs`
- `cargo test` passes
- implementation remains structurally close to the original module without extra facilities