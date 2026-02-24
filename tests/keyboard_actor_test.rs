use std::time::Duration;
use vype::actors::spawn_actor;
use vype::keyboard::{KeyboardActor, KeyboardMsg};
use vype::sources::fakes::{CaptureKeyboardSink, FakeAudioSource, MockTranscriber};

#[test]
fn keyboard_actor_orchestrates_flow_on_ptt_release() {
    let samples = vec![0.1f32, 0.2, 0.3];
    let audio_source = FakeAudioSource::new(samples.clone(), 48000, 1);
    let transcriber = MockTranscriber::new("hello world".to_string());
    let keyboard_sink = CaptureKeyboardSink::new();

    let audio_ref = keyboard_sink.clone();
    let _transcriber_ref = keyboard_sink.clone();

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
    keyboard_actor_ref.send(KeyboardMsg::PTTReleased).unwrap();

    std::thread::sleep(Duration::from_millis(100));

    let captured = audio_ref.captured();
    assert_eq!(captured, vec!["hello world"]);
}
