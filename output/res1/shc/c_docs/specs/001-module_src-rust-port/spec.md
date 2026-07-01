# spec.md

## Title

Rust Port Functional Specification: `module_src` for `shc`

## Document Control

- Project: `shc`
- Module: `module_src`
- Category: `module`
- Source file scope: `src/shc.c`
- Target Rust branch: `001-module_src-rust-port`
- Generation date: 2026-06-06

## Overview

This module is the command-line driver and code-generation unit for `shc`. It accepts user options, reads an input shell script, applies the module’s byte transformation and keying workflow, emits generated C source containing transformed script data and runtime support content, and can invoke compilation of the generated output.

The Rust rewrite must preserve the functional role of this module as an end-to-end transformation pipeline:

1. parse command-line arguments,
2. validate and collect generation settings,
3. read the source script and related file metadata,
4. prepare keyed/transformed byte content,
5. generate C output text with embedded arrays and helper content,
6. optionally invoke the build step.

This specification is limited to functionality evidenced by `src/shc.c` and its listed functions.

## In-Scope Functionality

The Rust version must implement the behavior covered by the following functional areas evidenced in `src/shc.c`:

- command-line argument parsing and dispatch,
- script file loading,
- file-derived keying input,
- internal state initialization and byte transformation operations,
- random/noise generation used during output generation,
- formatted emission of byte arrays and generated C source,
- orchestration of full processing flow,
- optional build invocation,
- program entry behavior.

## Out of Scope

The Rust port specification does not require functionality not evidenced in the analyzed module, including:

- new command-line options,
- new output formats beyond generated C source and the build step already implied by the module,
- public library APIs beyond what is needed to preserve module behavior,
- concurrency guarantees,
- persistence or serialization formats other than generated files,
- recovery, rollback, or remote execution features.

## Feature Specification

### Feature 1: Command-Line Driven Processing

The module accepts process arguments, interprets supported options, and derives the operational mode and inputs required to process a shell script into generated output.

The Rust rewrite must:

- accept program arguments equivalent in purpose to the C module,
- parse arguments one-by-one and as a complete argument vector,
- distinguish valid option forms from invalid or incomplete input,
- collect the effective source script path and output-related settings needed by the later stages,
- preserve the module’s top-level control flow from argument parsing into execution.

**Traceability:** `parse_an_arg`, `parse_args`, `do_all`, `main`

### Feature 2: Script and File-Based Input Handling

The module reads the input shell script from disk and also derives key material from file metadata for a named file.

The Rust rewrite must:

- read the full contents of the target script file for later embedding,
- fail when the script cannot be opened or read,
- inspect the named file required for file-based keying and use its metadata-dependent properties as a gating input to key setup,
- propagate failure from file inspection when the required file is unavailable or invalid for this purpose.

**Traceability:** `read_script`, `key_with_file`

### Feature 3: Stateful Keying and Byte Transformation

The module maintains an internal transformation state, initializes it, mixes key material into it, and applies the transformation to byte buffers.

The Rust rewrite must:

- provide the same functional stages of state reset, key mixing, and byte-buffer transformation,
- support repeated application across different generated data items in the same processing flow,
- preserve the dependency ordering where state initialization and keying precede transformation of protected content.

This specification requires behavioral equivalence of the transformation workflow used by the module, not source-level reproduction of C internals.

**Traceability:** `stte_0`, `key`, `arc4`, `key_with_file`, `write_C`

### Feature 4: Shell Validation/Evaluation Support

The module evaluates shell-related text to determine shell suitability or derive shell handling decisions used during generation.

The Rust rewrite must:

- preserve the module’s ability to process shell text input for shell-related decision making,
- return success/failure in a way that can influence downstream generation flow.

**Traceability:** `eval_shell`

### Feature 5: Randomized Noise Generation for Output

The module generates bounded random values, random characters, and synthetic noise content used during output generation.

The Rust rewrite must:

- support generation of random values constrained by a requested modulus,
- support generation of random characters,
- support generation of filler/noise into a provided byte region under caller-specified bounds and mode flags,
- preserve the functional use of this noise as part of generated output construction.

The exact random sequence need not be fixed unless required to preserve observable module behavior from the original tool invocation.

**Traceability:** `rand_mod`, `rand_chr`, `noise`, `write_C`

### Feature 6: Formatted Array and Byte Emission

The module emits transformed data as formatted C source fragments, including byte-oriented formatting and named array declarations.

The Rust rewrite must:

- output formatted byte data suitable for inclusion in generated C source,
- output named arrays for transformed or generated content,
- support emitting arrays with the declaration metadata needed by the generated C file.

**Traceability:** `prnt_bytes`, `prnt_array`, `dump_array`, `write_C`

### Feature 7: Generated C Source Construction

The module writes a complete generated C file that embeds transformed script-related data and supporting content derived from the current options and inputs.

The Rust rewrite must:

- create the designated output C source file,
- include embedded data derived from the source script and module processing flow,
- include the generated declarations and code fragments needed by the downstream build step,
- fail clearly when output cannot be written.

This requirement is about functional generation of the C artifact, not textual identity line-for-line unless required for build correctness.

**Traceability:** `write_C`

### Feature 8: Build-Step Invocation

The module can invoke a build step after generation to compile the produced C source into a resulting executable or equivalent artifact expected by the original flow.

The Rust rewrite must:

- preserve the existence of this optional build action,
- propagate build success or failure,
- integrate build invocation into the same end-to-end workflow position used by the original module.

**Traceability:** `make`, `do_all`

### Feature 9: End-to-End Program Orchestration

The module provides a complete top-level execution path from process startup through argument parsing, generation, and optional build.

The Rust rewrite must:

- preserve the overall sequencing performed by the original module,
- terminate with success or failure consistent with the outcome of parsing, reading, generation, and build steps,
- expose an executable entry path corresponding to the original `main`.

**Traceability:** `do_all`, `main`

## User Scenarios & Testing

### Scenario 1: Generate C output from a shell script

A user invokes the tool with arguments selecting a source shell script and output settings. The module parses the arguments, reads the script, transforms the script-related content, and writes a generated C source file.

**Expected result:**

- arguments are accepted when valid,
- the script file is read successfully,
- the generated C file is created,
- the file contains emitted C arrays and embedded processed content.

**Primary traceability:** `parse_args`, `read_script`, `write_C`, `do_all`

### Scenario 2: Reject invalid invocation

A user invokes the tool with unsupported, malformed, or incomplete arguments.

**Expected result:**

- argument parsing detects the problem,
- processing does not continue as though configuration were valid,
- program termination indicates failure.

**Primary traceability:** `parse_an_arg`, `parse_args`, `main`

### Scenario 3: Fail when the script file cannot be read

A user provides a script path that does not exist or cannot be opened.

**Expected result:**

- script loading fails,
- generated output is not silently produced from missing input,
- the overall operation reports failure.

**Primary traceability:** `read_script`, `do_all`

### Scenario 4: Use file-derived keying input

A user runs the tool in a mode where a file is used to influence the keying process.

**Expected result:**

- the module inspects the specified file,
- key setup depends on that file-based input,
- failure to inspect the file prevents successful completion of the keyed generation path.

**Primary traceability:** `key_with_file`, `write_C`

### Scenario 5: Emit correctly formatted embedded arrays

During generation, the module writes byte-oriented content into C syntax.

**Expected result:**

- generated arrays are syntactically valid C declarations,
- emitted byte sequences correspond to the processed input buffers,
- formatting supports inclusion in the generated source file.

**Primary traceability:** `prnt_bytes`, `prnt_array`, `dump_array`, `write_C`

### Scenario 6: Run the full generation-and-build flow

A user invokes the tool in a configuration that performs both source generation and compilation.

**Expected result:**

- generation occurs before the build step,
- the build step is invoked,
- success or failure from the build is reflected by the tool outcome.

**Primary traceability:** `write_C`, `make`, `do_all`

### Scenario 7: Shell-related input affects generation decisions

The module processes shell text as part of shell validation or selection behavior.

**Expected result:**

- shell text is evaluated,
- the result can alter whether generation proceeds or how it is configured.

**Primary traceability:** `eval_shell`, `write_C`

## Requirements

### Functional Requirements

#### FR-1: Argument Parsing
The module shall parse command-line arguments and determine the operational inputs and mode needed for processing.

**Traceability:** `parse_an_arg`, `parse_args`

#### FR-2: Invalid Argument Handling
The module shall detect invalid, unsupported, or incomplete argument input and prevent successful continuation of the normal generation flow.

**Traceability:** `parse_an_arg`, `parse_args`, `main`

#### FR-3: Script File Reading
The module shall read the full content of the designated shell script file from disk for use in generation.

**Traceability:** `read_script`

#### FR-4: File-Based Keying
The module shall support deriving key input from properties of a named file and shall fail that step when the file cannot be inspected as required.

**Traceability:** `key_with_file`

#### FR-5: Transformation State Initialization
The module shall support resetting its internal transformation/keying state before key setup and data transformation.

**Traceability:** `stte_0`

#### FR-6: Key Mixing
The module shall support mixing caller-provided key material into the transformation state.

**Traceability:** `key`

#### FR-7: Byte Buffer Transformation
The module shall support transforming arbitrary byte buffers using the current transformation state.

**Traceability:** `arc4`

#### FR-8: Shell Text Evaluation
The module shall evaluate shell-related text and return a success/failure result used by the module’s generation logic.

**Traceability:** `eval_shell`

#### FR-9: Bounded Random Value Generation
The module shall generate random values constrained by a caller-provided modulus.

**Traceability:** `rand_mod`

#### FR-10: Random Character Generation
The module shall generate random characters for use in output construction.

**Traceability:** `rand_chr`

#### FR-11: Noise/Filler Generation
The module shall fill caller-supplied storage with generated noise subject to minimum, extra, and mode parameters.

**Traceability:** `noise`

#### FR-12: Byte Formatting for C Output
The module shall format byte content into textual C-source-compatible output.

**Traceability:** `prnt_bytes`

#### FR-13: Named Array Emission
The module shall emit named array declarations for generated C source using provided element and declaration metadata.

**Traceability:** `prnt_array`, `dump_array`

#### FR-14: Generated C File Writing
The module shall write a generated C source file containing embedded processed data and required supporting content.

**Traceability:** `write_C`

#### FR-15: Build Invocation
The module shall support invoking the build step after generation and shall return success or failure from that invocation.

**Traceability:** `make`

#### FR-16: End-to-End Workflow Control
The module shall orchestrate argument parsing, generation, and optional build execution in the program’s top-level flow.

**Traceability:** `do_all`, `main`

### Key Entities

#### Entity 1: Transformation State
An internal mutable state supports reset, key mixing, and byte transformation across the processing workflow.

- used by state initialization,
- updated by key input,
- consumed by buffer transformation,
- participates in generated-data preparation.

**Traceability:** `stte_0`, `key`, `arc4`

#### Entity 2: Script Content Buffer
An in-memory byte or character buffer holds the full contents of the input shell script after reading.

- populated from the script file,
- consumed during generation,
- may be transformed and emitted as embedded data.

**Traceability:** `read_script`, `write_C`

#### Entity 3: File Metadata Input
A file inspection result provides metadata-derived input used for file-based keying decisions or key material derivation.

- obtained from a named filesystem object,
- used to influence keying,
- failure to obtain it blocks the dependent flow.

**Traceability:** `key_with_file`, referenced anonymous `struct stat`

#### Entity 4: Shell Text Input
A text input representing shell-related content is evaluated to support shell handling decisions within generation.

**Traceability:** `eval_shell`

#### Entity 5: Generated Output Sink
A writable file handle or output destination receives formatted byte data, array declarations, and the complete generated C source text.

**Traceability:** `prnt_bytes`, `prnt_array`, `dump_array`, `write_C`

#### Entity 6: Random/Noise Output Region
A caller-provided writable memory region receives generated random filler/noise used in constructing output content.

**Traceability:** `noise`

#### Entity 7: Process Arguments
The argument vector and count define the requested operation and provide the user-supplied inputs for the module workflow.

**Traceability:** `parse_an_arg`, `parse_args`, `do_all`, `main`

## Success Criteria

1. **Argument parsing parity:** Valid invocations accepted by the original module’s supported flow are accepted by the Rust port, and invalid or incomplete invocations are rejected before successful generation.
   **Traceability:** `parse_an_arg`, `parse_args`, `main`

2. **Script loading correctness:** Given a readable input script, the Rust port loads its content and uses it in generation; given an unreadable or missing script, the operation fails.
   **Traceability:** `read_script`, `do_all`

3. **Keyed transformation workflow preservation:** The Rust port preserves the functional sequence of state reset, key setup, and buffer transformation used by the module’s generation process.
   **Traceability:** `stte_0`, `key`, `arc4`, `key_with_file`, `write_C`

4. **File-based keying enforcement:** When a required file-based keying input is unavailable or invalid, the Rust port fails the dependent processing path rather than silently proceeding as success.
   **Traceability:** `key_with_file`

5. **C output generation:** For successful generation runs, the Rust port creates a C source file containing formatted embedded data and named arrays sufficient for the subsequent build step.
   **Traceability:** `prnt_bytes`, `prnt_array`, `dump_array`, `write_C`

6. **Shell evaluation continuity:** Shell-related text evaluation in the Rust port returns a result that can affect generation flow in the same functional role as the original module.
   **Traceability:** `eval_shell`, `write_C`

7. **Noise/random support availability:** The Rust port provides the random modulus, random character, and noise-generation behaviors required by the generation path without omitting those stages.
   **Traceability:** `rand_mod`, `rand_chr`, `noise`, `write_C`

8. **Build-step result propagation:** When the build step is requested by the workflow, the Rust port invokes it after generation and reports its success or failure in the module outcome.
   **Traceability:** `make`, `do_all`

9. **Top-level execution equivalence:** The Rust executable entry path performs the same end-to-end role as the original module: parse inputs, run processing, and terminate with success/failure based on the workflow result.
   **Traceability:** `do_all`, `main`

## Acceptance Notes

- Behavioral equivalence is required for the module’s evidenced functionality, but not literal C implementation structure.
- Output C text may differ in non-essential formatting if it remains functionally valid for the module’s generation and build flow.
- This specification intentionally avoids adding capabilities not evidenced by `src/shc.c`.