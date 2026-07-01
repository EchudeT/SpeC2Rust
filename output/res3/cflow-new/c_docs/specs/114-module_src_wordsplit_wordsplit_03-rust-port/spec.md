# spec.md

## Title

Functional Specification for `module_src_wordsplit_wordsplit_03` Rust Port

## Summary

This module covers the later-stage word expansion behaviors implemented in `src/wordsplit/wordsplit.c` for the `wordsplit` engine. The evidenced scope is limited to:

- variable expansion across existing split nodes,
- parameter-vector expansion support,
- command substitution expansion,
- removal of null results produced by expansion,
- trimming of whitespace after expansion.

The Rust rewrite on branch `114-module_src_wordsplit_wordsplit_03-rust-port` must preserve the observable behavior of these expansion stages as they operate on the module’s internal `wordsplit` state and `wordsplit_node` list.

## Scope

### In Scope

Based on the analyzed functions, the Rust version must implement:

- expansion of variable expressions from node text,
- recovery behavior for variable expansion parsing failures or non-expanded cases where the C code routes through `expvar_recover`,
- expansion of parameter vectors where supported by the current `wordsplit` state,
- command substitution expansion from node text,
- generic node-by-node expansion traversal used by variable and command expansion stages,
- elimination of null nodes created by expansion,
- post-expansion whitespace trimming.

### Out of Scope

This specification does not require functionality not evidenced in the analyzed module slice, including:

- initial lexical splitting,
- quote parsing outside what is necessary for the listed expansion functions,
- pathname/glob expansion,
- new public APIs beyond the module-equivalent Rust surface,
- guarantees about concurrency, persistence, FFI, or performance.

## Source Basis

This specification is derived from the following evidenced functions in `src/wordsplit/wordsplit.c`:

- `expvar_recover`
- `expand_paramv`
- `expvar`
- `node_expand`
- `wsnode_nullelim`
- `wordsplit_varexp`
- `expcmd`
- `wordsplit_cmdexp`
- `wordsplit_trimws`

It also relies on the module’s internal state carriers:

- `struct wordsplit`
- `struct wordsplit_node`

## Feature Specification

### 1. Variable Expansion Stage

The module shall process existing word nodes and expand variable expressions found in node text.

Behavior evidenced by `expvar`, `expand_paramv`, `expvar_recover`, `node_expand`, and `wordsplit_varexp` indicates that the Rust port must support:

- scanning node text for variable-expansion syntax,
- producing replacement text or replacement nodes as required by the current expansion result,
- updating the node list so expanded content participates in later stages,
- preserving parsing position information sufficiently to continue processing after each expansion attempt,
- handling flags and quote-sensitive context that affect expansion behavior,
- invoking recovery behavior when a candidate variable expansion cannot be completed in the normal path.

The Rust version must preserve the distinction between:

- successful variable expansion,
- recoverable non-expansion or fallback handling,
- error outcomes that stop the stage.

### 2. Parameter-Vector Expansion

The module shall support expansion from a parameter-vector source where the C implementation uses `expand_paramv`.

This includes:

- converting the parameter-vector value into node output suitable for the surrounding word-splitting pipeline,
- appending expansion results into the active node chain,
- respecting stage flags and quote-related mode passed into expansion.

The Rust version must preserve the observable result shape: parameter-vector expansion may yield multiple output words rather than a single flat string when that is what the underlying `wordsplit` state dictates.

### 3. Generic Node Expansion Traversal

The module shall provide node-oriented expansion processing equivalent to the behavior driven by `node_expand`.

This stage behavior must include:

- iterating over applicable nodes in the `wordsplit` node list,
- selecting nodes whose content begins with or contains text relevant to the supplied expansion stage,
- replacing, extending, or preserving nodes based on the called expansion function’s result,
- maintaining a valid node sequence for subsequent passes.

This traversal behavior is required because both variable and command expansion stages are applied over previously built node content rather than over the raw original input.

### 4. Null-Result Elimination

The module shall remove empty or null-result nodes produced by expansion where the C code uses `wsnode_nullelim`.

The Rust rewrite must ensure that after expansion stages:

- nodes representing eliminated expansion results do not remain as active word outputs when the C behavior would remove them,
- list integrity is preserved after removals,
- downstream stages receive the filtered node list.

### 5. Command Substitution Expansion Stage

The module shall process command substitution expressions from node text, as evidenced by `expcmd` and `wordsplit_cmdexp`.

The Rust version must support:

- detecting command-substitution syntax in node text,
- obtaining substitution output through the module-equivalent command-expansion mechanism,
- inserting the resulting text or words into the node chain,
- honoring the same stage-driving flags and quote-sensitive handling used by the C module,
- reporting errors when command expansion fails.

The specification only requires command expansion behavior evidenced by the module’s role in transforming node text; it does not prescribe any new execution interface beyond what the Rust port needs internally to match behavior.

### 6. Post-Expansion Whitespace Trimming

The module shall trim whitespace after expansion, as evidenced by `wordsplit_trimws`.

The Rust version must implement a trimming stage that:

- visits the expanded node results,
- removes leading and/or trailing whitespace where the C behavior does so,
- preserves non-whitespace content,
- returns an error status if the stage can fail in the same situations as the C implementation.

This trimming must occur as a distinct post-expansion behavior so that final word outputs match the C module’s processed state.

## User Scenarios & Testing

### Scenario 1: Expand variables in existing word nodes

A caller has already built a `wordsplit` state containing one or more nodes with variable expressions. The variable expansion stage runs and replaces those expressions with their resolved values.

The Rust version must support tests showing that:

- variable expressions in node text are expanded,
- unaffected text remains unchanged,
- node ordering remains valid after expansion.

### Scenario 2: Preserve behavior when variable expansion falls back or recovers

A caller provides node text containing a sequence that enters the variable-expansion path but requires the recovery logic used by `expvar_recover`.

The Rust version must support tests showing that:

- the recovery path is reachable for the same class of inputs,
- the resulting node content and parse continuation match the C module’s observable behavior,
- the stage does not silently convert recoverable cases into hard failures.

### Scenario 3: Expand parameter-vector content into multiple words

A caller’s `wordsplit` state contains a variable reference that resolves through the parameter-vector expansion path.

The Rust version must support tests showing that:

- expansion can append multiple word results,
- those results appear as separate nodes or equivalent separate final words,
- quote/flag mode changes the output only where the C behavior does.

### Scenario 4: Run command substitution on node text

A caller provides node content containing command substitution syntax. The command expansion stage runs and substitutes command output.

The Rust version must support tests showing that:

- command substitution is detected and expanded,
- successful substitution updates the node list,
- command-expansion failure is reported as a stage error.

### Scenario 5: Remove null expansion results

A caller provides input where an expansion yields an empty or null-result node that the C module removes.

The Rust version must support tests showing that:

- null-result nodes are eliminated after the designated stage,
- no spurious empty output words remain when the C behavior removes them.

### Scenario 6: Trim whitespace after expansion

A caller provides expansions whose results include surrounding whitespace requiring trimming by the final stage.

The Rust version must support tests showing that:

- leading/trailing whitespace is trimmed where the C module trims it,
- internal non-trimmed content is preserved,
- trimming operates on post-expansion results, not only on original input text.

### Scenario 7: Combined expansion pipeline

A caller runs the full late-stage pipeline over a `wordsplit` state containing a mix of literal text, variable expressions, command substitutions, and values that may collapse to null.

The Rust version must support tests showing that:

- variable expansion and command substitution can both operate over the node list,
- null elimination and trimming are applied afterward,
- the final output sequence matches the C module for the same inputs and flags.

## Requirements

### Functional Requirements

#### FR-1: Variable expansion over node content
The module shall provide a variable-expansion stage over the current `wordsplit` node list, transforming node text according to variable-expression syntax and current expansion flags.

Traceability: `expvar`, `wordsplit_varexp`, `node_expand`.

#### FR-2: Recoverable variable-expansion handling
The module shall preserve the C module’s recoverable handling for variable-expansion cases that do not complete through the main expansion path.

Traceability: `expvar_recover`, `expvar`.

#### FR-3: Parameter-vector expansion
The module shall expand parameter-vector values into node output within variable expansion, including cases that produce multiple resulting words.

Traceability: `expand_paramv`, `expvar`.

#### FR-4: Quote- and flag-sensitive expansion behavior
The module shall preserve behavior differences driven by expansion flags and quote context in variable and command expansion stages.

Traceability: `expand_paramv`, `expvar`, `expcmd`.

#### FR-5: Node-based expansion traversal
The module shall apply expansion functions across the current node list and update that list in place or equivalently for later stages.

Traceability: `node_expand`, `wordsplit_varexp`, `wordsplit_cmdexp`.

#### FR-6: Command substitution expansion
The module shall provide command substitution over node content and integrate substitution results into the active node sequence.

Traceability: `expcmd`, `wordsplit_cmdexp`, `node_expand`.

#### FR-7: Null-result node elimination
The module shall remove expansion-produced null or empty-result nodes in the cases handled by the C module.

Traceability: `wsnode_nullelim`.

#### FR-8: Post-expansion whitespace trimming
The module shall trim whitespace from expanded node results according to the C stage behavior.

Traceability: `wordsplit_trimws`.

#### FR-9: Error propagation by stage
The module shall return success or failure status for variable expansion, command substitution, and trimming stages so callers can observe stage failure.

Traceability: `expvar_recover`, `expand_paramv`, `expvar`, `node_expand`, `wordsplit_varexp`, `expcmd`, `wordsplit_cmdexp`, `wordsplit_trimws`.

### Key Entities

#### `wordsplit`
The primary mutable processing state for the word-splitting pipeline.

Required role in the Rust port:

- owns or references the current list of word nodes,
- carries option/flag state used by expansion stages,
- provides the contextual data needed for variable, parameter-vector, and command expansion,
- receives the updated node list after each stage.

Traceability: all listed functions accept `struct wordsplit *`.

#### `wordsplit_node`
The per-word or per-fragment node operated on by expansion stages.

Required role in the Rust port:

- stores text subject to expansion,
- participates in an ordered chain or equivalent sequence,
- can be replaced, appended to, removed, or preserved by expansion stages,
- is the unit traversed by generic node expansion and null elimination.

Traceability: `expvar_recover`, `expand_paramv`, `expvar`, `node_expand`, `expcmd`, `wsnode_nullelim`.

#### Relationship between `wordsplit` and `wordsplit_node`
The `wordsplit` state manages an ordered collection of `wordsplit_node` items. Expansion stages traverse this collection, transform individual nodes, create replacement output, eliminate null results, and leave the collection in a valid form for subsequent stages.

Traceability: `node_expand`, `wordsplit_varexp`, `wordsplit_cmdexp`, `wsnode_nullelim`, `wordsplit_trimws`.

## Success Criteria

### Behavioral Equivalence

1. For representative inputs containing variable expressions, the Rust module produces the same final word outputs and stage success/failure results as the C module.
2. For representative recoverable variable-expansion cases, the Rust module matches the C module’s fallback behavior rather than converting them into different outputs or hard failures.
3. For parameter-vector cases, the Rust module preserves whether expansion yields one word or multiple words.
4. For representative command substitution inputs, the Rust module matches the C module’s resulting node/output transformation and failure reporting.
5. For cases where expansion yields null results, the Rust module removes those results in the same situations as the C module.
6. For post-expansion whitespace cases, the Rust module trims output whitespace to the same final results as the C module.

### Structural Integrity

7. After each expansion stage, the internal node sequence remains valid and usable by the next stage.
8. Combined execution of variable expansion, command substitution, null elimination, and trimming completes without leaving orphaned or duplicate final outputs relative to the C behavior.

### Test Coverage Expectations

9. Automated tests cover each scenario listed in this specification.
10. Each functional requirement from FR-1 through FR-9 is exercised by at least one test that compares Rust-port behavior against expected C-module behavior derived from the analyzed functionality.