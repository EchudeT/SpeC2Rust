# spec.md

## Title
Rust Functional Specification for `main_root` Module

## Metadata
- Project: `which`
- Module: `main_root`
- Category: `main`
- Target branch: `001-main_root-rust-port`
- Generation date: `2026-06-09`

## Overview
The `main_root` module provides the executable-facing behavior for the `which` program and includes the command-line option parsing support used by that program. Its evidenced responsibilities are:

- parsing command-line options and arguments;
- printing usage and version output;
- resolving command names against a PATH-like search list;
- determining whether candidate files are executable for the current user;
- handling current-user and current-directory context needed for path resolution;
- reporting found commands and failures;
- processing shell-style alias/function text where supported by the original module’s command-processing flow.

The Rust rewrite must preserve the observable behavior of this module as a command-oriented entry module, including option handling, command lookup behavior, and user-visible output categories that are evidenced by the source functions.

## Scope
This specification covers the functionality evidenced by these source areas:

- `which.c`: user-facing program behavior for usage/version output, command lookup, path normalization, alias/function-related search flow.
- `bash.c`: user, group, environment, pathname, and executable-status helpers used by lookup behavior.
- `getopt.c` and `getopt1.c`: option parsing behavior used by command-line processing.

This specification does not require adding capabilities not evidenced by the analyzed module.

## Feature Specification

### 1. Command-line entry behavior
The module shall behave as the main entry path for the `which` executable.

It must:
- accept command-line arguments;
- parse supported short and long options through getopt-style behavior;
- distinguish option processing from command operands;
- invoke usage or version output when the corresponding options are requested;
- process one or more command names for lookup after option parsing.

### 2. Usage and version reporting
The module shall provide user-visible help and version reporting.

It must:
- print a usage/help message to a selected output stream;
- print version information;
- terminate command-processing flow appropriately after such informational output.

### 3. PATH-based command lookup
The module shall search for command names within a colon-separated path list.

It must:
- accept a command name and a path list;
- iterate through path elements in order;
- construct candidate full pathnames from each path element and the command name;
- test candidate files for existence and executable accessibility for the current user context;
- return or print matching pathnames according to the command-processing flow;
- report failure when no matching executable is found in the path list.

### 4. Absolute or path-qualified command handling
The module shall recognize when a command string is already path-qualified.

It must:
- detect whether a command string is absolute or otherwise contains pathname information requiring direct treatment rather than pure PATH search;
- evaluate such names against executable/file status rules used by the module.

### 5. Executable status evaluation
The module shall determine whether a filesystem object is executable by the invoking user.

It must:
- inspect file metadata;
- account for current user identity and group membership;
- distinguish executable candidates from non-executable or missing files;
- use that result during command lookup.

### 6. User and environment context
The module shall obtain runtime context needed for lookup behavior.

It must:
- obtain the current user’s uid-related information;
- obtain group membership information needed for executable checks;
- obtain environment values used by the module, including home directory information where requested by the original logic;
- obtain the current working directory when required for path cleanup or relative-path interpretation.

### 7. Colon-separated path element extraction
The module shall support parsing PATH-like strings.

It must:
- extract path elements from a colon-separated list in sequence;
- preserve ordered traversal semantics;
- support empty or relative path elements as required by the original path-processing flow.

### 8. Path cleanup and normalization for output/search flow
The module shall provide path cleanup behavior used by command search.

It must:
- derive a cleaned-up path representation from input path text;
- use current working directory information where the original logic requires it;
- return a path form suitable for subsequent lookup/output behavior.

### 9. Alias and shell-function search flow
The module shall support the alias/function-oriented processing evidenced in `which.c`.

It must:
- evaluate function/alias-related command descriptions using the module’s `function_st` records and processing functions;
- support searching a function list for a command name;
- support alias-processing flow that can expand or inspect command text and continue resolution through path searching where the original control flow requires it.

The Rust rewrite must preserve only the behavior evidenced by:
- `func_search`
- `path_search`
- `process_alias`

This does not imply invention of a broader shell parser beyond that evidenced flow.

### 10. Failure reporting
The module shall report unsuccessful command lookup.

It must:
- emit a failure message that identifies the command name and the path list context used for the failed search, as evidenced by `print_fail`.

## User Scenarios & Testing

### Scenario 1: User requests help
A user invokes the executable with a help option.

Expected support:
- options are parsed successfully;
- usage text is printed;
- normal command lookup is not performed afterward for that invocation path.

Test evidence mapping:
- `print_usage`
- getopt-based parsing in `getopt.c` / `getopt1.c`

### Scenario 2: User requests version information
A user invokes the executable with a version option.

Expected support:
- options are parsed successfully;
- version text is printed;
- normal command lookup is not performed afterward for that invocation path.

Test evidence mapping:
- `print_version`
- getopt-based parsing

### Scenario 3: User looks up a simple command via PATH
A user asks for the location of a command name that has no slash or absolute prefix.

Expected support:
- the module reads or receives the applicable path list;
- path elements are examined in order;
- the first or applicable matching executable path is identified according to the original command-processing behavior;
- the resulting pathname is output.

Test evidence mapping:
- `find_command_in_path`
- `path_search`
- `extract_colon_unit`
- `get_next_path_element`
- `make_full_pathname`
- `file_status`

### Scenario 4: Command is not present in PATH
A user looks up a command name that does not resolve to an executable in the path list.

Expected support:
- all path elements are searched in order;
- no false positive is reported;
- a failure message is produced.

Test evidence mapping:
- `find_command_in_path`
- `print_fail`
- `file_status`

### Scenario 5: User provides a path-qualified command
A user passes a command string that is already absolute or path-qualified.

Expected support:
- the module detects that the command is not a pure PATH-search token;
- executable/file accessibility is evaluated directly under the same status rules;
- output/failure behavior follows the original command-processing logic.

Test evidence mapping:
- `absolute_program`
- `file_status`

### Scenario 6: Executability depends on user/group permissions
A file exists but executability depends on the current user and group memberships.

Expected support:
- current user information is loaded;
- relevant group membership is available;
- file accessibility is determined using that user/group context;
- lookup result reflects executable permission accurately for the current user.

Test evidence mapping:
- `get_current_user_info`
- `uidget`
- `getmaxgroups`
- `initialize_group_array`
- `group_member`
- `file_status`

### Scenario 7: PATH contains empty or relative elements
A user’s path list includes empty fields or relative entries.

Expected support:
- colon-separated parsing preserves element order;
- the module handles such entries consistently with original path extraction and cleanup behavior;
- command search works using the cleaned path forms expected by the original module.

Test evidence mapping:
- `extract_colon_unit`
- `get_next_path_element`
- `path_clean_up`
- `get_current_working_directory`

### Scenario 8: Alias/function processing path
A caller provides alias text or function list data and asks the module to process a command through that flow.

Expected support:
- function-list searching can identify a matching command entry;
- alias-processing logic can inspect/process the supplied text and continue with path-based command resolution where the original module does so;
- behavior remains bounded to the evidenced alias/function search flow.

Test evidence mapping:
- `func_search`
- `process_alias`
- `path_search`
- `function_st`

## Requirements

### Functional Requirements

#### FR-1: Main program command processing
The Rust module shall provide the executable’s main command-processing flow, including argument intake, option parsing, and command operand handling.

Traceability:
- `main` in `getopt.c`
- `main` in `getopt1.c`

#### FR-2: Getopt-compatible option parsing
The Rust module shall support getopt-style parsing for short options and evidenced long-option handling needed by the module’s main flow.

Traceability:
- `_getopt_internal`
- `getopt`
- `getopt_long`
- `getopt_long_only`
- `struct option`

#### FR-3: Usage output
The Rust module shall print usage/help text through a dedicated usage-reporting path.

Traceability:
- `print_usage`

#### FR-4: Version output
The Rust module shall print version information through a dedicated version-reporting path.

Traceability:
- `print_version`

#### FR-5: Failed lookup reporting
The Rust module shall report unsuccessful search results with the command name and path-list context.

Traceability:
- `print_fail`

#### FR-6: PATH search
The Rust module shall search for commands in a colon-separated path list and evaluate candidate pathnames in path order.

Traceability:
- `find_command_in_path`
- `path_search`
- `extract_colon_unit`
- `get_next_path_element`
- `make_full_pathname`

#### FR-7: Direct pathname detection
The Rust module shall distinguish path-qualified command strings from plain command names.

Traceability:
- `absolute_program`

#### FR-8: Executable file status determination
The Rust module shall determine whether a candidate file is executable for the current user.

Traceability:
- `file_status`

#### FR-9: Current user context acquisition
The Rust module shall obtain and use current user identity and related account information required for executable checks and environment-derived behavior.

Traceability:
- `get_current_user_info`
- `uidget`
- `sh_get_home_dir`
- `sh_get_env_value`
- `struct user_info`
- `struct passwd`

#### FR-10: Group membership support
The Rust module shall obtain and use group membership information required for file executability decisions.

Traceability:
- `getmaxgroups`
- `initialize_group_array`
- `group_member`

#### FR-11: PATH element extraction
The Rust module shall parse colon-separated path strings incrementally.

Traceability:
- `substring`
- `extract_colon_unit`
- `get_next_path_element`

#### FR-12: Full pathname construction
The Rust module shall construct candidate full pathnames from a path element and command name.

Traceability:
- `make_full_pathname`

#### FR-13: Current working directory acquisition
The Rust module shall obtain current working directory information when needed by path cleanup/search behavior.

Traceability:
- `get_current_working_directory`

#### FR-14: Path cleanup
The Rust module shall provide path cleanup behavior used before or during command resolution.

Traceability:
- `path_clean_up`

#### FR-15: Function-list search
The Rust module shall support searching supplied function records for a command name.

Traceability:
- `func_search`
- `struct function_st`

#### FR-16: Alias processing flow
The Rust module shall support the alias-processing path evidenced by the original module, including interaction with command arguments and path searching where applicable.

Traceability:
- `process_alias`
- `path_search`
- `struct function_st`

### Key Entities

#### `user_info`
A user-context record used by the module to hold current-user information relevant to executable permission checks and environment/home-related behavior.

Relationships:
- populated by `get_current_user_info`;
- used by `file_status`;
- related to group membership helpers and passwd-derived data.

#### Group membership state
Module-held current-group information used to determine whether a file’s group execute bit applies to the invoking user.

Relationships:
- initialized by `initialize_group_array`;
- sized or bounded by `getmaxgroups`;
- queried by `group_member`;
- consumed by `file_status`.

#### `struct option`
An option-definition record for getopt-compatible parsing of short/long command-line options.

Relationships:
- consumed by `_getopt_internal`;
- used by `getopt_long` and `getopt_long_only`;
- supports main command-processing flow.

#### `function_st`
A function/alias-related record used by shell-style command search logic.

Relationships:
- searched by `func_search`;
- participates in `process_alias`;
- can lead into `path_search`.

#### Path-list text and path elements
A colon-separated string and its extracted elements used to search for command executables.

Relationships:
- parsed by `extract_colon_unit` / `get_next_path_element`;
- combined with command names by `make_full_pathname`;
- cleaned by `path_clean_up`;
- consumed by `find_command_in_path` and `path_search`.

#### Candidate file status
Filesystem metadata and executable accessibility outcome for a candidate pathname.

Relationships:
- obtained through `file_status`;
- depends on current user and group context;
- drives found/not-found search outcomes.

## Success Criteria

1. The Rust module accepts command-line arguments and parses supported short/long options through getopt-compatible behavior sufficient for the evidenced main flows.
   - Traceability: `main`, `_getopt_internal`, `getopt`, `getopt_long`, `getopt_long_only`

2. When invoked through the help path, the Rust module emits usage output and does not continue normal lookup processing for that invocation.
   - Traceability: `print_usage`

3. When invoked through the version path, the Rust module emits version output and does not continue normal lookup processing for that invocation.
   - Traceability: `print_version`

4. For a plain command name and a supplied PATH-like list, the Rust module searches path elements in order and resolves executable candidates using the module’s file-status rules.
   - Traceability: `find_command_in_path`, `path_search`, `extract_colon_unit`, `get_next_path_element`, `make_full_pathname`, `file_status`

5. For a command that is not resolvable in the supplied path list, the Rust module emits failure output rather than reporting a non-existent executable.
   - Traceability: `print_fail`, `find_command_in_path`

6. For a path-qualified command string, the Rust module recognizes that it is not a plain PATH token and applies direct executable evaluation behavior.
   - Traceability: `absolute_program`, `file_status`

7. Executable-status decisions in the Rust module reflect current user identity and group membership rather than only file existence.
   - Traceability: `get_current_user_info`, `group_member`, `file_status`

8. Colon-separated path parsing in the Rust module preserves ordered traversal and supports the path element extraction behavior required for lookup.
   - Traceability: `substring`, `extract_colon_unit`, `get_next_path_element`

9. The Rust module supports current-directory-dependent path cleanup behavior used by command searching.
   - Traceability: `get_current_working_directory`, `path_clean_up`

10. The Rust module supports the evidenced function/alias search flow, including searching provided function records and processing alias text in conjunction with path search.
    - Traceability: `func_search`, `process_alias`, `path_search`, `function_st`

11. All observable outputs required by usage, version, successful lookup, and failed lookup flows are produced through the corresponding command-processing paths of the Rust rewrite.
    - Traceability: `print_usage`, `print_version`, `print_fail`, `path_search`, `find_command_in_path`