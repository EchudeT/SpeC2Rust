# spec.md

## Title

Rust Functional Specification for `module_src_depmap.c_23`

## Overview

This module provides an in-memory dependency map over a fixed number of items. It supports:

- allocating a square dependency map for a known item count,
- marking a directed dependency from one item to another,
- checking whether a dependency is present,
- computing the transitive closure of the full map.

The Rust rewrite must preserve the module’s observable behavior as evidenced by `src/depmap.c`: representing directed reachability among indexed items and updating the map so that indirect dependencies become directly queryable after closure computation.

## Scope

In scope:

- dependency map creation for a fixed item count,
- mutation by setting directed dependencies,
- read access to individual dependency relations,
- whole-map transitive closure computation.

Out of scope:

- dynamic resizing,
- deletion or clearing of individual dependencies,
- persistence or serialization,
- concurrency guarantees,
- APIs beyond the behavior evidenced by the source module.

## Source Basis

This specification is derived from the following module elements in `src/depmap.c`:

- `struct cflow_depmap`
- `depmap_alloc`
- `depmap_set`
- `depmap_isset`
- `depmap_tc`
- internal closure behavior evidenced by `transitive_closure`

## Feature Specification

### Feature: Fixed-size dependency map

The module shall represent dependencies among `count` indexed elements in a square relation space. Each relation is directed: setting `(row, col)` records that the item at `row` depends on or reaches the item at `col`.

The Rust version must preserve the fixed-size nature of the map established at allocation time.

### Feature: Direct dependency recording

The module shall allow callers to mark a dependency between two valid indices. After a dependency is marked, querying that same pair shall report that the relation is present.

### Feature: Dependency query

The module shall allow callers to test whether a directed relation is currently present for a given `(row, col)` pair.

This query must reflect:

- directly set relations, and
- indirect relations after transitive closure has been applied.

### Feature: Transitive closure computation

The module shall support converting the current dependency relation into its transitive closure across the full map.

After closure computation, if there exists a path from `A` to `B` through one or more intermediate elements already represented in the map, then querying `(A, B)` shall report the relation as present.

The closure operation is whole-map in scope; it is not evidenced as operating on subsets.

## User Scenarios & Testing

### Scenario 1: Create an empty dependency map

A caller creates a dependency map for `N` items and performs queries before setting any relations.

Expected behavior:

- the map exists for exactly `N` indexed items,
- no dependency is reported present unless it has been introduced by later operations or by closure from existing paths.

Test coverage:

- allocate with a small count such as `3`,
- verify that unrelated pairs are not reported as set in the initial state.

### Scenario 2: Record and query a direct dependency

A caller records that item `0` depends on item `2`, then checks that relation.

Expected behavior:

- querying `(0, 2)` reports present,
- querying unrelated pairs remains unaffected unless separately set.

Test coverage:

- allocate a map,
- set one pair,
- verify that the exact pair is present,
- verify that at least one distinct pair remains absent.

### Scenario 3: Compute indirect dependencies

A caller records a chain such as `0 -> 1` and `1 -> 2`, then computes transitive closure.

Expected behavior:

- before closure, the module need only report the directly set pairs,
- after closure, `(0, 2)` reports present because it is implied by the chain.

Test coverage:

- set a two-step chain,
- verify direct edges before closure,
- run closure,
- verify the implied edge after closure.

### Scenario 4: Preserve existing direct dependencies through closure

A caller sets several direct dependencies and then computes closure.

Expected behavior:

- all directly set dependencies remain present after closure,
- closure adds implied reachability but does not remove existing relations.

Test coverage:

- set multiple direct pairs,
- record query results for those pairs,
- run closure,
- verify those same direct pairs are still present.

### Scenario 5: Closure over longer paths

A caller records a longer chain such as `0 -> 1`, `1 -> 2`, `2 -> 3`, then computes closure.

Expected behavior:

- the module reports all reachability implied by the chain after closure, including `(0, 2)`, `(1, 3)`, and `(0, 3)`.

Test coverage:

- build a path of length greater than two,
- run closure,
- verify all implied reachable pairs.

## Requirements

### Functional Requirements

#### FR-1: Allocate a fixed-size dependency map

The Rust module shall provide a way to create a dependency map for a specified number of items, corresponding to the behavior of `depmap_alloc`.

Acceptance notes:

- the created map represents relations among exactly `count` indexed items,
- subsequent operations are defined against that fixed index space.

Traceability:

- `src/depmap.c`: `depmap_alloc`
- `src/depmap.c`: `struct cflow_depmap`

#### FR-2: Store directed dependency relations

The Rust module shall provide a way to mark a directed relation from `row` to `col`, corresponding to `depmap_set`.

Acceptance notes:

- setting a relation makes that exact relation observable through the query operation,
- relations are directed; setting `(row, col)` is not evidenced to imply `(col, row)`.

Traceability:

- `src/depmap.c`: `depmap_set`

#### FR-3: Query directed dependency relations

The Rust module shall provide a way to test whether a relation from `row` to `col` is present, corresponding to `depmap_isset`.

Acceptance notes:

- the result distinguishes present from absent relations,
- the result reflects the current map state at the time of the query.

Traceability:

- `src/depmap.c`: `depmap_isset`

#### FR-4: Compute whole-map transitive closure

The Rust module shall provide a way to transform the current relation into its transitive closure, corresponding to `depmap_tc` and the internal closure logic.

Acceptance notes:

- if `A -> B` and `B -> C` are present before closure, then `A -> C` is present after closure,
- this extends to paths of arbitrary length within the allocated map,
- direct relations already present remain present after closure.

Traceability:

- `src/depmap.c`: `depmap_tc`
- `src/depmap.c`: `transitive_closure`

#### FR-5: Operate over index-addressed rows and columns

The Rust module shall preserve the index-based access model evidenced by row and column parameters across allocation, set, query, and closure operations.

Acceptance notes:

- all relations are addressed by zero-based positional indices within the allocated map extent,
- behavior is defined in terms of pairwise item positions rather than names or external identifiers.

Traceability:

- `src/depmap.c`: `depmap_set`
- `src/depmap.c`: `depmap_isset`
- `src/depmap.c`: `depmap_alloc`
- `src/depmap.c`: `struct cflow_depmap`

### Key Entities

#### Dependency Map

A dependency map is the module’s central entity, corresponding to `struct cflow_depmap`. It represents a directed relation over a fixed population of indexed items.

Relationships:

- it is created with a total item count,
- it stores relations from row indices to column indices,
- it is mutated by setting relations,
- it is observed by querying relations,
- it can be globally expanded to implied reachability by transitive closure.

#### Directed Relation Entry

A directed relation entry is the logical association between one row index and one column index.

Relationships:

- entries exist within a dependency map,
- entries may be introduced directly by set operations,
- entries may also become present indirectly after closure if implied by paths through other entries.

## Success Criteria

### SC-1: Direct set/query correctness

For a newly allocated map, after setting a relation `(r, c)`, querying `(r, c)` returns present.

Traceability:

- `depmap_alloc`
- `depmap_set`
- `depmap_isset`

### SC-2: No unintended symmetry

When a relation `(r, c)` is set, the reverse relation `(c, r)` is not required to be present unless separately set or implied by other relations.

Traceability:

- `depmap_set`
- `depmap_isset`

### SC-3: Transitive implication correctness

Given a map containing `A -> B` and `B -> C`, after closure, querying `(A, C)` returns present.

Traceability:

- `depmap_set`
- `depmap_tc`
- `depmap_isset`
- `transitive_closure`

### SC-4: Multi-step closure correctness

Given a path of length three or more, closure makes all implied reachable endpoints queryable as present.

Traceability:

- `depmap_tc`
- `depmap_isset`
- `transitive_closure`

### SC-5: Closure preserves existing relations

Any relation present before closure remains present after closure.

Traceability:

- `depmap_set`
- `depmap_tc`
- `depmap_isset`
- `transitive_closure`

### SC-6: Empty-map initial behavior

For a newly allocated map with no set operations performed, queries for at least sampled distinct pairs return absent.

Traceability:

- `depmap_alloc`
- `depmap_isset`