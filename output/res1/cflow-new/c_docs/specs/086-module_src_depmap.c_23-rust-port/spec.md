# spec.md

## Title

Functional Specification: `module_src_depmap.c_23` Rust Port

## Document Control

- Project: `cflow-new`
- Module: `module_src_depmap.c_23`
- Category: `module_cluster`
- Source file: `src/depmap.c`
- Target branch: `086-module_src_depmap.c_23-rust-port`
- Generation date: 2026-06-11

## Overview

This module provides a compact dependency-map abstraction over a square relation of `count × count` entries. It supports:

- allocation of a dependency map sized by item count,
- marking a dependency relation from one item to another,
- querying whether a dependency relation is present,
- expanding the relation to its transitive closure.

The Rust rewrite must preserve the observable behavior of this module: it must represent a finite directed dependency relation, allow individual relations to be set and queried, and provide a closure operation that makes all transitively reachable dependencies directly queryable afterward.

## Scope

In scope for this module:

- creation of a dependency map for a fixed number of items,
- storage of directed dependency bits,
- point updates for individual dependency pairs,
- point queries for individual dependency pairs,
- computation of transitive closure across the entire map.

Out of scope:

- dynamic resizing after allocation,
- removing a previously set dependency,
- APIs beyond the behavior evidenced by the source module,
- persistence, serialization, concurrency guarantees, or foreign-function bindings.

## Feature Specification

### Feature: Fixed-size dependency map

The module shall provide a dependency map whose dimensions are determined at creation time by a single item count. The map models a directed relation between rows and columns over the same index domain.

The Rust version must support constructing a map for `count` items and using indices in the range implied by that count for subsequent set, query, and closure operations.

Traceability:

- `depmap_alloc`
- `struct cflow_depmap`

### Feature: Directed dependency marking

The module shall support recording that one indexed item depends on another indexed item by setting the relation at a specific row and column.

Setting a dependency must make the corresponding query return true thereafter, unless altered by whole-map closure processing that only adds further reachable relations.

Traceability:

- `depmap_set`
- `depmap_rowptr`
- `struct cflow_depmap`

### Feature: Directed dependency query

The module shall support querying whether a specific directed dependency relation is present for a row and column pair.

The query result must reflect direct relations previously set and any additional relations introduced by transitive closure.

Traceability:

- `depmap_isset`
- `depmap_rowptr`

### Feature: Whole-map transitive closure

The module shall support transforming the dependency map so that if item `A` depends on `B`, and `B` depends on `C`, then `A` is reported as depending on `C` after closure is applied.

Closure processing applies across the full square relation represented by the map.

Traceability:

- `depmap_tc`
- `transitive_closure`

## User Scenarios & Testing

### Scenario 1: Create an empty dependency map

A caller needs a dependency map for `N` source items before any dependency edges are known.

Expected support in Rust:

- constructing a map for a specified count,
- all dependency queries initially report absence unless a relation has been set or added by closure.

Suggested tests:

- allocate a map with a small positive count,
- verify that several row/column pairs query as unset before any updates.

Traceability:

- `depmap_alloc`
- `depmap_isset`

### Scenario 2: Record direct dependencies

A caller discovers direct dependencies while analyzing source items and records them one by one.

Expected support in Rust:

- setting a dependency from row `i` to column `j`,
- querying the same pair reports presence.

Suggested tests:

- set one relation and verify it is present,
- set multiple unrelated relations and verify each is independently present.

Traceability:

- `depmap_set`
- `depmap_isset`

### Scenario 3: Distinguish directionality

A caller must treat dependencies as directed, not symmetric.

Expected support in Rust:

- setting `A -> B` does not by itself imply `B -> A`.

Suggested tests:

- set `row=1, col=2`,
- verify `(1,2)` is set and `(2,1)` remains unset unless explicitly set or implied through closure from other edges.

Traceability:

- `depmap_set`
- `depmap_isset`

### Scenario 4: Compute transitive dependencies

A caller has direct dependencies only, but later needs full reachability.

Expected support in Rust:

- after closure, every transitively reachable dependency is queryable directly.

Suggested tests:

- set `A -> B` and `B -> C`,
- verify `A -> C` is absent before closure,
- apply closure,
- verify `A -> C` is present after closure.

Traceability:

- `depmap_set`
- `depmap_isset`
- `depmap_tc`
- `transitive_closure`

### Scenario 5: Closure preserves existing direct relations

A caller must not lose any already recorded direct dependencies when closure is computed.

Expected support in Rust:

- closure adds reachable dependencies and does not remove existing ones.

Suggested tests:

- set several direct edges,
- snapshot expected direct edges,
- apply closure,
- verify all original direct edges are still present.

Traceability:

- `depmap_set`
- `depmap_isset`
- `depmap_tc`
- `transitive_closure`

### Scenario 6: Closure over longer chains

A caller needs reachability across chains longer than two steps.

Expected support in Rust:

- closure propagates dependencies through arbitrary intermediate nodes within the fixed map size.

Suggested tests:

- set `A -> B`, `B -> C`, `C -> D`,
- apply closure,
- verify `A -> C`, `A -> D`, and `B -> D` are present afterward.

Traceability:

- `depmap_set`
- `depmap_tc`
- `depmap_isset`
- `transitive_closure`

## Requirements

### Functional Requirements

#### FR-1: Allocate a dependency map for a fixed item count

The Rust module shall provide creation of a dependency map whose row and column domains both equal the provided item count.

Traceability:

- `depmap_alloc`
- `struct cflow_depmap`

#### FR-2: Represent an initially empty relation

Immediately after allocation, the dependency map shall contain no recorded dependencies.

Traceability:

- `depmap_alloc`
- `depmap_isset`
- `struct cflow_depmap`

#### FR-3: Set a directed dependency relation

The Rust module shall support marking the dependency relation for a specified `(row, col)` pair as present.

Traceability:

- `depmap_set`
- `depmap_rowptr`

#### FR-4: Query a directed dependency relation

The Rust module shall support testing whether the dependency relation for a specified `(row, col)` pair is present.

Traceability:

- `depmap_isset`
- `depmap_rowptr`

#### FR-5: Preserve directionality of relations

The dependency map shall treat `(row, col)` and `(col, row)` as distinct relations. Setting one pair shall not imply the other unless that other relation is separately set or produced by transitive closure from other existing relations.

Traceability:

- `depmap_set`
- `depmap_isset`
- `transitive_closure`

#### FR-6: Compute transitive closure over the full map

The Rust module shall provide an operation that updates the dependency map so that any dependency reachable by one or more intermediate dependencies is present after the operation.

Traceability:

- `depmap_tc`
- `transitive_closure`

#### FR-7: Closure shall be additive with respect to existing relations

Applying transitive closure shall not remove any dependency relations that were already present before closure.

Traceability:

- `depmap_tc`
- `transitive_closure`
- `depmap_isset`

#### FR-8: Closure results shall be queryable through the standard query operation

Any dependency relation introduced by transitive closure shall be observable through the same query mechanism used for direct relations.

Traceability:

- `depmap_tc`
- `depmap_isset`
- `transitive_closure`

### Key Entities

#### Entity: Dependency map

A dependency map is the module’s core stateful object. It represents a square directed relation over a fixed count of indexed items.

Properties evidenced by the source module:

- it stores the total item count,
- it stores relation data for all row/column combinations in that count-defined domain.

Relationships:

- allocation creates one dependency map,
- set and query operations address entries within that map,
- transitive closure transforms the relation stored in that map.

Traceability:

- `struct cflow_depmap`
- `depmap_alloc`
- `depmap_set`
- `depmap_isset`
- `depmap_tc`

#### Entity: Dependency relation entry

A dependency relation entry is the logical boolean state associated with one ordered pair `(row, col)` in a dependency map.

Relationships:

- `depmap_set` marks an entry as present,
- `depmap_isset` reads whether an entry is present,
- `depmap_tc` may cause additional entries to become present based on reachability through other present entries.

Traceability:

- `depmap_set`
- `depmap_isset`
- `depmap_tc`
- `transitive_closure`

## Success Criteria

### SC-1: Correct empty-state behavior

For a newly created dependency map of count `N`, queries for sampled valid pairs return false before any set or closure operation.

Traceability:

- `depmap_alloc`
- `depmap_isset`

### SC-2: Correct direct-set behavior

After setting a valid pair `(r, c)`, querying `(r, c)` returns true.

Traceability:

- `depmap_set`
- `depmap_isset`

### SC-3: Correct directional behavior

After setting `(r, c)` only, querying `(c, r)` returns false unless independently established by another set operation or by closure from other edges.

Traceability:

- `depmap_set`
- `depmap_isset`
- `transitive_closure`

### SC-4: Correct two-step closure behavior

Given direct relations `A -> B` and `B -> C`, querying `A -> C` returns false before closure and true after closure.

Traceability:

- `depmap_set`
- `depmap_isset`
- `depmap_tc`

### SC-5: Correct multi-step closure behavior

Given a chain of length greater than two, closure causes all reachable downstream dependencies to become queryable from each upstream node.

Traceability:

- `depmap_set`
- `depmap_isset`
- `depmap_tc`
- `transitive_closure`

### SC-6: No loss of preexisting relations after closure

For any set of direct dependencies established before closure, all of those dependencies remain present after closure is applied.

Traceability:

- `depmap_set`
- `depmap_tc`
- `depmap_isset`
- `transitive_closure`

### SC-7: Full module behavior is covered by Rust tests

The Rust port shall include tests covering:

- empty map queries,
- direct set/query behavior,
- directionality,
- two-step transitive closure,
- longer-chain transitive closure,
- preservation of original direct relations after closure.

Traceability:

- `depmap_alloc`
- `depmap_set`
- `depmap_isset`
- `depmap_tc`
- `transitive_closure`

## Acceptance Notes

The Rust rewrite is acceptable when it preserves the module’s evidenced functional boundary: a fixed-size directed dependency map with direct set/query operations and a whole-map transitive closure operation that makes reachability observable through subsequent queries.