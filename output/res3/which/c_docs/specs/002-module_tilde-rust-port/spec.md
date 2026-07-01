# module_tilde Functional Specification

## Overview

This specification defines the required behavior for the Rust rewrite of `module_tilde` from the `which` project.

The module is responsible for tilde-related path handling behavior evidenced by the analyzed source files:

- `tilde/shell.c`
- `tilde/tilde.c`

The analyzed functions show two functional areas:

1. retrieval of the current user's home directory
2. parsing and expansion support for strings containing tilde-prefixed path segments

The Rust version on branch `002-module_tilde-rust-port` must preserve the observable behavior of this module as a tilde expansion component. The specification is limited to functionality evidenced by the analyzed files and functions and does not introduce unrelated APIs or capabilities.

## Scope

In scope:

- determining the current user's home directory
- recognizing where a tilde expression begins within a string
- recognizing where a tilde expression ends within a string
- supporting expansion logic for tilde-prefixed path text
- handling allocation failure as a fatal module error

Out of scope:

- shell parsing beyond tilde boundary recognition
- unrelated path normalization
- filesystem validation of expanded paths
- features not evidenced by the analyzed module files

## Feature Specification

### Feature: Current user home directory lookup

The module must provide behavior equivalent to obtaining the current user's home directory.

This behavior is evidenced by `get_home_dir` in `tilde/shell.c` and use of `struct passwd` in both source files. The Rust rewrite must support resolving a home directory for the current user in a way that preserves the module's purpose: enabling tilde expansion that depends on the current user's home path.

Expected behavior:

- A caller can request the current user's home directory.
- The returned value is suitable for use as the expansion result for a bare `~` or `~/...` form.
- The result must represent a path string.
- If the home directory cannot be determined, the module must not fabricate a path.

### Feature: Tilde expression boundary detection

The module must identify whether a tilde prefix is present and where it begins, and must identify where a tilde expression ends.

This behavior is evidenced by:

- `tilde_find_prefix`
- `tilde_find_suffix`

Expected behavior:

- The module can inspect an input string and determine whether it contains a tilde expression eligible for expansion.
- The module can determine the starting position of that eligible tilde expression.
- The module can determine the boundary at which the tilde expression stops, so the username or tilde token can be separated from following text.
- The identified suffix boundary must support forms where path text follows the tilde expression.

The Rust rewrite must preserve boundary recognition behavior needed to distinguish:

- no tilde expression present
- a tilde expression for the current user
- a tilde expression containing a username segment

### Feature: Tilde expansion support

The module must support the functional behavior implied by combining home-directory lookup with tilde boundary detection: expansion of tilde-prefixed path text into a concrete path string.

Expected behavior:

- A bare `~` expands using the current user's home directory.
- A `~/...` form expands by replacing `~` with the current user's home directory and preserving the remainder.
- A `~name` or `~name/...` form must be recognized as a user-specific tilde form.
- User-specific tilde handling must be based on user account information, as evidenced by `struct passwd` use in `tilde/tilde.c`.

If a named user's home directory cannot be resolved, the Rust version must preserve failure behavior consistent with the original module's role: the module must not silently invent a replacement path.

### Feature: Fatal memory failure behavior

The module must preserve the behavior that allocation failure during its internal processing is treated as a fatal error.

This is directly evidenced by `memory_error_and_abort`.

Expected behavior:

- If the module cannot complete required memory allocation for tilde processing, execution must terminate through a fatal error path rather than returning a successful result.
- The Rust rewrite must preserve this fail-stop semantic at the module boundary where allocation failure would otherwise affect correctness.

## User Scenarios & Testing

### Scenario 1: Expand the current user's home shorthand

A caller provides a path-like string beginning with `~`.

Examples:

- `~`
- `~/bin`
- `~/project/file`

Expected outcomes:

- The module recognizes the tilde expression at the start.
- The module determines the boundary of the tilde token correctly.
- The module replaces the tilde token with the current user's home directory.
- Any remaining suffix after the tilde token is preserved.

Test intent:

- verify successful current-user expansion
- verify suffix preservation
- verify handling of both bare and slash-followed forms

### Scenario 2: Expand another user's home shorthand

A caller provides a path-like string beginning with a named user tilde form.

Examples:

- `~alice`
- `~alice/src`

Expected outcomes:

- The module recognizes the tilde expression and username segment.
- The module determines where the username segment ends.
- The module resolves the named user's home directory using account information.
- The module substitutes the resolved home directory and preserves any trailing suffix.

Test intent:

- verify named-user recognition
- verify username/suffix boundary detection
- verify substitution using user account data

### Scenario 3: Input contains no eligible tilde expression

A caller provides a string with no tilde expansion candidate.

Examples:

- `/usr/bin`
- `alice/~tmp` when not in an eligible prefix position
- `plain-text`

Expected outcomes:

- The module reports no tilde expansion candidate.
- The input remains unchanged by tilde processing.

Test intent:

- verify that non-candidate strings are not altered
- verify correct negative result from prefix detection

### Scenario 4: Unresolvable home directory

A caller requests expansion, but the needed home directory cannot be resolved.

Examples:

- current user home lookup fails
- named user does not exist or has no resolvable home directory

Expected outcomes:

- The module does not invent or substitute an arbitrary path.
- Expansion does not produce a false successful path.

Test intent:

- verify failure path for missing current user home
- verify failure path for missing named user home

### Scenario 5: Memory exhaustion during tilde processing

A caller triggers module behavior that requires allocation, and allocation fails.

Expected outcomes:

- The module follows the fatal error path.
- The module does not return a normal successful expansion result after allocation failure.

Test intent:

- verify fail-stop behavior equivalent to `memory_error_and_abort`

## Requirements

### Functional Requirements

#### FR-1: Resolve current user home directory
The module shall provide behavior to obtain the current user's home directory for use in tilde expansion.

Traceability: `tilde/shell.c`, `get_home_dir`, `struct passwd`

#### FR-2: Detect tilde expansion prefix
The module shall inspect an input string and determine whether it contains a tilde expansion candidate, including the position of the candidate and the relevant token length needed for expansion processing.

Traceability: `tilde/tilde.c`, `tilde_find_prefix`

#### FR-3: Detect tilde expression suffix boundary
The module shall determine the end boundary of a tilde expression so that the tilde token or username portion can be separated from any trailing path text.

Traceability: `tilde/tilde.c`, `tilde_find_suffix`

#### FR-4: Expand bare and current-user tilde forms
The module shall support expansion of `~` and `~/...` forms using the current user's home directory.

Traceability: `tilde/shell.c`, `get_home_dir`; `tilde/tilde.c`, `tilde_find_prefix`, `tilde_find_suffix`

#### FR-5: Expand named-user tilde forms
The module shall support recognition and user-account-based resolution of `~name` and `~name/...` forms.

Traceability: `tilde/tilde.c`, `tilde_find_prefix`, `tilde_find_suffix`, `struct passwd`

#### FR-6: Preserve non-tilde text outside the expansion token
When expanding a tilde expression, the module shall preserve text outside the detected tilde token, including any trailing suffix after the token boundary.

Traceability: `tilde/tilde.c`, `tilde_find_suffix`

#### FR-7: Do not report successful expansion when required user home data is unavailable
If the current or named user's home directory cannot be resolved, the module shall not produce a fabricated expanded path as a successful result.

Traceability: `tilde/shell.c`, `get_home_dir`; `tilde/tilde.c`, `struct passwd`

#### FR-8: Abort on memory allocation failure during tilde processing
If required memory allocation for module processing fails, the module shall terminate via a fatal error path rather than completing normally.

Traceability: `tilde/tilde.c`, `memory_error_and_abort`

### Key Entities

#### Entity: User account record
The module relies on user account information represented by `struct passwd`.

Role:

- supplies the home directory for the current user
- supplies the home directory for a named user in `~name` forms

Traceability: `struct passwd` usages in `tilde/shell.c` and `tilde/tilde.c`

#### Entity: Input string under tilde analysis
The module processes an input string that may contain a tilde expansion candidate.

Role:

- source text for prefix detection
- source text for suffix boundary detection
- source text to be partially replaced during expansion

Traceability: `tilde_find_prefix`, `tilde_find_suffix`

#### Entity: Tilde token boundaries
The module derives logical boundaries for a tilde expression: where it begins and where it ends.

Role:

- determines whether expansion applies
- isolates the token to replace
- separates username content from trailing path content

Traceability: `tilde_find_prefix`, `tilde_find_suffix`

#### Entity: Home directory path
The module produces or consumes a home directory path string to substitute for a tilde token.

Role:

- replacement value for `~`
- replacement value for `~name`

Traceability: `get_home_dir`, `struct passwd`

## Success Criteria

### SC-1: Current-user expansion correctness
Given inputs `~` and `~/subdir`, the Rust module expands them using the current user's resolved home directory and preserves any trailing suffix.

Traceability: `get_home_dir`, `tilde_find_prefix`, `tilde_find_suffix`

### SC-2: Named-user expansion correctness
Given inputs `~name` and `~name/subdir` for a resolvable user, the Rust module resolves that user's home directory and substitutes it correctly.

Traceability: `tilde_find_prefix`, `tilde_find_suffix`, `struct passwd`

### SC-3: Non-candidate strings are not expanded
Given strings without an eligible tilde expression, the Rust module reports no expansion candidate or leaves the string unchanged.

Traceability: `tilde_find_prefix`

### SC-4: Boundary detection preserves suffix text
For inputs where path text follows a tilde token, the Rust module identifies the token boundary such that trailing text after the boundary is preserved exactly in the expanded result.

Traceability: `tilde_find_suffix`

### SC-5: Missing user-home data does not yield fabricated success
When current-user or named-user home lookup cannot be resolved, the Rust module does not return a falsely expanded path.

Traceability: `get_home_dir`, `struct passwd`

### SC-6: Fatal behavior on allocation failure
Under induced allocation failure in module-controlled memory use, the Rust module terminates through a fatal error path consistent with the original module's fail-stop behavior.

Traceability: `memory_error_and_abort`

## Acceptance Notes

- Conformance is determined by observable behavior of tilde recognition, boundary detection, home-directory substitution, and fatal handling of allocation failure.
- This specification does not require adding new public interfaces beyond what is needed to preserve the evidenced module behavior.
- This specification does not require support for behaviors not evidenced by `tilde/shell.c` and `tilde/tilde.c`.