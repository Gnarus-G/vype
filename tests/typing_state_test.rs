use vype::pure::typing_state::TypingState;
use vype::sources::KeyOp;

#[test]
fn new_state_is_empty() {
    let state = TypingState::new();
    assert_eq!(state.typed(), "");
}

#[test]
fn transition_from_empty_types_all() {
    let mut state = TypingState::new();
    let ops = state.transition("hello");
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
    assert_eq!(state.typed(), "hello");
}

#[test]
fn transition_to_same_text_no_ops() {
    let mut state = TypingState::new();
    state.transition("hello");
    let ops = state.transition("hello");
    assert!(ops.is_empty());
    assert_eq!(state.typed(), "hello");
}

#[test]
fn transition_updates_state() {
    let mut state = TypingState::new();
    state.transition("hello");
    state.transition("hello world");
    assert_eq!(state.typed(), "hello world");
}

#[test]
fn clear_resets_state() {
    let mut state = TypingState::new();
    state.transition("hello");
    state.clear();
    assert_eq!(state.typed(), "");
}

#[test]
fn partial_to_final_transition() {
    let mut state = TypingState::new();

    let _ = state.transition("hello");
    assert_eq!(state.typed(), "hello");

    let ops = state.transition("hello world");
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
    assert_eq!(state.typed(), "hello world");
}

#[test]
fn correction_transition() {
    let mut state = TypingState::new();

    let _ = state.transition("helo");
    let ops = state.transition("hello");
    assert_eq!(state.typed(), "hello");
    assert_eq!(
        ops,
        vec![KeyOp::Backspace(1), KeyOp::Type('l'), KeyOp::Type('o'),]
    );
}
