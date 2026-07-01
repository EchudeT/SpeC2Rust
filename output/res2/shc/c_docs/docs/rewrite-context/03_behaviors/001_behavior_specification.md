# 03_behaviors

## Scope and evidence basis

This behavior document is derived strictly from the provided module analysis summary for `src/shc.c`. It describes observable runtime organization from the available function set and naming/call-structure signals only.

Where implementation-body details are not present in the summary, the document explicitly states that **the current module summary is insufficient to support a more detailed behavior judgment**.

---

## 1. Initialization flow and startup order

### 1.1 Program entry sequence

The module exposes a conventional top-level startup chain:

1. `main(int argc, char * argv[])`
2. `do_all(int argc, char * argv[])`
3. lower-level helpers involved in argument handling, transformation, output generation, and build actions

From the function list and naming, the runtime begins in `main`, then transfers control into `do_all`, which serves as the main orchestration point for the module's work.

### 1.2 Early argument-processing stage

The module contains a two-level argument parsing flow:

- `parse_args(int argc, char * argv[])`
- `parse_an_arg(int argc, char * argv[])`

This indicates startup includes:
- a whole-argument-set parsing pass in `parse_args`
- per-argument or per-option processing delegated to `parse_an_arg`

The current module summary is insufficient to support a more detailed behavior judgment about:
- whether parsing is single-pass or iterative over all arguments
- whether `parse_an_arg` advances an index or consumes multiple tokens at once
- which options are mandatory or optional
- exact startup failure behavior on invalid arguments

### 1.3 Internal cryptographic/state initialization

The function set includes:

- `stte_0(void)`
- `key(void * str, int len)`
- `arc4(void * str, int len)`
- `key_with_file(char * file)`

These names indicate a startup or pre-processing phase that initializes internal transformation state and then seeds or modifies that state using text or file-derived material.

A behaviorally meaningful startup order is visible at the level of responsibilities:

1. initialize internal state with `stte_0`
2. inject keying material via `key` and/or `key_with_file`
3. apply stream transformation with `arc4` to target buffers
4. continue with script processing, code generation, or build steps

The current module summary is insufficient to support a more detailed behavior judgment about:
- whether `stte_0` is always called exactly once
- whether state reset occurs between operations
- whether `key_with_file` is mandatory before `arc4`
- whether these routines are used only during generation, only during execution, or both

### 1.4 Script/content acquisition stage

The presence of:

- `read_script(char * file)`
- `eval_shell(char * text)`

shows a startup-to-processing flow where program operation reaches a point of obtaining script text from a file and evaluating shell-related content.

A high-level startup progression supported by the summary is:

1. parse invocation arguments
2. obtain source/input script text from a file path
3. process or transform content
4. evaluate shell context and/or generate output
5. optionally invoke a build action

The current module summary is insufficient to support a more detailed behavior judgment about the exact ordering of `read_script`, `eval_shell`, `write_C`, and `make`.

---

## 2. Main user operation flows

### 2.1 Top-level orchestration flow

The module structure suggests one central end-to-end workflow managed by `do_all`. The runtime behavior is organized around a sequence of phases rather than independent services.

Observed functional phases are:

- command-line parsing
- internal state preparation
- script reading
- shell-related evaluation
- randomized/noise data generation
- formatted byte/array emission
- C source output generation
- build/compilation step

### 2.2 Argument-driven control flow

`parse_args` and `parse_an_arg` define the user-facing operation entry behavior. User-supplied arguments determine which downstream actions run.

Because `write_C`, `make`, and script-processing functions coexist in the same orchestration module, user operation flow includes control decisions that enable or skip later phases.

The current module summary is insufficient to support a more detailed behavior judgment about:
- which command-line options switch between generation-only and generation-plus-build flows
- whether any mode executes decrypted/evaluated content directly
- whether shell evaluation is a validation step or a runtime-execution step

### 2.3 Script ingestion and transformation flow

The main content-processing behavior includes at least these transitions:

1. user identifies a file or script-related input
2. `read_script` retrieves textual content
3. internal state may be initialized and keyed
4. `arc4` transforms one or more buffers
5. downstream output is generated or shell-related analysis occurs

This indicates dynamic behavior centered on moving content through successive representations:
- file path
- loaded text buffer
- transformed/encrypted/decrypted byte sequence
- emitted C representation
- optional build artifact

The current module summary is insufficient to support a more detailed behavior judgment about whether transformation occurs before storage, before output, or around execution.

### 2.4 Output generation flow

The module contains a coherent output-emission pipeline:

- `prnt_bytes(FILE * o, char * ptr, int m, int l, int n)`
- `prnt_array(FILE * o, void * ptr, char * name, int l, char * cast)`
- `dump_array(FILE * o, void * ptr, char * name, int l, char * cast)`
- `write_C(char * file, char * argv[])`

This indicates an operation flow in which internal byte/text data is formatted and written to an output stream, then assembled into a generated C source file.

Behaviorally, the flow is:

1. obtain or construct byte-oriented data
2. format that data into printable output fragments
3. emit named arrays or equivalent structured C fragments
4. assemble a complete generated file through `write_C`

`dump_array` appears behaviorally subordinate to the array-printing path and likely acts as a convenience wrapper or simplified emission path, but the current module summary is insufficient to support a more detailed behavior judgment.

### 2.5 Build/invocation flow

The presence of `make(void)` after `write_C` strongly indicates a second major user workflow phase:

1. generate C output
2. invoke a build/compilation step

This separates the main operation into:
- generation phase
- build phase

The current module summary is insufficient to support a more detailed behavior judgment about:
- whether `make` is always invoked after successful generation
- whether `make` calls an external build tool or performs internal compilation orchestration
- what conditions suppress the build phase

### 2.6 Randomization/noise generation flow

The module includes:

- `rand_mod(unsigned mod)`
- `rand_chr(void)`
- `noise(char * ptr, unsigned min, unsigned xtra, int str)`

This establishes a runtime flow in which pseudo-random or randomized content is generated and inserted into buffers. Behaviorally:

1. a caller requests randomized values under a modulus or character domain
2. `noise` writes randomized data into a caller-supplied buffer
3. resulting data is used in output generation or obfuscation-related processing

The current module summary is insufficient to support a more detailed behavior judgment about:
- whether noise is used for source obfuscation, padding, naming, or data masking
- whether `str` changes termination or output character constraints
- whether `min` and `xtra` control a minimum-plus-random-length policy

---

## 3. State machines and state transitions

### 3.1 Global execution state machine

At a coarse level, the program behaves like a linear phase machine:

1. **Startup**
   - enter `main`
   - transfer to `do_all`

2. **Argument Parsing**
   - process invocation arguments via `parse_args`
   - process individual argument units via `parse_an_arg`

3. **Preparation**
   - initialize transformation state with `stte_0`
   - load key material with `key` or `key_with_file`

4. **Content Processing**
   - read script text with `read_script`
   - evaluate shell-related content with `eval_shell`
   - transform byte buffers with `arc4`

5. **Emission**
   - create formatted byte/array output
   - generate a C file using `write_C`

6. **Build**
   - perform build step through `make`

7. **Exit**
   - return from orchestration to `main`
   - terminate process

This is the highest-confidence state model supported by the summary.

### 3.2 Internal transformation-state lifecycle

The functions `stte_0`, `key`, and `arc4` imply an internal mutable transformation state.

Behaviorally, the state machine is:

- **Uninitialized/previous state**
- `stte_0` transitions state to **initialized/reset**
- `key` or `key_with_file` transitions state to **keyed**
- `arc4` consumes the keyed state while transforming data, transitioning to **advanced/used** state

The current module summary is insufficient to support a more detailed behavior judgment about:
- whether `arc4` is valid before keying
- whether repeated `arc4` calls continue the same stream state
- whether any function restores the state to a prior checkpoint

### 3.3 Output-construction substate flow

For generated output, another substate flow is visible:

- **No output open/selected**
- `write_C` begins output composition
- `prnt_bytes` emits raw/formatted data fragments
- `prnt_array` or `dump_array` emits declaration-style structures
- **Output complete**

The current module summary is insufficient to support a more detailed behavior judgment about buffering, flushing, or ordering constraints among the print helpers.

### 3.4 Random-data substate flow

The randomization helpers suggest a simple request/produce/commit cycle:

- caller enters **needs randomized content**
- `rand_mod`/`rand_chr` produce randomized unit values
- `noise` writes units into a provided region
- caller transitions to **buffer populated**

The current module summary is insufficient to support a more detailed behavior judgment about deterministic seeding, reproducibility, or cross-run stability.

---

## 4. Error-handling flows

### 4.1 Error-reporting structure visible from signatures

Several functions return `int`, which indicates explicit success/failure or status propagation paths exist:

- `parse_an_arg`
- `key_with_file`
- `eval_shell`
- `write_C`
- `make`
- `main`

Other orchestration functions return `void`, indicating they may handle errors internally or terminate control flow through shared state rather than direct return values:

- `parse_args`
- `do_all`

The current module summary is insufficient to support a more detailed behavior judgment about exact status meanings.

### 4.2 Argument error flow

Because argument parsing is split across `parse_args` and `parse_an_arg`, an error-handling path exists where an individual argument fails validation or processing.

Behaviorally:

1. `parse_args` dispatches an argument to `parse_an_arg`
2. `parse_an_arg` returns a status
3. upstream control flow reacts by either continuing parsing or stopping the operation

The current module summary is insufficient to support a more detailed behavior judgment about:
- whether invalid arguments cause immediate termination
- whether usage/help text is emitted
- whether unrecognized arguments are ignored or rejected

### 4.3 File-related error flow

`key_with_file(char * file)` and `read_script(char * file)` indicate file-dependent runtime steps. Error paths must exist for unavailable or unusable file inputs.

Behaviorally:
- a file-based operation is requested
- file access or file-derived processing is attempted
- failure prevents normal downstream progression in keying and/or script ingestion

The current module summary is insufficient to support a more detailed behavior judgment about:
- whether `read_script` returns a null pointer on failure
- whether `key_with_file` distinguishes open/read/format errors
- whether failure is recoverable with alternative inputs

### 4.4 Generation/build error flow

`write_C` and `make` each return `int`, showing that output generation and build execution are distinct failure points.

Behaviorally:
- if generated source creation fails, build cannot proceed normally
- if build fails after source generation, the workflow reaches a later-stage failure

This creates at least two separable operational error states:

1. **generation failure**
2. **build failure**

The current module summary is insufficient to support a more detailed behavior judgment about cleanup actions or partial-output retention.

### 4.5 Shell-evaluation error flow

`eval_shell` returns `int`, indicating shell-related text evaluation can fail as a runtime decision point.

Behaviorally:
- text is supplied for shell evaluation
- evaluation returns a status
- upstream logic branches based on that result

The current module summary is insufficient to support a more detailed behavior judgment about whether failure means syntax rejection, unsupported shell form, or execution refusal.

---

## 5. Boundary conditions and special-case handling

### 5.1 Argument-count and empty-input boundaries

Since the entry chain accepts `argc` and `argv`, startup behavior necessarily depends on argument count and argument presence.

The module summary supports these boundary categories:
- no user arguments
- one or more arguments
- invalid or incomplete argument sequences

The current module summary is insufficient to support a more detailed behavior judgment about the exact behavior for zero-argument invocation.

### 5.2 Zero-length and small-buffer boundaries

The signatures for:

- `key(void * str, int len)`
- `arc4(void * str, int len)`
- `prnt_array(..., int l, ...)`
- `prnt_bytes(..., int m, int l, int n)`

show repeated length-controlled processing. This means runtime behavior must handle size boundaries such as:
- zero-length buffers
- single-byte buffers
- short arrays
- larger arrays

The current module summary is insufficient to support a more detailed behavior judgment about whether zero lengths are treated as no-ops or errors.

### 5.3 Modulus boundary in random generation

`rand_mod(unsigned mod)` establishes a clear boundary-sensitive path around the `mod` parameter.

Because modulus-based generation depends directly on the provided bound, special handling for very small values, especially zero, is a relevant behavioral boundary.

The current module summary is insufficient to support a more detailed behavior judgment about zero-modulus handling.

### 5.4 File-path and content boundaries

`read_script` and `key_with_file` operate on `char * file`, so behavior depends on:
- empty path strings
- invalid paths
- inaccessible files
- files with unusual content lengths

The current module summary is insufficient to support a more detailed behavior judgment about path normalization, size limits, or binary-versus-text distinctions.

### 5.5 Noise-generation boundaries

`noise(char * ptr, unsigned min, unsigned xtra, int str)` exposes several boundary-sensitive controls:
- target pointer validity
- minimum amount `min`
- extra/random amount `xtra`
- mode/control flag `str`

The current module summary is insufficient to support a more detailed behavior judgment about:
- whether `xtra == 0` disables variability
- whether `str` requests string-safe output or termination behavior
- maximum writable length expectations

### 5.6 Output-formatting boundaries

`prnt_bytes`, `prnt_array`, and `dump_array` all operate on caller-provided output streams and length values. Behaviorally relevant boundaries include:
- empty output
- exact line/column breaks controlled by integer parameters
- named arrays with zero elements
- raw pointers cast under different textual type names

The current module summary is insufficient to support a more detailed behavior judgment about formatting conventions under these edge conditions.

---

## 6. Behaviors that must remain consistent with the C version

### 6.1 Entry and orchestration order

The following behavioral ordering must remain intact:

- `main` is the entry point
- `do_all` acts as the main orchestration body
- argument parsing occurs before dependent operations
- generation/build activities occur only after earlier prerequisite phases

### 6.2 Separation between per-argument and whole-parse logic

The distinction between:
- `parse_args`
- `parse_an_arg`

must remain preserved. Any reimplementation must keep the behavior in which overall parsing delegates individual argument handling to a lower-level unit.

### 6.3 Stateful transformation semantics

The behavioral relationship among:
- `stte_0`
- `key`
- `arc4`
- `key_with_file`

must remain consistent as a mutable-state transformation pipeline. In particular:
- initialization/reset is distinct from key injection
- key injection is distinct from byte transformation
- file-derived keying remains a separate operation from direct keying

### 6.4 Distinct script-read and shell-evaluation stages

`read_script` and `eval_shell` must remain separate behavioral stages. A reimplementation should not collapse them into an indistinguishable single step if that would alter call timing, failure points, or state transitions.

### 6.5 Distinct output-generation helper roles

The helper layering among:
- `prnt_bytes`
- `prnt_array`
- `dump_array`
- `write_C`

must remain recognizable. The runtime behavior should continue to use lower-level formatting/emission helpers as part of a larger C-file generation flow.

### 6.6 Separation of generation and build phases

`write_C` and `make` are separate operations with separate return-bearing control points. This separation must remain intact:
- source generation is one phase
- build execution is another phase
- each can fail independently

### 6.7 Randomized helper behavior as explicit sub-operations

`rand_mod`, `rand_chr`, and `noise` must remain explicit runtime contributors to content generation rather than being removed or merged in a way that changes when or how randomized content enters the flow.

---

## 7. Performance-sensitive paths

### 7.1 Byte-wise transformation path

`arc4(void * str, int len)` is a length-driven buffer operation and is therefore a performance-sensitive path. Runtime cost scales with processed data length. Any implementation change should preserve efficient sequential processing characteristics.

The current module summary is insufficient to support a more detailed behavior judgment about algorithmic complexity beyond buffer-length dependence.

### 7.2 Script reading path

`read_script(char * file)` is performance-sensitive for larger inputs because it forms the content-acquisition stage for downstream operations. Delays here propagate into the entire workflow.

The current module summary is insufficient to support a more detailed behavior judgment about buffering strategy or file read granularity.

### 7.3 Output formatting and emission path

`prnt_bytes`, `prnt_array`, `dump_array`, and `write_C` together define a potentially high-volume formatting path. If large byte arrays are emitted into generated C code, this path can dominate runtime through repeated formatting and stream writes.

Performance-sensitive characteristics that must remain respected:
- repeated per-byte formatting work
- repeated stream output calls
- length-driven output expansion from binary/text buffers to textual C source

### 7.4 Random-noise generation path

`noise` can become performance-sensitive when asked to populate larger regions or when invoked repeatedly. Since it likely depends on `rand_mod` and/or `rand_chr`, repeated random-unit generation may amplify cost.

The current module summary is insufficient to support a more detailed behavior judgment about whether this path is hot in typical runs.

### 7.5 Build phase as end-to-end latency contributor

`make(void)` is a major latency stage in full workflows because it follows output generation and likely represents the final step before completion.

Even without external dependency metadata, behaviorally it is a late-stage operation whose duration dominates user-visible completion time when enabled.

The current module summary is insufficient to support a more detailed behavior judgment about internal versus delegated build cost.

---

## 8. Consolidated runtime behavior picture

At the highest supported level, this module behaves as a single-process orchestrator for a script-oriented transformation and generation pipeline:

1. start in `main`
2. enter `do_all`
3. parse command-line input through `parse_args` and `parse_an_arg`
4. initialize and key internal transformation state through `stte_0`, `key`, and/or `key_with_file`
5. obtain script content via `read_script`
6. evaluate shell-related text via `eval_shell`
7. transform content buffers with `arc4`
8. generate randomized/noise content when needed through `rand_mod`, `rand_chr`, and `noise`
9. format and emit arrays/bytes using `prnt_bytes`, `prnt_array`, and `dump_array`
10. produce generated C source through `write_C`
11. perform a build step through `make`
12. return control and exit

This phased control flow, the existence of mutable transformation state, the separation of generation from build, and the presence of explicit formatting/randomization helpers are the primary dynamic behaviors that must be preserved.