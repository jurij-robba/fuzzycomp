/// Reexporting assert_approx_eq macro from assert_approx_eq
pub use assert_approx_eq::assert_approx_eq;

/// Equal within margin (equal enough)
///
/// Compares two f64 with a given margin. Returns true if
/// they are equal enough.
///
/// ```
/// if(fuzzycomp::eq(std::f64::consts::PI, 3.0, 0.2)) {
///     println!("Precision engineering!");
/// }
/// ```
pub fn eq<T>(lh: T, rh: T, margin: T) -> bool
where
    T: PartialOrd + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    (rh <= (lh + margin)) & (rh >= (lh - margin))
}

/// Not equal within margin (unequal enough)
///
/// Compares two f64 with a given margin. Returns true if
/// they are unequal enough.
///
/// ```
/// if(fuzzycomp::ne(std::f64::consts::PI, 3.0, 0.1)) {
///     println!("We need better approximation!");
/// }
/// ```
pub fn ne<T>(lh: T, rh: T, margin: T) -> bool
where
    T: PartialOrd + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    (lh + margin < rh) | (lh - margin > rh)
}

/// Surely greater (greater enough)
///
/// Compares two f64 with a given margin. Returns true if
/// first is obviously greater (despite margins).
///
/// ```
/// if(fuzzycomp::gt(std::f64::consts::PI, 3.0, 0.1)) {
///     println!("π is more than 3!");
/// }
/// ```
pub fn gt<T>(lh: T, rh: T, margin: T) -> bool
where
    T: PartialOrd + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    lh > (rh + margin)
}

/// Surely less (less enough)
///
/// Compares two f64 with a given margin. Returns true if
/// first is obviously lesser (despite margins).
///
/// ```
/// if(fuzzycomp::lt(std::f64::consts::PI, 4.0, 0.5)) {
///     println!("π is a lot less than 3!");
/// }
/// ```
pub fn lt<T>(lh: T, rh: T, margin: T) -> bool
where
    T: PartialOrd + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    lh < (rh - margin)
}

/// Not surely smaller (at least equal)
///
/// Compares two f64 with a given margin. Returns true if
/// first is not obviously lesser (despite margins).
///
/// ```
/// if(fuzzycomp::ge(1.0, 0.0, 0.1)) {
///     println!("1.0 is a at least 0, give or take!");
/// }
/// ```
pub fn ge<T>(lh: T, rh: T, margin: T) -> bool
where
    T: PartialOrd + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    lh >= (rh - margin)
}

/// Not surely larger (at most equal)
///
/// Compares two f64 with a given margin. Returns true if
/// first is not obviously lesser (despite margins).
///
/// ```
/// if(fuzzycomp::le(0.0, 1.0, 0.1)) {
///     println!("0 is not even 1!");
/// }
/// ```
pub fn le<T>(lh: T, rh: T, margin: T) -> bool
where
    T: PartialOrd + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    lh <= (rh + margin)
}

#[cfg(test)]
mod tests {

    #[test]
    fn eq() {
        assert!(super::eq(std::f64::consts::PI, 3.0, 0.2));
        assert!(super::eq(9.1, 9.1, 0.0));
        assert!(super::eq(8.4, 9.1, 1.0));
        assert!(super::eq(-0.1, -0.3, 0.2));
        assert!(super::eq(-0.1, 0.1, 0.2));
        assert!(!super::eq(0.1, 0.2, 0.01));
        assert!(super::eq(1, 1, 0));
    }

    #[test]
    fn ne() {
        assert!(!super::ne(9.1, 9.1, 0.0));
        assert!(!super::ne(8.4, 9.1, 1.0));
        assert!(!super::ne(-0.1, -0.3, 0.2));
        assert!(!super::ne(-0.1, 0.1, 0.2));
        assert!(super::ne(0.1, 0.2, 0.01));
        assert!(super::ne(1, 3, 1));
    }

    #[test]
    fn gt() {
        assert!(super::gt(8.1, 8.0, 0.0));
        assert!(super::gt(8.1, 8.0, 0.05));
        assert!(!super::gt(8.1, 8.0, 0.1));
        assert!(super::gt(10, 8, 1));
    }

    #[test]
    fn lt() {
        assert!(super::lt(8.0, 8.1, 0.0));
        assert!(super::lt(8.0, 8.1, 0.05));
        assert!(!super::lt(8.0, 8.1, 0.1));
        assert!(super::lt(8, 10, 1));
    }

    #[test]
    fn ge() {
        assert!(super::ge(8.1, 8.0, 0.0));
        assert!(super::ge(8.1, 8.0, 0.05));
        assert!(super::ge(8.1, 8.0, 0.1));
        assert!(super::ge(8.0, 8.1, 0.1));
        assert!(!super::ge(7.9, 8.1, 0.1));
        assert!(super::ge(8, 7, 1));
    }

    #[test]
    fn le() {
        assert!(super::le(8.0, 8.1, 0.0));
        assert!(super::le(8.0, 8.1, 0.05));
        assert!(super::le(8.0, 8.1, 0.1));
        assert!(super::le(8.1, 8.0, 0.1));
        assert!(!super::le(8.2, 8.0, 0.1));
        assert!(super::le(8, 8, 1));
    }
}
