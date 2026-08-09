use std::f64::consts::PI;

/// Converts a floating-point angle from degrees to radians.
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Converts a floating-point angle from radians to degrees.
pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / PI
}

/// Wraps a floating-point angle from -PI to PI in radians.
pub fn wrap_radians(radians: f64) -> f64 {
    let x = (radians + PI).rem_euclid(2.0 * PI) - PI;
    if x <= -PI {PI} else {x}
}

/// Returns the hypotenuse given the two leg lengths.
pub fn hypotenuse(a: f64, b: f64) -> f64 {
    (a * a + b * b).sqrt()
}

/// Unit tests for all utility methods
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degrees_to_radians() {
        let degrees = 180.0;
        let radians = degrees_to_radians(degrees);
        let expected = PI;
        assert!((radians - expected).abs() < 1e-10);
    }

    #[test]
    fn test_radians_to_degrees() {
        let radians = PI;
        let degrees = radians_to_degrees(radians);
        let expected = 180.0;
        assert!((degrees - expected).abs() < 1e-10);
    }

    #[test]
    fn test_wrap_radians() {
        let radians = 8.0 * PI / 3.0;
        let wrapped_radians = wrap_radians(radians);
        let expected = 2.0 * PI / 3.0;
        assert!((wrapped_radians - expected).abs() < 1e-10);
    }

    #[test]
    fn test_hypotenuse() {
        let a = 3.0;
        let b = 4.0;
        let hypotenuse_length = hypotenuse(a, b);
        let expected = 5.0;
        assert!((hypotenuse_length - expected).abs() < 1e-10);
    }
}