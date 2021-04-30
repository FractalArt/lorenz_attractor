//! Plot a Lorenz Attractor from Rust.
use ndarray::{array, Array, Ix1};
use std::io::prelude::*;
use structopt::StructOpt;

/// Compute a Lorenz attractor.
#[derive(Debug, StructOpt)]
struct Opt {
    /// The Rayleigh parameter of the Lorenz equations.
    #[structopt(short = "r", long = "rayleigh", default_value = "28.0")]
    r: f64,

    /// The Prandtl number of the Lorenz equations.
    #[structopt(short = "s", long = "sigma", default_value = "10.0")]
    sigma: f64,

    /// The `b` parameter appearing in the Lorenz equations.
    #[structopt(short = "b", long = "b", default_value = "2.666666666666")]
    b: f64,

    /// The time step to use in solving the Lorenz equations using the fourth-order Runge-Kutta method.
    #[structopt(short = "t", long = "delta_t", default_value = "0.001")]
    delta_t: f64,

    /// The number of time steps to iterate in the numerical solution of the Lorenz equations.
    #[structopt(short = "n", long = "number_steps", default_value = "250000")]
    time_steps: usize,

    /// The x-coordinate of the initial condition.
    #[structopt(
        short = "x",
        long = "x0",
        default_value = "1.0",
        allow_hyphen_values = true
    )]
    x0: f64,

    /// The y-coordinate of the initial condition.
    #[structopt(
        short = "y",
        long = "y0",
        default_value = "1.0",
        allow_hyphen_values = true
    )]
    y0: f64,

    /// The z-coordinate of the initial condition.
    #[structopt(
        short = "z",
        long = "z0",
        default_value = "1.0",
        allow_hyphen_values = true
    )]
    z0: f64,
}

fn main() {
    let opt = Opt::from_args();
    let mut initial = array![opt.x0, opt.y0, opt.z0];
    let r = opt.r;
    let sigma = opt.sigma;
    let b = opt.b;
    let delta_t = opt.delta_t;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("./data/points.csv")
        .unwrap();

    for _ in 0..opt.time_steps {
        writeln!(file, "{},{},{}", initial[0], initial[1], initial[2]).unwrap();
        initial = runge_kutta(
            |x| {
                array![
                    sigma * (x[1] - x[0]),
                    r * x[0] - x[1] - x[0] * x[2],
                    x[0] * x[1] - b * x[2]
                ]
            },
            delta_t,
            initial,
        );
    }
}

/// Implementation of fourth order Runge-Kutta Method.
fn runge_kutta<F>(f: F, delta_t: f64, initial: Array<f64, Ix1>) -> Array<f64, Ix1>
where
    F: Fn(&Array<f64, Ix1>) -> Array<f64, Ix1>,
{
    let k1 = f(&initial) * delta_t;
    let k2 = f(&(&initial + &(0.5 * &k1))) * delta_t;
    let k3 = f(&(&initial + &(0.5 * &k2))) * delta_t;
    let k4 = f(&(&initial + &k3)) * delta_t;
    initial + 1. / 6. * (k1 + 2. * k2 + 2. * k3 + k4)
}
