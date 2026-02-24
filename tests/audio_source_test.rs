use vype::sources::fakes::FakeAudioSource;
use vype::sources::AudioSource;

#[test]
fn fake_audio_source_returns_current_samples_while_recording() {
    let samples = vec![0.1f32, 0.2, 0.3, 0.4, 0.5];
    let mut source = FakeAudioSource::new(samples.clone(), 48000, 1);

    source.start().unwrap();
    let current = source.get_current_samples();
    assert_eq!(current, samples);

    let final_samples = source.stop();
    assert_eq!(final_samples, samples);
}
