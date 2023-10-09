mod complex;
mod quantum;

use complex::*;
use quantum::*;

use rand::prelude::*;

fn main() {
    
    //println!("{:?}", random::<f64>());
    //println!("{:?}", random::<u64>());

    let mut s = State::new(4, 1);

    s.qft(2, 0);

    s.pretty_amplitudes();
}