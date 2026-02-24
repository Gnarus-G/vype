pub use crate::sources::KeyOp;

pub fn edit_ops(previous: &str, current: &str) -> Vec<KeyOp> {
    if previous == current {
        return vec![];
    }

    if previous.is_empty() {
        return current.chars().map(KeyOp::Type).collect();
    }

    if current.is_empty() {
        return vec![KeyOp::Backspace(previous.chars().count())];
    }

    let prev_chars: Vec<char> = previous.chars().collect();
    let curr_chars: Vec<char> = current.chars().collect();

    let common_prefix_len = prev_chars
        .iter()
        .zip(curr_chars.iter())
        .take_while(|(a, b)| a == b)
        .count();

    let mut ops = Vec::new();

    let backspace_count = prev_chars.len() - common_prefix_len;
    if backspace_count > 0 {
        ops.push(KeyOp::Backspace(backspace_count));
    }

    for c in curr_chars[common_prefix_len..].iter() {
        ops.push(KeyOp::Type(*c));
    }

    ops
}

pub fn cost(ops: &[KeyOp]) -> usize {
    ops.iter()
        .map(|op| match op {
            KeyOp::Backspace(n) | KeyOp::Delete(n) | KeyOp::Left(n) | KeyOp::Right(n) => *n,
            KeyOp::Type(_) => 1,
        })
        .sum()
}
