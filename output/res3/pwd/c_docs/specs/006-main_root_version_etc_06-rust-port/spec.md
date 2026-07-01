# spec.md

## Title

Functional Specification: `main_root_version_etc_06`

## Metadata

- Project: `pwd`
- Module: `main_root_version_etc_06`
- Category: `main_cluster`
- Source file: `version-etc.c`
- Rust branch target: `006-main_root_version_etc_06-rust-port`
- Generation date: 2026-06-09

## Overview

This module provides formatted version-and-authorship output for command-line programs. It emits a standard textual version banner to a caller-supplied output stream using a command name, package name, version string, and one or more author names.

The Rust rewrite must preserve the observable behavior of this module as a formatting/output utility:
- support fixed-count author lists,
- support null-terminated author lists,
- support variadic-style author input through an equivalent Rust-facing design,
- write the resulting text to a caller-selected output target.

This module is output-oriented only. Its role is to produce the version information text; it does not parse command-line arguments, discover version values, or manage program lifecycle.

## Scope

### In Scope

- Producing version text for a command or package.
- Including authorship information in the produced text.
- Supporting the call patterns represented by the source module's public entry points.
- Writing output to a provided stream/handle.

### Out of Scope

- Option parsing such as `--version`.
- Localization policy beyond reproducing module-observable output behavior.
- Version discovery from build metadata or environment.
- Storage or ownership management of author metadata beyond what is required to format output.

## Feature Specification

### Feature: Standard version information output

The module shall generate a standard version information block when given:
- an output stream,
- a command name,
- a package name,
- a version string,
- author information.

The output must be suitable for CLI version reporting and must preserve the source module's distinction between:
- command/program identity,
- package identity,
- release version,
- authorship listing.

### Feature: Multiple author input forms

The module shall support the author input forms evidenced by the source module:

1. **Explicit author count**
   - Accept an array/slice of author strings plus an explicit count.
   - Use exactly the provided number of author entries.

2. **Null-terminated author list**
   - Accept a sequence/collection equivalent to a null-terminated array of author strings.
   - Stop at the logical terminator rather than requiring a separate count.

3. **Variadic author forwarding behavior**
   - Support a caller-facing form equivalent in capability to the variadic C entry points.
   - The Rust rewrite may expose this through Rust-appropriate APIs, but must preserve the ability to provide a variable number of authors and produce the same formatted result.

### Feature: Authorship formatting by author count

The module shall vary the authorship portion of the emitted text according to how many authors are supplied. The Rust rewrite must preserve the source-visible behavior for different author counts, including the singular/plural/list formatting distinctions observable from the original module.

At minimum, formatting behavior must correctly handle:
- zero authors,
- one author,
- two authors,
- three or more authors.

### Feature: Stream-directed output

The module shall write its generated text to the stream provided by the caller. It must not require global stdout/stderr selection as part of this module's interface. The Rust rewrite must preserve caller control over the destination writer.

## User Scenarios & Testing

### Scenario 1: Emit version text for a program with one author

A CLI program wants to print version information to standard output and has one credited author.

**Expected support**
- The caller provides command name, package name, version, and one author.
- The module writes a complete version block to the chosen output stream.
- The authorship wording reflects a single author.

**Testing**
- Invoke the fixed-count form with `n_authors = 1`.
- Verify the output contains the provided command/package/version fields.
- Verify exactly one author name is included.
- Verify single-author wording is used.

### Scenario 2: Emit version text with multiple authors using explicit count

A caller has an array of author names and knows the exact count.

**Expected support**
- The module consumes only the specified number of authors.
- The output includes all supplied authors in the module's standard list format.

**Testing**
- Invoke the explicit-count form with 2 authors and with 3+ authors.
- Verify the output changes appropriately for each author-count case.
- Verify no extra author data beyond the provided count is read or emitted.

### Scenario 3: Emit version text from a null-terminated author list

A caller has author names stored in a null-terminated list.

**Expected support**
- The module reads authors until the logical terminator.
- The resulting output is equivalent to the explicit-count form for the same names.

**Testing**
- Provide a null-terminated author list with 1, 2, and 3 authors.
- Compare output against the explicit-count form using the same author names.
- Verify outputs are equivalent in content and formatting.

### Scenario 4: Emit version text through a variadic call style

A caller wants to pass authors as a variable-length argument sequence.

**Expected support**
- The Rust rewrite offers an equivalent capability for variable-length author provision.
- The output matches the standard module formatting for the same arguments.

**Testing**
- Exercise the Rust-facing equivalent with 0, 1, 2, and 3 authors.
- Verify output equivalence with the fixed-count form.

### Scenario 5: Direct output to a non-default writer

A caller wants to capture version text in memory or direct it to a specific file/stream.

**Expected support**
- The module writes to the caller-provided output destination.
- No hidden dependency on a process-global output stream is required.

**Testing**
- Write output to an in-memory buffer.
- Write output to a file-backed writer in test harnesses.
- Verify the produced bytes/text match expected formatting.

## Requirements

### Functional Requirements

#### FR-1: Version information emission
The module shall emit version information text from the supplied command name, package name, and version inputs to a caller-provided output stream.

**Traceability:** `version_etc_arn`, `version_etc_ar`, `version_etc_va`, `version_etc` in `version-etc.c`

#### FR-2: Fixed-count author support
The module shall support version output where authors are supplied as a string list plus an explicit author count.

**Traceability:** `version_etc_arn` in `version-etc.c`

#### FR-3: Null-terminated author-list support
The module shall support version output where authors are supplied as a null-terminated string list without a separate count.

**Traceability:** `version_etc_ar` in `version-etc.c`

#### FR-4: Variable-length author-call support
The module shall support a call pattern equivalent to passing a variable number of author strings.

**Traceability:** `version_etc_va`, `version_etc` in `version-etc.c`

#### FR-5: Author-count-sensitive formatting
The module shall format the authorship portion of the output according to the number of provided authors, preserving the observable distinctions present in the source behavior.

**Traceability:** `version_etc_arn` in `version-etc.c`

#### FR-6: Writer-directed output
The module shall send all generated output to the destination supplied by the caller rather than selecting its own destination.

**Traceability:** all listed functions in `version-etc.c`

#### FR-7: Equivalent output across supported entry forms
For the same command name, package, version, and author set, the supported entry forms shall produce equivalent textual output.

**Traceability:** relationship among `version_etc_arn`, `version_etc_ar`, `version_etc_va`, and `version_etc` in `version-etc.c`

### Key Entities

#### Output destination
A caller-supplied writable stream/handle that receives the generated version text.

**Relationship:** Every public function in this module writes to this entity.

#### Program/package identity fields
The textual inputs representing:
- command name,
- package name,
- version string.

**Relationship:** These fields are combined into the emitted version banner.

#### Author collection
A logical collection of author names represented in one of three source-evidenced forms:
- explicit-count list,
- null-terminated list,
- variable-length argument sequence.

**Relationship:** The author collection determines the authorship portion of the output, including count-sensitive formatting.

## Success Criteria

1. The Rust module can generate version output to a caller-provided writer using command name, package name, version, and authors.
   - **Traceability:** `version_etc_arn`, `version_etc_ar`, `version_etc_va`, `version_etc`

2. For identical logical inputs, the Rust equivalents of the supported author-input forms produce equivalent textual output.

3. Tests demonstrate correct behavior for author-count cases of 0, 1, 2, and 3 or more authors.
   - **Traceability:** formatting logic centered in `version_etc_arn`

4. Tests demonstrate that null-terminated author-list behavior matches explicit-count behavior for the same author names.
   - **Traceability:** `version_etc_ar` and `version_etc_arn`

5. Tests demonstrate that variable-length author provision matches fixed-list output for the same author names.
   - **Traceability:** `version_etc_va`, `version_etc`, and `version_etc_arn`

6. Tests confirm output can be directed to non-default destinations such as an in-memory buffer.
   - **Traceability:** all listed functions taking `FILE *stream`

7. The Rust rewrite does not require capabilities beyond the source module's evidenced role as a version-text formatting/output helper.
   - **Traceability:** module scope defined by `version-etc.c`