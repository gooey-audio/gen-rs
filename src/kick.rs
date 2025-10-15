use crate::envelope::{ADSREnvelope, ExponentialEnvelope};
use crate::oscillator::{Oscillator};

pub struct KickParams {
    pub start_frequency: f32,
    pub end_frequency: f32,
    pub pitch_decay_time: f32,
    pub amplitude_attack: f32,
    pub amplitude_decay: f32,
    pub amplitude_sustain: f32,
    pub amplitude_release: f32,
    pub click_level: f32,
}

impl Default for KickParams {
    fn default() -> Self {
        Self {
            start_frequency: 150.0,
            end_frequency: 50.0,
            pitch_decay_time: 0.1,
            amplitude_attack: 0.001,
            amplitude_decay: 0.3,
            amplitude_sustain: 0.1,
            amplitude_release: 0.1,
            click_level: 0.3,
        }
    }
}

pub struct KickSynth {
    oscillator: Oscillator,
    pitch_envelope: ExponentialEnvelope,
    amplitude_envelope: ADSREnvelope,
    click_oscillator: Oscillator,
    params: KickParams,
    sample_rate: f32,
    is_playing: bool,
}

impl KickSynth {
    pub fn new(params: KickParams, sample_rate: f32) -> Self {
        Self {
            oscillator: Oscillator::new(sample_rate),
            pitch_envelope: ExponentialEnvelope::new(
                params.start_frequency,
                params.end_frequency,
                params.pitch_decay_time,
                sample_rate,
            ),
            amplitude_envelope: ADSREnvelope::new(
                params.amplitude_attack,
                params.amplitude_decay,
                params.amplitude_sustain,
                params.amplitude_release,
                sample_rate,
            ),
            click_oscillator: Oscillator::new(sample_rate),
            params,
            sample_rate,
            is_playing: false,
        }
    }

    pub fn trigger(&mut self) {
        self.oscillator.reset();
        self.click_oscillator.reset();
        self.pitch_envelope.reset();
        self.amplitude_envelope.trigger();
        self.is_playing = true;
    }

    // pub fn release(&mut self) {
    //     self.amplitude_envelope.release();
    // }

    pub fn next_sample(&mut self) -> f32 {
        if !self.is_playing {
            return 0.0;
        }

        let frequency = self.pitch_envelope.get_value();
        let amplitude = self.amplitude_envelope.get_value();
        
        let main_signal = self.oscillator.sine(frequency);
        
        let click_signal = self.click_oscillator.sine(frequency * 8.0) * self.params.click_level;
        let click_decay = (-self.pitch_envelope.current_time * 50.0).exp();
        let click = click_signal * click_decay;
        
        // sum the components in the "mix"
        // later would like to make mixer story for each part
        let output = (main_signal + click) * amplitude;

        self.pitch_envelope.advance();
        self.amplitude_envelope.advance();

        if self.amplitude_envelope.is_finished() {
            self.is_playing = false;
        }

        output * 0.5
    }

    // pub fn set_params(&mut self, params: KickParams) {
  
    // }
}
