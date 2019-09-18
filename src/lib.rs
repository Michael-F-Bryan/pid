#![no_std]

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pid {
    k_i: f32,
    k_d: f32,
    k_p: f32,
    set_point: f32,
    cummulative_error: f32,
    last_error: f32,
}

impl Pid {
    pub const fn new(k_i: f32, k_p: f32, k_d: f32) -> Self {
        Pid {
            k_i,
            k_p,
            k_d,
            set_point: 0.0,
            cummulative_error: 0.0,
            last_error: 0.0,
        }
    }

    pub const fn proportional(&self) -> f32 { self.k_p }

    pub const fn integral(&self) -> f32 { self.k_i }

    pub const fn derivative(&self) -> f32 { self.k_d }

    pub const fn set_point(&self) -> f32 { self.set_point }

    pub fn set_derivative(&mut self, k_d: f32) { self.k_d = k_d; }

    pub fn set_proportional(&mut self, k_p: f32) { self.k_p = k_p; }

    pub fn set_integral(&mut self, k_i: f32) { self.k_i = k_i; }

    pub fn set_set_point(&mut self, set_point: f32) {
        self.set_point = set_point;
    }

    pub fn with_proportional(&mut self, k_p: f32) -> &mut Self {
        self.set_proportional(k_p);
        self
    }

    pub fn with_integral(&mut self, k_i: f32) -> &mut Self {
        self.set_integral(k_i);
        self
    }

    pub fn with_derivative(&mut self, k_d: f32) -> &mut Self {
        self.set_derivative(k_d);
        self
    }

    pub fn with_set_point(&mut self, desired_value: f32) -> &mut Self {
        self.set_point = desired_value;
        self
    }
}

impl Pid {
    pub fn compute(&mut self, input: f32, delta_time: f32) -> f32 {
        // compute all the working error variables
        let error = self.set_point - input;
        self.cummulative_error += error * delta_time;
        let delta_error = (error - self.last_error) / delta_time;

        // Remember some variables for next time
        self.last_error = error;

        // Compute the PID output
        self.k_p * error
            + self.k_i * self.cummulative_error
            + self.k_d * delta_error
    }
}
