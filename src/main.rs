mod complex;
mod quantum;

use complex::*;
use quantum::*;
use shor::*;

fn main() 
{
    println!("{}", shor(15));
}