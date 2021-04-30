//! Plot a Lorenz Attractor from Rust.
use ndarray::{array, Array, Ix1};
use std::io::prelude::*;

fn main() {
    let mut initial = array![1.0, 1.0, 1.0];
    let r = 28.0;
    let sigma = 10.;
    let b = 8. / 3.;
    let delta_t = 0.001;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("./data/points.csv")
        .unwrap();

    for _ in 0..250_000 {
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
