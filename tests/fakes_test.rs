use vype::sources::{AudioSource, KeyboardSink, Transcriber};

#[test]
fn fake_audio_source_returns_preloaded_samples() {
    let samples = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    let mut fake = vype::sources::fakes::FakeAudioSource::new(samples.clone(), 16000, 1);

    assert_eq!(fake.sample_rate(), 16000);
    assert_eq!(fake.channels(), 1);

    fake.start().unwrap();
    let result = fake.stop();

    assert_eq!(result, samples);
}

#[test]
fn fake_audio_source_can_be_started_and_stopped() {
    let mut fake = vype::sources::fakes::FakeAudioSource::new(vec![0.0; 100], 44100, 2);

    assert!(!fake.is_recording());
    fake.start().unwrap();
    assert!(fake.is_recording());
    fake.stop();
    assert!(!fake.is_recording());
}

#[test]
fn capture_keyboard_sink_collects_events() {
    let mut sink = vype::sources::fakes::CaptureKeyboardSink::new();

    sink.type_text("hello");
    sink.type_text(" ");
    sink.type_text("world");

    let events = sink.captured();
    assert!(events.contains(&"hello".to_string()));
    assert!(events.contains(&" ".to_string()));
    assert!(events.contains(&"world".to_string()));
}

#[test]
fn mock_transcriber_returns_preset_text() {
    let transcriber = vype::sources::fakes::MockTranscriber::new("hello world".to_string());

    let result = transcriber.transcribe(&[0.0; 100]).unwrap();
    assert_eq!(result, "hello world");
}

#[test]
fn mock_transcriber_can_return_error() {
    let transcriber = vype::sources::fakes::MockTranscriber::with_error("transcription failed");

    let result = transcriber.transcribe(&[0.0; 100]);
    assert!(result.is_err());
}
