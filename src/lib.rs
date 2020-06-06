pub fn eq(lh: f64, rh: f64, margin: f64) -> bool {
    (lh + margin >= rh) & (lh - margin <= rh)
}

pub fn ne(lh: f64, rh: f64, margin: f64) -> bool {
    (lh + margin < rh) | (lh - margin > rh)
}

pub fn gt(lh: f64, rh: f64, margin: f64) -> bool {
    lh > rh + margin
}

pub fn lt(lh: f64, rh: f64, margin: f64) -> bool {
    lh < rh - margin
}

pub fn ge(lh: f64, rh: f64, margin: f64) -> bool {
    lh >= rh - margin
}

pub fn le(lh: f64, rh: f64, margin: f64) -> bool {
    lh <= rh + margin
}

#[cfg(test)]
mod tests {

    #[test]
    fn eq() {
        assert!(super::eq(9.1, 9.1, 0.0));
        assert!(super::eq(8.4, 9.1, 1.0));
        assert!(super::eq(-0.1, -0.3, 0.2));
        assert!(super::eq(-0.1, 0.1, 0.2));
        assert!(!super::eq(0.1, 0.2, 0.01));
    }

    #[test]
    fn ne() {
        assert!(!super::ne(9.1, 9.1, 0.0));
        assert!(!super::ne(8.4, 9.1, 1.0));
        assert!(!super::ne(-0.1, -0.3, 0.2));
        assert!(!super::ne(-0.1, 0.1, 0.2));
        assert!(super::ne(0.1, 0.2, 0.01));
    }

    #[test]
    fn gt() {
        assert!(super::gt(8.1, 8.0, 0.0));
        assert!(super::gt(8.1, 8.0, 0.05));
        assert!(!super::gt(8.1, 8.0, 0.1));
    }

    #[test]
    fn lt() {
        assert!(super::lt(8.0, 8.1, 0.0));
        assert!(super::lt(8.0, 8.1, 0.05));
        assert!(!super::lt(8.0, 8.1, 0.1));
    }

    #[test]
    fn ge() {
        assert!(super::ge(8.1, 8.0, 0.0));
        assert!(super::ge(8.1, 8.0, 0.05));
        assert!(super::ge(8.1, 8.0, 0.1));
        assert!(super::ge(8.0, 8.1, 0.1));
        assert!(!super::ge(7.9, 8.1, 0.1));
    }

    #[test]
    fn le() {
        assert!(super::le(8.0, 8.1, 0.0));
        assert!(super::le(8.0, 8.1, 0.05));
        assert!(super::le(8.0, 8.1, 0.1));
        assert!(super::le(8.1, 8.0, 0.1));
        assert!(!super::le(8.2, 8.0, 0.1));
    }
}
