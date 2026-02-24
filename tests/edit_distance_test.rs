use vype::pure::edit_distance::edit_ops;
use vype::sources::KeyOp;

#[test]
fn empty_to_text_types_all() {
    let ops = edit_ops("", "hello");
    assert_eq!(
        ops,
        vec![
            KeyOp::Type('h'),
            KeyOp::Type('e'),
            KeyOp::Type('l'),
            KeyOp::Type('l'),
            KeyOp::Type('o'),
        ]
    );
}

#[test]
fn text_to_empty_backspaces_all() {
    let ops = edit_ops("hello", "");
    assert_eq!(ops, vec![KeyOp::Backspace(5)]);
}

#[test]
fn same_text_no_ops() {
    let ops = edit_ops("hello", "hello");
    assert!(ops.is_empty());
}

#[test]
fn prefix_match_types_suffix() {
    let ops = edit_ops("hello", "hello world");
    assert_eq!(
        ops,
        vec![
            KeyOp::Type(' '),
            KeyOp::Type('w'),
            KeyOp::Type('o'),
            KeyOp::Type('r'),
            KeyOp::Type('l'),
            KeyOp::Type('d'),
        ]
    );
}

#[test]
fn middle_edit_uses_backspace_and_retype() {
    let ops = edit_ops("hello", "hallo");
    assert_eq!(
        ops,
        vec![
            KeyOp::Backspace(4),
            KeyOp::Type('a'),
            KeyOp::Type('l'),
            KeyOp::Type('l'),
            KeyOp::Type('o'),
        ]
    );
}

#[test]
fn complete_replace() {
    let ops = edit_ops("abc", "xyz");
    assert_eq!(
        ops,
        vec![
            KeyOp::Backspace(3),
            KeyOp::Type('x'),
            KeyOp::Type('y'),
            KeyOp::Type('z'),
        ]
    );
}

#[test]
fn backspace_is_more_efficient_than_retyping_prefix() {
    let ops = edit_ops("abcdefgh", "abcd");
    assert_eq!(ops, vec![KeyOp::Backspace(4)]);
}

#[test]
fn handles_unicode_correctly() {
    let ops = edit_ops("", "héllo");
    assert_eq!(
        ops,
        vec![
            KeyOp::Type('h'),
            KeyOp::Type('é'),
            KeyOp::Type('l'),
            KeyOp::Type('l'),
            KeyOp::Type('o'),
        ]
    );
}
