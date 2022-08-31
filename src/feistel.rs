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
        // Spilt the state into halves.
        let (left, right) = state.split_at(state.len() / 2);

        // Apply f to the right half and xor it into the left half.
        let f_right = f(i, right);
        let (new_left, new_right) = (
            left.iter().zip(f_right.iter()).map(|(&l, &r)| l ^ r),
            right.iter().cloned(),
        );

        // Swap the left and right halves.
        let (new_left, new_right) = (new_right, new_left);

        state = new_left.chain(new_right).collect();
    }

    state
}
