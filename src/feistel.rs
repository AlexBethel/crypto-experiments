//! Generic Feistel network implementation.

/// Encrypts or decrypts using a Feistel network. `rounds` is the number of rounds, `input` is the
/// input data, and `f` is the round function, which takes as input the round number and the right
/// half of the data.
pub fn feistel(
    rounds: usize,
    f: impl Fn(usize, &[bool]) -> Vec<bool>,
    input: &[bool],
    encrypt: bool,
) -> Vec<bool> {
    let mut state = input.to_owned();
    for i in (0..rounds).map(|i| if encrypt { i } else { rounds - i - 1 }) {
        let (left, right) = state.split_at(state.len() / 2);
        let (new_left, new_right) = (
            left.iter().zip(f(i, right).iter()).map(|(&l, &r)| l ^ r),
            left.iter().cloned(),
        );

        state = new_left.chain(new_right).collect();
    }

    state
}
