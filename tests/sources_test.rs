use vype::sources::{AudioSource, KeyboardSink, Transcriber};

#[test]
fn audio_source_trait_has_required_methods() {
    struct MockAudioSource;

    impl AudioSource for MockAudioSource {
        fn start(&mut self) -> Result<(), vype::sources::SourceError> {
            Ok(())
        }

        fn stop(&mut self) -> Vec<f32> {
            vec![]
        }

        fn sample_rate(&self) -> u32 {
            16000
        }

        fn channels(&self) -> u16 {
            1
        }
    }

    let mut source = MockAudioSource;
    assert!(source.start().is_ok());
    assert_eq!(source.sample_rate(), 16000);
    assert_eq!(source.channels(), 1);
}

#[test]
fn keyboard_sink_trait_has_type_method() {
    struct MockKeyboardSink;

    impl KeyboardSink for MockKeyboardSink {
        fn type_text(&self, _text: &str) {
            // no-op
        }
    }

    let sink = MockKeyboardSink;
    sink.type_text("hello");
}

#[test]
fn transcriber_trait_has_transcribe_method() {
    struct MockTranscriber;

    impl Transcriber for MockTranscriber {
        fn transcribe(&self, _audio: &[f32]) -> Result<String, vype::sources::SourceError> {
            Ok("mock transcription".to_string())
        }
    }

    let transcriber = MockTranscriber;
    let result = transcriber.transcribe(&[]).unwrap();
    assert_eq!(result, "mock transcription");
}
