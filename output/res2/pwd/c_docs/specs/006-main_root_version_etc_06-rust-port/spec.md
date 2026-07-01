# spec.md

## Title

Functional Specification: `main_root_version_etc_06`

## Summary

This module provides formatted version-report output for a command-line program. It emits version text and author attribution text to a caller-provided output stream, with support for author lists supplied in three forms:

- explicit array plus author count
- null-terminated author array
- variadic argument list

The Rust rewrite must preserve the observable formatting behavior and the supported calling patterns represented by the source module’s exported functions in `version-etc.c`.

## Scope

In scope:

- producing version/help-style textual output to a provided stream
- accepting command/package/version identity values from the caller
- formatting author information from bounded arrays, null-terminated arrays, and variadic-style inputs
- handling different author-count cases through the same externally visible output routine family

Out of scope:

- discovering version information automatically
- parsing command-line options
- owning or managing process exit behavior
- localization design changes beyond preserving module behavior evidenced by the source
- introducing additional public interfaces beyond those needed to cover the existing module functionality

## Source Traceability

Primary source file:

- `version-etc.c`

Primary exported functions:

- `version_etc_arn`
- `version_etc_ar`
- `version_etc_va`
- `version_etc`

---

## 1. Feature Specification

### 1.1 Purpose

The module prints a program version report to an output stream. The report is parameterized by:

- destination stream
- command name
- package name
- version string
- author names

The module exists to centralize consistent version-report formatting so callers can produce the same style of output regardless of how author data is supplied.

### 1.2 Supported Functional Behavior

The Rust version must implement behavior equivalent to the C module for the following cases.

#### A. Version report with explicit author count

A caller can provide a stream, command name, package name, version string, an author array, and an explicit author count. The module must format and write a version report using exactly the specified number of authors.

This behavior is traced to:

- `version_etc_arn`

#### B. Version report with null-terminated author array

A caller can provide a stream, command name, package name, version string, and a null-terminated author array. The module must determine the author list extent from the terminating null entry and produce the same report style as the explicit-count variant.

This behavior is traced to:

- `version_etc_ar`

#### C. Version report with variadic author list via `va_list`

A caller can provide a stream, command name, package name, version string, and a variadic author sequence through `va_list`. The module must consume the author entries according to the source module’s supported calling convention and emit the same style of report.

This behavior is traced to:

- `version_etc_va`

#### D. Version report with direct variadic arguments

A caller can provide a stream, command name, package name, version string, and author names as variadic arguments. The module must accept this form and emit the same report style as the other entry points.

This behavior is traced to:

- `version_etc`

### 1.3 Output Content Expectations

The Rust rewrite must preserve the module’s externally visible output role:

- identify the program/package/version in the report
- include author attribution text
- vary author formatting according to how many authors are supplied, while preserving equivalent output semantics to the source module
- write the report to the caller-selected stream rather than returning it as a string

The specification does not require inventing new text, new fields, or new formatting modes beyond what is evidenced by the source module family.

---

## 2. User Scenarios & Testing

### 2.1 Scenario: CLI program prints version information to standard output

A command-line utility needs to print its version information when invoked with a version option. It already knows its command name, package name, version string, and author list. It passes standard output as the destination stream and receives a human-readable version report.

Must be supported by:

- bounded author array input
- null-terminated author array input
- variadic author input

### 2.2 Scenario: Program shares one formatting facility across multiple call sites

Different parts of a codebase hold author data in different forms. One call site has a fixed array and count; another has a null-terminated list; another wraps a variadic helper. All call sites must produce the same style of version output for the same logical inputs.

### 2.3 Scenario: Single-author and multi-author reporting

A utility with one credited author and a utility with several credited authors both use the module. The report must remain grammatically and structurally appropriate to the number of supplied authors, matching the source module’s behavior.

### 2.4 Scenario: Output redirected to a non-stdout stream

A caller sends version output to an alternate writable stream, such as standard error or a file stream, and expects the report to be written there without changing report content rules.

### 2.5 Testable scenarios

The Rust version must support tests that validate:

1. **Equivalent output across entry points**
   Given the same logical command/package/version/authors, the explicit-count, null-terminated-array, `va_list`-style, and variadic wrappers produce equivalent report text.

2. **Author-count-sensitive formatting**
   Representative author-count cases produce output matching the source behavior for those counts.

3. **Correct stream targeting**
   Output appears in the provided stream sink and not implicitly elsewhere.

4. **Bounded-array adherence**
   The explicit-count form uses exactly `n_authors` entries from the supplied author sequence.

5. **Null-terminated-array adherence**
   The null-terminated-array form stops at the first terminator and formats the authors found before it.

---

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Emit version report to caller-provided stream
The module shall write version-report text to a stream supplied by the caller.

Traceability:

- `version_etc_arn`
- `version_etc_ar`
- `version_etc_va`
- `version_etc`

#### FR-2: Accept command, package, and version identity inputs
The module shall accept caller-provided command name, package name, and version string values and include them in the emitted version report according to source behavior.

Traceability:

- `version_etc_arn`
- `version_etc_ar`
- `version_etc_va`
- `version_etc`

#### FR-3: Support explicit-count author input
The module shall support author input as an array with an explicit author count and shall format output using exactly that many authors.

Traceability:

- `version_etc_arn`

#### FR-4: Support null-terminated author-array input
The module shall support author input as a null-terminated array and shall determine the author list length from the terminator.

Traceability:

- `version_etc_ar`

#### FR-5: Support variadic author input through wrapper forms
The module shall support author input supplied through variadic interfaces represented by direct variadic arguments and `va_list`-based forwarding.

Traceability:

- `version_etc_va`
- `version_etc`

#### FR-6: Preserve equivalent report semantics across entry points
For equivalent logical inputs, the module shall produce equivalent version-report content regardless of whether authors are supplied by counted array, null-terminated array, `va_list`, or direct variadic arguments.

Traceability:

- relationship among `version_etc_arn`, `version_etc_ar`, `version_etc_va`, `version_etc`

#### FR-7: Handle author-count-dependent attribution formatting
The module shall format author attribution in a way that depends on the number of supplied authors, matching the source module’s observable behavior for the supported counts.

Traceability:

- `version_etc_arn` as the core count-aware formatter
- wrappers `version_etc_ar`, `version_etc_va`, `version_etc`

### 3.2 Key Entities

#### Entity: Output stream
A writable destination selected by the caller. All exported entry points direct their report output to this entity.

Relationship:

- every version-report operation writes to one output stream

Traceability:

- `FILE *stream` parameter in all exported functions

#### Entity: Program identity
The caller-supplied textual identity of the reported program, consisting of:

- command name
- package name
- version string

Relationship:

- one version-report operation combines one program identity with one author collection and writes the resulting formatted report to one output stream

Traceability:

- `command_name`, `package`, `version` parameters in all exported functions

#### Entity: Author collection
A caller-supplied set or sequence of author names used for attribution in the report.

Representations evidenced by the module:

- counted author array
- null-terminated author array
- variadic author list / forwarded `va_list`

Relationship:

- one version-report operation consumes one author collection in one of the supported representations

Traceability:

- `authors` and `n_authors` in `version_etc_arn`
- `authors` in `version_etc_ar`
- `va_list authors` in `version_etc_va`
- variadic author arguments in `version_etc`

---

## 4. Success Criteria

### 4.1 Behavioral Equivalence

1. The Rust module provides functionality covering all four source entry-point behaviors represented by:
   - counted author array
   - null-terminated author array
   - `va_list`-forwarded author input
   - direct variadic author input or an equivalent Rust-facing adaptation preserving the same functional boundary

2. For identical logical inputs, all supported entry paths produce equivalent visible report content.

3. The emitted report includes the caller-supplied command name, package name, and version information in the same functional role as the source module.

### 4.2 Output Correctness

4. Output is written to the caller-designated stream/sink.

5. The counted-array path uses exactly the requested number of author entries.

6. The null-terminated-array path stops consuming authors at the terminator.

7. Author attribution formatting varies correctly with author count, matching source-observable output for representative counts covered by tests.

### 4.3 Test Completion Criteria

8. Automated tests compare Rust output against source-derived expected output for:
   - one author
   - multiple authors
   - counted array input
   - null-terminated array input
   - wrapper-based forwarding behavior

9. No required module behavior evidenced by `version-etc.c` is omitted in the Rust rewrite.