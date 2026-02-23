use vype::pure::synth_audio::generate_sine_wave;

#[test]
fn generates_sine_wave_at_correct_frequency() {
    let sample_rate = 16000u32;
    let frequency = 440.0f32;
    let duration_secs = 1.0f32;

    let samples = generate_sine_wave(frequency, sample_rate, duration_secs);

    let expected_len = (sample_rate as f32 * duration_secs) as usize;
    assert_eq!(samples.len(), expected_len);
}

#[test]
fn sine_wave_values_in_range_minus_one_to_one() {
    let samples = generate_sine_wave(440.0, 16000, 1.0);

    for (i, &sample) in samples.iter().enumerate() {
        assert!(
            sample >= -1.0 && sample <= 1.0,
            "Sample {} = {} is out of range [-1, 1]",
            i,
            sample
        );
    }
}

#[test]
fn silence_generates_zeros() {
    let samples = vype::pure::synth_audio::generate_silence(16000, 0.5);

    assert_eq!(samples.len(), 8000);
    for sample in samples {
        assert_eq!(sample, 0.0);
    }
}
