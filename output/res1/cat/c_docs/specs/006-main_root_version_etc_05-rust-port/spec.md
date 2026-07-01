# spec.md

## Title

Functional Specification: `main_root_version_etc_05`

## Status

Draft

## Scope

This specification defines the required behavior for the Rust rewrite of the `main_root_version_etc_05` module from the `cat` project. The module’s responsibility is to emit version and authorship information to a caller-supplied output stream through a small family of public entry points that differ only in how author names are supplied.

This specification covers only behavior evidenced by `version-etc.c` and its exported functions:
- `version_etc_arn`
- `version_etc_ar`
- `version_etc_va`
- `version_etc`

## Feature Specification

### Summary

The module formats and writes a standard version-information block for a program or command. The output includes:
- an identification line based on command name, package, and version
- copyright text
- license/warranty text
- authorship text derived from one or more author names

The Rust version must preserve the observable formatting role of the C module: given a target output stream and identifying strings, it must emit the same logical information, including correct handling of different author-list forms.

### Supported entry behaviors

The Rust rewrite must support the behaviors represented by the four C entry points:

1. **Fixed-count author list behavior**
   Accept a stream, command name, package, version, an array/slice of author names, and an explicit author count; write the full version-information block using exactly the supplied count.

2. **Null-terminated author array behavior**
   Accept a stream, command name, package, version, and a null-terminated array/list of author names; determine author count from the terminator and write the same version-information block.

3. **Variadic-list behavior via forwarded argument list**
   Accept a stream, command name, package, version, and a variadic author sequence represented by forwarded call-state; consume author names until the terminating null sentinel and write the same version-information block.

4. **Direct variadic behavior**
   Accept a stream, command name, package, version, and author names passed variadically; consume author names until the terminating null sentinel and write the same version-information block.

### Output responsibilities

The Rust version must produce output equivalent in content and author grouping to the C module:

- A leading version line identifying the program/package and version.
- Standard copyright and license/warranty text.
- An authorship section that reflects the supplied authors.
- Author formatting that adapts to the number of supplied authors rather than treating all cases identically.

The Rust version may expose Rust-appropriate interfaces internally or externally, but the functional result must match the above behaviors.

## User Scenarios & Testing

### Scenario 1: Print version information for a command with a known number of authors

A caller has a stream for standard output and a slice of author names with a known count. The module is invoked to print version information for a command.

**Expected result:**
- Output is written to the provided stream.
- The version line includes the provided command/package/version identifiers.
- The authorship portion includes exactly the counted authors, in the supplied order.

### Scenario 2: Print version information from a null-terminated author array

A caller has author names in a null-terminated list and does not separately track the count.

**Expected result:**
- The module determines the author list length from the terminator.
- Output matches the fixed-count behavior for the same author sequence.

### Scenario 3: Print version information from a variadic wrapper

A caller uses an API shape equivalent to C variadic arguments and terminates the author list with a null sentinel.

**Expected result:**
- The module consumes author names until the sentinel.
- Output matches the fixed-count or null-terminated-array behavior for the same author sequence.

### Scenario 4: Vary author count

A caller prints version information with differing numbers of authors, including small and larger author lists.

**Expected result:**
- The module emits a valid authorship section for each case.
- Author presentation changes appropriately according to count, without dropping or reordering names.

### Scenario 5: Distinguish command name and package

A caller provides both a command name and a package identifier.

**Expected result:**
- The output uses the provided identifiers in the same roles as the C module.
- The command/program identification remains coherent across all entry behaviors.

### Testing expectations

The Rust rewrite must be tested with output-capture assertions that verify:
- all entry behaviors produce equivalent output for the same logical inputs
- author counting from a null-terminated list stops at the first terminator
- variadic-style input handling stops at the terminating null sentinel
- author names appear in input order
- version/package/command strings are present in the correct output sections
- multiple author-count cases produce the expected authorship text form

## Requirements

### Functional Requirements

#### FR-1: Emit version-information text to a caller-supplied stream
The module shall write the complete version-information block to the output stream supplied by the caller.

**Traceability:** `version_etc_arn`, `version_etc_ar`, `version_etc_va`, `version_etc` in `version-etc.c`

#### FR-2: Include command/package/version identification
The module shall include identification derived from the provided `command_name`, `package`, and `version` inputs in the emitted version text.

**Traceability:** `version_etc_arn` and wrapper functions in `version-etc.c`

#### FR-3: Include standard copyright and license/warranty text
The module shall emit the standard legal/disclaimer text that is part of the module’s version-information output.

**Traceability:** `version_etc_arn` in `version-etc.c`

#### FR-4: Include authorship information
The module shall emit authorship text based on the supplied author names.

**Traceability:** `version_etc_arn` in `version-etc.c`

#### FR-5: Preserve author order
When multiple author names are supplied, the module shall present them in the same order provided by the caller.

**Traceability:** `version_etc_arn`, `version_etc_ar`, `version_etc_va`, `version_etc` in `version-etc.c`

#### FR-6: Support explicit-count author input
The module shall support behavior equivalent to accepting a sequence of author names together with an explicit author count.

**Traceability:** `version_etc_arn` in `version-etc.c`

#### FR-7: Support terminator-based author-array input
The module shall support behavior equivalent to accepting a null-terminated author array/list and deriving the number of authors by scanning to the terminator.

**Traceability:** `version_etc_ar` in `version-etc.c`

#### FR-8: Support variadic author input terminated by a null sentinel
The module shall support behavior equivalent to accepting author names from a variadic sequence and stopping at a terminating null sentinel.

**Traceability:** `version_etc_va`, `version_etc` in `version-etc.c`

#### FR-9: Produce equivalent output across entry behaviors
For the same stream destination and the same logical command/package/version/authors input, all supported entry behaviors shall produce equivalent output.

**Traceability:** relationship among `version_etc_arn`, `version_etc_ar`, `version_etc_va`, `version_etc` in `version-etc.c`

#### FR-10: Adapt authorship text to author count
The module shall format the authorship portion according to the number of supplied authors rather than using a single fixed representation for all counts.

**Traceability:** `version_etc_arn` in `version-etc.c`

### Key Entities

#### Output stream
A writable destination supplied by the caller. All generated text is emitted to this destination.

**Traceability:** `FILE *stream` parameter in all functions in `version-etc.c`

#### Command name
A caller-supplied string identifying the command/program presentation in the version output.

**Traceability:** `const char *command_name` parameter in all functions in `version-etc.c`

#### Package
A caller-supplied string identifying the package associated with the version output.

**Traceability:** `const char *package` parameter in all functions in `version-etc.c`

#### Version
A caller-supplied string identifying the version associated with the output.

**Traceability:** `const char *version` parameter in all functions in `version-etc.c`

#### Author sequence
An ordered collection of author-name strings supplied in one of three forms:
- explicit count plus array
- null-terminated array
- variadic sequence terminated by null sentinel

This entity is consumed to generate the authorship text.

**Traceability:** `authors` and `n_authors` parameters in `version_etc_arn`; `authors` parameter in `version_etc_ar` and `version_etc_va`; variadic arguments in `version_etc`

## Success Criteria

### SC-1: Output generation
For valid inputs, invoking the Rust rewrite writes a complete version-information block to the provided output destination.

**Traceability:** all functions in `version-etc.c`

### SC-2: Identifier inclusion
Automated tests confirm that output contains the supplied command name, package, and version in their expected roles.

**Traceability:** all functions, primarily `version_etc_arn`

### SC-3: Author coverage
Automated tests confirm that every supplied author is present exactly once in output, in input order, for explicit-count, terminator-based, and variadic-style inputs.

**Traceability:** `version_etc_arn`, `version_etc_ar`, `version_etc_va`, `version_etc`

### SC-4: Entry-point equivalence
For a shared logical input set, tests confirm that the Rust rewrite’s supported input forms produce equivalent output content.

**Traceability:** wrapper relationship among all four functions in `version-etc.c`

### SC-5: Terminator handling
Tests confirm that terminator-based author input stops at the first null terminator/sentinel and does not consume beyond it.

**Traceability:** `version_etc_ar`, `version_etc_va`, `version_etc`

### SC-6: Author-count-sensitive formatting
Tests covering multiple author counts confirm that the authorship section changes according to author count in the same way as the original module’s behavior.

**Traceability:** `version_etc_arn` in `version-etc.c`

### SC-7: Legal text presence
Tests confirm that the standard copyright and license/warranty text is included in the generated output.

**Traceability:** `version_etc_arn` in `version-etc.c`