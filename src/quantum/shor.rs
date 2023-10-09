use rand::Rng;

use crate::utils::*;
use crate::quantum::*;

pub fn shor(n: u64) -> u64
{
    if n % 2 == 0 {
        return 2;
    }

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(1..n);

    let d = gcd(x, n);
    if d > 1  {
        return d;
    }

    let mut m = 0;
    while (1 << m) < (n * n) {
        m += 1;
    }

    let mut k = bit_length(n);

    // Quantum part
    let mut qstate = State::new(m + k, 0);

    // Quantum mod exp
    qstate.pow_x_mod_n(m, x, n);
    // Measure second register
    for i in 0..k {
        qstate.measure_and_project(i);
    }
    // Quantum fourier transform
    qstate.iqft(m as i64, k as i64);
    // Measure state
    let measurement = qstate.measure();

    // Classical post processing
    let repr = continued_fraction_representation(measurement, 1 << m);
    let conv = continued_fraction_convergents(repr);

    // check periods
    let mut period: Option<u64> = None;
    for i in 0..conv.len() {
        if modexp(x, conv[i].1, n) == 1 {
            period = Some(conv[i].1);
        }
    }

    // check if period was found
    if period.is_none() {
        println!("No period found");
        return 1;
    } else {
        if period.unwrap() % 2 == 1 {
            println!("No period found");
            return 1;
        }
    }

    // Compute factor
    let factor = gcd(n, x.pow((period.unwrap() / 2) as u32) - 1);

    return factor;
}