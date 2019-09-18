use gnuplot::{Caption, Figure};
use pid::Pid;
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();
    println!("{:?}", args);

    // initialize the PID controller
    let mut pid = Pid::new(args.integral, args.proportional, args.derivative);
    pid.with_set_point(args.target);

    let mut data_points = Vec::new();
    let mut t = 0.0;
    let mut position: f32 = 0.0;
    data_points.push((t, position));

    while t < args.max_time {
        let new_speed = pid.compute(position, args.time_step);

        position += new_speed * args.time_step;
        data_points.push((t, position));
        t += args.time_step;
    }

    let mut fig = Figure::new();
    let xs = data_points.iter().map(|(x, _)| x);
    let ys = data_points.iter().map(|(_, y)| y);
    let label = format!(
        "p={}, i={}, d={}",
        args.proportional, args.integral, args.derivative
    );
    fig.axes2d().lines(xs, ys, &[Caption(&label)]);
    fig.show();
}

#[derive(Debug, StructOpt)]
pub struct Args {
    /// The integral coefficient.
    #[structopt(short, long, default_value = "0.0")]
    integral: f32,
    /// The proportional coefficient.
    #[structopt(short, long, default_value = "0.1")]
    proportional: f32,
    /// The derivative coefficient.
    #[structopt(short, long, default_value = "0.0")]
    derivative: f32,
    /// The PID loop's set point.
    #[structopt(short, long, default_value = "1.0")]
    target: f32,
    /// The amount of time between measurements.
    #[structopt(long, default_value = "0.1")]
    time_step: f32,
    /// The maximum amount of "time" to simulate, used to avoid running
    /// forever.
    #[structopt(long, default_value = "10.0")]
    max_time: f32,
}
