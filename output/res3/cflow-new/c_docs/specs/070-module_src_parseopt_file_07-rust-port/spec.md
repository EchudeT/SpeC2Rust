# spec.md

## Title

Functional Specification: `module_src_parseopt_file_07` Rust Port

## Overview

This module covers configuration-file based option loading in `cflow-new`, specifically the behavior represented by `optfile_lookup` and `parseopt_from_rc` in `src/main.c`.

The Rust rewrite must preserve the module’s role in locating a parse-option file by name and applying options from a runtime configuration file path. Its responsibility is limited to:

- resolving a named option file into a `parseopt_file` result;
- reading options from a specified rc/configuration file through the existing parse-option machinery;
- reporting success or failure through integer-style status outcomes compatible with surrounding program flow.

This specification does not extend beyond the behavior evidenced by the analyzed functions and referenced entities.

## Scope

### In Scope

- Lookup of an option/configuration file by a provided name.
- Population or update of a `parseopt_file` result structure during lookup.
- Processing of an rc/configuration file when given its file name.
- Integration with existing parse-option definitions and word-splitting/token parsing support as required by these functions.

### Out of Scope

- Redesign of the full command-line parser.
- New configuration syntaxes or file search policies not evidenced by the source behavior.
- New public APIs unrelated to option-file lookup or rc-file parsing.

## Feature Specification

### Feature: Named option file lookup

The module must support resolving a user- or program-supplied option file name into a `parseopt_file` description.

Behavioral expectations evidenced by `optfile_lookup`:

- Accept a file name input and an output `parseopt_file` target.
- Attempt to locate and validate the named option file using the program’s established option-file lookup rules.
- Use file-system state as part of the decision process, as indicated by the referenced `stat` type.
- Return a success/failure status.
- On success, provide the located file information through the `parseopt_file` output.
- On failure, indicate that the named option file could not be resolved or accepted.

The Rust version must preserve the observable outcome of successful and unsuccessful lookups, including whether the output structure is populated only when lookup succeeds.

### Feature: Parse options from rc/configuration file

The module must support applying options from a specified rc/configuration file.

Behavioral expectations evidenced by `parseopt_from_rc`:

- Accept a configuration file path/name.
- Attempt to read and parse options from that file using the existing parse-option definitions represented by `parseopt` and `optdef`.
- Support tokenization or shell-like word splitting as needed by the source behavior, as indicated by the referenced `wordsplit` type.
- Return a status indicating whether parsing from the rc file succeeded.

The Rust version must preserve the behavior that rc-file parsing is file-driven and feeds into the same option interpretation model used elsewhere in the program.

## User Scenarios & Testing

### Scenario 1: Successful option file resolution

A caller provides the name of an option file that exists and is acceptable under the program’s lookup rules.

Expected result:

- The lookup operation returns success.
- The `parseopt_file` result describes the located file.
- A subsequent consumer can use that resolved file information.

Test coverage:

- Provide a valid file name in an environment where the expected file exists.
- Verify success status.
- Verify that the returned result is populated consistently with a successful lookup.

### Scenario 2: Missing or invalid option file

A caller provides the name of an option file that does not exist or cannot be accepted by the lookup rules.

Expected result:

- The lookup operation returns failure.
- The result is not treated as a valid resolved file.

Test coverage:

- Use a definitely missing file name.
- Verify failure status.
- Verify no false-positive resolved file is produced.

### Scenario 3: Successful parsing from rc file

A caller requests option loading from an rc/configuration file containing valid option content supported by the existing parser definitions.

Expected result:

- The rc-file parsing operation returns success.
- The options from the file are interpreted through the same parse-option definitions used by the program.

Test coverage:

- Supply a valid rc file with known option entries supported by the host parser.
- Verify success status.
- Verify resulting option state changes match the parsed content.

### Scenario 4: Rc file contains invalid or unprocessable content

A caller requests parsing from an rc/configuration file that cannot be parsed successfully.

Expected result:

- The rc-file parsing operation returns failure.
- Invalid content is not reported as a successful parse.

Test coverage:

- Supply an rc file with malformed or unsupported option text.
- Verify failure status.

### Scenario 5: Rc file path cannot be opened or resolved

A caller requests parsing from a file path that is missing or inaccessible.

Expected result:

- The rc-file parsing operation returns failure.

Test coverage:

- Supply a non-existent rc file path.
- Verify failure status.

## Requirements

### Functional Requirements

#### FR-1: Option file lookup by name

The module shall accept an option file name and attempt to resolve it into a `parseopt_file` result.

Traceability:

- `optfile_lookup` in `src/main.c`

#### FR-2: File-system-aware validation during lookup

The module shall use file-system state in deciding whether the named option file is a valid lookup result.

Traceability:

- `optfile_lookup` in `src/main.c`
- referenced `stat` type

#### FR-3: Success/failure result for lookup

The module shall report lookup outcome as a success/failure status compatible with the source module’s integer-return convention.

Traceability:

- `optfile_lookup` in `src/main.c`

#### FR-4: Output population on successful lookup

The module shall provide resolved file information through a `parseopt_file` output when lookup succeeds.

Traceability:

- `optfile_lookup` in `src/main.c`
- `parseopt_file` referenced type

#### FR-5: Rc/configuration file driven option parsing

The module shall accept an rc/configuration file name and attempt to parse options from that file.

Traceability:

- `parseopt_from_rc` in `src/main.c`

#### FR-6: Parsing through existing option definition model

The module shall interpret rc-file content through the program’s parse-option definitions rather than through an unrelated configuration model.

Traceability:

- `parseopt_from_rc` in `src/main.c`
- referenced `parseopt` / `optdef` entities

#### FR-7: Token/word splitting support as required by rc parsing

The module shall support the word-splitting behavior required to convert rc-file content into parseable options.

Traceability:

- `parseopt_from_rc` in `src/main.c`
- referenced `wordsplit` type

#### FR-8: Success/failure result for rc parsing

The module shall report whether parsing options from the rc/configuration file succeeded.

Traceability:

- `parseopt_from_rc` in `src/main.c`

### Key Entities

#### `parseopt_file`

Represents the result of option-file lookup. It is the output target filled by the lookup operation when a named file is successfully resolved.

Relationship:

- Produced by option-file lookup.
- Consumed by surrounding parse-option file handling logic.

Traceability:

- Referenced by `optfile_lookup`

#### `parseopt` and `optdef`

Represent the program’s parse-option definitions used to recognize and apply options originating from configuration input.

Relationship:

- Rc-file parsing must use these definitions to interpret file content.

Traceability:

- Referenced around `parseopt_from_rc`

#### `wordsplit`

Represents tokenization/word-splitting support needed for rc-file option parsing.

Relationship:

- Supports conversion of rc-file text into parser-consumable option tokens.

Traceability:

- Referenced around `parseopt_from_rc`

#### `stat`

Represents file metadata/state used during option-file lookup.

Relationship:

- Supports validation/existence checks during file resolution.

Traceability:

- Referenced around `optfile_lookup`

## Success Criteria

1. A Rust implementation of named option-file lookup returns success for valid resolvable inputs and failure for missing or unacceptable inputs, matching the source module’s observable behavior.
2. On successful lookup, the Rust implementation produces a valid `parseopt_file` result; on failed lookup, it does not present a failed resolution as valid.
3. A Rust implementation of rc/configuration file parsing returns success when given a readable rc file whose contents are valid under the existing parse-option definitions.
4. Rc/configuration parsing returns failure when the file is unreadable, missing, or contains unprocessable option content.
5. Rc/configuration parsing in Rust uses the same option interpretation model as the host parser definitions represented by `parseopt` and `optdef`.
6. Module-level tests cover at least the five scenarios in this specification: successful lookup, failed lookup, successful rc parsing, malformed rc parsing failure, and missing rc file failure.

## Traceability Matrix

| Spec Item | Source Evidence |
|---|---|
| Named option file lookup | `optfile_lookup` in `src/main.c:1141-1218` |
| Lookup output via file descriptor/result structure | `optfile_lookup` parameter `struct parseopt_file *pf` |
| File-state-based lookup decision | `optfile_lookup`; referenced `stat` type |
| Rc/config file option parsing | `parseopt_from_rc` in `src/main.c:1253-1283` |
| Use of parser definitions | `parseopt_from_rc`; referenced `parseopt` / `optdef` |
| Word/token splitting during rc parsing | `parseopt_from_rc`; referenced `wordsplit` |