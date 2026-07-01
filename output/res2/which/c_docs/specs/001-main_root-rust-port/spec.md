# spec.md

## Title

Rust Functional Specification for `main_root` Module Port

## Metadata

- Project: `which`
- Module: `main_root`
- Category: `main`
- Target branch: `001-main_root-rust-port`
- Source basis date: `2026-06-07`

## Overview

This module defines the command-line program behavior for `which`. Its evidenced responsibilities are:

- parse command-line options, including long-option forms
- resolve command names against a PATH-style search list
- detect whether a candidate path refers to an executable command for the current user context
- print usage, version, successful matches, and failure diagnostics
- support lookup helpers tied to shell-style command resolution, including alias/function-oriented processing present in `which.c`
- obtain current process/user environment information needed for path resolution and permission checks

The Rust rewrite must preserve the observable behavior covered by these responsibilities. The specification is limited to behavior evidenced by the analyzed source files and functions:
`bash.c`, `getopt.c`, `getopt1.c`, and `which.c`.

## Feature Specification

### 1. Command-line entry behavior

The module provides the executable entry path for the program and accepts command-line arguments that control program behavior.

The Rust version must:

- accept positional command names to be resolved
- accept supported short and long options as evidenced by the option-parsing and option-table logic in `getopt.c`, `getopt1.c`, and `which.c`
- produce help/usage output when the usage path is selected
- produce version output when the version path is selected
- return process exit status consistent with success or failure of command resolution

This includes preserving argument parsing behavior sufficient for the program logic in `which.c` to receive the intended options and remaining command operands.

### 2. Option parsing

The module includes full option parsing support through `getopt`, `getopt_long`, and `getopt_long_only`.

The Rust version must implement option parsing behavior needed by this program, including:

- short option parsing
- long option parsing
- long-only parsing where applicable to the current executable behavior
- handling of option arguments where required by the option definitions
- tracking of the next non-option argument position so the remaining operands can be processed

The spec requires behavioral compatibility for the module’s own command-line handling, not a separately exposed general-purpose parser API.

### 3. PATH-based command lookup

The module resolves command names by searching a colon-separated path list.

The Rust version must:

- detect whether a command operand is already an absolute program path
- iterate path elements from a colon-delimited path list
- combine each path element with the command name to form a candidate pathname
- test candidates and return/report the first successful executable match according to module behavior
- support explicit reporting when a command is not found in the supplied search list

This behavior is evidenced by `find_command_in_path`, `path_search`, `extract_colon_unit`, `get_next_path_element`, `make_full_pathname`, `absolute_program`, and `print_fail`.

### 4. Executability and file status evaluation

The module determines whether a path is a usable command for the current process context.

The Rust version must:

- inspect filesystem status for a candidate path
- distinguish executable command candidates from non-usable paths using the current user and group context
- evaluate effective access based on user id, group membership, and executable permission bits as evidenced by `file_status`, `uidget`, `getmaxgroups`, `initialize_group_array`, `group_member`, and `get_current_user_info`

The spec requires preserving the program’s command-finding decision behavior, not reproducing internal C helper signatures.

### 5. Environment and user-context lookup

The module reads environment-derived values used by command resolution.

The Rust version must:

- obtain environment variable values needed by the module
- obtain the current user’s home directory when required by the current module behavior
- obtain and cache current user information relevant to file permission checks
- obtain the current working directory when required for path normalization or output behavior

This behavior is evidenced by `sh_get_env_value`, `sh_get_home_dir`, `get_current_user_info`, and `get_current_working_directory`.

### 6. Path element and path-string handling

The module includes helper behavior for path-string processing.

The Rust version must:

- extract substrings needed for path parsing
- read one colon-delimited path unit at a time from a search string
- normalize or clean up path strings where `which.c` requires it before reporting or matching
- form complete candidate pathnames from directory path plus command name

This behavior is evidenced by `substring`, `extract_colon_unit`, `get_next_path_element`, `make_full_pathname`, and `path_clean_up`.

### 7. Alias/function-related search support

The module includes shell-style lookup support beyond plain PATH search.

The Rust version must:

- support function-list searching as represented by `func_search`
- support alias or alias-like command processing as represented by `process_alias`
- allow these behaviors to participate in command resolution/output where the source module does so

The required scope is only what is evidenced by `which.c`; no additional shell integration or parsing features are implied.

### 8. Output behavior

The module prints user-facing text for normal and exceptional paths.

The Rust version must produce:

- usage/help text on the usage path
- version text on the version path
- found command paths during successful search
- failure text for unresolved commands where the source does so

The exact wording should remain compatible enough to preserve program intent and testability for these paths.

## User Scenarios & Testing

### Scenario 1: User requests help

A user runs the program with the help-triggering option.
Expected behavior:

- usage information is printed to the designated standard stream used by the program
- no command lookup is attempted
- the process exits successfully or with the same status behavior as the source path

Traceability: `print_usage`, option handling in `which.c`, option parsing in `getopt.c` / `getopt1.c`.

### Scenario 2: User requests version information

A user runs the program with the version-triggering option.
Expected behavior:

- version information is printed
- no command lookup is attempted
- exit behavior matches the source program’s version path

Traceability: `print_version`, option handling in `which.c`.

### Scenario 3: User searches for a simple command in PATH

A user invokes the program with a command name such as `ls` and a PATH environment containing one or more matching directories.
Expected behavior:

- the program reads the search path
- it iterates directories in order
- it constructs candidate full paths
- it identifies the first candidate that is executable for the current user context
- it prints the resolved path
- exit status indicates success

Traceability: `find_command_in_path`, `path_search`, `file_status`, path helper functions in `bash.c`.

### Scenario 4: User searches for a command that does not exist

A user invokes the program with a command name absent from the search path.
Expected behavior:

- all path elements are examined according to the module’s path iteration behavior
- no successful candidate is found
- failure output is produced where the source emits it
- exit status indicates failure

Traceability: `find_command_in_path`, `print_fail`, `path_search`.

### Scenario 5: User supplies an absolute command path

A user invokes the program with a command operand that is already absolute.
Expected behavior:

- the program recognizes it as an absolute program path
- resolution behavior follows the absolute-path handling evidenced by the source
- executability is checked using the same command-eligibility rules
- success or failure is reported accordingly

Traceability: `absolute_program`, `file_status`.

### Scenario 6: Candidate path exists but is not executable for the user

A candidate file is found in the search process but lacks executable permission for the current user or groups.
Expected behavior:

- the path is not treated as a successful command resolution
- search continues to later path elements if any remain
- if no valid executable is found, failure is reported

Traceability: `file_status`, `group_member`, `get_current_user_info`.

### Scenario 7: PATH contains empty or special elements relevant to current directory handling

A user’s path list contains entries that require per-element extraction and path cleanup behavior.
Expected behavior:

- the path list is consumed in sequence using colon-delimited parsing
- each unit is converted into a candidate path according to module behavior
- current-directory-related behavior, if triggered by empty/path-cleanup logic in the source, is preserved in results

Traceability: `extract_colon_unit`, `get_next_path_element`, `path_clean_up`, `get_current_working_directory`.

### Scenario 8: Alias or function-oriented lookup path is exercised

The program is invoked in a mode or with inputs that cause alias/function processing logic to run.
Expected behavior:

- function list searching and alias processing behave consistently with `which.c`
- any corresponding output or nested lookup behavior follows the source module

Traceability: `func_search`, `process_alias`, `struct function_st`.

### Testing expectations

The Rust port must be testable with at least:

- option parsing tests for help/version and operand handling
- path parsing tests for multi-element and empty-element PATH strings
- filesystem permission tests covering executable and non-executable candidates
- resolution tests for found, not found, and absolute-path inputs
- alias/function path tests for the behavior directly evidenced by `which.c`

## Requirements

### Functional Requirements

#### FR-1: Program entry and argument handling
The module shall provide the executable’s main command-processing behavior, accept command-line arguments, and dispatch to usage, version, or command-resolution paths.

Traceability: `main` in `which.c`; option parsing support in `getopt.c` and `getopt1.c`.

#### FR-2: Short and long option parsing
The module shall parse short options and long options required by the program, including option arguments where defined, and shall leave command operands available for subsequent processing.

Traceability: `_getopt_internal`, `getopt`, `getopt_long`, `getopt_long_only`; `struct option`.

#### FR-3: Usage and version output
The module shall print usage/help output and version output on their respective command paths.

Traceability: `print_usage`, `print_version`.

#### FR-4: PATH-list traversal
The module shall traverse a colon-separated path list in order, extracting one path element at a time for command lookup.

Traceability: `extract_colon_unit`, `get_next_path_element`.

#### FR-5: Candidate pathname construction
The module shall construct candidate full pathnames from a path element and a command name.

Traceability: `make_full_pathname`.

#### FR-6: Absolute-path recognition
The module shall recognize when a command operand is an absolute program path and apply the corresponding resolution logic.

Traceability: `absolute_program`.

#### FR-7: Command candidate status evaluation
The module shall evaluate whether a candidate filesystem path is a valid executable command for the current user context.

Traceability: `file_status`.

#### FR-8: User and group context support
The module shall obtain current user identity and group-membership information needed by executable-status evaluation.

Traceability: `uidget`, `getmaxgroups`, `initialize_group_array`, `group_member`, `get_current_user_info`, `struct user_info`.

#### FR-9: Environment lookup support
The module shall obtain environment variable values and current-user home-directory information needed by the module’s command-resolution behavior.

Traceability: `sh_get_env_value`, `sh_get_home_dir`.

#### FR-10: Current working directory support
The module shall obtain the current working directory where required by path cleanup or resolution behavior.

Traceability: `get_current_working_directory`.

#### FR-11: Path normalization helpers
The module shall perform the path cleanup behavior used by the program before reporting or continuing lookup.

Traceability: `path_clean_up`.

#### FR-12: Command search execution
The module shall search for commands in the provided path list and report success or failure according to the source program behavior.

Traceability: `find_command_in_path`, `path_search`, `print_fail`.

#### FR-13: Function-list search support
The module shall support searching a list of shell-style function descriptors where the program invokes that behavior.

Traceability: `func_search`, `struct function_st`.

#### FR-14: Alias processing support
The module shall support the alias-processing behavior evidenced by the program and allow it to trigger further command-resolution behavior as in the source.

Traceability: `process_alias`.

### Key Entities

#### `user_info`
A process/user-context record used to hold current user identity information relevant to permission checks and environment-derived user state.

Relationship:
- populated by current-user lookup behavior
- consumed by executable-status evaluation and group-membership logic

Traceability: `struct user_info`, `get_current_user_info`, `file_status`, `group_member`.

#### `option`
An option-definition record describing long-option parsing behavior for command-line handling.

Relationship:
- consumed by option parsing functions
- used by the main program flow to interpret command-line switches

Traceability: `struct option`, `_getopt_internal`, `getopt_long`, `getopt_long_only`, `main` in `which.c`.

#### `function_st`
A function-descriptor record used for function-list searching and related shell-style command reporting.

Relationship:
- consumed by `func_search`
- participates in alias/function-related lookup paths in `which.c`

Traceability: `struct function_st`, `func_search`, `process_alias`.

## Success Criteria

### SC-1: Help path
When invoked with the help-triggering option supported by the source program, the Rust port prints usage text and does not perform command lookup.

Traceability: `print_usage`, option handling in `which.c`.

### SC-2: Version path
When invoked with the version-triggering option supported by the source program, the Rust port prints version text and does not perform command lookup.

Traceability: `print_version`, option handling in `which.c`.

### SC-3: Found command resolution
Given a PATH list containing at least one executable match for an input command, the Rust port reports a successful resolution consistent with source search order.

Traceability: `find_command_in_path`, `path_search`, `file_status`.

### SC-4: Not-found handling
Given a PATH list with no valid executable match for an input command, the Rust port produces failure behavior consistent with the source program and returns a failure status.

Traceability: `print_fail`, `path_search`.

### SC-5: Absolute-path handling
Given an absolute command path, the Rust port recognizes it without PATH traversal beyond the source behavior and applies executable-status checks consistently.

Traceability: `absolute_program`, `file_status`.

### SC-6: Permission-sensitive filtering
If a candidate file exists in a searched directory but is not executable for the current user/group context, the Rust port does not report it as a successful command.

Traceability: `file_status`, `group_member`, `get_current_user_info`.

### SC-7: PATH parsing correctness
For colon-separated path input, including cases that exercise empty-unit extraction or cleanup behavior, the Rust port preserves the source module’s path-element iteration behavior.

Traceability: `extract_colon_unit`, `get_next_path_element`, `path_clean_up`.

### SC-8: Alias/function behavior preservation
For inputs that exercise alias processing or function-list search in the source module, the Rust port preserves the same observable lookup/output path supported by `which.c`.

Traceability: `func_search`, `process_alias`, `struct function_st`.