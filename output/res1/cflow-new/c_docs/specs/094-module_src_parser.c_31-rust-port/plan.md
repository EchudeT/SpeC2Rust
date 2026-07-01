# Implementation Plan

## Summary

Port the parser-related logic currently concentrated in `src/parser.c` into Rust on branch `094-module_src_parser.c_31-rust-port`, preserving existing behavior and call ordering rather than redesigning the parser. The Rust implementation should keep the current procedural parsing flow for declaration parsing, typedef handling, K&R argument handling, symbol lookup, and reference/call recording.

The migration approach is:

- move the functionality from the C file into a single Rust module with closely corresponding functions;
- convert C global/stateful parsing context into explicit mutable Rust state passed through functions or held in one parser state struct;
- replace pointer-based string/token handling with borrowed string slices and owned `String` where persistence is required;
- model C structs and anonymous records as named Rust structs/enums local to the parser module;
- use `Result` for parse/lookup failures where the C code relied on sentinel values or side effects, while preserving observable module behavior;
- avoid adding new parser features or refactoring into extra subsystems beyond what is required to express the current logic safely.

## Technical Context

- **Language/Version:** Rust 1.78+
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - Maintain parsing throughput comparable to the C implementation for the same input scale.
  - Avoid unnecessary string cloning during token and identifier processing.
  - Preserve mostly linear processing over the token/input stream.
  - Keep symbol/reference storage allocations bounded to actual parsed entities.

## Module Mapping

### Source Mapping

- **C source:** `src/parser.c`
- **Rust target:** `src/parser.rs`

If the crate already exposes parser functionality through another root file, `src/parser.rs` should be included using normal Rust module declarations without introducing additional helper modules unless already required by the existing crate layout.

### Function Mapping

The Rust module should retain one-for-one function correspondence where practical to reduce migration risk:

| C Function | Rust Mapping | Notes |
|---|---|---|
| `skip_struct` | `fn skip_struct(...)` | Preserve token-skipping semantics for struct bodies/declarations. |
| `parse_typedef` | `fn parse_typedef(...) -> Result<..., ...>` | Return explicit parse outcome instead of implicit C status where needed. |
| `parse_dcl` | `fn parse_dcl(...) -> Result<..., ...>` | Keep declaration parsing entry behavior intact. |
| `dcl` | `fn dcl(...) -> Result<..., ...>` | Internal declaration parser; keep recursive/procedural structure. |
| `getident` | `fn getident(...) -> Option<...>` or `Result<..., ...>` | Use slice-based token extraction. |
| `dirdcl` | `fn dirdcl(...) -> Result<..., ...>` | Preserve direct declarator handling. |
| `parmdcl` | `fn parmdcl(...) -> Result<..., ...>` | Maintain parameter declaration parsing behavior. |
| `maybe_parm_list` | `fn maybe_parm_list(...) -> Result<bool, ...>` | Return explicit presence/parse outcome. |
| `func_body` | `fn func_body(...) -> Result<..., ...>` | Keep body-skipping/body-processing logic local to parser state. |
| `get_knr_args` | `fn get_knr_args(...) -> Result<..., ...>` | Preserve K&R argument collection behavior. |
| `declare` | `fn declare(...) -> Result<..., ...>` | Keep declaration emission/update logic. |
| `declare_type` | `fn declare_type(...) -> Result<..., ...>` | Preserve type-declaration side effects. |
| `get_symbol` | `fn get_symbol(...) -> Option<...>` | Map symbol lookup to borrowed/mutable table access. |
| `add_reference` | `fn add_reference(...)` | Record symbol references without changing semantics. |
| `call` | `fn call(...)` | Record function call usage, likely via parser/symbol state mutation. |

## Data Model

The source analysis only identifies anonymous C data structures. In Rust, these should be converted into named internal types based on their usage in `parser.c`, not expanded into broader abstractions.

### Data Structure Mapping Strategy

| C Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous parser state record | `struct ParserState` | Holds mutable parse position, current token, and any former file-local state. |
| anonymous declaration record | `struct Declaration` | Represents in-progress declarator/type assembly. |
| anonymous parameter record | `struct ParameterDecl` | Used for parameter list and K&R argument handling. |
| anonymous symbol record | `struct Symbol` | Stores identifier, classification, and declaration/use metadata. |
| anonymous type record | `struct TypeInfo` | Encodes declaration/type state formerly spread across C fields. |
| anonymous reference record | `struct Reference` | Captures source-level symbol reference information. |
| anonymous call record | `struct CallSite` or folded into `Reference` | Use only if the C logic distinguishes calls from general references. |
| anonymous token/input record | `struct Token` / `enum TokenKind` | Only if `parser.c` directly manipulates token structure data. |
| anonymous list node / linked storage | `Vec<T>` or `Option<Box<T>>` | Prefer `Vec<T>` unless pointer-linked behavior is semantically required. |
| anonymous flags/bitfield group | Rust struct fields or `u32` flags | Preserve exact conditions before introducing enums. |
| anonymous scope/context record | `struct ScopeState` | Only if needed by the existing parser logic in this file. |

### Ownership and Memory Management

- Replace raw pointers to transient text with:
  - `&str` for input slices tied to parser input lifetime;
  - `String` for identifiers or type names stored in symbol tables/reference lists.
- Replace manually managed linked records with `Vec<T>` where append/traversal semantics are sufficient.
- Use `Option<T>` instead of null pointers.
- Use mutable references to shared parser/symbol state instead of aliasing raw pointers.
- Keep lifetimes local and explicit; prefer owning copies only when data outlives the current token buffer.

### Error Handling

- Replace C error signaling via return codes, nulls, or implicit globals with:
  - `Result<T, ParseError>` for parse operations;
  - `Option<T>` for ordinary lookup absence;
  - small internal error enums scoped to `parser.rs`.
- Preserve original failure boundaries: the port should not introduce recovery logic or alternate parse paths not present in the C code.

## Implementation Phases

### Phase 1: Skeleton Port and State Extraction

- Create `src/parser.rs` as the Rust home for the migrated logic from `src/parser.c`.
- Identify file-local C state used across the listed functions and consolidate it into a `ParserState` struct.
- Define the minimum named Rust structs/enums needed to replace the anonymous C records referenced by these functions.
- Establish direct Rust signatures for all listed functions, keeping names and internal responsibilities closely aligned with the C source.
- Add placeholder/internal types for symbols, declarations, references, and tokens sufficient to compile incremental ports.

### Phase 2: Core Declaration Parsing Migration

- Port the declaration parsing chain in dependency order:
  - `getident`
  - `dcl`
  - `dirdcl`
  - `parmdcl`
  - `maybe_parm_list`
  - `parse_dcl`
  - `parse_typedef`
  - `skip_struct`
- Preserve the original parsing order and recursive structure instead of reinterpreting the grammar.
- Convert C string/buffer manipulation into safe Rust slice handling.
- Introduce focused parse error types only where required to express current control flow safely.

### Phase 3: Symbol and Usage Recording Migration

- Port symbol-table related behavior:
  - `get_symbol`
  - `declare`
  - `declare_type`
  - `add_reference`
  - `call`
- Translate pointer-based symbol access into mutable table lookups using standard library collections.
- Preserve insertion/update semantics for declarations, typedefs, references, and call records.
- Ensure any identifier storage that outlives the current parse step becomes owned `String` data.

### Phase 4: Function Body and Legacy Argument Handling Completion

- Port:
  - `get_knr_args`
  - `func_body`
- Preserve K&R argument parsing behavior exactly as currently implemented.
- Complete integration between declaration parsing, body handling, and symbol/reference recording.
- Add `cargo test` coverage around:
  - typedef parsing;
  - nested/direct declarators;
  - parameter list detection;
  - K&R argument handling;
  - symbol declaration/reference/call recording;
  - struct skipping paths.

## Testing Notes

- Prefer unit tests in the same module or adjacent Rust test modules targeting the migrated functions directly.
- Build test inputs from representative declaration fragments and parser edge cases inferred from current C behavior.
- Verify behavior parity at the level of produced declarations, symbol updates, and recorded references/calls rather than introducing new output formats.

## Constraints and Non-Goals

- Do not split the migrated parser logic into additional architectural layers beyond `src/parser.rs` unless required by existing crate structure.
- Do not introduce concurrency primitives, FFI shims, serialization, benchmark work, or parser feature expansion.
- Do not normalize or redesign data models beyond what is necessary to replace anonymous C records and unsafe memory patterns.
- Do not change external behavior for typedef parsing, declaration interpretation, or symbol/reference tracking except where Rust error typing requires explicit expression.