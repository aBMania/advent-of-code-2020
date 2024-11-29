use num_traits::int::PrimInt;
use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq, Eq)]
pub struct SolveCrtError;

pub fn euclide<T>(a: T, b: T) -> (T, T, T)
where
    T: PrimInt + Display + Debug,
{
    let (mut r, mut u, mut v, mut r_prime, mut u_prime, mut v_prime) =
        (a, T::one(), T::zero(), b, T::zero(), T::one());

    while r_prime != T::zero() {
        let q = r / r_prime;
        (r, u, v, r_prime, u_prime, v_prime) = (
            r_prime,
            u_prime,
            v_prime,
            r - q * r_prime,
            u - q * u_prime,
            v - q * v_prime,
        );
    }

    (r, u, v)
}

// We are assuming all n_i are co-primes
pub fn solve_crt<T>(a_i: &[T], n_i: &[T]) -> Result<(T, T), SolveCrtError>
where
    T: PrimInt + Display + Debug,
{
    if a_i.len() != n_i.len() {
        return Err(SolveCrtError);
    }

    let n = n_i.iter().fold(T::one(), |acc: T, x| acc * *x);

    let solution = n_i
        .iter()
        .map(|&n_i| {
            let n_hat = n / n_i;
            let (_, u, _) = euclide(n_hat, n_i);
            u * n_hat
        })
        .zip(a_i)
        .fold(T::zero(), |acc: T, (a, &e)| acc + a * e);

    let mut solution = solution % n;

    if solution < T::zero() {
        solution = solution + n
    }
    Ok((solution, n))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_euclide() {
        let result = euclide(120, 23);

        assert_eq!(result, (1, -9, 47));
    }

    #[test]
    fn test_extended_gcd() {
        for i in 1..100 {
            for j in (i + 1)..100 {
                let (gcd, x, y) = euclide(i, j);
                assert_eq!(i % gcd, 0);
                assert_eq!(j % gcd, 0);
                assert_eq!(i * x + j * y, gcd);
            }
        }
    }

    #[test]
    fn test_solve_crt() {
        let result = solve_crt(vec![2, 3, 2].as_ref(), vec![3, 5, 7].as_ref());

        assert_eq!(result, Ok((23, 105)));
    }
}
