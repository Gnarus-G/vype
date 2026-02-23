use vype::pure::resample::resample_to_16khz_mono;

#[test]
fn no_op_when_already_16khz_mono() {
    let samples: Vec<f32> = vec![0.0, 0.5, 1.0, 0.5, 0.0];
    let result = resample_to_16khz_mono(&samples, 16000, 1);

    assert_eq!(result, samples);
}

#[test]
fn downsample_44100_to_16000() {
    let samples: Vec<f32> = (0..44100)
        .map(|i| (i as f32 / 44100.0 * 2.0 - 1.0))
        .collect();
    let result = resample_to_16khz_mono(&samples, 44100, 1);

    assert!(result.len() > 0);
    assert!(result.len() < samples.len(), "Should be downsampled");

    let expected_len = (16000.0 / 44100.0 * samples.len() as f64) as usize;
    assert!((result.len() as i32 - expected_len as i32).abs() < 100);
}

#[test]
fn stereo_to_mono_conversion() {
    let samples: Vec<f32> = vec![1.0, 0.0, 0.5, 0.5, 0.0, 1.0];
    let result = resample_to_16khz_mono(&samples, 16000, 2);

    assert!(result.len() > 0);

    let expected_frames = samples.len() / 2;
    assert_eq!(result.len(), expected_frames);
}

#[test]
fn empty_input_returns_empty_output() {
    let samples: Vec<f32> = vec![];
    let result = resample_to_16khz_mono(&samples, 16000, 1);

    assert!(result.is_empty());
}
