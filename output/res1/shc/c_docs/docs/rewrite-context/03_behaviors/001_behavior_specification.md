# 03_behaviors

## Scope

This document describes the runtime behavior observable from the provided module summary for `src/shc.c`. It focuses on execution flow, state progression, and behavior-preservation points. It does **not** invent semantics that are not supported by the summary.

The current module exposes a single-file execution-oriented flow centered on these functions:

- argument handling: `parse_an_arg`, `parse_args`
- internal cipher/key state handling: `stte_0`, `key`, `arc4`, `key_with_file`
- script/shell-related actions: `eval_shell`, `read_script`
- random/noise/output generation helpers: `rand_mod`, `rand_chr`, `noise`, `prnt_bytes`, `prnt_array`, `dump_array`
- generation/build orchestration: `write_C`, `make`, `do_all`, `main`

Because only function names, signatures, file locations, and aggregate call counts are available, many deeper behavior details cannot be established. Where that is the case, this document states that explicitly.

---

## 1. Initialization flow and startup order

### 1.1 Program entry

The startup sequence begins at:

- `main(int argc, char * argv[])`

From the module layout and naming, `main` is the top-level runtime entry for the module.

### 1.2 High-level startup progression

The available function set indicates a startup flow organized around command-line processing and then execution of a broader orchestration routine:

1. `main` receives process arguments.
2. `main` transfers control into higher-level module logic.
3. `do_all(int argc, char * argv[])` acts as the main orchestration function for the module.
4. Argument processing is handled by:
   - `parse_args(int argc, char * argv[])`
   - which, by naming and signature, iterates through arguments and delegates per-argument handling to:
   - `parse_an_arg(int argc, char * argv[])`

The exact call graph from `main` to `do_all` and from `do_all` to parsing is not listed explicitly in the summary, but these are the only top-level orchestration functions available, and the module has substantial internal call activity. A more detailed call ordering than this is not supported by the current module summary.

### 1.3 Initialization of internal transformation state

The functions:

- `stte_0(void)`
- `key(void * str, int len)`
- `arc4(void * str, int len)`
- `key_with_file(char * file)`

form a tightly related behavior group centered on internal state initialization, keying, and transformation.

Observed startup-relevant ordering constraints from naming and grouping:

- `stte_0` appears to be the state reset/initialization entry.
- `key` appears to apply key material into the state.
- `arc4` appears to consume the prepared state to transform a buffer.
- `key_with_file` appears to derive or apply keying based on a file.

Because the summary does not include bodies or explicit call links, the exact points where state initialization occurs during startup cannot be confirmed. However, behavior consistency requires preserving the relative role separation between:
- state reset,
- state keying,
- data transformation.

### 1.4 Generation/build startup path

The module also contains a generation pipeline:

- `read_script(char * file)`
- `write_C(char * file, char * argv[])`
- `make(void)`

This indicates a second major startup path after argument parsing: once configuration is established, the program proceeds into reading source content, generating C output, and invoking a build step.

The current module summary is insufficient to support a more detailed behavior judgment about whether all three steps always run, or whether they are conditionally selected by parsed options.

---

## 2. Main user operation flows

## 2.1 Command-line driven execution flow

The dominant user interaction surface is command-line invocation. The main operational flow is:

1. User invokes the program with `argc/argv`.
2. The program parses command-line arguments.
3. Parsed arguments affect downstream behavior.
4. The program executes one or more processing steps such as:
   - reading a script,
   - evaluating shell-related text,
   - applying keying/transformation logic,
   - generating C output,
   - triggering a build step.

The exact mapping from specific command-line options to specific later actions is not visible in the summary.

## 2.2 Argument parsing flow

### `parse_args(int argc, char * argv[])`

This function is the batch argument-processing stage. Runtime behavior that must remain stable:

- it accepts the full command-line vector,
- it performs multi-argument handling rather than one-off parsing,
- it delegates or breaks work down using `parse_an_arg`.

### `parse_an_arg(int argc, char * argv[])`

This function is the unit-level parser for one argument or one logical parsing step.

Behavioral implications from signature and naming:

- it returns an `int`, so parsing includes a control result that affects the caller’s flow,
- it receives the entire argument vector rather than a single string, so parsing behavior depends on surrounding argument context and/or current position.

The current module summary is insufficient to support a more detailed behavior judgment about:
- whether the return value encodes consumed-argument count,
- whether it signals success/failure,
- whether it selects modes.

### Combined parsing behavior

The parser subsystem therefore behaves as a stateful command-line walk:

- `parse_args` controls the overall progression through arguments,
- `parse_an_arg` processes one step and returns information used to continue that progression.

This stateful traversal behavior must remain consistent even if implementation changes.

## 2.3 Script processing flow

### `read_script(char * file)`

This function introduces a file-based content ingestion step. Dynamic behavior visible from the summary:

- it takes a file path,
- it returns a `char *`,
- therefore later steps can consume loaded textual/script content.

What must remain consistent:
- script content is obtained before any downstream text-dependent generation/evaluation step that requires it.

The current module summary is insufficient to support a more detailed behavior judgment about:
- ownership/lifetime of the returned pointer,
- encoding rules,
- whether the full file or partial file is read.

## 2.4 Shell evaluation flow

### `eval_shell(char * text)`

This function forms a text-to-result execution/evaluation path. Dynamic behavior visible from the summary:

- it consumes a text buffer,
- it returns an `int`,
- it represents an active execution/evaluation stage rather than passive formatting.

Preserved behavior constraints:
- shell-related evaluation is a runtime action occurring after text is available,
- its result influences later control flow or outcome reporting.

The current module summary is insufficient to support a more detailed behavior judgment about:
- whether the text is interpreted directly or transformed first,
- whether the return value is a shell exit status, validation result, or another control code.

## 2.5 Keyed transformation flow

The functions `stte_0`, `key`, `arc4`, and `key_with_file` together define a separate operational path where some internal transformation state is prepared and then applied to data.

Likely runtime ordering supported by function roles:

1. initialize/reset internal state with `stte_0`
2. apply key material:
   - direct with `key(void * str, int len)`, or
   - file-derived with `key_with_file(char * file)`
3. transform a target buffer with `arc4(void * str, int len)`

This sequence is stronger than isolated function existence because the names indicate an interdependent transformation lifecycle. Even so, the exact caller and exact input buffers are not known from the summary.

## 2.6 Output generation flow

The module has a structured generation/output path:

- `prnt_bytes`
- `prnt_array`
- `dump_array`
- `write_C`

This suggests the following runtime behavior:

1. raw/script/transformed data becomes available,
2. helper routines serialize or format memory regions,
3. `write_C` emits a generated C representation to a file.

### Formatting helper behavior

- `prnt_bytes(FILE * o, char * ptr, int m, int l, int n)`:
  runtime behavior involves writing formatted byte-oriented output to a stream.
- `prnt_array(FILE * o, void * ptr, char * name, int l, char * cast)`:
  runtime behavior involves printing a named array representation.
- `dump_array(FILE * o, void * ptr, char * name, int l, char * cast)`:
  runtime behavior involves array emission, likely as a wrapper or alternate formatting path.

What must remain consistent:
- output-generation helpers are stream-oriented,
- `write_C` is the higher-level generation stage that depends on such formatting behavior.

The current module summary is insufficient to support a more detailed behavior judgment about output syntax or formatting exactness.

## 2.7 Build invocation flow

### `make(void)`

This function indicates a post-generation action that advances the process from source generation to build execution.

Runtime behavior visible from the summary:

- it is a no-argument stage,
- it returns an `int`,
- it sits after code generation in the function inventory.

Preserved behavior constraints:
- build triggering remains a distinct stage rather than being merged invisibly into unrelated logic,
- the build stage produces a success/failure-like control result consumed by callers.

The current module summary is insufficient to support a more detailed behavior judgment about the build tool, command string, or environmental dependencies.

## 2.8 End-to-end orchestration flow

### `do_all(int argc, char * argv[])`

This function appears to be the main procedural coordinator. The likely runtime structure is:

1. receive process arguments,
2. invoke parsing,
3. select/prepare input,
4. process keying/transformation and/or shell/script evaluation,
5. emit generated C,
6. invoke build,
7. return control to `main`.

Because `do_all` returns `void`, outcome signaling likely occurs through side effects, global state, later `main` handling, or helper return values. The current module summary is insufficient to support a more detailed behavior judgment.

---

## 3. State machines and state transitions

## 3.1 Command-line parsing state machine

A clear stateful subsystem exists in argument parsing.

### States
- **Start**: process begins with raw `argc/argv`
- **Scanning**: `parse_args` is iterating over arguments
- **Per-argument handling**: `parse_an_arg` evaluates one parsing unit
- **Updated parse state**: parser advances based on the returned result from `parse_an_arg`
- **Parsing complete**: all arguments have been handled or parsing terminates

### Transitions
- `main` / orchestrator enters **Scanning**
- `parse_args` enters **Per-argument handling**
- `parse_an_arg` returns control information
- parser advances to next argument or next parse phase
- parser reaches **Parsing complete**

What must remain consistent:
- parsing is not a stateless single-call lookup,
- parser progress depends on prior parser position and current argument context.

## 3.2 Cipher/key state machine

The functions `stte_0`, `key`, and `arc4` strongly indicate a mutable internal state machine.

### States
- **Uninitialized/unknown state**
- **Initialized state** after `stte_0`
- **Keyed state** after one or more `key` applications or `key_with_file`
- **Transforming state** during `arc4`
- **Post-transform state** after `arc4` completes

### Transitions
- `stte_0` transitions internal cipher state into a reset baseline
- `key` transitions from initialized to keyed, or from keyed to re-keyed/updated
- `key_with_file` transitions the system by incorporating file-based key input
- `arc4` consumes current state while processing a buffer

Preserved behavior constraints:
- transformation must continue to depend on prior state preparation,
- direct transformation without proper state lifecycle would change dynamic behavior.

The current module summary is insufficient to support a more detailed behavior judgment about:
- whether state is global/static,
- whether `arc4` mutates state permanently,
- whether multiple `key` calls are cumulative.

## 3.3 Generation pipeline state machine

The module also shows a producer pipeline.

### States
- **No source content loaded**
- **Source content available** after `read_script`
- **Generated output in progress** during `write_C`
- **Generated output complete**
- **Build pending**
- **Build executed** after `make`

### Transitions
- file input enters via `read_script`
- generation begins in `write_C`
- helper printing routines advance output emission
- completion of `write_C` enables `make`

What must remain consistent:
- build execution is downstream of generation,
- helper printers remain subordinate to a higher-level generation phase.

## 3.4 Random/noise generation state machine

The helper group:

- `rand_mod`
- `rand_chr`
- `noise`

indicates another small runtime state progression around random-derived output or mutation.

### States
- **Random source ready**
- **Random value requested** via `rand_mod`
- **Random character requested** via `rand_chr`
- **Noise insertion/generation active** via `noise`
- **Noise generation complete**

The current module summary is insufficient to support a more detailed behavior judgment about whether these routines:
- mutate buffers,
- inject text,
- pad generated code,
- depend on a seeded PRNG state.

Still, the behavior category is dynamic rather than declarative: these helpers generate variable content used by later output or transformation stages.

---

## 4. Error-handling flows

## 4.1 Return-bearing control points

Several functions return `int` and therefore participate in explicit runtime control flow:

- `parse_an_arg`
- `key_with_file`
- `eval_shell`
- `noise`
- `write_C`
- `make`
- `main`

Error-handling behavior that must remain stable at minimum:

- these functions produce control outcomes that callers can branch on,
- callers must preserve distinction between successful progress and non-successful progress,
- orchestration must continue to honor returned control information.

The current module summary is insufficient to support a more detailed behavior judgment about the meaning of specific return values.

## 4.2 Parsing-related failure flow

The parser subsystem includes a returned result from `parse_an_arg`, so parse failure or special control transfer is part of the runtime behavior.

Behaviorally stable expectations:
- invalid, unsupported, or specially handled arguments alter parse progression,
- `parse_args` must react to `parse_an_arg`’s result rather than ignoring it.

The current module summary is insufficient to support a more detailed behavior judgment about whether parsing errors terminate the program, emit diagnostics, or skip arguments.

## 4.3 File-related failure flow

The module contains two file-oriented operations:

- `key_with_file(char * file)`
- `read_script(char * file)`

This means file access failures are part of runtime behavior.

Preserved behavior constraints:
- failure in file-based key acquisition remains distinguishable from successful keying,
- script read failure remains distinguishable from successful content acquisition because `read_script` returns a pointer that later steps depend on.

The current module summary is insufficient to support a more detailed behavior judgment about:
- null returns,
- retry logic,
- fallback file sources.

## 4.4 Generation/build failure flow

`write_C` and `make` both return `int`, indicating explicit control outcomes for generation and build stages.

Behavioral flow:
- generation may succeed or fail,
- build may succeed or fail,
- orchestration must preserve sequencing and respect stage results.

The current module summary is insufficient to support a more detailed behavior judgment about whether build is skipped after generation failure or whether partial outputs are retained.

## 4.5 Evaluation failure flow

`eval_shell` also returns `int`, so shell evaluation has an explicit success/non-success path.

Behavior-preservation points:
- evaluation is not fire-and-forget,
- its outcome participates in later control flow.

The current module summary is insufficient to support a more detailed behavior judgment about exact failure consequences.

---

## 5. Boundary conditions and special-case handling

## 5.1 Empty or minimal argument lists

Since the module begins with `main(argc, argv)` and immediately centers on parsing, handling of minimal command-line input is a boundary condition.

What can be stated safely:
- the parser must handle the actual `argc/argv` passed at runtime,
- argument-processing flow must remain well-defined even when the argument list is short.

The current module summary is insufficient to support a more detailed behavior judgment about default actions when no user options are supplied.

## 5.2 Variable-length buffer handling

Several functions operate on caller-supplied buffers and explicit lengths:

- `key(void * str, int len)`
- `arc4(void * str, int len)`
- `prnt_bytes(FILE * o, char * ptr, int m, int l, int n)`
- `prnt_array(FILE * o, void * ptr, char * name, int l, char * cast)`
- `dump_array(FILE * o, void * ptr, char * name, int l, char * cast)`

Boundary conditions that must remain consistent:
- length parameters govern runtime processing extent,
- the functions are designed for non-fixed-size data,
- behavior depends on the provided length rather than implicit termination alone.

The current module summary is insufficient to support a more detailed behavior judgment about zero-length handling, negative-value handling, or truncation rules.

## 5.3 File path edge cases

Functions accepting file paths:
- `key_with_file`
- `read_script`
- `write_C`

must retain distinct handling for the provided path argument. Boundary-sensitive aspects include:
- file argument presence,
- file accessibility,
- file usability in downstream stages.

The current module summary is insufficient to support a more detailed behavior judgment about path normalization or special filename conventions.

## 5.4 Modulus and random boundaries

`rand_mod(unsigned mod)` exposes an explicit modulus boundary.

Behavior that must remain consistent:
- random generation is constrained by a caller-supplied bound,
- `rand_chr` is derived from a random-character selection flow rather than arbitrary unrelated logic,
- `noise` uses randomness-related helpers or equivalent behavior to generate content.

The current module summary is insufficient to support a more detailed behavior judgment about:
- `mod == 0`,
- distribution guarantees,
- deterministic seeding behavior.

## 5.5 Stream/output boundaries

Formatting functions accept `FILE * o`, which creates stream-boundary conditions:
- valid output stream supplied,
- variable output lengths,
- repeated emission over a stream.

Behavior-preservation points:
- helper routines remain stream-writing operations,
- generated output must still be emit-able incrementally through these helpers.

The current module summary is insufficient to support a more detailed behavior judgment about line wrapping, indentation, or flushing behavior.

---

## 6. Behaviors that must remain consistent with the C version

## 6.1 Top-level orchestration order

The C version defines a staged execution model involving:
- program entry,
- argument parsing,
- processing/generation actions,
- optional build/evaluation-related stages.

This staged runtime shape must remain consistent:
- `main` remains the single process entry,
- orchestration remains centralized rather than fragmented into unrelated flows,
- parser-driven configuration continues to precede downstream work.

## 6.2 Stateful transformation lifecycle

The C version separates:
- state initialization (`stte_0`),
- state keying (`key`, `key_with_file`),
- transformation (`arc4`).

That lifecycle separation is behaviorally important and must remain intact. Any rewrite must preserve that transformation depends on prior state setup.

## 6.3 Parser-driven control flow

The presence of both `parse_args` and `parse_an_arg` indicates:
- iterative parsing,
- per-step parse control,
- caller reaction to parser results.

This parser progression behavior must remain consistent even if option tables or implementation details change.

## 6.4 Generation helpers as subordinate emitters

The C version has helper emitters (`prnt_bytes`, `prnt_array`, `dump_array`) beneath a higher-level generator (`write_C`). This hierarchy must remain visible in behavior:
- helpers format pieces of output,
- `write_C` owns full output-file generation.

## 6.5 Multi-stage output pipeline

The overall runtime includes distinct phases for:
- reading content,
- evaluating or transforming content,
- generating C output,
- building produced artifacts.

Even where conditional, these are separate operational phases in the C version. Their separation and ordering constraints must remain consistent.

## 6.6 Return-based branching behavior

Functions returning `int` must continue to drive runtime branching in the translated behavior. Even without confirmed numeric semantics, the distinction between:
- continuing successfully,
- alternative control path,
- non-success path

must be preserved.

## 6.7 Data-length-driven processing

The C version repeatedly uses explicit length arguments. Any equivalent implementation must preserve data processing based on explicit byte/element counts, not only on sentinel-terminated strings.

---

## 7. Performance-sensitive paths

## 7.1 Repeated argument parsing path

`parse_args` and `parse_an_arg` participate in a loop-like traversal over command-line inputs. This is a repeated execution path and therefore performance-relevant in proportion to argument count.

What must remain stable:
- per-argument processing remains bounded to argument traversal,
- parser result handling does not introduce unnecessary repeated rescans unless present in the C implementation.

The current module summary is insufficient to support a more detailed behavior judgment about actual parser complexity.

## 7.2 Buffer transformation path

`key` and especially `arc4` take a pointer and length, indicating byte-range processing over caller-supplied memory. This is a naturally performance-sensitive path because runtime cost scales with `len`.

Preserved behavior constraints:
- transformation remains buffer-oriented,
- per-byte or per-range semantics implied by `(ptr, len)` remain intact.

## 7.3 Output emission path

`prnt_bytes`, `prnt_array`, `dump_array`, and `write_C` form an output-intensive path.

Performance relevance:
- runtime work scales with generated content size,
- helper functions are likely called repeatedly during generation,
- stream output behavior is central to end-to-end runtime cost.

Behaviorally important:
- helper usage should remain aligned with generation flow rather than duplicating formatting work across multiple layers.

## 7.4 Random/noise generation path

`rand_mod`, `rand_chr`, and `noise` are likely called repeatedly when generating randomized or obfuscating content. This makes them performance-sensitive when output size is large or when noise insertion is frequent.

The current module summary is insufficient to support a more detailed behavior judgment about whether this path is dominant or auxiliary.

## 7.5 File-based processing path

`read_script`, `key_with_file`, and `write_C` all imply file I/O boundaries. For large input/output files, this path is inherently performance-sensitive.

What must remain consistent:
- file-based stages stay in the critical execution path when selected,
- downstream processing depends on completion of these I/O stages.

## 7.6 Build stage latency

`make` may be a major wall-clock contributor because it represents a separate build step after generation. Even though internal details are unavailable, this stage should be treated as operationally expensive relative to in-process helper calls.

---

## 8. Overall dynamic behavior summary

At runtime, this module behaves as a command-line driven orchestration unit with several coordinated subsystems:

1. **Entry and orchestration**
   - `main` starts execution.
   - `do_all` coordinates the overall procedure.

2. **Configuration and mode selection**
   - `parse_args` and `parse_an_arg` walk and interpret command-line inputs.

3. **State preparation**
   - internal transformation state is reset and keyed through `stte_0`, `key`, and/or `key_with_file`.

4. **Content acquisition and processing**
   - script text is obtained via `read_script`.
   - text may be evaluated through `eval_shell`.
   - data may be transformed through `arc4`.

5. **Generation**
   - formatted byte/array emitters support `write_C` in producing C output.

6. **Post-generation action**
   - `make` executes a build-related stage.

7. **Control outcomes**
   - multiple `int`-returning functions feed success/non-success information back into the orchestration flow.

Where exact runtime details are not derivable from the provided summary, the current module summary is insufficient to support a more detailed behavior judgment.