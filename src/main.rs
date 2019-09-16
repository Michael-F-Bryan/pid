use core::time::Duration;

pub struct Pid {
    k_i: f32,
    k_d: f32,
    k_p: f32,
    set_point: f32,
    cummulative_error: f32,
    last_error: f32,
    last_tick: Duration,
}

impl Pid {
    pub fn compute(&mut self, input: f32, now: Duration) -> f32 {
        let dt = now - self.last_tick;
        let dt = dt.as_secs_f32();

        // compute all the working error variables
        let error = self.set_point - input;
        self.cummulative_error += error * dt;
        let delta_error = (error - self.last_error) / dt;

        // Remember some variables for next time
        self.last_error = error;
        self.last_tick = now;

        // Compute the PID output
        self.k_p * error
            + self.k_i * self.cummulative_error
            + self.k_d * delta_error
    }
}

fn main() {
    println!("Hello, world!");
}
