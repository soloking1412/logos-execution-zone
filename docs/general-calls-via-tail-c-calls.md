# General Cross-Program Calls via Tail Calls

**LP-0015 Specification Document**

---

## Overview

This document specifies the design and implementation of *general cross-program calls* in the Logos Execution Zone (LEZ), where "call B then return to A and continue" is expressed purely using tail calls (the only existing execution primitive), and where a *capability-based security mechanism* enforces that only legitimately chained executions can reach *internal-only* continuation handlers.

---

## 1. Motivation

Today, every LEZ function is effectively externally callable: there is no mechanism to mark a function as reachable *only* during chained execution.  This creates two problems:

1. **Security**: A user can construct a transaction that directly invokes a "continuation" handler, bypassing the prior program steps that were meant to set up invariants.
2. **Composability**: Without a principled way to protect internal continuations, developers must add fragile ad-hoc guards rather than relying on a structural guarantee.

This specification introduces an *unforgeable capability ticket* mechanism that allows programs to distinguish **External** (user-callable) from **Internal** (chain-only) entrypoints.

---

## 2. Execution Model

### 2.1 Tail Calls as the Only Primitive

LEZ programs express all inter-program calls as tail calls: a program commits its `ProgramOutput` (including a `chained_calls` list) and terminates.  The sequencer then dispatches each chained call in order.  This is consistent with Continuation-Passing Style (CPS), where every call is a tail call.

### 2.2 Continuation-Passing Style

A general call of the form:

```
A:
  result = call B(args)   // blocking call semantics from developer POV
  continue(result)
```

Is compiled under CPS into:

```
A_entry:
  tail_call B(args, continuation=A_internal)

B_entry:
  do_work(args)
  tail_call A_internal(result, capabilities=[program_a_entry_id])

A_internal:
  assert_capability(capabilities, program_a_entry_id)
  continue(result)
```

Every step is a tail call; the "continuation" is just another program invocation protected by a capability ticket.

---

## 3. Capability System

### 3.1 Definitions

| Term | Definition |
|------|------------|
| **Capability ticket** | A `ProgramId` carried in the `capabilities` field of `ProgramInput` / `ChainedCall`. |
| **Minting** | A program is always permitted to include its own `ProgramId` in any `ChainedCall.capabilities` it emits. |
| **Forwarding** | A program may include in a `ChainedCall.capabilities` any capability that was present in its own received `ProgramInput.capabilities`. |
| **Forgery attempt** | Placing in `ChainedCall.capabilities` a `ProgramId` that is neither the emitting program's own ID nor appeared in its received capabilities. |
| **External entrypoint** | A function callable directly by users.  Receives `capabilities = []`. |
| **Internal entrypoint** | A function that must only be reached through a legitimate chain.  Must call `assert_capability` at its top. |

### 3.2 Invariants

1. **User calls are always capability-free**: The sequencer constructs the initial `ChainedCall` with `capabilities = []`.  No user-supplied input can place a capability ticket into the initial call.
2. **Minting is self-scoped**: A program P may only mint a capability with `program_id == P.id()`.
3. **Forwarding is transitive**: Any capability received may be forwarded one or more hops down the chain.
4. **Forgery is rejected**: Any attempt to emit a chained call containing a capability that the emitting program neither owns nor received causes the entire transaction to be rejected with `InvalidProgramBehavior`.

These four invariants together make capabilities unforgeable.

### 3.3 Security Argument

An attacker who wants to invoke an Internal entrypoint directly submits a transaction targeting that program.  The sequencer constructs the initial call with `capabilities = []`.  The program reads an empty capabilities list, and `assert_capability` panics, causing the zkVM execution to fail; the sequencer rejects the transaction.

An attacker who wants to replay a capability from one chain into another illegitimate chain cannot do so: capabilities are not stored in on-chain state; they exist only for the lifetime of a single transaction's tail-call chain.  There is no way to "save" a capability between transactions.

An attacker who wants to forge a capability from within a rogue deployed program cannot do so: the sequencer validates every capability in every emitted `ChainedCall` against the executing program's own ID and received capabilities before dispatching it.  A rogue program that emits a forged capability will cause `InvalidProgramBehavior` before any further execution occurs.

---

## 4. Interface Specification

### 4.1 `nssa_core::program` Changes

#### `ProgramInput<T>` (extended)

```rust
pub struct ProgramInput<T> {
    pub pre_states: Vec<AccountWithMetadata>,
    pub instruction: T,
    /// Runtime-issued capability tickets.
    /// Empty when invoked directly by a user (External entrypoint).
    pub capabilities: Vec<ProgramId>,
}
```

#### `ChainedCall` (extended)

```rust
pub struct ChainedCall {
    pub program_id: ProgramId,
    pub pre_states: Vec<AccountWithMetadata>,
    pub instruction_data: InstructionData,
    pub pda_seeds: Vec<PdaSeed>,
    /// Capability tickets to forward to the callee.
    /// Subject to sequencer enforcement (see §3.2).
    pub capabilities: Vec<ProgramId>,
}
```

Builder extension:

```rust
impl ChainedCall {
    pub fn with_capabilities(mut self, capabilities: Vec<ProgramId>) -> Self;
}
```

#### `read_nssa_inputs<T>()` (extended)

Reads `capabilities` as a third field after `pre_states` and `instruction_data`.

#### `assert_capability(capabilities, required_caller)` (new)

```rust
pub fn assert_capability(capabilities: &[ProgramId], required_caller: ProgramId);
```

Panics if `required_caller` is absent from `capabilities`.  Use at the top of every Internal entrypoint.

### 4.2 `Program::write_inputs` (extended)

```rust
pub(crate) fn write_inputs(
    pre_states: &[AccountWithMetadata],
    instruction_data: &[u32],
    capabilities: &[ProgramId],    // NEW
    env_builder: &mut ExecutorEnvBuilder,
) -> Result<(), NssaError>;
```

### 4.3 `Program::execute` (extended)

```rust
pub(crate) fn execute(
    &self,
    pre_states: &[AccountWithMetadata],
    instruction_data: &InstructionData,
    capabilities: &[ProgramId],    // NEW
) -> Result<ProgramOutput, NssaError>;
```

---

## 5. Sequencer Enforcement

In `validate_and_produce_public_state_diff` (public transactions) and `execute_and_prove_program` (privacy-preserving circuit):

1. Initial `ChainedCall` is created with `capabilities: vec![]`.
2. For each emitted `ChainedCall` in a program's output:

```
for cap in new_call.capabilities:
    assert(cap == executing_program_id  ||  cap in caller_received_capabilities)
    → else: return Err(InvalidProgramBehavior)
```

3. The validated chained call is dispatched with its `capabilities` intact.

---

## 6. Developer Ergonomics (SDK)

### 6.1 Calling pattern (External → Internal via B)

**Program A External** (`program_a_entry.rs`):

```rust
let call_to_b = ChainedCall {
    program_id: program_b_id,
    instruction_data: risc0_zkvm::serde::to_vec(&program_a_internal_id).unwrap(),
    pre_states: pre_states.clone(),
    pda_seeds: vec![],
    capabilities: vec![my_program_id],  // mint own capability
};
ProgramOutput::new(...).with_chained_calls(vec![call_to_b]).write();
```

**Program B** (`program_b.rs`):

```rust
let call_to_a_internal = ChainedCall {
    program_id: program_a_internal_id,
    instruction_data: ...,
    pre_states: pre_states.clone(),
    pda_seeds: vec![],
    capabilities: capabilities.clone(), // forward received caps
};
```

**Program A Internal** (`program_a_internal.rs`):

```rust
let (ProgramInput { capabilities, .. }, _) = read_nssa_inputs::<ProgramId>();
assert_capability(&capabilities, required_caller); // guard
// ... continuation work ...
```

### 6.2 `call_program!` macro (ergonomic shorthand)

For convenience, the following macro pattern can be used:

```rust
let call = ChainedCall::new(target_program_id, accounts, &instruction)
    .with_capabilities(vec![my_id]);
```

---

## 7. Entrypoint Enumeration and Routing

Since each LEZ binary is a separate ELF, the "entrypoint" is the ELF itself.  External vs. Internal distinction is:

| Property | External | Internal |
|----------|----------|----------|
| Callable by users | ✓ | ✗ (rejected by `assert_capability`) |
| Callable in chain | ✓ | ✓ (if capability present) |
| Receives capabilities | No (always `[]`) | Yes (forwarded by caller) |
| First instruction in tx | Allowed | Will fail at `assert_capability` |

### Ordering and Determinism

Chained calls are dispatched in FIFO order from the head of the `chained_calls` Vec produced by each program's output.  The sequencer processes them in a depth-first order (calls emitted by a program are pushed to the *front* of the queue with `.push_front`), ensuring that the full sub-tree of a call is resolved before sibling calls.

### Error Semantics

| Condition | Error |
|-----------|-------|
| Direct call to Internal (no capability) | `ProgramExecutionFailed` (zkVM panics) |
| Capability forgery (emitting unowned capability) | `InvalidProgramBehavior` |
| Chain exceeds depth limit | `MaxChainedCallsDepthExceeded` |

---

## 8. Test Coverage

| Test | Location | Scenario |
|------|----------|----------|
| `general_call_capability_positive_path` | `nssa/src/state.rs` | A→B→A_internal succeeds |
| `direct_call_to_internal_entrypoint_is_rejected` | `nssa/src/state.rs` | Direct user call fails |
| `sequencer_rejects_forged_capability_in_chained_call` | `nssa/src/public_transaction/transaction.rs` | Forgery rule fires |
| `public_chained_call` | `nssa/src/state.rs` | Existing chain caller still works |
| All pre-existing `chain_*` tests | various | Backward-compatibility |

---

## 9. Backwards Compatibility

All existing programs that do not use `capabilities` continue to work unchanged:
- Their `ChainedCall` structs default to `capabilities: vec![]`.
- Their `ProgramInput.capabilities` is always `[]` and they ignore it.
- The sequencer enforcement rule is vacuously true for empty capability lists.

---

## 10. Future Work

- **Typed capabilities**: Replace `ProgramId` tickets with typed tokens carrying additional data (e.g., allowed sub-function selectors).
- **Capability attenuation**: Allow a program to forward a *weakened* capability (subset of permissions) to limit what the callee can authorize.
- **Tooling**: A `#[external]` / `#[internal]` procedural macro to automate `assert_capability` injection and enforced at compile time.
