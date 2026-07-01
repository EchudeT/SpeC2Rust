# spec.md

## Title
Rust Functional Specification for `module_src_depmap.c_23`

## Summary
This module provides a compact dependency map for representing directed relationships among a fixed number of items and for computing their transitive closure. The Rust rewrite must preserve the observed behavior of the C module in `src/depmap.c`: allocation of a square dependency map, setting individual dependencies, querying whether a dependency is present, and expanding the map so that indirect dependencies become directly represented.

## Scope
In scope for the Rust version:

- Creating a dependency map sized for a fixed item count.
- Representing directed dependencies from one indexed item to another.
- Marking a dependency as present.
- Testing whether a dependency is present.
- Computing transitive closure over the full map.

Out of scope:

- Resizing an existing map.
- Removing dependencies.
- Any persistence, serialization, concurrency, or external interface behavior not evidenced by this module.

## Source Basis
This specification is derived from the following observed module surface in `src/depmap.c`:

- `struct cflow_depmap`
- `depmap_alloc`
- `depmap_set`
- `depmap_isset`
- `depmap_tc`

Internal helpers and internal closure computation inform behavior but do not expand required public capability beyond the items above.

## Feature Specification

### Feature: Fixed-size dependency map
The module must provide a dependency map for a caller-specified number of items. The map models a square relation where each row and column corresponds to an item index in the same bounded index space.

The Rust version must preserve these functional properties:

- The map is created for a fixed count.
- The relation is directional: a dependency from `row` to `col` is distinct from a dependency from `col` to `row`.
- The map stores whether a dependency is present for any valid ordered pair of item indices.

### Feature: Direct dependency marking
The module must allow callers to mark a dependency from one item to another as present.

The Rust version must implement behavior equivalent to setting a relation bit for a given `(row, col)` pair so that later membership checks for that pair report present.

### Feature: Dependency membership query
The module must allow callers to ask whether a dependency from one item to another is currently present.

The Rust version must return a boolean-equivalent result for a given `(row, col)` pair based on the current state of the map.

### Feature: Transitive closure expansion
The module must support transforming the dependency map so that indirect dependencies become explicitly represented.

After closure is computed:

- If item `A` depends on `B`, and `B` depends on `C`, then `A` must be reported as depending on `C`.
- This must apply across chains of arbitrary length within the fixed map size.
- Existing direct dependencies must remain present.

The feature is defined by observable relation behavior, not by any required algorithm.

## User Scenarios & Testing

### Scenario 1: Create an empty dependency map
A caller creates a dependency map for `N` items and has not yet set any dependencies.

Expected support:

- Queries for pairs not explicitly set must report not present.
- The map must support subsequent set and closure operations within the same fixed size.

Suggested tests:

- Allocate a map with a small count such as 3.
- Query several valid pairs.
- Verify they are all absent before any set operation.

### Scenario 2: Record and query a direct dependency
A caller records that one item directly depends on another and then checks that relation.

Expected support:

- After setting `(A, B)`, querying `(A, B)` reports present.
- Setting one pair must not imply unrelated pairs are present.

Suggested tests:

- Allocate a map with count 4.
- Set `(1, 2)`.
- Verify `(1, 2)` is present.
- Verify `(2, 1)` and `(1, 3)` remain absent unless separately set.

### Scenario 3: Compute closure over a simple chain
A caller models a dependency chain and then requests closure so indirect dependencies are added.

Expected support:

- Given direct dependencies `(0, 1)` and `(1, 2)`, after closure `(0, 2)` must be present.
- Original direct dependencies remain present.

Suggested tests:

- Allocate a map with count 3.
- Set `(0, 1)` and `(1, 2)`.
- Run closure.
- Verify `(0, 1)`, `(1, 2)`, and `(0, 2)` are present.

### Scenario 4: Compute closure over longer paths
A caller uses multiple chained dependencies and expects all reachable targets to become directly visible after closure.

Expected support:

- For a path `A -> B -> C -> D`, closure must make `A -> C`, `A -> D`, and `B -> D` present.

Suggested tests:

- Allocate a map with count 4.
- Set `(0, 1)`, `(1, 2)`, `(2, 3)`.
- Run closure.
- Verify all reachable ordered pairs implied by the path are present.

### Scenario 5: Closure does not invent disconnected relations
A caller expects closure to add only reachability-implied dependencies.

Expected support:

- If there is no path from `X` to `Y`, closure must not report `(X, Y)` as present.

Suggested tests:

- Allocate a map with count 4.
- Set `(0, 1)` and `(2, 3)`.
- Run closure.
- Verify `(0, 3)` and `(2, 1)` remain absent.

## Requirements

### Functional Requirements

#### FR-1: Allocate a fixed-size dependency map
The Rust module shall create a dependency map for a caller-specified item count, with rows and columns sharing the same index range.

Traceability: `depmap_alloc`, `struct cflow_depmap`

#### FR-2: Represent directed dependency presence
The Rust module shall represent dependency presence per ordered pair of valid indices so that direction is preserved.

Traceability: `struct cflow_depmap`, `depmap_set`, `depmap_isset`

#### FR-3: Mark a dependency as present
The Rust module shall provide an operation equivalent to setting dependency presence for a specified `(row, col)` pair.

Traceability: `depmap_set`

#### FR-4: Query dependency presence
The Rust module shall provide an operation equivalent to testing whether a specified `(row, col)` pair is present in the map.

Traceability: `depmap_isset`

#### FR-5: Compute full transitive closure
The Rust module shall provide an operation that updates the map so that for any indices `a`, `b`, and `c`, if dependencies `a -> b` and `b -> c` are present before or during closure propagation, then `a -> c` is present after closure completes.

Traceability: `depmap_tc`, `transitive_closure`

#### FR-6: Preserve existing dependencies during closure
The Rust module shall not remove already present dependencies when computing transitive closure.

Traceability: `depmap_tc`, `transitive_closure`

#### FR-7: Limit closure effects to reachable pairs
The Rust module shall add only those dependencies implied by reachability in the directed relation and shall not mark pairs present when no path exists between them.

Traceability: `depmap_tc`, `transitive_closure`, `depmap_isset`

### Key Entities

#### Dependency Map
A dependency map is the module’s central entity. It stores:

- A fixed item count defining the square relation domain.
- Dependency presence across ordered pairs of indices within that domain.

Traceability: `struct cflow_depmap`

#### Dependency Relation
A dependency relation is a directed edge from one index to another within the map.

Properties:

- It is addressed by `(row, col)`.
- It may be directly set by the caller.
- It may also become present through transitive closure.

Traceability: `depmap_set`, `depmap_isset`, `depmap_tc`

## Success Criteria

### SC-1: Empty-map query correctness
Given a newly created dependency map of size `N`, queries for valid pairs that have not been set return absent.

Traceability: `depmap_alloc`, `depmap_isset`

### SC-2: Direct-set correctness
After setting a direct dependency `(r, c)`, querying `(r, c)` returns present.

Traceability: `depmap_set`, `depmap_isset`

### SC-3: Directionality correctness
Setting `(r, c)` does not by itself cause `(c, r)` to be reported present.

Traceability: `depmap_set`, `depmap_isset`

### SC-4: Simple transitive closure correctness
For a map containing `A -> B` and `B -> C`, after closure the query for `A -> C` returns present.

Traceability: `depmap_tc`, `depmap_set`, `depmap_isset`

### SC-5: Multi-step closure correctness
For a dependency chain of length greater than two, closure causes all reachable downstream pairs to be present.

Traceability: `depmap_tc`, `transitive_closure`

### SC-6: No false positive closure pairs
After closure, pairs with no directed path between them remain absent.

Traceability: `depmap_tc`, `transitive_closure`, `depmap_isset`

### SC-7: Existing relation preservation
Any dependency present before closure remains present after closure.

Traceability: `depmap_tc`, `transitive_closure`