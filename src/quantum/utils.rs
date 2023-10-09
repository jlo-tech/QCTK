pub fn bit_length(n: u64) -> u64
{
    return (64 - n.leading_zeros()).into();
}

pub fn modexp(x: u64, a: u64, n: u64) -> u64
{
    let mut r = 1;
    for i in 0..a {
        r = r * x % n;
    }
    return r;
}

pub fn gcd(a: u64, b: u64) -> u64
{
    let mut x = a;
    let mut y = b;
    
    loop {
        if x < y {
            y = y - x;
        }
        if x > y {
            x = x - y;
        }
        if x == y {
            return x;
        }
    }
}

pub fn continued_fraction_representation(numerator: u64, denominator: u64) -> Vec<u64>
{
    let mut v: Vec<u64> = Vec::new();
    
    let mut num = numerator;
    let mut den = denominator;

    let mut d;
    let mut m;
    
    loop
    {
        d = num / den;
        m = num % den;

        num = den;
        den = m;

        v.push(d);

        if den == 0 {
            return v;
        }
    }
}

pub fn continued_fraction_convergents(l: Vec<u64>) -> Vec<(u64, u64)>
{
    let mut v = Vec::new();

    v.push((l[0], 1));
    v.push((1 + l[0] * l[1], l[1]));

    let mut p;
    let mut q;

    for i in 2..l.len()
    {
        p = l[i] * v[i-1].0 + v[i-2].0;
        q = l[i] * v[i-1].1 + v[i-2].1;

        v.push((p, q));
    }

    return v;
}