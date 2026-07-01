# spec.md

## Title

Rust Functional Specification for `module_tilde`

## Metadata

- Project: `which`
- Module: `module_tilde`
- Category: `module`
- Target branch: `002-module_tilde-rust-port`
- Source basis:
  - `tilde/shell.c`
  - `tilde/tilde.c`
- Generation date: 2026-06-06

## Overview

`module_tilde` provides tilde-related pathname expansion support.

Based on the analyzed source, the module is responsible for:
- obtaining the current user's home directory,
- recognizing whether a string begins with a tilde expression that is eligible for expansion,
- recognizing where a tilde expression ends,
- using system user account information (`struct passwd`) to resolve home-directory-based expansions,
- aborting processing when internal memory allocation cannot be completed.

The Rust rewrite must preserve this functional boundary: it must implement tilde expansion behavior centered on home-directory resolution and user-name-based lookup, with equivalent acceptance and rejection boundaries for tilde prefixes and suffixes as defined by the source module.

## Feature Specification

### Summary

The Rust version must implement tilde expression handling for path-like strings.

A tilde expression is a string form beginning with `~` that may refer either to:
- the current user's home directory, or
- another user's home directory.

The module must support the following functional areas evidenced by the source:
1. current-user home directory retrieval,
2. detection of a valid tilde prefix at the start of a string,
3. detection of the boundary where the tilde username portion ends,
4. expansion of eligible tilde expressions using system account information,
5. hard failure on unrecoverable internal memory exhaustion.

### Supported behavior

The Rust module must support expansion decisions consistent with these source responsibilities:

- If input begins with a tilde form representing the current user home reference, the module resolves it using the current user's home directory.
- If input begins with a tilde form containing a user name, the module resolves that user name through system account information and substitutes the matched home directory.
- If the input does not begin with a valid tilde expression, the module leaves it unexpanded.
- If the tilde expression cannot be resolved to a home directory, the module must not fabricate a path; behavior must remain consistent with unresolved expansion rather than inventing a substitute.
- Detection of the expandable portion must stop at the appropriate suffix boundary rather than consuming unrelated trailing text.

### Out of scope

The Rust version must not claim or introduce functionality not evidenced by the analyzed files, including:
- new public capabilities unrelated to tilde expansion,
- persistence or serialization,
- concurrency guarantees,
- recovery-oriented memory failure handling beyond the existing abort behavior,
- expansion of unrelated shell syntax beyond tilde-related logic.

## User Scenarios & Testing

### Scenario 1: Expand current user home shorthand

A caller passes a string that begins with `~` and represents the current user's home directory reference.

Expected behavior:
- the module recognizes the prefix as expandable,
- obtains the current user's home directory,
- returns the input with the tilde prefix replaced by that home directory,
- preserves the remainder of the path after the tilde expression boundary.

Test examples:
- input equivalent to `~`
- input equivalent to `~/bin`
- input equivalent to `~/<rest>`

Acceptance checks:
- expansion begins only when the tilde is at the start of the string,
- the resulting string begins with the resolved home directory,
- the suffix after the tilde expression remains in order.

### Scenario 2: Expand named user home shorthand

A caller passes a string beginning with `~username` optionally followed by more path content.

Expected behavior:
- the module recognizes the tilde prefix and determines the end of the username segment,
- looks up the named user via system account information,
- if found, replaces the tilde expression with that user's home directory,
- preserves any trailing suffix after the username boundary.

Test examples:
- input equivalent to `~root`
- input equivalent to `~root/bin`
- input equivalent to `~user/<rest>`

Acceptance checks:
- only the username portion after `~` is used for account lookup,
- trailing path content is appended after expansion,
- the output uses the looked-up home directory for the matched user.

### Scenario 3: Non-tilde-leading input is not expanded

A caller passes a string containing `~` somewhere other than the start, or with no tilde expression.

Expected behavior:
- the module does not treat the string as a tilde expansion candidate,
- no home directory lookup is performed for expansion purposes,
- the content remains unexpanded.

Test examples:
- input equivalent to `abc~`
- input equivalent to `/tmp/~user`
- input equivalent to `plain/path`

Acceptance checks:
- the module rejects these as tilde-prefix matches,
- output is unchanged with respect to tilde expansion.

### Scenario 4: Unresolvable named user remains unexpanded

A caller passes `~username` where the user does not exist or cannot be resolved from account data.

Expected behavior:
- the module identifies the candidate tilde expression,
- attempts user lookup,
- if lookup fails, does not substitute an invented directory.

Test examples:
- input equivalent to `~definitely_not_a_real_user`
- input equivalent to `~definitely_not_a_real_user/bin`

Acceptance checks:
- no successful expansion occurs,
- the module does not return a fabricated home path.

### Scenario 5: Prefix and suffix boundaries are respected

A caller passes a string where a tilde expression is followed by additional content.

Expected behavior:
- prefix detection determines whether expansion is valid,
- suffix detection determines where the tilde user portion stops,
- only the intended initial tilde expression is replaced.

Test examples:
- input equivalent to `~user/path`
- input equivalent to `~/path`
- input equivalent to `~user`
- input equivalent to `~`

Acceptance checks:
- the username boundary is not extended into later path separators or unrelated suffix text,
- the non-prefix remainder is preserved verbatim after expansion.

### Scenario 6: Memory exhaustion causes hard failure

During processing, the module encounters an internal memory allocation failure.

Expected behavior:
- the module treats this as unrecoverable for this module's operation,
- processing aborts rather than silently continuing with incomplete state.

Acceptance checks:
- failure mode is explicit and terminal for the operation,
- no partial successful result is reported after such a failure.

## Requirements

### Functional Requirements

#### FR-1: Current user home directory resolution

The module shall provide behavior equivalent to retrieving the current user's home directory for use in tilde expansion.

Traceability:
- `get_home_dir` in `tilde/shell.c`

#### FR-2: Tilde-prefix recognition

The module shall determine whether the start of an input string is a valid tilde expansion prefix and shall identify the prefix length needed for subsequent processing.

Traceability:
- `tilde_find_prefix` in `tilde/tilde.c`

#### FR-3: Tilde-expression suffix detection

The module shall determine where the tilde expression's user-reference portion ends so that only the intended leading expression is expanded.

Traceability:
- `tilde_find_suffix` in `tilde/tilde.c`

#### FR-4: Current-user tilde expansion

When the input begins with the current-user tilde form, the module shall expand that form to the current user's home directory.

Traceability:
- `get_home_dir` in `tilde/shell.c`
- `tilde_find_prefix` in `tilde/tilde.c`
- `tilde_find_suffix` in `tilde/tilde.c`
- use of `struct passwd` in `tilde/tilde.c`

#### FR-5: Named-user tilde expansion

When the input begins with a named-user tilde form, the module shall attempt to resolve that user through system account information and, if successful, expand the leading expression to that user's home directory.

Traceability:
- `tilde_find_prefix` in `tilde/tilde.c`
- `tilde_find_suffix` in `tilde/tilde.c`
- use of `struct passwd` in `tilde/tilde.c`

#### FR-6: No expansion for non-matching inputs

If the input string does not begin with a valid tilde expansion prefix, the module shall not perform tilde expansion on that input.

Traceability:
- `tilde_find_prefix` in `tilde/tilde.c`

#### FR-7: No fabricated result for unresolved named user

If a named-user tilde expression cannot be resolved from system account information, the module shall not synthesize a replacement home directory.

Traceability:
- use of `struct passwd` in `tilde/tilde.c`
- suffix/prefix-driven expansion logic in `tilde/tilde.c`

#### FR-8: Preserve non-expanded suffix content

When a leading tilde expression is expanded, the module shall preserve and append the remainder of the input after the tilde-expression boundary.

Traceability:
- `tilde_find_suffix` in `tilde/tilde.c`

#### FR-9: Abort on internal memory allocation failure

If the module cannot allocate required memory during its operation, it shall terminate processing via unrecoverable failure behavior equivalent to the source module.

Traceability:
- `memory_error_and_abort` in `tilde/tilde.c`

### Key Entities

#### System user account record

The module relies on system user account records represented in the source by `struct passwd`.

Role:
- supplies home directory information for the current user and for named users,
- acts as the authoritative lookup source for tilde expansion resolution.

Traceability:
- `struct passwd` usages in `tilde/shell.c`
- `struct passwd` usages in `tilde/tilde.c`

#### Input string

The primary input entity is a string that may begin with a tilde expression.

Role:
- source text inspected for an expandable prefix,
- source of the user-name portion for account lookup,
- source of the trailing suffix preserved after expansion.

Traceability:
- `tilde_find_prefix`
- `tilde_find_suffix`

#### Prefix length / suffix boundary

The module derives boundaries within the input string to control expansion.

Role:
- prefix recognition determines whether expansion is allowed,
- suffix detection determines how much of the leading text participates in expansion,
- together they define the replacement span.

Traceability:
- `tilde_find_prefix`
- `tilde_find_suffix`

#### Home directory string

The resolved home directory is the replacement value inserted when expansion succeeds.

Role:
- replacement for `~`,
- replacement for `~username`.

Traceability:
- `get_home_dir`
- `struct passwd`-based lookups in `tilde/tilde.c`

## Success Criteria

### Behavioral correctness

- Inputs beginning with current-user tilde syntax are expanded to the current user's home directory in all supported cases evidenced by the module.
- Inputs beginning with named-user tilde syntax expand to the matched user's home directory when that user exists in system account data.
- Inputs not beginning with a valid tilde expression remain unexpanded.
- For successful expansions, only the leading tilde expression is replaced; the remaining suffix is preserved unchanged.
- Inputs referencing unresolved users do not produce fabricated home-directory substitutions.

### Boundary correctness

- Prefix recognition behavior is consistent with the source module's acceptance boundary for expandable tilde-leading strings.
- Suffix recognition behavior is consistent with the source module's boundary for where the tilde user reference ends.

### Failure correctness

- Internal memory exhaustion during module operation produces unrecoverable failure behavior equivalent to the source module's abort path.
- The Rust rewrite does not silently ignore or partially complete an operation after such a memory failure.

### Traceable verification targets

At minimum, the Rust rewrite shall be verified with tests covering:
- `~`
- `~/subpath`
- `~existinguser`
- `~existinguser/subpath`
- non-prefix tilde occurrences
- nonexistent-user tilde input
- cases demonstrating correct end-of-expression boundary handling
- memory-failure behavior at the module-defined unrecoverable boundary