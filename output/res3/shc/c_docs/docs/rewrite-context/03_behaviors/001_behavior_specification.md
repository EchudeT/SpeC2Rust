# 03_behaviors

## Scope and evidence basis

This document describes runtime behavior only to the extent supported by the provided module analysis results for `src/shc.c`. The evidence includes function names, signatures, source ordering, and internal call presence at module level. It does not include function bodies, call graph edges, data definitions, or concrete return-value semantics.

Where implementation details are not visible from the summary, the text explicitly states that the current module summary is insufficient to support a more detailed behavior judgment.

---

## 1. Initialization flow and startup order

### 1.1 Program entry sequence

The module contains a conventional top-level entry point:

- `main(int argc, char * argv[])` at `src/shc.c:1355-1363`

Near the end of the file and immediately before `main`, the module also defines:

- `do_all(int argc, char * argv[])` at `src/shc.c:1340-1353`

Given the naming and source order, the observable startup organization is:

1. The process enters through `main`.
2. A consolidated execution phase exists in `do_all`.
3. Argument-related setup exists earlier in:
   - `parse_an_arg`
   - `parse_args`
4. Cryptographic/state-reset support exists in:
   - `stte_0`
   - `key`
   - `arc4`
   - `key_with_file`
5. Script/shell-related processing exists in:
   - `eval_shell`
   - `read_script`
6. Output-generation and build steps exist in:
   - `write_C`
   - `make`

The current module summary is insufficient to support a more detailed behavior judgment about the exact call order among these functions.

### 1.2 Early argument processing phase

The presence of:

- `parse_an_arg(int argc, char * argv[])`
- `parse_args(int argc, char * argv[])`

shows that startup includes a command-line interpretation phase. The separation into singular and plural forms indicates a two-level flow:

- one routine handles one argument-processing step,
- another routine manages the overall argument list traversal or aggregation.

The current module summary is insufficient to support a more detailed behavior judgment about:
- whether parsing stops on first failure,
- whether positional and option arguments are treated differently,
- whether parsed state is stored globally or locally.

### 1.3 Internal state initialization phase

The grouping and naming of:

- `stte_0(void)`
- `key(void * str, int len)`
- `arc4(void * str, int len)`
- `key_with_file(char * file)`

shows a distinct initialization-and-transformation subsystem. `stte_0` strongly suggests an explicit reset or zero-state initialization step before key scheduling or stream transformation. `key` and `key_with_file` indicate state seeding from memory or file-originated material. `arc4` indicates a stateful transformation stage acting on caller-provided memory.

The current module summary is insufficient to support a more detailed behavior judgment about:
- whether `stte_0` is always called before `key` or `arc4`,
- whether multiple rekeying cycles occur,
- whether `key_with_file` is mandatory or optional in startup.

### 1.4 Generation/build phase

The late-file grouping:

- `write_C(char * file, char * argv[])`
- `make(void)`

indicates that after parsing and preparation, the program enters a generation phase that emits C-related output and then a build or compilation phase.

The most defensible startup-to-execution outline supported by the summary is:

1. process entry,
2. argument parsing,
3. state setup / keying / transformation preparation,
4. script or input reading,
5. C output generation,
6. build step,
7. process exit.

The current module summary is insufficient to support a more detailed behavior judgment about whether all phases always execute in one run.

---

## 2. Main user operation flows

### 2.1 Command-line driven control flow

This module is driven entirely from `argc`/`argv` input at its public entry boundary. The key observable runtime flow is command-line initiated and likely proceeds through `parse_args`.

Behaviorally, the user-facing operation model is:

1. User invokes the program with command-line arguments.
2. The program interprets those arguments.
3. The parsed configuration determines which downstream actions run.
4. The program performs file/script processing, data transformation, output emission, and build-related actions.

The current module summary is insufficient to support a more detailed behavior judgment about which argument combinations select which branches.

### 2.2 Script ingestion flow

The function:

- `read_script(char * file)`

indicates a flow where a path-like input identifies a script resource, and the function returns a `char *`, indicating that subsequent steps consume script text as in-memory character data.

A supported high-level runtime flow is:

1. obtain a file path from parsed inputs,
2. read script content via `read_script`,
3. pass text-oriented data into later processing stages.

The current module summary is insufficient to support a more detailed behavior judgment about:
- whether the whole script is read at once,
- how errors are surfaced,
- whether empty files are treated specially.

### 2.3 Shell/text evaluation flow

The function:

- `eval_shell(char * text)`

shows a distinct text-processing or shell-evaluation phase acting on a script or shell string. Since it returns `int`, this phase yields a status-like outcome used by later control flow.

Supported runtime description:

1. a text buffer becomes available,
2. `eval_shell` processes that text,
3. the integer result influences continuation or termination.

The current module summary is insufficient to support a more detailed behavior judgment about:
- whether this evaluation executes shell commands, validates shell syntax, or extracts metadata,
- what result values mean,
- whether evaluation is optional.

### 2.4 Data-randomization / obfuscation support flow

The presence of:

- `rand_mod(unsigned mod)`
- `rand_chr(void)`
- `noise(char * ptr, unsigned min, unsigned xtra, int str)`

shows an explicit randomized-data generation path.

A supported behavior flow is:

1. a caller requests random bounded values through `rand_mod`,
2. a caller requests random character generation through `rand_chr`,
3. `noise` writes or injects generated data into caller-provided storage.

Because `noise` accepts a pointer, minimum size, extra quantity, and a final integer selector, it acts as a configurable randomized content producer or mutator.

The current module summary is insufficient to support a more detailed behavior judgment about:
- whether `noise` creates printable text, binary padding, or source-code-safe content,
- how `str` alters behavior,
- whether these values affect functional correctness or only output form.

### 2.5 Output formatting and emission flow

The functions:

- `prnt_bytes(FILE * o, char * ptr, int m, int l, int n)`
- `prnt_array(FILE * o, void * ptr, char * name, int l, char * cast)`
- `dump_array(FILE * o, void * ptr, char * name, int l, char * cast)`

show a structured output pipeline centered on a `FILE *` destination.

A supported runtime flow is:

1. internal buffers or arrays are prepared,
2. helper routines format byte-level or array-level representations,
3. formatted data is written to an output stream,
4. higher-level generation logic uses these helpers to produce output artifacts.

The relationship among these functions suggests layered formatting:
- `prnt_bytes` handles low-level byte rendering,
- `prnt_array` handles named array output,
- `dump_array` is a wrapper or specialized form of array emission.

The current module summary is insufficient to support a more detailed behavior judgment about exact output syntax.

### 2.6 C file generation flow

The function:

- `write_C(char * file, char * argv[])`

is the central generation routine. Its signature shows:
- a target file parameter,
- access to argument-derived context.

Supported runtime behavior:

1. a target output file is selected,
2. generation helpers and transformation state are used,
3. a C source artifact is emitted,
4. the function returns an integer status-like result.

The current module summary is insufficient to support a more detailed behavior judgment about:
- whether it writes a complete compilable translation unit,
- whether it embeds transformed script data,
- whether failure in helper routines aborts generation immediately.

### 2.7 Build/execution completion flow

The function:

- `make(void)`

shows a terminal phase after output generation. By naming and file placement, this is a post-generation action and likely part of the normal end-to-end user workflow.

Supported runtime behavior:

1. generation finishes,
2. a build-related step runs,
3. the final status contributes to overall program completion.

The current module summary is insufficient to support a more detailed behavior judgment about whether `make` invokes an external tool, performs local file assembly, or only coordinates internal steps.

---

## 3. State machines and state transitions

### 3.1 Global execution state model

From the function set and file ordering, the module behavior can be represented as a coarse state machine:

1. **Start**
   - entered via `main`

2. **Argument Processing**
   - handled by `parse_args` / `parse_an_arg`

3. **Internal State Reset / Key Preparation**
   - handled by `stte_0`, `key`, `key_with_file`

4. **Transformation / Encoding State Active**
   - handled by `arc4`

5. **Input Acquisition**
   - handled by `read_script`

6. **Text Evaluation**
   - handled by `eval_shell`

7. **Randomized Content Preparation**
   - handled by `rand_mod`, `rand_chr`, `noise`

8. **Output Formatting**
   - handled by `prnt_bytes`, `prnt_array`, `dump_array`

9. **C Output Generation**
   - handled by `write_C`

10. **Build Step**
    - handled by `make`

11. **Exit**
    - returns through `main`

This state model is supported only at the coarse behavioral level. The current module summary is insufficient to support a more detailed behavior judgment about loops, backtracking, or repeated phase transitions.

### 3.2 Parser sub-state behavior

The separation of `parse_args` and `parse_an_arg` supports a parser sub-state machine:

- **Parser Idle**
- **Consume Next Argument**
- **Interpret Argument**
- **Update Parsed Configuration**
- **Continue or Stop**

The current module summary is insufficient to support a more detailed behavior judgment about:
- whether unknown arguments lead to an error state,
- whether some arguments require lookahead,
- whether parsing mutates `argv` or uses indices.

### 3.3 Cryptographic/transformation state behavior

The functions `stte_0`, `key`, and `arc4` imply a stateful transformation engine with at least these states:

- **Uninitialized / stale state**
- **Reset state** via `stte_0`
- **Keyed state** via `key` or `key_with_file`
- **Active transformation state** via `arc4`

The important transition constraint visible from naming is that transformation behavior depends on prior internal state preparation. The current module summary is insufficient to support a more detailed behavior judgment about whether misuse is prevented internally.

### 3.4 Output-generation state behavior

The output subsystem suggests these states:

- **No output target selected**
- **Output stream active**
- **Formatting bytes/arrays**
- **C artifact complete**
- **Build-ready**

The current module summary is insufficient to support a more detailed behavior judgment about whether partial output is rolled back on failure.

---

## 4. Error-handling flows

### 4.1 Observable error-bearing boundaries

Several functions return `int`, indicating explicit success/failure or branch-control signaling:

- `parse_an_arg`
- `key_with_file`
- `eval_shell`
- `noise`
- `write_C`
- `make`
- `main`

These are the module’s primary visible error-bearing or status-bearing boundaries.

### 4.2 Parsing error flow

Because `parse_an_arg` returns `int` while `parse_args` returns `void`, a supported behavior pattern is:

1. `parse_args` orchestrates processing,
2. `parse_an_arg` produces per-argument status,
3. `parse_args` consumes or reacts to that status.

The current module summary is insufficient to support a more detailed behavior judgment about whether parsing errors terminate the run, print diagnostics, or select defaults.

### 4.3 File/key error flow

`key_with_file(char * file)` indicates a file-dependent preparation step with a status result. A supported behavior pattern is:

1. file-based keying is requested,
2. the function attempts to use the file,
3. an integer result reports outcome,
4. caller decides whether to continue.

The current module summary is insufficient to support a more detailed behavior judgment about recovery strategy if this step fails.

### 4.4 Script read and evaluation error flow

`read_script` returns `char *` and `eval_shell` returns `int`, indicating a two-stage text-processing pipeline with distinct failure surfaces:
- acquisition failure or unusable text at read time,
- processing/evaluation failure at evaluation time.

The current module summary is insufficient to support a more detailed behavior judgment about:
- sentinel return values,
- whether null/empty text is handled separately,
- whether failure in evaluation still permits generation.

### 4.5 Output-generation and build error flow

`write_C` and `make` both return `int`, so the later pipeline stages clearly expose status to their callers. Supported behavior pattern:

1. generation may fail independently of parsing and preparation,
2. build may fail independently of generation,
3. top-level orchestration must sequence and interpret both results.

The current module summary is insufficient to support a more detailed behavior judgment about whether a build step is skipped when generation fails.

---

## 5. Boundary conditions and special-case handling

### 5.1 Argument-count boundaries

All top-level control begins with `argc`/`argv`. Therefore these boundaries exist at runtime:

- zero or minimal effective arguments,
- one or more operational arguments,
- malformed or unsupported argument sequences.

The current module summary is insufficient to support a more detailed behavior judgment about which of these are accepted.

### 5.2 Pointer-and-length boundaries

Several functions operate on caller-provided memory and lengths:

- `key(void * str, int len)`
- `arc4(void * str, int len)`
- `noise(char * ptr, unsigned min, unsigned xtra, int str)`
- `prnt_bytes(FILE * o, char * ptr, int m, int l, int n)`
- `prnt_array(FILE * o, void * ptr, char * name, int l, char * cast)`
- `dump_array(FILE * o, void * ptr, char * name, int l, char * cast)`

This establishes important boundary-sensitive behavior areas:
- zero-length operations,
- very small lengths,
- large lengths,
- null or invalid pointer inputs,
- signed/unsigned interaction at length boundaries.

The current module summary is insufficient to support a more detailed behavior judgment about how these cases are checked or rejected.

### 5.3 Random range boundaries

`rand_mod(unsigned mod)` exposes a direct modulus-style boundary:
- `mod == 0`
- `mod == 1`
- large modulus values

These are meaningful runtime edge cases because bounded random generation depends on them. The current module summary is insufficient to support a more detailed behavior judgment about how zero modulus is handled.

### 5.4 Text and file special cases

The text/file-oriented functions expose these evident boundary classes:

- empty file path,
- unreadable file path,
- empty script text,
- non-empty script text.

The current module summary is insufficient to support a more detailed behavior judgment about their exact treatment.

### 5.5 Output formatting special cases

The formatting helpers accept names, casts, and lengths. This implies special cases such as:
- empty array names,
- zero-length arrays,
- output streams not ready for writing,
- formatting requests for binary versus string-like content.

The current module summary is insufficient to support a more detailed behavior judgment about formatting fallback behavior.

---

## 6. Behaviors that must remain consistent with the C version

The following dynamic behaviors are directly grounded in the module summary and should remain consistent in any reimplementation, refactor, or port.

### 6.1 Startup remains command-line driven

- Program entry must remain through `main(int argc, char * argv[])`.
- Overall execution must remain organized around argument-driven behavior.
- `do_all` remains the visible consolidated work phase immediately below `main`.

### 6.2 Argument parsing remains a two-level operation

- There must remain a distinction between overall argument processing (`parse_args`) and at least one per-argument parsing action (`parse_an_arg`).
- Parsing must continue to influence downstream execution phases.

### 6.3 Stateful transformation support remains present

- The reset/key/transform lifecycle represented by `stte_0`, `key`, `arc4`, and `key_with_file` must remain.
- Operations that transform caller-provided buffers using a maintained internal state must continue to exist as observable behavior.

### 6.4 Script/text handling remains part of the core flow

- Reading script-like content from a file (`read_script`) must remain a distinct behavior.
- Processing text through `eval_shell` must remain a distinct behavior with a returned status.

### 6.5 Randomized data production remains part of generation behavior

- Bounded random selection (`rand_mod`),
- random character generation (`rand_chr`),
- configurable noise generation or insertion (`noise`)

must remain behaviorally available because they are first-class functions in the module.

### 6.6 Structured output emission remains layered

- Byte-level output formatting (`prnt_bytes`)
- array-level formatting (`prnt_array`, `dump_array`)
- higher-level C artifact creation (`write_C`)

must remain separate stages in the runtime flow.

### 6.7 Final generation/build sequencing remains present

- There must remain a generation phase returning status (`write_C`).
- There must remain a later build-oriented phase returning status (`make`).
- Top-level orchestration must continue to sequence these actions as part of the module’s end-to-end operation.

### 6.8 Status-bearing control points remain status-bearing

Functions currently returning `int` must continue to act as dynamic decision points in control flow rather than being collapsed into side-effect-only routines without status.

---

## 7. Performance-sensitive paths

### 7.1 Repeated argument parsing path

`parse_args` and `parse_an_arg` form a loop-oriented front-end path. Any implementation work done per argument is on the direct startup path and can affect responsiveness for long command lines.

The current module summary is insufficient to support a more detailed behavior judgment about actual complexity.

### 7.2 Buffer transformation path

`key` and especially `arc4(void * str, int len)` operate on caller-provided buffers and lengths, indicating length-proportional processing. This is a performance-sensitive path because runtime cost will scale with the amount of transformed data.

### 7.3 Script reading path

`read_script(char * file)` is performance-sensitive with respect to script size because it produces text data consumed by later stages.

The current module summary is insufficient to support a more detailed behavior judgment about whether it uses incremental or whole-buffer processing.

### 7.4 Randomized-content generation path

`noise`, `rand_mod`, and `rand_chr` are performance-sensitive when used repeatedly during output generation, especially if they populate large buffers or many formatting fragments.

### 7.5 Output formatting path

`prnt_bytes`, `prnt_array`, and `dump_array` are likely on a hot path during emission because they convert in-memory data into textual output. Formatting byte arrays is typically repetitive and output-volume dependent.

### 7.6 End-to-end generation path

`write_C` is the central likely high-cost operation in the module because it sits above reading, transformation, randomization, and formatting helpers. Any large data set or large script input will concentrate cost here.

### 7.7 Build step sensitivity

`make(void)` is the last visible phase and may dominate total runtime depending on what it does. The current module summary is insufficient to support a more detailed behavior judgment about whether its cost is internal computation or delegated work.

---

## 8. Integrated runtime narrative

At the broadest level supported by the summary, the module behaves as a command-line workflow engine for `shc`:

1. execution begins in `main`,
2. a top-level orchestration phase runs,
3. command-line arguments are parsed,
4. internal transformation state is reset and keyed,
5. script text is acquired from a file source,
6. text is evaluated or processed through a shell-related routine,
7. randomized/noise-producing helpers may prepare content,
8. byte and array formatting helpers serialize data to an output stream,
9. a C output artifact is written,
10. a build step is executed,
11. status propagates back to the process exit path.

Beyond this coarse execution narrative, the current module summary is insufficient to support a more detailed behavior judgment.