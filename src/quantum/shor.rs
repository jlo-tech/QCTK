use rand::Rng;

use crate::utils::*;
use crate::quantum::*;

fn mask(l: u64) -> u64
{
    return (1 << l) - 1;
}

pub fn shor(n: u64) -> u64
{
    if n < 15 {
        panic!("[!] Number is too small!");
    }

    println!("[*] Checking if n is even...");
    if n % 2 == 0 {
        return 2;
    }

    println!("[*] Try to guess factor...");

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(2..n);
    println!("[*] Generate random... {}", x);

    let d = gcd(x, n);
    if d > 1  {
        return d;
    }

    println!("[*] Compute number of needed qubits...");
    let mut m = 0;
    while (1 << m) < (n * n) {
        m += 1;
    }
    let mut k = bit_length(n);

    // Quantum part
    println!("[*] Starting quantum computer...");
    let mut qstate = State::new(m + k, 0);
    // Quantum mod exp
    println!("[*] Performing quantum modular exponentiation...");
    qstate.pow_x_mod_n(m, x, n);
    // Measure second register
    println!("[*] Measure register...");
    for i in 0..k {
        qstate.measure_and_project(i);
    }
    // Quantum fourier transform
    println!("[*] Performing quantum fourier transform...");
    qstate.qft((m + k) as i64, 0);

    println!("[*] Meassure state...");
    // Measure state
    let mut measurement = qstate.measure() & mask(m);
    while measurement == 0 {
        measurement = qstate.measure() & mask(m);
    }
    println!("[*] Measured {}", measurement);

    println!("[*] Starting post processing to determine period...");
    // Classical post processing
    let repr = continued_fraction_representation(measurement, 1 << m);
    let conv = continued_fraction_convergents(repr.clone());

    println!("[*] Compute continued fraction representation... {:?}", repr.clone());
    println!("[*] Compute continued fraction convergents... {:?}", conv.clone());

    // check periods
    let mut period: Option<u64> = None;
    for i in (0..conv.len()).rev() {
        if modexp(x, conv[i].1, n) == 1 && conv[i].1 > 1 {
            period = Some(conv[i].1);
        }
    }

    // check if period was found
    if period.is_none() {
        println!("No period found, try again!");
        return 1;
    } else {
        if period.unwrap() % 2 == 1 {
            println!("No period found, try again!");
            return 1;
        }
    }

    println!("[*] Compute factor...");
    // Compute factor
    let factor = gcd(n, x.pow((period.unwrap() / 2) as u32) - 1);

    if factor > 1 && factor < n {
        println!("[*] Factor: {}", factor);
        return factor;
    }

    println!("[-] Computing the factor failed, try again!");
    return 1;
}