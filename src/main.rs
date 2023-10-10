mod complex;
mod quantum;

use complex::*;
use quantum::*;
use shor::*;

use rand::prelude::*;

fn main() {

    /*
    let mut s = quantum::State::new(12, 0);
    s.pow_x_mod_n(8, 13, 15);
    println!("Qubit 0: {}", s.measure_and_project(0));
    println!("Qubit 1: {}", s.measure_and_project(1));
    println!("Qubit 2: {}", s.measure_and_project(2));
    println!("Qubit 3: {}", s.measure_and_project(3));
    s.qft(12, 0);
    s.pretty_probabilities();
    */

    println!("{}", shor(15));
}