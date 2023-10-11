#[path = "../src/complex/mod.rs"] mod complex;
use crate::complex::Complex;

#[cfg(test)]

#[test]
pub fn test_add()
{
    assert_eq!(Complex::new(1.0, 2.0).add(Complex::new(2.0, 1.0)), Complex::new(3.0, 3.0));
    assert_eq!(Complex::new(-1.0, -2.0).add(Complex::new(2.0, 1.0)), Complex::new(1.0, -1.0));
}

#[test]
pub fn test_mul()
{
    assert_eq!(Complex::new(8.0, -5.0).mul(Complex::new(3.0, -2.0)), Complex::new(14.0, -31.0));
}

#[test]
pub fn test_conj()
{
    assert_eq!(Complex::new(1.0, 1.0).conj(), Complex::new(1.0, -1.0));
}

#[test]
pub fn test_abd_squared()
{
    assert_eq!(Complex::new(2.0, 2.0).abs_squared(), 8.0);
    assert_eq!(Complex::new(-2.0, -2.0).abs_squared(), 8.0);
}

#[test]
pub fn test_exp()
{
    let e = Complex::new(2.0, -3.0).exp();
    
    assert!(e.real - (-7.31511009) < 0.0001);
    assert!(e.imag - (-1.04274366) < 0.0001);
}