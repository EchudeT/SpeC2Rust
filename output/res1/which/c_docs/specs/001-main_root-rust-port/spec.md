# spec.md

## Title

Rust Functional Specification for `main_root` (`which`)

## Metadata

- Project: `which`
- Module: `main_root`
- Category: `main`
- Target Rust branch: `001-main_root-rust-port`
- Generation date: `2026-06-06`

## Overview

This module provides the executable-search behavior of the `which` program together with the command-line option handling required to run it as a CLI tool. Its observed responsibilities are:

- parse command-line options and operands,
- print usage and version information,
- inspect environment and current-user context,
- search for command names in a PATH-style list,
- determine whether candidate files are executable for the current user,
- normalize and print discovered paths,
- report lookup failures,
- process alias/function descriptions when supplied by command-line modes implemented in `which.c`.

The Rust rewrite must preserve the user-visible behavior evidenced by the analyzed sources in `which.c` and the support routines in `bash.c`, while also providing option parsing behavior corresponding to the module-local `getopt` and `getopt_long` entry points in `getopt.c` and `getopt1.c`.

## Scope

Included in scope:

- CLI option parsing used by this module.
- Usage/version output.
- PATH-based command lookup.
- Current-user and group-based executability checks.
- Current working directory retrieval when needed for path normalization/output.
- Alias/function text processing paths present in `which.c`.

Out of scope unless directly needed to match current behavior:

- Any new CLI features not evidenced by the analyzed functions.
- Any public library API beyond what is required for the executable’s behavior.
- Any persistence, networking, concurrency, or serialization behavior.

## Feature Specification

### 1. Command-line entry and option handling

The module must act as the main entry point for the `which` program and accept command-line arguments for command lookup and informational options.

Observed support includes:

- short-option parsing,
- long-option parsing,
- usage display,
- version display.

The Rust version must implement option handling sufficient to preserve the behavior exercised by the `which.c` main program and its documented helper output functions. If the original module distinguishes between normal option parsing and long-option parsing modes, the Rust rewrite must preserve the resulting CLI behavior visible to users of `which`.

### 2. Executable lookup in PATH-style search lists

The module must search for a command name in a colon-separated path list and return or print the first matching executable path according to the module’s current search mode.

This includes:

- iterating over path elements,
- handling empty or relative path elements consistently with the source behavior,
- joining directory elements with the command name,
- evaluating each candidate’s status before accepting it,
- tracking search progress through a path index during iterative search.

The Rust rewrite must preserve the command search behavior implemented by `find_command_in_path`, `path_search`, `extract_colon_unit`, `get_next_path_element`, and `make_full_pathname`.

### 3. Executability and file status evaluation

The module must determine whether a candidate filesystem path is a usable executable for the current user context.

This behavior is evidenced by:

- current uid/gid retrieval,
- supplementary group initialization,
- group membership checks,
- filesystem status inspection,
- executable permission evaluation.

The Rust version must preserve the effective acceptance/rejection behavior of `file_status` as used during command lookup.

### 4. Absolute path handling

The module must distinguish command operands that are already absolute pathnames from those requiring PATH search.

The Rust rewrite must preserve the behavior evidenced by `absolute_program` and its use within command lookup.

### 5. Environment and user-context access

The module must read environment-derived values used during lookup and path-related behavior, including:

- PATH-style values,
- HOME directory lookup support,
- current user account information.

The Rust rewrite must preserve the behavior required by `sh_get_env_value`, `sh_get_home_dir`, and `get_current_user_info` where these affect lookup results or output.

### 6. Current working directory support and path cleanup

The module must retrieve the current working directory when required and normalize resulting path strings before output.

The Rust rewrite must preserve the behavior evidenced by:

- `get_current_working_directory`,
- `path_clean_up`.

This requirement is limited to normalization/output behavior directly exercised by command lookup and reporting.

### 7. Alias and shell-function related processing

The module includes behavior for processing alias strings and function descriptions.

The Rust rewrite must implement the functional behavior evidenced by:

- `process_alias`,
- `func_search`,
- the `function_st` data structure and list traversal.

This includes searching a provided function list for a command and handling alias-driven command processing in the same operational contexts as the original module. No broader shell integration beyond the evidenced behavior is required.

### 8. Informational and failure output

The module must emit:

- usage text,
- version text,
- failure messages when a command cannot be found in the searched path list.

The Rust rewrite must preserve the existence and purpose of these outputs as evidenced by `print_usage`, `print_version`, and `print_fail`.

## User Scenarios & Testing

### Scenario 1: Show usage information

A user invokes the program with the option that requests usage/help text.

Expected behavior:

- the program prints usage information to the appropriate output stream,
- no command lookup is performed,
- process termination reflects informational execution rather than successful discovery of a command.

Traceability: `print_usage`, `main` in `which.c`, option-parsing support in `getopt.c` / `getopt1.c`.

### Scenario 2: Show version information

A user invokes the program with the option that requests version output.

Expected behavior:

- the program prints version information,
- no PATH search is required.

Traceability: `print_version`, `main` in `which.c`.

### Scenario 3: Find a command through PATH

A user runs `which` with a command name that is not an absolute path.

Expected behavior:

- the program obtains the relevant path list,
- iterates over path elements in order,
- combines each directory with the command name,
- checks whether the candidate is executable for the current user,
- prints the discovered path when a match is found.

Traceability: `find_command_in_path`, `path_search`, `file_status`, `get_next_path_element`, `make_full_pathname`, `get_current_user_info`.

### Scenario 4: Handle command not found

A user searches for a command that is absent from the searched path list.

Expected behavior:

- the program completes the search,
- prints a failure indication tied to the command and path list,
- exits in a manner indicating failure to locate the command.

Traceability: `print_fail`, `path_search`, `find_command_in_path`.

### Scenario 5: Accept an absolute pathname operand

A user passes an operand that is already an absolute program path.

Expected behavior:

- the program recognizes the operand as absolute,
- does not require normal PATH concatenation for that operand,
- applies file status checks consistent with executable validation.

Traceability: `absolute_program`, `file_status`.

### Scenario 6: Respect current user permissions

A candidate file exists in a searched directory, but its executable permission only applies to certain users/groups.

Expected behavior:

- the program evaluates the file against the invoking user’s uid, gid, and supplementary groups,
- it accepts the candidate only when executable according to the current user context.

Traceability: `uidget`, `getmaxgroups`, `initialize_group_array`, `group_member`, `get_current_user_info`, `file_status`.

### Scenario 7: Process empty or multiple PATH elements

A user runs the program with a PATH-style list containing multiple colon-delimited entries, including edge positions that require iterative parsing.

Expected behavior:

- each path element is extracted in sequence,
- candidate paths are generated consistently for each element,
- search order matches the original iteration behavior.

Traceability: `extract_colon_unit`, `get_next_path_element`, `find_command_in_path`.

### Scenario 8: Normalize output paths when working-directory context matters

A lookup path or resulting candidate requires cleanup relative to current directory context.

Expected behavior:

- the program can obtain the current working directory,
- output path strings are cleaned up before final reporting where the original module does so.

Traceability: `get_current_working_directory`, `path_clean_up`.

### Scenario 9: Search shell-function descriptions

The program is supplied with a function list and a target command for function-oriented search mode.

Expected behavior:

- the program scans the provided function entries,
- detects a matching function name,
- reports/searches according to the function search behavior implemented by the original module.

Traceability: `func_search`, `function_st`.

### Scenario 10: Process alias-provided command text

The program is supplied alias text plus command operands in the alias-processing path.

Expected behavior:

- alias text is processed,
- resulting command names are subjected to the same lookup machinery required by the original behavior,
- function-start handling follows the original mode distinctions where applicable.

Traceability: `process_alias`, `path_search`, `func_search`.

## Requirements

### Functional Requirements

#### FR-1: CLI execution entry
The Rust module shall provide the executable entry behavior for the `which` program and shall accept command-line arguments for option handling and command lookup.

Traceability: `main` in `which.c`; module-local option support in `getopt.c`, `getopt1.c`.

#### FR-2: Usage output
The Rust module shall output usage/help text when the corresponding command-line mode is selected.

Traceability: `print_usage`, `main` in `which.c`.

#### FR-3: Version output
The Rust module shall output version information when the corresponding command-line mode is selected.

Traceability: `print_version`, `main` in `which.c`.

#### FR-4: Short and long option parsing
The Rust module shall support the command-line parsing behavior required by this program for both short options and long options as evidenced by the current module.

Traceability: `_getopt_internal`, `getopt`, `getopt_long`, `getopt_long_only`, `main` in `which.c`.

#### FR-5: PATH element iteration
The Rust module shall parse colon-separated path lists element by element in search order.

Traceability: `extract_colon_unit`, `get_next_path_element`.

#### FR-6: Candidate pathname construction
For non-absolute command names, the Rust module shall construct candidate pathnames from a path element and a command name.

Traceability: `make_full_pathname`, `find_command_in_path`.

#### FR-7: Absolute-path recognition
The Rust module shall detect when a command operand is already an absolute program path and shall handle it without normal PATH concatenation.

Traceability: `absolute_program`.

#### FR-8: File executability evaluation
The Rust module shall determine whether a candidate filesystem object is an executable command for the current user context before accepting it as a search result.

Traceability: `file_status`.

#### FR-9: Current user context acquisition
The Rust module shall obtain and use current user identity data required for executability checks, including uid, gid, and supplementary group information.

Traceability: `uidget`, `getmaxgroups`, `initialize_group_array`, `group_member`, `get_current_user_info`.

#### FR-10: Environment-derived lookup values
The Rust module shall read environment-derived values needed by the current behavior, including variable lookup and home directory retrieval support.

Traceability: `sh_get_env_value`, `sh_get_home_dir`.

#### FR-11: Command search
The Rust module shall search for command names in the provided PATH-style list and shall return/report matches according to the original module behavior.

Traceability: `find_command_in_path`, `path_search`.

#### FR-12: Failure reporting
When no command match is found in the searched path list, the Rust module shall emit the module’s failure report for that command and path list.

Traceability: `print_fail`, `path_search`.

#### FR-13: Current working directory retrieval
The Rust module shall obtain the current working directory when required by path normalization or output behavior.

Traceability: `get_current_working_directory`.

#### FR-14: Path cleanup
The Rust module shall clean up path strings in the same operational contexts as the original module before output or further use.

Traceability: `path_clean_up`.

#### FR-15: Function-list search
The Rust module shall support searching a provided function list for a named command in the function-search path.

Traceability: `func_search`, `struct function_st`.

#### FR-16: Alias processing
The Rust module shall support processing alias text and applying the resulting lookup flow as evidenced by the original module.

Traceability: `process_alias`, `func_search`, `path_search`.

### Key Entities

#### `user_info`
A user-context record used to support executability decisions. It represents the current user identity information used by permission checks and related environment/account lookups.

Traceability: `struct user_info` in `bash.c`; `get_current_user_info`; `file_status`.

Relationship:
- populated from current account/system state,
- consumed by file executability checks.

#### Current group set
A module-maintained representation of the current process’s supplementary groups, used to determine whether a file’s group-execute permission applies to the user.

Traceability: `getmaxgroups`, `initialize_group_array`, `group_member`.

Relationship:
- derived from process/user context,
- consulted by `file_status`.

#### Filesystem candidate path
A constructed pathname formed from a path element and command name, or supplied directly as an absolute command path, that is tested for executability.

Traceability: `absolute_program`, `make_full_pathname`, `file_status`, `find_command_in_path`.

Relationship:
- produced during path search,
- validated by file-status logic,
- normalized when required for output.

#### Path list / path element
A colon-separated string and its extracted units used for ordered command search.

Traceability: `extract_colon_unit`, `get_next_path_element`, `find_command_in_path`, `path_search`.

Relationship:
- provides the search space for candidate path generation.

#### `function_st`
A function-description record used by function-oriented lookup behavior.

Traceability: `struct function_st` in `which.c`; `func_search`, `process_alias`.

Relationship:
- arranged as a searchable list,
- consumed by function and alias processing flows.

#### Option descriptor
The option structure used by long-option parsing.

Traceability: `struct option` usages in `getopt.c`, `getopt1.c`, `which.c`.

Relationship:
- drives command-line parsing behavior for the main program.

## Success Criteria

1. The Rust executable accepts the same categories of CLI inputs evidenced by this module: command operands, help/usage requests, version requests, and long-option forms used by the program.
   - Traceability: `main` in `which.c`; `_getopt_internal`; `getopt_long`; `getopt_long_only`.

2. When invoked in usage/help mode, the Rust executable emits usage text and does not attempt normal command lookup.
   - Traceability: `print_usage`, `main` in `which.c`.

3. When invoked in version mode, the Rust executable emits version text and does not require PATH search.
   - Traceability: `print_version`, `main` in `which.c`.

4. For a command name searched through a PATH-style list, the Rust executable examines path entries in order and reports a match only when the candidate satisfies the same executability criteria as the original module.
   - Traceability: `find_command_in_path`, `path_search`, `file_status`, `get_next_path_element`.

5. For a missing command, the Rust executable emits failure reporting corresponding to the original module’s not-found path.
   - Traceability: `print_fail`, `path_search`.

6. For an absolute pathname operand, the Rust executable bypasses ordinary PATH concatenation and applies direct candidate validation consistent with the original behavior.
   - Traceability: `absolute_program`, `file_status`.

7. Executability decisions in the Rust rewrite reflect current-user uid/gid and supplementary-group membership in the same decision space as the source module.
   - Traceability: `uidget`, `initialize_group_array`, `group_member`, `get_current_user_info`, `file_status`.

8. Path-list parsing in the Rust rewrite preserves ordered colon-unit extraction behavior used by the original search logic.
   - Traceability: `extract_colon_unit`, `get_next_path_element`.

9. When the original module’s lookup flow requires current working directory retrieval and path cleanup, the Rust rewrite produces cleaned path output consistent with that flow.
   - Traceability: `get_current_working_directory`, `path_clean_up`.

10. Function-list search and alias-processing paths present in `which.c` remain functionally available in the Rust rewrite for the same input categories handled by the original module.
    - Traceability: `func_search`, `process_alias`, `struct function_st`.