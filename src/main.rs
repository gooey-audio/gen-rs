mod envelope;
mod filter;
mod kick;
mod oscillator;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use kick::{KickParams, KickSynth};
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");

    println!("Output device: {}", device.name()?);

    let config = device.default_output_config()?;
    println!("Default output config: {:?}", config);

    let sample_rate = config.sample_rate().0 as f32;

    let params = KickParams {
        start_frequency: 80.0,
        end_frequency: 45.0,
        pitch_decay_time: 0.08,
        amplitude_attack: 0.001,
        amplitude_decay: 0.4,
        amplitude_sustain: 0.0,
        amplitude_release: 0.0,
        click_level: 0.4,
    };

    let kick_synth = Arc::new(Mutex::new(KickSynth::new(params, sample_rate)));

    let kick_synth_clone = kick_synth.clone();

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                write_data(data, &kick_synth_clone)
            },
            |err| eprintln!("an error occurred on stream: {}", err),
            None,
        )?,
        cpal::SampleFormat::I16 => device.build_output_stream(
            &config.into(),
            move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                write_data_i16(data, &kick_synth_clone)
            },
            |err| eprintln!("an error occurred on stream: {}", err),
            None,
        )?,
        cpal::SampleFormat::U16 => device.build_output_stream(
            &config.into(),
            move |data: &mut [u16], _: &cpal::OutputCallbackInfo| {
                write_data_u16(data, &kick_synth_clone)
            },
            |err| eprintln!("an error occurred on stream: {}", err),
            None,
        )?,
        _ => return Err("unsupported sample format".into()),
    };

    stream.play()?;

    println!("\nDrum synthesizer ready!");
    println!("Commands:");
    println!("  k - trigger kick");
    println!("  q - quit");

    println!("\nKick parameters:");
    println!("  Start frequency: 80 Hz");
    println!("  End frequency: 45 Hz");
    println!("  Pitch decay: 0.08s");
    println!("  Click level: 0.4");
    println!("  Noise level: 0.3");
    println!("  Noise filter cutoff: 2000 Hz");
    println!("  Noise decay: 0.05s");

    use std::io::{self, BufRead};
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                let input = input.trim();
                if input == "q" || input == "quit" {
                    break;
                } else if input == "k" {
                    let mut synth = kick_synth.lock().unwrap();
                    synth.trigger();
                    println!("Kick Triggered!");
                } else {
                    println!("not a key");
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }

    drop(stream);
    println!("Goodbye!");

    Ok(())
}

fn write_data(output: &mut [f32], kick_synth: &Arc<Mutex<KickSynth>>) {
    let mut kick = kick_synth.lock().unwrap();

    for frame in output.chunks_mut(2) {
        let kick_sample = kick.next_sample();

        // later here we'd sum the "mix"
        let mixed = kick_sample;

        for channel in frame.iter_mut() {
            *channel = mixed;
        }
    }
}

// duplicate output functions as above for different sample
fn write_data_i16(output: &mut [i16], kick_synth: &Arc<Mutex<KickSynth>>) {
    let mut kick = kick_synth.lock().unwrap();

    for frame in output.chunks_mut(2) {
        let kick_sample = kick.next_sample();

        let mixed = kick_sample;
        let sample_i16 = (mixed * i16::MAX as f32) as i16;
        for channel in frame.iter_mut() {
            *channel = sample_i16;
        }
    }
}

fn write_data_u16(output: &mut [u16], kick_synth: &Arc<Mutex<KickSynth>>) {
    let mut kick = kick_synth.lock().unwrap();

    for frame in output.chunks_mut(2) {
        let kick_sample = kick.next_sample();

        let mixed = kick_sample;
        let sample_u16 = ((mixed + 1.0) * 0.5 * u16::MAX as f32) as u16;
        for channel in frame.iter_mut() {
            *channel = sample_u16;
        }
    }
}
