# spec.md

## Title
Rust Functional Specification for `module_src_wordsplit_wordsplit_03`

## Metadata
- Project: `cflow-new`
- Module: `module_src_wordsplit_wordsplit_03`
- Category: `module_cluster`
- Source file: `src/wordsplit/wordsplit.c`
- Rust branch: `114-module_src_wordsplit_wordsplit_03-rust-port`
- Generation date: `2026-06-17`

## Overview
This module is the expansion and post-processing portion of the `wordsplit` engine. It operates on an existing `wordsplit` state and its linked list of word nodes to perform:

- variable expansion,
- positional-parameter vector expansion,
- command substitution,
- null-result elimination after expansion,
- and whitespace trimming of resulting words.

The Rust rewrite must preserve the same functional role inside the larger `wordsplit` pipeline: it receives already parsed word nodes, transforms node contents according to expansion rules and flags, updates the node list, and reports success or failure through module-level status results.

## Scope
Included in scope for this module:
- expansion of variable references from text fragments,
- handling of variable-expansion fallback or recovery paths,
- expansion of parameter vectors into word nodes,
- expansion over selected nodes in the word list,
- command substitution expansion over selected nodes,
- removal of nodes made empty by expansion where required,
- trimming of leading and trailing whitespace in resulting words.

Out of scope:
- initial lexical splitting and tokenization,
- definition of the full public `wordsplit` API beyond what is needed to support this module’s behavior,
- shell execution semantics beyond command-substitution behavior evidenced by this module,
- unrelated memory-management or utility helpers not required by these functions.

## Source Evidence
This specification is derived from the following functions in `src/wordsplit/wordsplit.c`:

- `expvar_recover`
- `expand_paramv`
- `expvar`
- `node_expand`
- `wsnode_nullelim`
- `wordsplit_varexp`
- `expcmd`
- `wordsplit_cmdexp`
- `wordsplit_trimws`

It also depends on the module’s core state and node structures:
- `struct wordsplit`
- `struct wordsplit_node`

## Feature Specification

### 1. Variable Expansion
The module must expand variable expressions found in node text segments selected for variable expansion.

Supported behavior, as evidenced by the variable-expansion functions, includes:
- scanning text for expandable variable syntax,
- replacing a recognized variable reference with its resolved value,
- appending resulting text and/or newly produced nodes into the current output sequence,
- tracking where parsing of the source fragment stopped,
- honoring expansion-control flags and quote-sensitive behavior passed through the expansion pipeline.

The Rust version must preserve the observable result of variable expansion on the word list:
- text outside variable syntax remains present in order,
- resolved variable values are inserted in place of the original reference,
- expansion status is propagated to the caller,
- partial parsing endpoints are available internally so higher-level expansion logic can continue processing the source fragment correctly.

### 2. Variable-Expansion Recovery
When a variable expression cannot be fully expanded under the active rules, the module includes a recovery path.

The Rust version must implement equivalent functional behavior:
- detect expansion cases that must fall back instead of producing a normal expansion result,
- preserve or reconstruct the source text according to the recovery path,
- continue list construction consistently,
- return an error or non-error status consistent with whether recovery succeeds.

This requirement is limited to the recovery behavior evidenced by `expvar_recover`; it must not introduce additional recovery features not shown by the source module.

### 3. Positional/Parameter Vector Expansion
The module must support expansion of a parameter vector into the output node sequence.

This behavior is evidenced by `expand_paramv` and must include:
- reading parameter-vector content from the active `wordsplit` state,
- converting parameter-vector entries into one or more output word nodes,
- honoring flags and quote-sensitive mode when forming the result,
- appending expanded entries at the current output tail position.

The Rust version must preserve the distinction between:
- expansion that yields multiple words,
- and expansion that yields text merged into a current word context,
when that distinction is implied by the original flags and calling path.

### 4. Node-Oriented Expansion Pass
The module performs expansion by iterating over word nodes and applying an expansion function to nodes whose text begins with, or contains, expandable syntax.

The Rust rewrite must support:
- traversing the current linked sequence of `wordsplit_node` items,
- selecting nodes for a given expansion pass,
- replacing a single source node with transformed text and/or a sequence of nodes,
- preserving node order for unaffected content,
- updating list links so the transformed list remains valid.

This behavior is evidenced by `node_expand` and the higher-level pass functions.

### 5. Null Result Elimination
Expansion may produce empty or null-equivalent nodes. The module includes a cleanup step that removes such nodes where required by the wordsplit rules.

The Rust version must:
- identify nodes that became empty as a result of expansion,
- remove them from the active node list when this cleanup pass is invoked,
- preserve remaining node order and list integrity.

This behavior is evidenced by `wsnode_nullelim`.

### 6. Variable Expansion Pass over the Word List
The module exposes a pass-level operation that applies variable expansion across the current `wordsplit` node list.

The Rust version must provide equivalent module behavior:
- invoke node-oriented expansion using variable expansion rules,
- update the active `wordsplit` list in place,
- perform any required post-processing associated with this pass,
- return pass status to the caller.

This behavior is evidenced by `wordsplit_varexp`.

### 7. Command Substitution Expansion
The module must expand command-substitution expressions embedded in node text.

The Rust version must preserve the functional behavior evidenced by `expcmd`:
- identify command-substitution syntax within a text fragment,
- obtain substituted command output through the `wordsplit` execution/evaluation facilities available in module state,
- replace the command-substitution source text with resulting output,
- append generated content to the output node sequence,
- track the parse endpoint for continued processing of remaining source text,
- honor control flags affecting expansion behavior.

The specification does not require inventing new execution interfaces; it requires only equivalent command-substitution behavior within the existing `wordsplit` context.

### 8. Command-Expansion Pass over the Word List
The module must provide a pass-level operation that applies command substitution across the current node list.

The Rust version must:
- traverse eligible nodes,
- apply command-substitution expansion,
- splice results back into the active list,
- return completion status.

This behavior is evidenced by `wordsplit_cmdexp`.

### 9. Whitespace Trimming
After expansion, the module performs a whitespace-trimming pass on resulting words.

The Rust version must:
- trim leading and trailing whitespace from node text where this pass applies,
- preserve non-whitespace internal content,
- update node contents in place or with equivalent replacement,
- return status reflecting completion or failure.

This behavior is evidenced by `wordsplit_trimws`.

## User Scenarios & Testing

### Scenario 1: Expand variables in existing word nodes
A caller has already built a `wordsplit` instance containing parsed word nodes, and some nodes contain variable syntax. Running the variable-expansion pass updates those nodes so variable references are replaced with resolved values.

Test expectations:
- nodes without variable syntax remain unchanged,
- nodes with variable syntax contain expanded results,
- list order is preserved,
- pass returns success when all expansions succeed.

### Scenario 2: Recover from a non-normal variable expansion case
A node contains text that enters the variable-expansion path but must use the module’s recovery behavior instead of a normal expansion result.

Test expectations:
- the pass does not corrupt the node list,
- the recovered text matches the original module behavior,
- the parser endpoint advances consistently enough for the pass to continue,
- the function returns the same success/failure class as the C behavior for that case.

### Scenario 3: Expand a parameter vector into multiple words
The active `wordsplit` state includes parameter-vector data, and expansion inserts those parameters into the result.

Test expectations:
- all parameter entries appear in the result in source order,
- multi-entry expansion can produce multiple output nodes,
- quote/flag mode changes result shape only where the original behavior requires it,
- the resulting node chain remains valid.

### Scenario 4: Apply variable expansion to a list containing mixed plain and expandable nodes
A `wordsplit` list contains plain text nodes, variable-containing nodes, and nodes that become empty after expansion.

Test expectations:
- only eligible nodes are transformed,
- plain nodes remain in place,
- empty nodes are removed when null-elimination is invoked as part of the pass,
- remaining nodes preserve their original relative order.

### Scenario 5: Expand command substitution inside a word
A node contains command-substitution syntax. Running the command-expansion pass replaces that syntax with command output.

Test expectations:
- non-command text surrounding the substitution is preserved,
- command output appears in the correct position,
- pass status reflects execution success or failure,
- the resulting list is structurally valid.

### Scenario 6: Run command expansion across multiple nodes
A `wordsplit` list contains several nodes, some with command substitution and some without.

Test expectations:
- only nodes containing eligible command syntax are transformed,
- all eligible nodes are processed in one pass,
- the final node order matches the original semantic order,
- any generated replacement nodes are linked correctly.

### Scenario 7: Trim whitespace after expansion
After variable or command expansion, some node values have leading or trailing whitespace. The trimming pass normalizes them.

Test expectations:
- leading whitespace is removed,
- trailing whitespace is removed,
- internal non-edge whitespace remains,
- nodes reduced to empty text are handled consistently with the module’s cleanup behavior.

## Requirements

### Functional Requirements

#### FR-1: Variable text expansion
The module shall expand variable expressions found in word-node text fragments and insert the resolved value into the output sequence.
- Traceability: `expvar`

#### FR-2: Variable-expansion recovery path
The module shall support the recovery path used when variable expansion cannot proceed normally, preserving valid output construction and returning the corresponding status.
- Traceability: `expvar_recover`

#### FR-3: Parameter-vector expansion
The module shall expand parameter-vector content from the active `wordsplit` state into the node output sequence.
- Traceability: `expand_paramv`

#### FR-4: Expansion-pass application to nodes
The module shall apply a supplied expansion routine across eligible word nodes, replacing source nodes with transformed output while preserving list integrity.
- Traceability: `node_expand`, `struct wordsplit_node`

#### FR-5: Null-node elimination
The module shall remove nodes that represent empty results after expansion when the cleanup pass is invoked.
- Traceability: `wsnode_nullelim`, `struct wordsplit_node`

#### FR-6: Variable-expansion pass
The module shall provide a pass that applies variable expansion across the active word-node list in the `wordsplit` state.
- Traceability: `wordsplit_varexp`, `struct wordsplit`

#### FR-7: Command-substitution expansion
The module shall expand command-substitution expressions embedded in node text fragments and insert substituted output into the output sequence.
- Traceability: `expcmd`

#### FR-8: Command-expansion pass
The module shall provide a pass that applies command-substitution expansion across the active word-node list in the `wordsplit` state.
- Traceability: `wordsplit_cmdexp`, `struct wordsplit`

#### FR-9: Whitespace trimming pass
The module shall trim leading and trailing whitespace from resulting word-node text when the trim pass is invoked.
- Traceability: `wordsplit_trimws`, `struct wordsplit_node`

#### FR-10: Status propagation
Each expansion and pass-level operation shall report success or failure to its caller in a way equivalent to the C module’s pass/function result behavior.
- Traceability: all listed functions

#### FR-11: Ordered result preservation
For all supported expansion passes, the module shall preserve the semantic order of unaffected text, expanded replacements, and remaining nodes in the final word list.
- Traceability: `expvar`, `expand_paramv`, `expcmd`, `node_expand`, `struct wordsplit_node`

### Key Entities

#### `wordsplit`
Primary module state for an in-progress wordsplitting operation.

Functional role evidenced by the source:
- owns or references the active sequence of word nodes,
- provides expansion context and control flags,
- carries parameter-vector and command-expansion context needed by this module,
- receives in-place updates as expansion passes transform the word list.

Relationships:
- contains or references one or more `wordsplit_node` instances,
- is passed to all expansion and pass-level functions as the controlling context.

Traceability:
- `struct wordsplit`
- `wordsplit_varexp`
- `wordsplit_cmdexp`
- `wordsplit_trimws`
- `expand_paramv`

#### `wordsplit_node`
Represents an individual word or word fragment within the mutable wordsplit result list.

Functional role evidenced by the source:
- stores node text subject to expansion,
- participates in an ordered linked sequence,
- may be replaced, removed, or supplemented during expansion passes,
- is the unit of null elimination and whitespace trimming.

Relationships:
- belongs to a `wordsplit` instance’s active list,
- is traversed and modified by node-oriented expansion routines.

Traceability:
- `struct wordsplit_node`
- `node_expand`
- `wsnode_nullelim`
- `wordsplit_trimws`

## Success Criteria

### Functional Equivalence Criteria
1. Variable expansion on a prepared `wordsplit` node list produces the same word content and node ordering as the C module for representative inputs that include plain text, simple variable references, and mixed text-plus-variable fragments.
   - Traceability: `expvar`, `wordsplit_varexp`

2. Variable-expansion recovery cases produce outputs and status classes equivalent to the C module for the same inputs.
   - Traceability: `expvar_recover`

3. Parameter-vector expansion reproduces the same number and order of resulting words as the C module for representative single-entry and multi-entry parameter data.
   - Traceability: `expand_paramv`

4. Command substitution produces replacement text and node ordering equivalent to the C module for representative command-substitution inputs.
   - Traceability: `expcmd`, `wordsplit_cmdexp`

5. Expansion passes do not leave the word-node list structurally inconsistent after node replacement, insertion, or deletion.
   - Traceability: `node_expand`, `wsnode_nullelim`, `struct wordsplit_node`

6. Null-result cleanup removes the same empty-result nodes as the C module for representative expansion outputs.
   - Traceability: `wsnode_nullelim`

7. Whitespace trimming removes leading and trailing whitespace, while preserving internal content, in the same cases as the C module.
   - Traceability: `wordsplit_trimws`

8. All pass-level functions return success/failure outcomes equivalent to the C module for matched input cases.
   - Traceability: `wordsplit_varexp`, `wordsplit_cmdexp`, `wordsplit_trimws`

### Test Completion Criteria
1. Automated tests cover:
   - variable expansion,
   - variable-expansion recovery,
   - parameter-vector expansion,
   - command substitution,
   - null-node elimination,
   - whitespace trimming.

2. For each covered scenario, the Rust result is compared against the C module’s observable outputs:
   - final word strings,
   - final node count/order,
   - operation status.

3. No scenario covered by this specification requires behavior not evidenced by the listed source functions and types.