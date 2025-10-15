use std::f32::consts::PI;

pub struct Oscillator {
    sample_rate: f32,
    phase: f32,
}

impl Oscillator {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            phase: 0.0,
        }
    }

    pub fn sine(&mut self, frequency: f32) -> f32 {
        let value = (self.phase * 2.0 * PI).sin();
        self.phase += frequency / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        value
    }

    pub fn reset(&mut self) {
        self.phase = 0.0;
    }
}
