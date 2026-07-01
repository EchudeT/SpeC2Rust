# spec.md

## Title

Functional Specification for Rust Port of `module_src_wordsplit_wordsplit_03`

## Document Control

- Project: `cflow-new`
- Module: `module_src_wordsplit_wordsplit_03`
- Category: `module_cluster`
- Source file: `src/wordsplit/wordsplit.c`
- Rust branch: `114-module_src_wordsplit_wordsplit_03-rust-port`
- Generation date: `2026-06-11`

## Overview

This module is the expansion and cleanup stage of the `wordsplit` pipeline. It operates on an existing `wordsplit` state and its linked list of `wordsplit_node` items, and performs:

- variable expansion
- positional/parameter-vector expansion
- command substitution expansion
- removal of empty nodes created by expansion
- whitespace trimming of expanded results

The Rust rewrite must preserve the observable behavior of these transformations as driven by the current `wordsplit` configuration and flags.

This specification is limited to the functionality evidenced by the analyzed functions:

- `expvar_recover`
- `expand_paramv`
- `expvar`
- `node_expand`
- `wsnode_nullelim`
- `wordsplit_varexp`
- `expcmd`
- `wordsplit_cmdexp`
- `wordsplit_trimws`

## Feature Specification

### Feature 1: Variable and Parameter Expansion

The module must expand variable-like expressions found in eligible nodes of the `wordsplit` node list.

Supported behavior evidenced by the module includes:

- scanning text for expansion openings
- expanding shell-style parameter references
- using module flags and quoting context to decide expansion behavior
- producing replacement node content and appending it into the output node chain
- tracking where parsing of the source text stopped
- handling recovery/fallback for expansion forms that cannot be completed as valid expansions

The Rust version must preserve the distinction between:

- successful expansion
- invalid or incomplete expansion that is recovered as literal text where the C logic does so
- expansion behavior affected by flags and quote state

### Feature 2: Parameter-Vector Expansion

The module must support expansion from a parameter vector source associated with the `wordsplit` state.

This behavior is evidenced by `expand_paramv`, and the Rust version must preserve:

- expansion that may emit one or more output nodes
- behavior controlled by flags and quote state
- integration into the same node-output chain used by other variable expansions

### Feature 3: Node-Oriented Expansion Pass

The module must apply expansion to the `wordsplit_node` list node by node, using a generic expansion pass helper.

The Rust version must preserve the behavior of:

- selecting nodes that are eligible for an expansion pass
- invoking the appropriate expansion function on node text
- replacing or appending node content according to expansion results
- preserving the overall node ordering except where expansion intentionally replaces a node with zero, one, or multiple result nodes

### Feature 4: Null/Empty Node Elimination

After expansion, the module must remove nodes that are considered empty by this stage.

The Rust version must preserve:

- elimination of expansion-produced empty nodes from the active node list
- non-retention of empty artifacts that would otherwise affect later word processing

### Feature 5: Command Substitution Expansion

The module must expand command substitution syntax found in eligible nodes.

The Rust version must preserve:

- parsing command substitution expressions from node text
- executing the command-substitution expansion path under module control
- replacing source text with resulting text in the node chain
- honoring flags that affect parsing or expansion context
- returning an error status when command expansion fails in ways the C module reports as failure

This specification does not require any broader shell implementation beyond the command-expansion behavior evidenced by `expcmd` and `wordsplit_cmdexp`.

### Feature 6: Whitespace Trimming After Expansion

The module must trim whitespace from node content during its cleanup stage as evidenced by `wordsplit_trimws`.

The Rust version must preserve:

- trimming behavior applied across the current node list
- update of node content after trimming
- status reporting consistent with the C stage entry point

## User Scenarios & Testing

### Scenario 1: Expand variables in a prepared wordsplit state

A caller has already built a `wordsplit` state containing one or more nodes with text that includes variable references. The module performs the variable-expansion pass and updates the node list so that variable references are replaced by their expanded values.

#### Expected outcomes

- nodes containing expandable variable syntax are transformed
- nodes without such syntax remain unchanged
- any parse endpoint tracking used by the stage produces equivalent results to the C behavior
- the stage returns success or failure consistently with the C logic

#### Test focus

- simple variable reference expands to text
- mixed literal and variable text expands correctly
- invalid/incomplete variable syntax follows recovery behavior instead of arbitrary failure where `expvar_recover` applies

### Scenario 2: Expand positional or parameter-vector content

A caller configures the `wordsplit` state with parameter-vector data and provides a node containing syntax that resolves through that parameter source. The module expands it into the node chain.

#### Expected outcomes

- parameter-vector expansion can produce replacement content
- quoted/unquoted context affects output in the same way as in C
- multi-item results are represented correctly in the node list

#### Test focus

- single parameter-vector item expansion
- multiple parameter-vector items expansion
- behavior differences under quote-related flags

### Scenario 3: Apply expansion across multiple nodes

A caller has a linked node list where only some nodes begin with or contain expansion syntax eligible for a given pass. The module runs the node-oriented expansion helper over the full list.

#### Expected outcomes

- only eligible nodes are expanded during the pass
- output node order remains stable
- replacement of one source node by zero, one, or many nodes is handled correctly

#### Test focus

- list with alternating expandable and non-expandable nodes
- node that expands to empty content
- node that expands to multiple result nodes

### Scenario 4: Remove empty nodes after expansion

Expansion produces empty node entries. The module performs null elimination.

#### Expected outcomes

- empty nodes are removed from the active list
- non-empty neighboring nodes remain linked in the correct order
- no empty expansion artifacts remain visible to subsequent processing

#### Test focus

- single empty node in middle of list
- leading empty node
- trailing empty node
- list containing only empty nodes

### Scenario 5: Perform command substitution expansion

A caller provides nodes containing command substitution syntax. The module executes the command-expansion pass and rewrites node content with the resulting command output.

#### Expected outcomes

- recognized command substitution syntax is expanded
- resulting text is inserted into the node chain
- failure paths are reported as non-success status when the C code would do so

#### Test focus

- simple command substitution
- mixed literal text plus command substitution
- malformed command substitution syntax
- command-substitution result that includes whitespace requiring later trimming

### Scenario 6: Trim whitespace after expansions

After variable or command expansion, some node values contain leading or trailing whitespace. The cleanup stage trims that whitespace.

#### Expected outcomes

- leading and trailing whitespace are removed as defined by the C module behavior
- node content is updated in place in logical terms
- pass-level status matches the C function result contract

#### Test focus

- leading whitespace only
- trailing whitespace only
- both leading and trailing whitespace
- node content entirely composed of whitespace

## Requirements

### Functional Requirements

#### FR-1: Variable expansion pass
The module shall provide a variable-expansion pass over the current `wordsplit` node list, traceable to `wordsplit_varexp`, `node_expand`, and `expvar`.

#### FR-2: Variable expression parsing and replacement
The module shall parse variable-expression syntax from node text and replace recognized expressions with expanded content, traceable to `expvar`.

#### FR-3: Recovery for incomplete or invalid variable forms
The module shall preserve the C module’s recovery behavior for variable expressions that cannot be completed as valid expansions, traceable to `expvar_recover` and `expvar`.

#### FR-4: Parameter-vector expansion
The module shall support expansion based on parameter-vector content held in the `wordsplit` state, traceable to `expand_paramv` and `expvar`.

#### FR-5: Quote- and flag-sensitive expansion behavior
The module shall preserve expansion behavior that depends on flags and quoting context during variable and parameter expansion, traceable to `expand_paramv`, `expvar`, `node_expand`, and `expcmd`.

#### FR-6: Node-list transformation semantics
The module shall transform the `wordsplit_node` linked list by replacing one input node with zero, one, or multiple output nodes according to expansion results, traceable to `node_expand` and `wordsplit_node`.

#### FR-7: Empty-node elimination
The module shall remove empty nodes produced or exposed by expansion passes, traceable to `wsnode_nullelim`.

#### FR-8: Command substitution expansion pass
The module shall provide a command-substitution expansion pass over eligible nodes in the current `wordsplit` state, traceable to `wordsplit_cmdexp`, `node_expand`, and `expcmd`.

#### FR-9: Command substitution parsing and replacement
The module shall parse command-substitution syntax from node text and replace it with the resulting expansion text, traceable to `expcmd`.

#### FR-10: Whitespace trimming pass
The module shall provide a pass that trims whitespace in node content after expansion, traceable to `wordsplit_trimws`.

#### FR-11: Stage result reporting
Each expansion or cleanup pass shall return success/failure status consistent with the source module’s stage functions, traceable to `wordsplit_varexp`, `wordsplit_cmdexp`, `wordsplit_trimws`, `expvar`, and `expcmd`.

### Key Entities

#### `wordsplit`
Primary per-operation state for the wordsplitting pipeline.

Role evidenced by the analyzed functions:

- owns or references the active node list
- carries flags that affect parsing and expansion behavior
- provides access to parameter-vector data used by parameter expansion
- is passed through all expansion and cleanup passes as the shared context

#### `wordsplit_node`
Linked-list element representing a word fragment or expanded result.

Role evidenced by the analyzed functions:

- stores text subject to expansion or trimming
- participates in ordered node-chain transformations
- may be replaced, removed, or expanded into multiple nodes during a pass

#### Relationship between entities

- one `wordsplit` instance manages a sequence of `wordsplit_node` items
- expansion passes read node text from that sequence and write transformed nodes back into that same logical sequence
- cleanup passes operate on the resulting sequence to remove empty nodes and trim whitespace

## Success Criteria

### SC-1: Equivalent variable-expansion outcomes
For representative inputs exercising `wordsplit_varexp`, `node_expand`, `expvar`, and `expvar_recover`, the Rust port produces the same final node texts and pass status as the C module.

### SC-2: Equivalent parameter-vector expansion outcomes
For representative inputs exercising `expand_paramv`, the Rust port produces the same node segmentation, text content, and status as the C module under matching flags and quote context.

### SC-3: Equivalent node-list rewrite behavior
For node-list inputs where expansion yields zero, one, or multiple replacement nodes, the Rust port preserves the same node order and effective list contents as the C module, traceable to `node_expand` and `wordsplit_node`.

### SC-4: Equivalent empty-node removal
For node lists containing empty entries after expansion, the Rust port removes the same nodes as the C module and retains the same non-empty sequence, traceable to `wsnode_nullelim`.

### SC-5: Equivalent command-substitution outcomes
For representative inputs exercising `wordsplit_cmdexp` and `expcmd`, the Rust port produces the same expanded text results and success/failure status as the C module.

### SC-6: Equivalent whitespace trimming
For representative post-expansion node texts, the Rust port trims whitespace to the same final node texts and returns the same pass status as the C module, traceable to `wordsplit_trimws`.

### SC-7: No unsupported feature expansion
The Rust port limits itself to the behaviors evidenced in `src/wordsplit/wordsplit.c` for this module scope and does not require additional public capabilities beyond variable expansion, parameter-vector expansion, command substitution, empty-node elimination, and whitespace trimming.