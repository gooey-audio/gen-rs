pub struct ADSREnvelope {
    attack_time: f32,
    decay_time: f32,
    sustain_level: f32,
    release_time: f32,
    sample_rate: f32,
    current_time: f32,
    is_active: bool,
    release_start_level: f32,
    release_start_time: f32,
}

impl ADSREnvelope {
    pub fn new(
        attack_time: f32,
        decay_time: f32,
        sustain_level: f32,
        release_time: f32,
        sample_rate: f32,
    ) -> Self {
        Self {
            attack_time,
            decay_time,
            sustain_level,
            release_time,
            sample_rate,
            current_time: 0.0,
            is_active: false,
            release_start_level: 0.0,
            release_start_time: 0.0,
        }
    }

    pub fn trigger(&mut self) {
        self.current_time = 0.0;
        self.is_active = true;
    }

    // pub fn release(&mut self) {
    //     if self.is_active {
    //         self.release_start_level = self.get_value();
    //         self.release_start_time = self.current_time;
    //         self.is_active = false;
    //     }
    // }

    pub fn get_value(&self) -> f32 {
        if self.is_active {
            if self.current_time < self.attack_time {
                self.current_time / self.attack_time
            } else if self.current_time < self.attack_time + self.decay_time {
                let decay_progress = (self.current_time - self.attack_time) / self.decay_time;
                1.0 - (1.0 - self.sustain_level) * decay_progress
            } else {
                self.sustain_level
            }
        } else {
            let release_time = self.current_time - self.release_start_time;
            if release_time < self.release_time {
                self.release_start_level * (1.0 - release_time / self.release_time)
            } else {
                0.0
            }
        }
    }

    pub fn advance(&mut self) {
        self.current_time += 1.0 / self.sample_rate;
    }

    pub fn is_finished(&self) -> bool {
        !self.is_active && (self.current_time - self.release_start_time) >= self.release_time
    }
}

pub struct ExponentialEnvelope {
    start_value: f32,
    end_value: f32,
    duration: f32,
    sample_rate: f32,
    pub current_time: f32,
}

impl ExponentialEnvelope {
    pub fn new(start_value: f32, end_value: f32, duration: f32, sample_rate: f32) -> Self {
        Self {
            start_value,
            end_value,
            duration,
            sample_rate,
            current_time: 0.0,
        }
    }

    pub fn get_value(&self) -> f32 {
        if self.current_time >= self.duration {
            return self.end_value;
        }

        let progress = self.current_time / self.duration;
        let exp_progress = (-5.0 * progress).exp();
        self.start_value + (self.end_value - self.start_value) * (1.0 - exp_progress)
    }

    pub fn advance(&mut self) {
        self.current_time += 1.0 / self.sample_rate;
    }

    pub fn reset(&mut self) {
        self.current_time = 0.0;
    }
}
