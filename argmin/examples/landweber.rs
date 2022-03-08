// Copyright 2018-2022 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use argmin::core::{Error, Executor, Gradient, ObserverMode, SlogLogger};
use argmin::solver::landweber::Landweber;
use argmin_testfunctions::rosenbrock_2d_derivative;

struct Rosenbrock {}

impl Gradient for Rosenbrock {
    type Param = Vec<f64>;
    type Gradient = Vec<f64>;

    fn gradient(&self, p: &Vec<f64>) -> Result<Vec<f64>, Error> {
        Ok(rosenbrock_2d_derivative(p, 1.0, 100.0))
    }
}

fn run() -> Result<(), Error> {
    // define inital parameter vector
    let init_param: Vec<f64> = vec![1.2, 1.2];
    let operator = Rosenbrock {};

    let iters = 10;
    let solver = Landweber::new(0.001);

    let res = Executor::new(operator, solver)
        .configure(|config| config.param(init_param).max_iters(iters))
        .add_observer(SlogLogger::term(), ObserverMode::Always)
        .run()?;

    // Wait a second (lets the logger flush everything before printing to screen again)
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("{}", res);
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("{}", e);
    }
}