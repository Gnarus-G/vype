use rubato::{FftFixedIn, Resampler};

pub fn resample_to_16khz_mono(samples: &[f32], from_rate: u32, channels: u16) -> Vec<f32> {
    if samples.is_empty() {
        return Vec::new();
    }

    let mono = if channels > 1 {
        stereo_to_mono(samples, channels as usize)
    } else {
        samples.to_vec()
    };

    if from_rate == 16000 {
        return mono;
    }

    resample(&mono, from_rate, 16000)
}

fn stereo_to_mono(samples: &[f32], channels: usize) -> Vec<f32> {
    let frames = samples.len() / channels;
    let mut mono = Vec::with_capacity(frames);

    for frame_idx in 0..frames {
        let offset = frame_idx * channels;
        let sum: f32 = (0..channels).map(|c| samples[offset + c]).sum();
        mono.push(sum / channels as f32);
    }

    mono
}

fn resample(samples: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
    let chunk_size = 1024;

    if samples.len() < chunk_size {
        let ratio = to_rate as f64 / from_rate as f64;
        let output_len = (samples.len() as f64 * ratio) as usize;
        let mut output = Vec::with_capacity(output_len);
        for i in 0..output_len {
            let src_idx = i as f64 / ratio;
            let idx = src_idx as usize;
            let frac = src_idx - idx as f64;
            if idx + 1 < samples.len() {
                output.push(samples[idx] * (1.0 - frac as f32) + samples[idx + 1] * frac as f32);
            } else if idx < samples.len() {
                output.push(samples[idx]);
            }
        }
        return output;
    }

    let mut resampler = FftFixedIn::<f32>::new(
        from_rate as usize,
        to_rate as usize,
        samples.len(),
        chunk_size,
        1,
    )
    .expect("Failed to create resampler");

    let input = vec![samples.to_vec()];
    let output = resampler.process(&input, None).expect("Failed to resample");

    output.into_iter().next().unwrap_or_default()
}
