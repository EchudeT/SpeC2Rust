# spec.md

## Title
Rust Functional Specification for `module_src` in `shc`

## Metadata
- Project: `shc`
- Module: `module_src`
- Category: `module`
- Source basis: `src/shc.c`
- Target branch: `001-module_src-rust-port`
- Generation date: 2026-06-16

## Overview
This module is the main operational unit of the `shc` program. It parses command-line arguments, reads a source script, derives and applies an ARC4-style transformation key, generates randomized padding data, emits C source that embeds protected script content and related metadata, and invokes the build step that produces the final output artifact.

The Rust rewrite must preserve the module’s observed functional scope:

- accept and interpret command-line options and operands,
- validate and store generation settings,
- read script input from the filesystem,
- derive a key from file metadata when requested,
- initialize and apply the module’s byte-stream transformation logic,
- generate formatted embedded byte arrays and surrounding C output,
- optionally evaluate shell text used by the module’s workflow,
- run the full end-to-end generation flow from argument parsing to output creation.

The Rust version is a rewrite target for this module’s behavior, not an expansion of features.

## Functional Boundaries
Included in scope for the Rust version:

- command-line driven configuration and dispatch,
- script file loading,
- internal key scheduling and byte-stream transformation,
- file-metadata-based keying support,
- randomized character and noise generation used during output emission,
- C source generation for embedded data,
- orchestration of the complete processing flow,
- process entry behavior equivalent to the module’s current role.

Out of scope unless directly required to preserve current behavior evidenced by the source basis:

- new command-line options,
- alternate output formats beyond generated C source and the existing build workflow,
- library-style stable public API commitments,
- concurrency behavior,
- persistence or serialization formats beyond emitted C text,
- cryptographic guarantees beyond reproducing existing transformation behavior.

## Source Traceability
Primary traced functions from `src/shc.c`:

- argument handling: `parse_an_arg`, `parse_args`
- transformation state and operations: `stte_0`, `key`, `arc4`, `key_with_file`
- shell/script helpers: `eval_shell`, `read_script`
- randomized output helpers: `rand_mod`, `rand_chr`, `noise`
- C emission helpers: `prnt_bytes`, `prnt_array`, `dump_array`, `write_C`
- orchestration: `make`, `do_all`, `main`

## Feature Specification

### 1. Command-line Processing
The module accepts invocation arguments, processes options one by one, and builds the internal configuration needed for generation.

The Rust version must:

- parse command-line arguments in sequence,
- distinguish valid options from invalid or incomplete ones,
- capture the script input and generation-related settings required by the rest of the workflow,
- support a top-level parse operation that processes the full argument list before execution continues.

Behavioral traceability: `parse_an_arg`, `parse_args`, `do_all`, `main`.

### 2. Script Input Loading
The module reads the source script file into memory for later transformation and embedding.

The Rust version must:

- open and read the designated script file,
- return its contents for downstream processing,
- fail the operation when the file cannot be read.

Behavioral traceability: `read_script`, `do_all`.

### 3. Transformation State Initialization and Use
The module contains a mutable internal byte-stream transformation state with explicit initialization, key loading, and in-place application to byte buffers.

The Rust version must:

- provide a reset/initialization step for transformation state,
- accept arbitrary byte input as key material,
- transform provided byte buffers in place according to the module’s existing behavior,
- preserve ordering dependencies between initialization, keying, and transformation.

Behavioral traceability: `stte_0`, `key`, `arc4`.

### 4. File-Metadata-Based Key Derivation
The module can derive transformation key material from filesystem metadata for a specified file.

The Rust version must:

- inspect the target file’s metadata,
- use that metadata as part of the key-derivation behavior currently performed by the module,
- report failure when required metadata cannot be obtained.

Behavioral traceability: `key_with_file`.

### 5. Shell Text Evaluation Support
The module includes behavior to evaluate shell text as part of its workflow.

The Rust version must:

- accept shell text input,
- execute the module’s current evaluation step,
- return a success/failure result consistent with whether evaluation completed.

Behavioral traceability: `eval_shell`.

### 6. Randomized Padding and Noise Generation
The module generates randomized values and randomized textual or byte noise used during output generation.

The Rust version must:

- generate bounded random values,
- generate random character values used by noise creation,
- fill output regions with randomized noise according to requested size and mode parameters.

Behavioral traceability: `rand_mod`, `rand_chr`, `noise`.

### 7. C Source Emission
The module emits generated C source that includes embedded byte arrays and associated formatting.

The Rust version must:

- format byte data into emitted C-compatible literal output,
- emit named arrays with the expected declaration form,
- support direct dumping of array data through the module’s helper flow,
- write the full generated C source file containing the embedded protected content and module-produced metadata required by the downstream build step.

Behavioral traceability: `prnt_bytes`, `prnt_array`, `dump_array`, `write_C`.

### 8. Build Invocation
After generating the C source, the module performs the next build step.

The Rust version must:

- execute the module’s build phase after successful generation,
- return success/failure based on that phase’s result.

Behavioral traceability: `make`, `do_all`.

### 9. End-to-End Orchestration
The module coordinates parsing, reading, transformation preparation, output generation, and build execution in a complete run.

The Rust version must:

- perform the end-to-end workflow in the same high-level sequence as the current module,
- ensure earlier failures prevent later steps that depend on them,
- expose the same program-entry role currently fulfilled by this module.

Behavioral traceability: `do_all`, `main`.

## User Scenarios & Testing

### Scenario 1: Generate output from a valid script input
A user invokes the program with valid arguments identifying a script and output settings. The module parses the arguments, reads the script, prepares transformation state, generates the C source output, and runs the build step.

The Rust version must support a test that verifies:

- argument parsing succeeds,
- the script file is read successfully,
- generated C output is created,
- the build step is invoked,
- the overall run exits successfully.

Traceability: `parse_args`, `read_script`, `write_C`, `make`, `do_all`, `main`.

### Scenario 2: Reject invalid or incomplete command-line usage
A user invokes the program with unsupported options or missing required values. The module does not proceed as if generation were valid.

The Rust version must support a test that verifies:

- invalid or incomplete argument input is detected,
- generation does not continue to successful output creation under invalid configuration.

Traceability: `parse_an_arg`, `parse_args`, `main`.

### Scenario 3: Fail when the input script cannot be read
A user provides a script path that does not exist or cannot be opened. The module fails the read step and does not complete generation.

The Rust version must support a test that verifies:

- script read failure is reported,
- downstream generation/build steps are not reported as successful.

Traceability: `read_script`, `do_all`.

### Scenario 4: Apply file-based keying when configured
A user runs the module in a mode that depends on file metadata keying. The module reads metadata for the target file and uses it in key derivation.

The Rust version must support a test that verifies:

- metadata lookup succeeds for an existing file,
- file-based key derivation returns success,
- metadata lookup failure causes the operation to fail.

Traceability: `key_with_file`.

### Scenario 5: Transform byte buffers after initialization and keying
During processing, the module initializes state, loads key material, and transforms byte buffers.

The Rust version must support a test that verifies:

- state can be reset,
- key material can be loaded,
- applying the transform changes or processes the target buffer deterministically for the same initialized state and inputs.

Traceability: `stte_0`, `key`, `arc4`.

### Scenario 6: Emit C arrays for embedded data
During output generation, the module emits byte arrays in C source form.

The Rust version must support a test that verifies:

- a byte sequence is formatted into valid C-style byte output,
- a named array declaration is produced,
- the full C output file contains the emitted array content.

Traceability: `prnt_bytes`, `prnt_array`, `dump_array`, `write_C`.

### Scenario 7: Generate randomized noise for output decoration or padding
The module requests randomized noise with size and mode parameters while preparing output.

The Rust version must support a test that verifies:

- bounded random values stay within the requested modulus,
- generated noise length satisfies the requested minimum and extra range,
- noise generation completes for both relevant mode forms accepted by the current module.

Traceability: `rand_mod`, `rand_chr`, `noise`.

### Scenario 8: Evaluate shell text when invoked by workflow
The module evaluates shell text and returns a result.

The Rust version must support a test that verifies:

- shell text can be submitted to the evaluation step,
- the step returns a success/failure status.

Traceability: `eval_shell`.

## Requirements

### Functional Requirements

#### FR-1: Argument parsing
The module shall parse command-line inputs and derive the internal configuration required for execution.

Traceability: `parse_an_arg`, `parse_args`.

#### FR-2: Argument validation
The module shall detect invalid, unsupported, or incomplete argument usage and prevent successful completion under such conditions.

Traceability: `parse_an_arg`, `parse_args`, `main`.

#### FR-3: Script file reading
The module shall read the designated script file from disk into memory for processing.

Traceability: `read_script`.

#### FR-4: Read failure handling
The module shall surface failure when the script file cannot be opened or read.

Traceability: `read_script`, `do_all`.

#### FR-5: Transformation state reset
The module shall provide a way to initialize or reset its internal transformation state before keying or transforming data.

Traceability: `stte_0`.

#### FR-6: Key loading
The module shall accept key material as bytes and apply it to the transformation state.

Traceability: `key`.

#### FR-7: In-place byte transformation
The module shall transform supplied byte buffers in place using the initialized and keyed state.

Traceability: `arc4`.

#### FR-8: File-metadata key derivation
The module shall support deriving key input from metadata of a specified file and shall fail when the metadata cannot be obtained.

Traceability: `key_with_file`.

#### FR-9: Shell evaluation
The module shall evaluate provided shell text and return a success/failure result.

Traceability: `eval_shell`.

#### FR-10: Random bounded value generation
The module shall generate random values constrained by a provided modulus.

Traceability: `rand_mod`.

#### FR-11: Random character generation
The module shall generate random character values for use in noise creation.

Traceability: `rand_chr`.

#### FR-12: Noise generation
The module shall populate a target region with randomized noise according to minimum size, extra range, and mode parameters.

Traceability: `noise`.

#### FR-13: Byte formatting for C output
The module shall format byte sequences into C-compatible emitted text.

Traceability: `prnt_bytes`.

#### FR-14: Named C array emission
The module shall emit named arrays representing embedded data in generated C source.

Traceability: `prnt_array`, `dump_array`.

#### FR-15: Full C source generation
The module shall write the generated C source file that embeds processed script-related data and supporting emitted content.

Traceability: `write_C`.

#### FR-16: Build step execution
The module shall execute the build step after successful C source generation and report whether that build step succeeded.

Traceability: `make`.

#### FR-17: End-to-end workflow orchestration
The module shall coordinate parsing, input reading, transformation preparation, C generation, and build execution as a complete run.

Traceability: `do_all`, `main`.

#### FR-18: Failure short-circuiting
The module shall not treat downstream stages as successful when prerequisite stages have failed.

Traceability: `do_all`, `main`.

### Key Entities

#### 1. Runtime configuration
A module-level set of parsed settings governs how the workflow runs, including the chosen script input and generation options.

Relationship:
- produced by argument parsing,
- consumed by orchestration and output generation.

Traceability: `parse_an_arg`, `parse_args`, `do_all`, `write_C`, anonymous struct at `src/shc.c:23`.

#### 2. Transformation state
An internal mutable state supports reset, key loading, and in-place byte transformation.

Relationship:
- initialized before use,
- modified by key loading,
- consumed by byte transformation and file-based key derivation.

Traceability: `stte_0`, `key`, `arc4`, `key_with_file`.

#### 3. Script content buffer
An in-memory representation of the loaded script content used as generation input.

Relationship:
- produced by script reading,
- consumed during generation and embedding.

Traceability: `read_script`, `write_C`.

#### 4. File metadata snapshot
Filesystem metadata collected from a file and used for key derivation.

Relationship:
- produced from filesystem inspection,
- consumed by file-based key derivation logic.

Traceability: `key_with_file`, `struct stat`.

#### 5. Generated output buffers and emitted arrays
Intermediate or final byte sequences and textual representations written into generated C source.

Relationship:
- produced from processed script data and helper formatting,
- consumed by final C file writing.

Traceability: `prnt_bytes`, `prnt_array`, `dump_array`, `write_C`.

#### 6. Build execution result
The success/failure outcome of the post-generation build phase.

Relationship:
- produced by the build step,
- consumed by orchestration and program exit behavior.

Traceability: `make`, `do_all`, `main`.

## Success Criteria

1. The Rust module can complete an end-to-end run for a valid invocation path represented by the current module, producing generated C output and reporting build-step success when all stages succeed.
   - Traceability: `parse_args`, `read_script`, `write_C`, `make`, `do_all`, `main`

2. The Rust module rejects invalid or incomplete argument usage and does not report successful completion for such invocations.
   - Traceability: `parse_an_arg`, `parse_args`, `main`

3. When the input script file is missing or unreadable, the Rust module reports failure and does not satisfy end-to-end success.
   - Traceability: `read_script`, `do_all`

4. The Rust module reproduces the module’s transformation workflow boundaries by supporting state reset, key loading, and in-place byte transformation over caller-supplied buffers.
   - Traceability: `stte_0`, `key`, `arc4`

5. The Rust module supports file-based key derivation and returns failure when target file metadata cannot be obtained.
   - Traceability: `key_with_file`

6. The Rust module can emit C-compatible byte-array text and generate the full C source file required by the workflow.
   - Traceability: `prnt_bytes`, `prnt_array`, `dump_array`, `write_C`

7. The Rust module supports randomized helper generation such that bounded random generation respects the requested modulus and noise generation honors requested size parameters.
   - Traceability: `rand_mod`, `rand_chr`, `noise`

8. The Rust module preserves workflow sequencing so that generation and build stages are only treated as successful when prerequisite parsing and input preparation stages have succeeded.
   - Traceability: `do_all`, `main`

9. The Rust module provides a shell-text evaluation step that returns an explicit success/failure outcome.
   - Traceability: `eval_shell`