# spec.md

## Title

Rust Functional Specification for `module_tilde`

## Metadata

- **Project**: `which`
- **Module**: `module_tilde`
- **Category**: `module`
- **Rust branch**: `002-module_tilde-rust-port`
- **Source basis**: `tilde/shell.c`, `tilde/tilde.c`
- **Generation date**: 2026-06-07

## Overview

This module is responsible for tilde-related path handling and home-directory lookup behavior.

Based on the analyzed source, the module provides two closely related areas of functionality:

1. Determining a home directory for the current user.
2. Identifying and processing tilde expansion boundaries within strings, including recognition of the tilde prefix and the point where a username or tilde expression ends.

The Rust rewrite must preserve the module’s observable behavior in these areas. The specification is limited to behavior evidenced by the analyzed files and functions and does not introduce additional capabilities.

## Feature Specification

### Summary

The Rust version must implement tilde-processing behavior suitable for path-like strings that begin with or contain tilde expressions, along with home-directory lookup support used by that processing.

### Included functionality

#### 1. Current-user home directory lookup

The module must provide behavior equivalent to retrieving the home directory for the current user.

This behavior is evidenced by `get_home_dir` in `tilde/shell.c` and use of `struct passwd` in the analyzed files. The Rust version must support:

- obtaining the current user’s home directory when available;
- returning the home-directory value as string data suitable for path expansion;
- handling the case where the home directory cannot be determined without inventing substitute paths.

#### 2. Detection of a tilde expansion prefix

The module must detect whether a candidate string position begins a tilde expansion and determine the relevant prefix length or boundary information needed for subsequent processing.

This behavior is evidenced by `tilde_find_prefix` in `tilde/tilde.c`. The Rust version must support:

- examining an input string for a tilde-introduced segment;
- recognizing the start of a tilde expression;
- reporting enough boundary information to drive expansion logic;
- distinguishing strings that do not contain a recognized tilde prefix from those that do.

#### 3. Detection of the end of a tilde username/expression segment

The module must identify where the username or tilde-specific segment ends after a tilde prefix begins.

This behavior is evidenced by `tilde_find_suffix` in `tilde/tilde.c`. The Rust version must support:

- scanning forward from a tilde-introduced segment;
- detecting the boundary after the username or tilde token;
- returning a result that allows callers to separate the expandable portion from the remainder of the input.

#### 4. Failure behavior for unrecoverable memory allocation problems

The module includes explicit abort behavior for memory exhaustion or equivalent unrecoverable allocation failure.

This behavior is evidenced by `memory_error_and_abort` in `tilde/tilde.c`. The Rust rewrite must preserve the same functional boundary:

- memory-allocation failure in module-managed tilde-processing that is treated as unrecoverable must terminate operation rather than silently producing a fabricated expansion result.

No broader recovery contract is specified beyond this evidenced behavior.

## User Scenarios & Testing

The Rust version must support the following scenarios.

### Scenario 1: Expand a bare home shortcut

A caller provides a string beginning with `~` and expects the module to recognize the tilde prefix and use the current user’s home directory as the replacement basis.

**Expected behavior**
- The tilde prefix is recognized.
- The tilde expression boundary is identified.
- The current user’s home directory is obtained when available.
- The result uses that home directory in place of the tilde expression.

**Relevant source evidence**
- `get_home_dir`
- `tilde_find_prefix`
- `tilde_find_suffix`

### Scenario 2: Expand a tilde followed by a path remainder

A caller provides a string like `~/subdir/file` and expects only the tilde expression portion to be replaced, preserving the remainder of the path unchanged.

**Expected behavior**
- The start of the tilde expression is recognized.
- The end of the expandable tilde segment is identified before the remainder.
- The non-tilde suffix of the original string is preserved after expansion.

**Relevant source evidence**
- `tilde_find_prefix`
- `tilde_find_suffix`
- `get_home_dir`

### Scenario 3: Process a string that does not contain a recognized tilde prefix

A caller provides a string that does not begin with, or otherwise does not present, a recognized tilde expression for this module.

**Expected behavior**
- The module reports that no tilde prefix is available for expansion.
- No home-directory substitution is performed for that string.

**Relevant source evidence**
- `tilde_find_prefix`

### Scenario 4: Determine the username/expression boundary in a tilde-prefixed string

A caller provides a string with a tilde-introduced segment and needs the module to identify where that segment ends so the expandable portion can be separated from the remainder.

**Expected behavior**
- The module detects the end boundary of the username or tilde token.
- The returned boundary is consistent enough for a caller to split the input into expandable and non-expandable parts.

**Relevant source evidence**
- `tilde_find_suffix`

### Scenario 5: Home directory is unavailable

A caller triggers home-directory lookup, but the environment or account data does not yield a home directory.

**Expected behavior**
- The module does not invent a replacement home path.
- The unavailable-home case is surfaced consistently to the caller-facing logic of this module.

**Relevant source evidence**
- `get_home_dir`
- `struct passwd` usage in `tilde/shell.c` and `tilde/tilde.c`

### Scenario 6: Unrecoverable allocation failure during tilde processing

A caller triggers module behavior that requires allocation, and allocation fails in a context treated by the module as fatal.

**Expected behavior**
- The module terminates rather than returning a misleading expansion result.
- The failure behavior is consistent with the original module’s abort-oriented contract.

**Relevant source evidence**
- `memory_error_and_abort`

## Requirements

### Functional Requirements

#### FR-1: Provide current-user home directory lookup

The module shall provide functionality to obtain the current user’s home directory when such information is available from the host system.

**Traceability**
- `tilde/shell.c`: `get_home_dir`
- `struct passwd` usage in `tilde/shell.c`

#### FR-2: Indicate when no home directory is available

The module shall distinguish successful home-directory lookup from the case where no home directory can be determined, without fabricating a substitute path.

**Traceability**
- `tilde/shell.c`: `get_home_dir`
- `struct passwd` usage in `tilde/shell.c`

#### FR-3: Detect a tilde expansion prefix in input text

The module shall detect whether an input string contains a recognized tilde prefix suitable for expansion and shall provide boundary information required by the expansion flow.

**Traceability**
- `tilde/tilde.c`: `tilde_find_prefix`

#### FR-4: Detect the end of the tilde-specific segment

The module shall determine the endpoint of the tilde username/expression segment so that callers can separate the expandable portion from any following remainder.

**Traceability**
- `tilde/tilde.c`: `tilde_find_suffix`

#### FR-5: Support expansion flow that combines tilde detection and home lookup

For inputs representing current-user tilde forms, the module shall support the functional flow of recognizing the tilde segment, obtaining the home directory, and combining the expansion with any remaining suffix text.

**Traceability**
- `tilde/tilde.c`: `tilde_find_prefix`, `tilde_find_suffix`
- `tilde/shell.c`: `get_home_dir`

#### FR-6: Preserve non-expandable remainder text after the tilde segment

When a tilde expression is followed by additional path text, the module shall preserve that remainder after replacing only the tilde-related segment.

**Traceability**
- `tilde/tilde.c`: `tilde_find_suffix`
- `tilde/shell.c`: `get_home_dir`

#### FR-7: Fail fatally on unrecoverable memory error in this module’s allocation path

The module shall preserve the original module’s fatal behavior for allocation failures that are treated as unrecoverable within tilde processing.

**Traceability**
- `tilde/tilde.c`: `memory_error_and_abort`

### Key Entities

#### Entity: Input string under tilde analysis

A string value presented to the module for tilde recognition and expansion-related scanning.

**Role**
- Serves as the source text examined for a tilde prefix and suffix boundary.

**Traceability**
- `tilde_find_prefix`
- `tilde_find_suffix`

#### Entity: Current user home directory

A string value representing the home directory associated with the current user, when available.

**Role**
- Acts as the replacement content for current-user tilde expansion.

**Traceability**
- `get_home_dir`

#### Entity: System account record (`passwd`)

Host-system account metadata used to obtain home-directory information.

**Role**
- Provides the underlying account/home-directory source used by lookup logic.

**Traceability**
- `struct passwd` usage in `tilde/shell.c`
- `struct passwd` usage in `tilde/tilde.c`

#### Entity: Tilde prefix boundary

The identified start and length/boundary information for a recognized tilde expression.

**Role**
- Allows the module to determine whether expansion should occur and what portion of the input is part of the tilde syntax.

**Traceability**
- `tilde_find_prefix`

#### Entity: Tilde suffix boundary

The identified endpoint of the username or tilde token following the tilde introducer.

**Role**
- Allows separation of the expandable tilde portion from the remaining text.

**Traceability**
- `tilde_find_suffix`

#### Entity relationships

- The **input string under tilde analysis** is examined to locate a **tilde prefix boundary**.
- Once a tilde expression is recognized, the module identifies the **tilde suffix boundary** to delimit the expandable segment.
- For current-user expansion, the module resolves the **current user home directory** using the **system account record (`passwd`)**.
- The expansion flow combines the resolved home directory with any remaining text that follows the tilde-delimited segment.

## Success Criteria

### SC-1: Current-user home lookup parity

Given an environment where the current user has a resolvable home directory, the Rust module returns a home-directory string usable by tilde expansion logic.

**Traceability**
- `get_home_dir`

### SC-2: Missing-home behavior parity

Given an environment where the current user’s home directory cannot be determined, the Rust module reports absence of a home directory and does not substitute an invented path.

**Traceability**
- `get_home_dir`

### SC-3: Tilde prefix detection parity

For inputs with a recognized tilde expression, the Rust module detects that expression and provides boundary information sufficient for downstream expansion behavior; for inputs without such a recognized prefix, it indicates no match.

**Traceability**
- `tilde_find_prefix`

### SC-4: Tilde suffix detection parity

For tilde-prefixed inputs, the Rust module identifies the endpoint of the tilde-specific segment consistently enough that the expandable portion and the remaining suffix can be separated.

**Traceability**
- `tilde_find_suffix`

### SC-5: Expansion-flow parity for current-user tilde forms

For current-user tilde forms with trailing path content, the Rust module replaces only the tilde expression portion with the resolved home directory and preserves the trailing content unchanged.

**Traceability**
- `get_home_dir`
- `tilde_find_prefix`
- `tilde_find_suffix`

### SC-6: Fatal allocation failure parity

When the module encounters an allocation failure in a code path corresponding to the original module’s unrecoverable memory-error handling, the Rust rewrite does not continue with a fabricated result and instead terminates consistently with the original fatal contract.

**Traceability**
- `memory_error_and_abort`

## Out of Scope

The Rust rewrite specification does not require or imply:

- new public APIs beyond those needed to preserve the evidenced module behavior;
- support for unrelated shell expansion features not evidenced in the analyzed files;
- thread-safety guarantees;
- serialization or persistence behavior;
- recovery-oriented handling for failures that the original module treats as fatal;
- performance or benchmark targets.