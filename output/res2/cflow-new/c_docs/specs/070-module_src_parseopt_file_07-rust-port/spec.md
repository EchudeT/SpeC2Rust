# spec.md

## Title

Functional Specification: `module_src_parseopt_file_07` Rust Port

## Status

Draft

## Scope

This specification covers the Rust rewrite of the configuration-file option loading behavior represented by `optfile_lookup` and `parseopt_from_rc` in `src/main.c` of `cflow-new`.

The module scope is limited to:

- locating and opening an option file by name according to the original lookup behavior,
- loading options from a resolved rc/configuration file path,
- integrating with the existing parse-option definitions represented by `parseopt`, `optdef`, and `parseopt_file`.

This specification does not define unrelated command-line parsing features outside the file-based option loading path.

## Feature Specification

The Rust module shall provide the same functional behavior as the analyzed C module for file-based option parsing.

At a high level, the module supports two connected operations:

1. **Option file lookup**
   - Given an option file name and a `parseopt_file` context, the module determines whether the named file can be resolved and prepared for use as a source of options.
   - The lookup behavior is expected to follow the same decision boundaries as the C implementation, including use of filesystem metadata and population of the `parseopt_file` state needed by later parsing.

2. **Loading options from an rc/config file**
   - Given a file name, the module attempts to read options from that file as an rc/configuration source.
   - The loaded content is interpreted as command-style options using the existing option-definition model.
   - The operation returns success or failure status matching the original behavior boundaries.

The Rust port must preserve the behavioral contract visible from these functions:

- successful lookup enables subsequent option parsing from the resolved file source,
- failed lookup or invalid file access results in a failure status without inventing fallback behaviors beyond those evidenced by the C code,
- rc/config file parsing uses the same option-definition system already used by the program for recognized options.

## User Scenarios & Testing

### Scenario 1: Load options from a valid rc file

A caller requests parsing from a configuration file path that exists and is readable.

Expected behavior:

- the file is successfully located,
- the file is treated as an option source,
- recognized options contained in the file are passed through the existing parse-option system,
- the operation returns success.

Test evidence target:
- traceable to `parseopt_from_rc` and its dependence on option-file lookup.

### Scenario 2: Reject a missing rc file

A caller requests parsing from a configuration file path that does not exist or cannot be resolved.

Expected behavior:

- the lookup step fails,
- no option application occurs from that file,
- the operation returns failure status consistent with the C behavior.

Test evidence target:
- traceable to `optfile_lookup` and `parseopt_from_rc`.

### Scenario 3: Reject an unusable file source

A caller provides a path that names a filesystem object that is not acceptable as an option file source under the original logic, or a path that cannot be opened/read under the same conditions.

Expected behavior:

- the module detects that the source is unusable during lookup or file preparation,
- the file is not processed as an option source,
- a failure status is returned.

Test evidence target:
- traceable to `optfile_lookup` use of `stat`-based checks and file-source preparation.

### Scenario 4: Apply only defined options from rc parsing flow

A caller invokes rc parsing in a program configuration where option definitions are already established through the existing `parseopt` / `optdef` model.

Expected behavior:

- the rc parsing flow uses that same option-definition model,
- file-sourced tokens are interpreted as options rather than as arbitrary free-form configuration entries,
- behavior remains bounded by the recognized parse-option definitions available to the program.

Test evidence target:
- traceable to `parseopt_from_rc` and referenced `parseopt` / `optdef` structures.

## Requirements

### Functional Requirements

#### FR-1: File-name-based option source lookup

The module shall accept a file name input and attempt to resolve it into a usable option-file context.

Traceability:
- `optfile_lookup`
- `parseopt_file`

#### FR-2: Filesystem validation during lookup

The module shall perform the same category of filesystem validation used by the C module before treating the target as an option file source.

This includes validation based on filesystem metadata sufficient to accept or reject the candidate file according to the original behavior.

Traceability:
- `optfile_lookup`
- referenced `stat`

#### FR-3: Populate option-file parsing context on successful lookup

When lookup succeeds, the module shall populate the option-file context required for downstream parsing.

Traceability:
- `optfile_lookup`
- `parseopt_file`

#### FR-4: Return status for lookup outcome

The module shall report success or failure from option-file lookup using a status result equivalent in meaning to the C function’s integer return contract.

Traceability:
- `optfile_lookup`

#### FR-5: Parse options from a specified rc/config file

The module shall accept an rc/config file name input and attempt to process that file as a source of options.

Traceability:
- `parseopt_from_rc`

#### FR-6: Use the program’s existing option-definition model

The rc/config parsing flow shall interpret file-sourced options using the same option-definition structures used elsewhere in the program.

Traceability:
- `parseopt_from_rc`
- `parseopt`
- `optdef`

#### FR-7: Tokenize file content as option input

The module shall support parsing rc/config file content through the tokenized input model evidenced by the use of `wordsplit`, so that file content can be consumed as option-style input rather than as opaque text.

Traceability:
- `parseopt_from_rc`
- referenced `wordsplit`

#### FR-8: Return status for rc parsing outcome

The module shall report success or failure from rc/config file processing using a result equivalent in meaning to the C function’s integer return contract.

Traceability:
- `parseopt_from_rc`

#### FR-9: No unsupported extension of behavior

The Rust rewrite shall remain limited to file lookup and rc/config option ingestion behavior evidenced by the analyzed module and shall not require new configuration semantics not represented by the C functions and referenced types.

Traceability:
- `optfile_lookup`
- `parseopt_from_rc`
- `parseopt_file`
- `parseopt`
- `optdef`

### Key Entities

#### `parseopt_file`

Represents the mutable context for an option file source.

Role in this module:

- receives the result of option-file lookup,
- carries the resolved file-related state needed for later parsing.

Relationship:
- populated by `optfile_lookup`,
- consumed by the rc/config parsing flow.

#### `parseopt`

Represents the program’s option parsing configuration.

Role in this module:

- defines how file-sourced options are interpreted.

Relationship:
- works together with `optdef` during rc/config parsing.

#### `optdef`

Represents individual option definitions within the parse-option system.

Role in this module:

- constrains which options from the rc/config file are recognized and how they are processed.

Relationship:
- grouped under or associated with `parseopt`,
- indirectly used by `parseopt_from_rc`.

#### `wordsplit`

Represents the tokenization model used to split file content into option-like arguments.

Role in this module:

- bridges configuration-file text and parse-option processing.

Relationship:
- used during rc/config parsing before options are applied through the parse-option definitions.

#### `stat`

Represents filesystem metadata used during file lookup validation.

Role in this module:

- supports accept/reject decisions for candidate option files.

Relationship:
- consulted by `optfile_lookup` before successful population of `parseopt_file`.

## Success Criteria

1. **Lookup parity**
   - For valid option file inputs accepted by the C module, the Rust module reports successful lookup and prepares an equivalent option-file context.
   - For invalid or unusable inputs rejected by the C module, the Rust module reports failure.
   - Traceability: `optfile_lookup`, `parseopt_file`, `stat`

2. **RC file processing parity**
   - For a readable rc/config file containing supported options, the Rust module completes parsing and returns success.
   - Traceability: `parseopt_from_rc`, `parseopt`, `optdef`, `wordsplit`

3. **Failure signaling parity**
   - When rc/config file lookup or preparation fails, the Rust module returns a failure result and does not treat the file as successfully parsed.
   - Traceability: `optfile_lookup`, `parseopt_from_rc`

4. **Option-definition integration**
   - File-sourced options are interpreted through the existing option-definition model rather than through a separate or invented configuration grammar.
   - Traceability: `parseopt_from_rc`, `parseopt`, `optdef`

5. **Scenario coverage**
   - The Rust implementation passes tests covering:
     - successful rc file loading,
     - missing file rejection,
     - unusable file rejection,
     - processing through the existing option-definition model.
   - Traceability: all scenarios in this document

## Out of Scope

The following are not required by this specification because they are not evidenced by the analyzed module input:

- new public APIs beyond the Rust equivalents needed for these functions,
- new configuration file formats,
- environment-variable-based configuration loading,
- concurrency guarantees,
- serialization or persistence features,
- recovery workflows beyond success/failure behavior already implied by the C code.