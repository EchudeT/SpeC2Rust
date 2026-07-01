# Implementation Plan

## Summary

Port `src/wordsplit/wordsplit.c` into a single Rust module that preserves the existing initialization, allocation, error-state, sub-split context, and internal word-node list behavior exposed by the analyzed function set.

The Rust implementation should stay close to the current C file structure rather than redesigning the subsystem. The main technical approach is:

- translate the current module into one Rust source file under the project’s normal `src/wordsplit/` area;
- replace manual heap management and pointer arithmetic with owned Rust containers and explicit indices/references;
- convert C error code/state mutation helpers into internal Rust methods that update a module-local context object;
- keep helper/function boundaries aligned with the C functions where practical, so migration and review can happen function-by-function;
- preserve behavior of node allocation/append/remove and split-subcontext handling without adding new facilities.

## Technical Context

### Language / Version
- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.76+`

### Primary Dependencies
- Rust standard library only
- No third-party crates are recommended from the provided evidence

### Testing
- `cargo test`

### Performance Goals
- Maintain asymptotic behavior equivalent to the C implementation for initialization, node insertion, node removal, and temporary allocation growth
- Avoid unnecessary string/data copying beyond what is required by Rust ownership
- Use contiguous storage (`Vec`) for allocation arenas / node storage where the C code currently manages expandable memory regions
- Keep per-operation overhead low enough that the Rust port is operationally comparable to the C module for normal word-splitting workloads

## Module Mapping

### Source File Mapping
- C: `src/wordsplit/wordsplit.c`
- Rust: `src/wordsplit/wordsplit.rs`

### Function Mapping
| C Function | Rust Target | Notes |
|---|---|---|
| `is_name_char` | `fn is_name_char(ch: char) -> bool` or byte-based helper | Keep as small private helper; choose byte-based form if original logic is ASCII-oriented |
| `_wsplt_alloc_die` | `fn alloc_die(&mut self, ...) -> !` or `fn alloc_die(...) -> Result<_, WordSplitError>` depending on calling pattern | Prefer non-panicking error propagation unless the C behavior is process-aborting and required internally |
| `_wsplt_seterr` | `fn set_err(&mut self, code: ErrorCode, msg: Option<&str>)` | Internal context mutation helper |
| `_wsplt_nomem` | `fn no_mem(&mut self) -> Result<_, WordSplitError>` or state helper | Centralize allocation-failure mapping |
| `_wsplt_store_errctx` | `fn store_err_ctx(&mut self, ...)` | Preserve contextual error fields in state object |
| `_wsplt_setctxerr` | `fn set_ctx_err(&mut self, ...)` | Layered helper over stored context |
| `_wsplt_subsplit` | `fn subsplit(&mut self, ...) -> Result<..., WordSplitError>` | Keep private/internal unless existing Rust public API requires exposure |
| `_wsplt_seterr_sub` | `fn set_err_sub(&mut self, ...)` | Sub-split specific error propagation |
| `wordsplit_init0` | `fn init0(...) -> WordSplitContext` or `impl Default`/private initializer | Base zero/default initialization step |
| `wordsplit_init` | `pub fn init(...) -> Result<WordSplitContext, WordSplitError>` or equivalent constructor | Main externally used initializer |
| `alloc_space` | `fn alloc_space(&mut self, additional: usize) -> Result<(), WordSplitError>` | Replace realloc-style logic with `Vec::try_reserve` where appropriate |
| `wsnode_ptr` | `fn wsnode(&self, idx: usize) -> Option<&WsNode>` / mutable variant | Replace raw pointer lookup with index-based access |
| `wsnode_new` | `fn wsnode_new(&mut self, ...) -> Result<usize, WordSplitError>` | Return node index/handle instead of raw pointer |
| `wsnode_append` | `fn wsnode_append(&mut self, list: &mut NodeList, node: usize)` or context method | Preserve current list semantics |
| `wsnode_remove` | `fn wsnode_remove(&mut self, list: &mut NodeList, node: usize)` | Preserve unlink semantics safely |

### Rust Module Shape
Keep the port restrained to one module file with:
- private helper functions mirroring C internals;
- one primary context/state struct for the wordsplit working state;
- one internal node struct and list bookkeeping representation;
- one internal error representation aligned to the C module’s status-setting behavior.

## Data Model

Because the analysis only exposes anonymous C data structures, the Rust plan should introduce named internal types based strictly on observed function responsibilities rather than inventing broader abstractions.

### Data-Structure Mapping
| C Structure | Rust Representation | Mapping Decision |
|---|---|---|
| anonymous wordsplit working state | `struct WordSplitContext` | Central owner for configuration, temporary storage, error state, and node arena |
| anonymous error/status fields | `struct ErrorState` plus `enum ErrorCode` | Separate status code from optional context/message fields |
| anonymous sub-split context | `struct SubsplitContext` or embedded temporary fields in `WordSplitContext` | Use dedicated struct only if `_wsplt_subsplit` and `_wsplt_seterr_sub` clearly manipulate grouped state |
| anonymous allocation region / expandable storage | `Vec<T>` / `Vec<u8>` / `String` depending on actual stored content | Replace manual capacity tracking and realloc with owned growable containers |
| anonymous node record | `struct WsNode` | Named internal node type for append/remove operations |
| anonymous linked-list head/tail bookkeeping | `struct NodeList { head: Option<usize>, tail: Option<usize> }` | Use indices into node arena instead of pointers |
| anonymous prev/next links | `Option<usize>` fields on `WsNode` | Safe replacement for intrusive linked pointers |
| anonymous string/error context references | owned `String` or borrowed slices during processing | Prefer owned `String` when state outlives the input borrow |
| anonymous flags / mode fields | integer newtype, `u32`, or small enums/bitflags-like constants | Use plain integers/constants unless the original meaning is fully recoverable |
| anonymous counters / sizes | `usize` | Natural replacement for C size/count fields |

### Ownership and Memory Decisions
- Replace raw allocation helpers with owned containers on `WordSplitContext`.
- Use `Vec<WsNode>` as the node arena; node identity is an index, not an address.
- If stable node references are needed across mutation, avoid returning `&mut` across operations that may reallocate the arena; return indices and re-borrow when needed.
- For any “allocate space” behavior tied to text buffers, prefer `Vec<u8>` if the C code is byte-oriented, and convert to `String` only at validated UTF-8 boundaries if required by surrounding Rust APIs.
- Use `Default` where it cleanly matches zero-initialization from `wordsplit_init0`.

### Error Handling Mapping
- Replace module-wide mutable error code/message fields with:
  - `ErrorState` stored in `WordSplitContext` for compatibility with the C mutation pattern, and
  - `Result<T, WordSplitError>` returns for Rust-facing call boundaries.
- `_wsplt_seterr`, `_wsplt_store_errctx`, `_wsplt_setctxerr`, and `_wsplt_seterr_sub` become internal state-update helpers rather than public API.
- Allocation failure paths should use fallible reservation where possible (`try_reserve`) and map failures through the same internal no-memory helper to preserve behavior.

## Implementation Phases

## Phase 1: Create Module Skeleton and Core State

### Goals
- Establish the Rust file and core internal types aligned to the C module
- Port initialization and error-state helpers first so later functions have stable foundations

### Tasks
- Create `src/wordsplit/wordsplit.rs`
- Define `WordSplitContext` to hold:
  - initialization/default state,
  - allocation-backed storage,
  - node arena,
  - error/status fields,
  - any sub-split working fields required by the analyzed function group
- Define internal `ErrorCode`, `ErrorState`, and `WordSplitError`
- Port:
  - `wordsplit_init0`
  - `wordsplit_init`
  - `_wsplt_seterr`
  - `_wsplt_store_errctx`
  - `_wsplt_setctxerr`
  - `_wsplt_seterr_sub`
  - `_wsplt_nomem`
  - `_wsplt_alloc_die` as appropriate for the final error strategy
- Add focused unit tests for:
  - default/zero-style initialization,
  - error state replacement,
  - context error storage updates,
  - no-memory/error mapping behavior where testable

### Exit Criteria
- A context can be constructed in Rust with deterministic default state
- Internal error helpers compile and are covered by basic tests
- No raw pointers are introduced for these migrated responsibilities

## Phase 2: Port Allocation and Node Management

### Goals
- Replace C allocation and pointer-based node manipulation with safe Rust storage
- Preserve append/remove semantics without redesigning the data flow

### Tasks
- Define `WsNode` and `NodeList`
- Implement arena-backed node storage on `WordSplitContext`
- Port:
  - `alloc_space`
  - `wsnode_ptr`
  - `wsnode_new`
  - `wsnode_append`
  - `wsnode_remove`
- Translate pointer arithmetic/lookup into index-based access helpers
- Preserve list invariants:
  - empty list state,
  - single-node append/remove,
  - head/tail updates,
  - middle-node unlink
- Add unit tests for:
  - node creation,
  - append order,
  - remove from head/tail/middle,
  - repeated growth of allocation-backed storage

### Exit Criteria
- All node/list operations are represented with safe indices and `Option`
- No list-manipulation path depends on unstable references across vector growth
- Tests validate structural integrity after append/remove operations

## Phase 3: Port Character and Sub-split Logic

### Goals
- Migrate the remaining helper logic that depends on the established state and node storage
- Keep behavior close to the C implementation without broadening API surface

### Tasks
- Port `is_name_char` with the same character-class semantics as the C implementation
- Port `_wsplt_subsplit`
- Connect sub-split error propagation through the already-ported error helpers
- Ensure any temporary state used during sub-splitting is owned or borrowed safely
- Add unit tests for:
  - name-character classification,
  - successful sub-split flows that can be isolated from the larger subsystem,
  - sub-split error propagation and stored context updates

### Exit Criteria
- Remaining analyzed functions are migrated
- Sub-split logic compiles cleanly with the Rust context and error model
- Tests cover both success and failure paths for the migrated logic

## Phase 4: Stabilization and File-Level Review

### Goals
- Verify the Rust file is a faithful, restrained replacement for the analyzed C file segment
- Remove migration-only rough edges while keeping the structure close to the source

### Tasks
- Review naming and visibility to ensure only required entry points are public
- Re-check all former manual-memory paths for ownership correctness and unnecessary cloning
- Align integer types and boundary checks with original C assumptions where relevant
- Add regression tests for edge cases discovered during the port:
  - empty input/state,
  - repeated initialization,
  - node removal on minimal lists,
  - error overwrite precedence
- Run `cargo test` and fix any borrow/lifetime simplifications needed for maintainability

### Exit Criteria
- The Rust port covers the entire listed function set from `wordsplit.c`
- The module remains confined to the existing responsibility boundary
- Tests pass under `cargo test`

## Notes and Constraints

- Keep the migration file-scoped and function-scoped; do not split into extra modules unless required by the existing Rust crate layout.
- Prefer explicit state mutation on `WordSplitContext` over introducing callback systems or generalized frameworks.
- Preserve byte/character semantics from C carefully; avoid assuming full Unicode behavior unless the original logic clearly operates on Rust `char`.
- Do not add thread-safety layers, FFI wrappers, serialization, benchmarks, or recovery facilities.
- If the original C module uses sentinel/null conventions, map them directly to `Option` and small enums rather than redesigning control flow.