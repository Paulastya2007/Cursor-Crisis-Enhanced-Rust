use std::f32::consts::PI;

pub struct WavGenerator;

impl WavGenerator {
    pub fn generate_beep(frequency: f32, duration: f32, volume: f32) -> Vec<u8> {
        let sample_rate = 44100;
        let num_samples = (sample_rate as f32 * duration) as usize;
        let mut samples = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let time = i as f32 / sample_rate as f32;
            let angle = 2.0 * PI * frequency * time;

            // Envelope (fade in/out)
            let fade_time = 0.011;
            let envelope = if time < fade_time {
                time / fade_time
            } else if time > duration - fade_time {
                (duration - time) / fade_time
            } else {
                1.0
            };

            let sample = (32767.0 * volume * envelope * angle.sin()) as i16;
            samples.push(sample);
        }

        Self::create_wav_from_samples(&samples, sample_rate)
    }

    pub fn generate_noise(duration: f32, volume: f32) -> Vec<u8> {
        let sample_rate = 44100;
        let num_samples = (sample_rate as f32 * duration) as usize;
        let mut samples = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let time = i as f32 / sample_rate as f32;
            let fade_time = 0.01;
            let envelope = if time < fade_time {
                time / fade_time
            } else if time > duration - fade_time {
                (duration - time) / fade_time
            } else {
                1.0
            };

            let sample =
                (32767.0 * volume * envelope * (macroquad::rand::gen_range(-1.0, 1.0))) as i16;
            samples.push(sample);
        }

        Self::create_wav_from_samples(&samples, sample_rate)
    }

    fn create_wav_from_samples(samples: &[i16], sample_rate: u32) -> Vec<u8> {
        let mut wav = Vec::new();
        let data_size = (samples.len() * 2) as u32;
        let file_size = 36 + data_size;

        // RIFF header
        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&file_size.to_le_bytes());
        wav.extend_from_slice(b"WAVE");

        // FMT chunk
        wav.extend_from_slice(b"fmt ");
        wav.extend_from_slice(&16u32.to_le_bytes()); // Chunk size
        wav.extend_from_slice(&1u16.to_le_bytes()); // Audio format (PCM)
        wav.extend_from_slice(&1u16.to_le_bytes()); // Channels (Mono)
        wav.extend_from_slice(&sample_rate.to_le_bytes());
        wav.extend_from_slice(&(sample_rate * 2).to_le_bytes()); // Byte rate
        wav.extend_from_slice(&2u16.to_le_bytes()); // Block align
        wav.extend_from_slice(&16u16.to_le_bytes()); // Bits per sample

        // DATA chunk
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&data_size.to_le_bytes());
        for &sample in samples {
            wav.extend_from_slice(&sample.to_le_bytes());
        }

        wav
    }
}
