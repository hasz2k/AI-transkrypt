use anyhow::{anyhow, Result};
use rubato::{FftFixedIn, Resampler};
use std::fs::File;
use std::path::Path;
use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::{FormatOptions, TrackType};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

pub const TARGET_SAMPLE_RATE: u32 = 16000;

pub fn decode_to_16k_mono(path: &Path) -> Result<Vec<f32>> {
    let file = File::open(path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }

    let mut format = symphonia::default::get_probe()
    .probe(&hint, mss, FormatOptions::default(), MetadataOptions::default())
    .map_err(|e| anyhow!("Nie można otworzyć pliku audio: {e}"))?;

    let track = format
    .default_track(TrackType::Audio)
    .ok_or_else(|| anyhow!("Brak ścieżki audio w pliku"))?
    .clone();
    let track_id = track.id;

    let audio_params = track
    .codec_params
    .as_ref()
    .and_then(|p| p.audio())
    .ok_or_else(|| anyhow!("Brak parametrów audio"))?;

    let sample_rate = audio_params
    .sample_rate
    .ok_or_else(|| anyhow!("Nieznana częstotliwość próbkowania"))?;

    let mut decoder = symphonia::default::get_codecs()
    .make_audio_decoder(audio_params, &AudioDecoderOptions::default())
    .map_err(|e| anyhow!("Nie można utworzyć dekodera: {e}"))?;

    let mut all_samples: Vec<f32> = Vec::new();

    loop {
        let packet = match format.next_packet() {
            Ok(Some(p)) => p,
            Ok(None) => break,
            Err(e) => return Err(anyhow!("Błąd odczytu pakietu: {e}")),
        };

        if packet.track_id != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(decoded) => {
                let spec = decoded.spec();
                let channels = spec.channels().count();

                let mut frame_buf: Vec<f32> = Vec::new();
                decoded.copy_to_vec_interleaved::<f32>(&mut frame_buf);

                if channels == 1 {
                    all_samples.extend_from_slice(&frame_buf);
                } else {
                    for frame in frame_buf.chunks(channels) {
                        let sum: f32 = frame.iter().sum();
                        all_samples.push(sum / channels as f32);
                    }
                }
            }
            Err(symphonia::core::errors::Error::DecodeError(_)) => continue,
            Err(e) => return Err(anyhow!("Błąd dekodowania: {e}")),
        }
    }

    if all_samples.is_empty() {
        return Err(anyhow!("Plik nie zawiera danych audio"));
    }

    // Jeśli już 16 kHz — zwróć bez resamplingu
    if sample_rate == TARGET_SAMPLE_RATE {
        return Ok(all_samples);
    }

    // Resampling do 16 kHz
    let mut resampler = FftFixedIn::<f32>::new(
        sample_rate as usize,
        TARGET_SAMPLE_RATE as usize,
        1024, // chunk_size_in
        2,    // sub_chunks
        1,    // mono
    )
    .map_err(|e| anyhow!("Nie można utworzyć resamplera: {e}"))?;

    let mut output = Vec::new();
    let mut input_pos = 0;
    let chunk_size = resampler.input_frames_next();

    while input_pos + chunk_size <= all_samples.len() {
        let chunk = &all_samples[input_pos..input_pos + chunk_size];
        let resampled = resampler
        .process(&[chunk], None)
        .map_err(|e| anyhow!("Błąd resamplingu: {e}"))?;
        output.extend_from_slice(&resampled[0]);
        input_pos += chunk_size;
    }

    // Reszta (tail) — dopełnij zerami
    if input_pos < all_samples.len() {
        let tail = &all_samples[input_pos..];
        let mut padded = tail.to_vec();
        padded.resize(chunk_size, 0.0);
        let resampled = resampler
        .process(&[&padded], None)
        .map_err(|e| anyhow!("Błąd resamplingu (tail): {e}"))?;
        output.extend_from_slice(&resampled[0]);
    }

    Ok(output)
}
