use std::time::Duration;
use vype::actors::spawn_actor;
use vype::audio::AudioMsg;
use vype::keyboard::{KeyboardActor, KeyboardMsg};
use vype::sources::fakes::{CaptureKeyboardSink, FakeAudioSource, MockTranscriber};
use vype::sources::{KeyOp, KeyboardSink};
use vype::transcriber::TranscriberMsg;

#[test]
fn partial_transcription_updates_typed_text() {
    let samples = vec![0.1f32; 16000 * 4];
    let audio_source = FakeAudioSource::new(samples.clone(), 48000, 1);
    let transcriber = MockTranscriber::new("hello world".to_string());
    let keyboard_sink = CaptureKeyboardSink::new();

    let audio_ref = keyboard_sink.clone();

    let (audio_actor_ref, _audio_handle) = spawn_actor(vype::audio::AudioActor::new(
        audio_source,
        Duration::from_secs(30),
    ));
    let (transcriber_actor_ref, _transcriber_handle) =
        spawn_actor(vype::transcriber::TranscriberActor::new(transcriber));

    let (keyboard_actor_ref, _keyboard_handle) = spawn_actor(KeyboardActor::new(
        audio_actor_ref,
        transcriber_actor_ref,
        Box::new(keyboard_sink),
    ));

    keyboard_actor_ref.send(KeyboardMsg::PTTPressed).unwrap();
    keyboard_actor_ref
        .send(KeyboardMsg::PartialTranscribe)
        .unwrap();
    keyboard_actor_ref.send(KeyboardMsg::PTTReleased).unwrap();

    std::thread::sleep(Duration::from_millis(100));

    let captured = audio_ref.captured();
    assert!(!captured.is_empty());
}

#[test]
fn keyboard_sink_executes_key_ops() {
    let mut sink = CaptureKeyboardSink::new();

    let ops = vec![KeyOp::Type('a'), KeyOp::Type('b'), KeyOp::Backspace(1)];
    sink.execute_ops(&ops);

    let captured = sink.captured();
    assert_eq!(captured, vec!["ab", "\x08"]);
}

#[test]
fn keyboard_sink_batches_consecutive_types() {
    let mut sink = CaptureKeyboardSink::new();

    let ops = vec![
        KeyOp::Type('h'),
        KeyOp::Type('e'),
        KeyOp::Type('l'),
        KeyOp::Type('l'),
        KeyOp::Type('o'),
        KeyOp::Backspace(2),
        KeyOp::Type('!'),
    ];
    sink.execute_ops(&ops);

    let captured = sink.captured();
    assert_eq!(captured, vec!["hello", "\x08\x08", "!"]);
}
