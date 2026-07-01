# spec.md

## Title
Functional Specification: `main_root_progname.c_21` Rust Port

## Document Metadata
- Project: `pwd`
- Module: `main_root_progname.c_21`
- Category: `main_cluster`
- Source file: `progname.c`
- Primary function: `set_program_name`
- Rust branch: `021-main_root_progname.c_21-rust-port`
- Generation date: 2026-06-09

## Overview
This module is responsible for establishing the process program name from the `argv[0]` value supplied at program startup.

The Rust rewrite must preserve the observable behavior of this responsibility: accepting the startup program path string, deriving the program name component used by the process, and making that name available to the rest of the program in the same role as the C module.

This module is part of the main program startup path and serves as a normalization step between raw startup arguments and later code that depends on the program name identity.

## Feature Specification

### Purpose
The module provides a single startup-oriented capability: initialize the program name from the command invocation string.

### Functional Scope
The Rust version must implement the following behavior evidenced by the source module:
- Accept an input corresponding to `argv[0]`.
- Derive the program name from that input rather than treating the full invocation path as the final name.
- Support invocation strings that may include directory components.
- Establish the derived program name for subsequent program use after initialization.
- Operate as an initialization routine intended to be called during main-program startup.

### Out of Scope
The Rust version must not introduce capabilities not evidenced by this module, including:
- Additional command-line parsing behavior.
- New public APIs unrelated to setting the program name.
- Thread-safety guarantees beyond source-evidenced behavior.
- Persistence, serialization, recovery, or interprocess communication features.

## User Scenarios & Testing

### Scenario 1: Startup with a simple executable name
A program starts with `argv[0]` equal to a plain executable name such as `pwd`.

Expected behavior:
- The module accepts the input.
- The stored program name is `pwd`.

Test implication:
- A startup test passes `pwd` as the input and verifies that the program-name state becomes `pwd`.

### Scenario 2: Startup with a path-qualified executable
A program starts with `argv[0]` containing a path such as `/usr/bin/pwd` or `./pwd`.

Expected behavior:
- The module derives the final name component from the path.
- The stored program name is `pwd`, not the full path.

Test implication:
- Tests provide path-qualified startup inputs and verify basename extraction behavior.

### Scenario 3: Startup before other program logic
The main program invokes this module before code that depends on the program name.

Expected behavior:
- After initialization, later code can rely on the established program name being available in the module-defined role.

Test implication:
- An integration-oriented test sequence calls the initialization routine first and then verifies the program-name state is available to downstream logic.

### Scenario 4: Reuse across supported invocation forms
The same binary may be invoked through different path spellings.

Expected behavior:
- Different valid forms of `argv[0]` that identify the same executable name produce the same derived program name.

Test implication:
- Tests compare outputs for `pwd`, `./pwd`, and `/some/path/pwd`, expecting the same resulting name.

## Requirements

### Functional Requirements

#### FR-1: Program invocation input acceptance
The module shall accept a program invocation string corresponding to the process startup `argv[0]`.
- Traceability: `progname.c`, `set_program_name`

#### FR-2: Program name derivation
The module shall derive the program name from the invocation string, rather than preserving directory prefixes as part of the final program name.
- Traceability: `progname.c`, `set_program_name`

#### FR-3: Path-form handling
The module shall correctly handle invocation strings that include path separators and produce the final path component as the program name.
- Traceability: `progname.c`, `set_program_name`

#### FR-4: Initialization role
The module shall function as a startup initialization step whose effect is to establish program-name state for later use by the program.
- Traceability: `progname.c`, `set_program_name`

#### FR-5: Observable consistency
For equivalent invocation forms that differ only by included directory components, the derived program name shall be identical when the terminal executable name is the same.
- Traceability: `progname.c`, `set_program_name`

### Key Entities

#### Program invocation string
The startup string corresponding to `argv[0]`, provided as input to the module.

Relationship:
- This is the source value from which the program name is derived.

#### Program name
The normalized executable name established by the module for subsequent program use.

Relationship:
- This value is derived from the invocation string by removing any path prefix and retaining the terminal name component.

## Success Criteria

### SC-1: Simple-name correctness
When initialized with a simple executable name, the Rust module stores or exposes that same name as the program name.
- Traceability: `set_program_name`

### SC-2: Path-name normalization
When initialized with a path-qualified executable string, the Rust module stores or exposes only the terminal executable component.
- Traceability: `set_program_name`

### SC-3: Consistent derivation across invocation forms
Inputs representing the same executable terminal name through different path forms yield the same resulting program name.
- Traceability: `set_program_name`

### SC-4: Startup usability
The Rust module can be invoked during main-program startup and leaves program-name state available for subsequent program logic in the same functional role as the C module.
- Traceability: `progname.c`, `set_program_name`

### SC-5: Scope preservation
The Rust port provides the evidenced program-name initialization behavior without adding unrelated command-line or process-management responsibilities.
- Traceability: `progname.c`, `set_program_name`