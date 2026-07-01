# Implementation Plan

## Summary

Port `xalloc-die.c` into a minimal Rust module that preserves the current role of `xalloc_die`: terminate execution on unrecoverable allocation failure through a single centralized path. The Rust implementation should mirror the existing control flow rather than introduce broader allocation abstractions.

Technical approach:

- Implement a focused Rust module containing the equivalent of `xalloc_die`.
- Use Rust’s standard error-reporting and process-termination facilities (`eprintln!`, `std::process::exit`, or a panic path only if required by surrounding code expectations).
- Keep the function signature and visibility aligned with how the wider `cat` port will call it.
- Avoid adding new allocation layers or recovery logic; this module remains a terminal failure handler.

## Technical Context

- **Language/Version:** Rust 1.75+
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - No measurable runtime overhead beyond emitting the failure message and terminating.
  - No additional heap allocation on the normal path.
  - Preserve constant-time failure-path dispatch comparable to the C implementation.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `xalloc-die.c` | `src/xalloc_die.rs` | Direct migration of the module into a single Rust source file. |
| `xalloc-die.c` (`xalloc_die`) | `src/xalloc_die.rs` (`pub(crate) fn xalloc_die(...) -> !` or equivalent) | Rust function should remain a dedicated non-returning failure routine. |

## Data Model

This module has no persistent data structures to migrate.

| C Construct | Rust Construct | Notes |
|---|---|---|
| none | none | Implementation is function-only. |

## Implementation Phases

### Phase 1: Establish Module Skeleton

- Create `src/xalloc_die.rs`.
- Add the Rust declaration for `xalloc_die`.
- Choose a non-returning Rust signature (`-> !`) if the C behavior is unconditional termination.
- Wire the module into the crate using standard Rust module declarations only where needed by the existing port structure.

**Exit criteria:**

- The crate recognizes the new module.
- The function is callable from the rest of the Rust port.

### Phase 2: Port Termination Behavior

- Translate the C function body into Rust using only standard library facilities.
- Preserve the original semantic sequence:
  - report allocation failure in the expected way,
  - terminate immediately,
  - do not attempt cleanup/recovery beyond what the original module did.
- Ensure memory handling remains implicit and safe under Rust ownership rules, with no manual allocation logic introduced.

**Exit criteria:**

- `xalloc_die` behaves as a terminal failure path.
- No unsafe code is introduced unless strictly required by an already-existing crate interface.

### Phase 3: Integrate Call Sites

- Update the Rust equivalents of any existing users of `xalloc_die` to call the new module function.
- Align return types and control flow so callers do not expect recovery after invocation.
- Remove any temporary placeholders related to allocation-failure termination in the surrounding migrated code.

**Exit criteria:**

- All current Rust-side call sites compile against the shared `xalloc_die`.
- Control-flow typing reflects non-returning behavior cleanly.

### Phase 4: Validation and Cleanup

- Add focused tests where feasible for observable behavior short of process termination, or verify through integration-style command tests if termination must be observed externally.
- Run `cargo test`.
- Confirm the module remains narrowly scoped to the original C functionality and does not accumulate unrelated helpers.

**Exit criteria:**

- Tests pass.
- File/module layout matches the intended one-to-one migration.
- The implementation remains limited to the original `xalloc_die` responsibility.