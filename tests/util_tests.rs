#[path = "../src/quantum/utils.rs"] mod utils;
use crate::utils::*;

#[cfg(test)]

#[test]
pub fn test_gcd()
{
    assert_eq!(gcd(349, 343), 1);
    assert_eq!(gcd(923, 26), 13);
    assert_eq!(gcd(2, 8), 2);
}

#[test]
pub fn test_modexp()
{
    assert_eq!(modexp(23, 27, 21), 8);
    assert_eq!(modexp(84, 15, 342), 18);
}