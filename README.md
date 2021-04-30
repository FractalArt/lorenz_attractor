# Lorentz Attractor

Generate a plot of the [Lorenz attractor](https://en.wikipedia.org/wiki/Lorenz_attractor) using a combination of [`Rust`](https://www.rust-lang.org/) and [`Python`](https://www.python.org/).

![lorenz-attractor](data/attractor.png)

******

## Building and Usage

To build the program, both `Rust` and `Python` need to be installed. In the case of `Rust`, the simplest way is to install `rustup` as described on the [website](https://www.rust-lang.org/tools/install). Once this is done, the computation of the Lorenz attractor can be
started by running

```sh
> cargo r --release
```

which will output the file `data/points.csv`.

In order to produce the plot as it is shown on the top of this file, the `Python` script `plot.py` is used as follows

```sh
> python3 plot.py
```

which will output the the file [`data/attractor.png`](data/attractor.png).

## TODO

* ~~Add a command line interface to set initial condition and the parameters of the Lorenz equations dynamically.~~
* To the plotting entirely in `Rust`. If someone knows how this can be done, I'd be glad to hear suggestions. I know that there are plotting libraries out there, but so far, I have not found one producing results that are as good as those obtained using `matplotlib`.

## Licensing

`lorenz_attractor` is made available under the terms of either the MIT License or the Apache License 2.0, at your option.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files for license details.