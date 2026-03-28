use nssa_core::program::{
    AccountPostState, ChainedCall, DEFAULT_PROGRAM_ID, ProgramId, ProgramInput, ProgramOutput,
    read_nssa_inputs,
};

// Program B: receives a continuation address and forwarded capability from
// Program A, does its work, then tail-calls the continuation.

type Instruction = ProgramId; // program_a_internal_id

fn main() {
    let (
        ProgramInput {
            pre_states,
            instruction: program_a_internal_id,
            capabilities,
        },
        instruction_words,
    ) = read_nssa_inputs::<Instruction>();

    let post_states: Vec<AccountPostState> = pre_states
        .iter()
        .map(|pre| AccountPostState::new(pre.account.clone()))
        .collect();

    // Forward the capability so program_a_internal can verify it. If there are
    // no capabilities (e.g. a direct call to B trying to reach the continuation)
    // we pass DEFAULT_PROGRAM_ID, which program_a_internal will correctly reject.
    let required_caller: ProgramId = capabilities.first().copied().unwrap_or(DEFAULT_PROGRAM_ID);
    let continuation_instruction_data = risc0_zkvm::serde::to_vec(&required_caller).unwrap();

    let call_to_a_internal = ChainedCall {
        program_id: program_a_internal_id,
        instruction_data: continuation_instruction_data,
        pre_states: pre_states.clone(),
        pda_seeds: vec![],
        capabilities: capabilities.clone(),
    };

    ProgramOutput::new(instruction_words, pre_states, post_states)
        .with_chained_calls(vec![call_to_a_internal])
        .write();
}
