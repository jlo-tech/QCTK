mod complex;
mod quantum;

use complex::*;
use quantum::*;
use shor::*;

use rand::prelude::*;

fn main() {
    
    println!("{:?}", shor(15));
}