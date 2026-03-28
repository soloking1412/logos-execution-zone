use nssa_core::program::{
    AccountPostState, ChainedCall, ProgramId, ProgramInput, ProgramOutput, read_nssa_inputs,
};

// Public entrypoint for Program A.
// Mints its own capability and tail-calls Program B, passing the internal
// continuation address so B can call back.

type Instruction = (ProgramId, ProgramId, ProgramId); // (program_b_id, program_a_internal_id, my_id)

fn main() {
    let (
        ProgramInput {
            pre_states,
            instruction: (program_b_id, program_a_internal_id, my_program_id),
            ..
        },
        instruction_words,
    ) = read_nssa_inputs::<Instruction>();

    let continuation_instruction: Vec<u32> =
        risc0_zkvm::serde::to_vec(&program_a_internal_id).unwrap();

    let post_states: Vec<AccountPostState> = pre_states
        .iter()
        .map(|pre| AccountPostState::new(pre.account.clone()))
        .collect();

    let call_to_b = ChainedCall {
        program_id: program_b_id,
        instruction_data: continuation_instruction,
        pre_states: pre_states.clone(),
        pda_seeds: vec![],
        capabilities: vec![my_program_id],
    };

    ProgramOutput::new(instruction_words, pre_states, post_states)
        .with_chained_calls(vec![call_to_b])
        .write();
}
