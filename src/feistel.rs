//! Generic Feistel network implementation.

/// Encrypts or decrypts using a Feistel network. `rounds` is the number of rounds, `input` is the
/// input data, and `f` is the round function, which takes as input the round number and the right
/// half of the data.
pub fn feistel(
    rounds: usize,
    f: impl Fn(usize, Vec<bool>) -> Vec<bool>,
    input: Vec<bool>,
) -> Vec<bool> {
    todo!()
}
