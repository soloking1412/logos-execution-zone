use nssa_core::program::{AccountPostState, ProgramInput, read_nssa_inputs, write_nssa_outputs};

type Instruction = ();

fn main() {
    let (
        ProgramInput {
            pre_states,
            instruction: (),
        },
        instruction_words,
    ) = read_nssa_inputs::<Instruction>();

    let Ok([pre]) = <[_; 1]>::try_from(pre_states) else {
        return;
    };

    let account_pre = &pre.account;
    let account_pre_data = account_pre.data.clone();
    let clock =
        u64::from_le_bytes(account_pre_data.into_inner().try_into().expect(
            "Block context program account data should be the LE encoding of a u64 integer",
        ));

    let mut account_post = account_pre.clone();
    account_post.data = clock
        .checked_add(1)
        .expect("Next timestap should be within u64 boundaries")
        .to_le_bytes()
        .to_vec()
        .try_into()
        .expect("u64 byte length should fit in account data");

    let post = AccountPostState::new(account_post);

    write_nssa_outputs(instruction_words, vec![pre], vec![post]);
}
