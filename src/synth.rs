use std::f64::consts::PI;

pub struct Synth {
    pub frequency: f64,
    pub decay: f64,
    pub cutoff: f64,
    pub q: f64,
    pub amount: f64,
    pub gain: f64,
    wave_phase: i64,
    env_time: i64,
    biquad_in1: f64,
    biquad_in2: f64,
    biquad_out1: f64,
    biquad_out2: f64,
}

impl Synth {
    pub fn new() -> Synth {
        Synth {
            wave_phase: 0,
            env_time: -1,
            decay: 1.0,
            frequency: 440.0,
            gain: 0.5,
            cutoff: 1000.0,
            q: 0.8,
            amount: 0.1,
            biquad_in1: 0.0,
            biquad_in2: 0.0,
            biquad_out1: 0.0,
            biquad_out2: 0.0,
        }
    }

    pub fn process(&mut self, out_ptr: *mut f32, size: usize) {
        let wave_buf = self.generate_wave_buf(size);
        let filtered_buf = self.low_pass_filter(wave_buf);
        let amplified_buf = self.amplify(filtered_buf);

        let out_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(out_ptr, size) };
        for i in 0..size {
            out_buf[i] = amplified_buf[i] as f32
        }

        self.env_time = match self.env_time {
            -1 => -1,
            x if (x as f64) < self.decay * 44100.0 => x + size as i64,
            _ => -1
        }
    }

    pub fn env_start(&mut self) {
        self.env_time = 0;
    }

    fn env_val(&self, offset: usize) -> f64 {
        match self.env_time {
            x if x < 0 => 0.0,
            _ => {
                let dur = self.decay * 44100.0;
                let t = (self.env_time + offset as i64) as f64;
                match t {
                    x if x == 0.0 => 0.0,
                    x if x >= dur => 0.0,
                    _ => (dur - t) / dur,
                }
            }
        }
    }

    fn sawtooth_wave(&self, frequency: f64, phase: i64) -> f64 {
        let t = (phase as f64) / 44100.0;
        let t_factor = t * frequency;
        t_factor - t_factor.floor() - 0.5
    }

    fn generate_wave_buf(&mut self, size:usize) -> [f64; 128] {
        let mut output: [f64; 128] = [0.0; 128];
        for i in 0..size {
            let v = self.sawtooth_wave(self.frequency, self.wave_phase);
            output[i] = v;
            self.wave_phase = (self.wave_phase + 1) % 44100;
        }
        output
    }

    fn amplify(&self, input: [f64; 128]) -> [f64; 128] {
        let mut output: [f64; 128] = [0.0; 128];
        for i in 0..input.len() {
            let g = self.gain * self.env_val(i);
            output[i] = input[i] * g;
        }
        output
    }

    // http://vstcpp.wpblog.jp/?page_id=523
    fn low_pass_filter(&mut self, input: [f64; 128]) -> [f64; 128] {
        let mut output: [f64; 128] = [0.0; 128];
        let _q = match self.q {
            x if x == 0.0 => 0.01,
            _ => self.q,
        };

        for i in 0..input.len() {
            let cutoff = self.cutoff + self.amount * 1000.0 * self.env_val(i);

            let omega = 2.0 * PI * cutoff / 44100.0;
            let alpha = omega.sin() / (2.0 * _q);
            let a0 = 1.0 + alpha;
            let a1 = -2.0 * omega.cos();
            let a2 = 1.0 - alpha;
            let b0 = (1.0 - omega.cos()) / 2.0;
            let b1 = 1.0 - omega.cos();
            let b2 = (1.0 - omega.cos()) / 2.0;

            output[i] = b0 / a0 * input[i] + b1 / a0 * self.biquad_in1 + b2 / a0 * self.biquad_in2
                - a1 / a0 * self.biquad_out1
                - a2 / a0 * self.biquad_out2;
            self.biquad_in2 = self.biquad_in1;
            self.biquad_in1 = input[i];
            self.biquad_out2 = self.biquad_out1;
            self.biquad_out1 = output[i];
        }
        output
    }
}
