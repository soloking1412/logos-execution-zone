use nssa_core::program::{
    AccountPostState, ProgramId, ProgramInput, ProgramOutput, assert_capability,
    read_nssa_inputs,
};

// Internal continuation for Program A. Requires a capability ticket from
// program_a_entry — direct user invocations will always fail here.

type Instruction = ProgramId; // required_caller (= program_a_entry's id)

fn main() {
    let (
        ProgramInput {
            pre_states,
            instruction: required_caller,
            capabilities,
        },
        instruction_words,
    ) = read_nssa_inputs::<Instruction>();

    assert_capability(&capabilities, required_caller);

    let post_states: Vec<AccountPostState> = pre_states
        .iter()
        .map(|pre| AccountPostState::new(pre.account.clone()))
        .collect();

    ProgramOutput::new(instruction_words, pre_states, post_states).write();
}
