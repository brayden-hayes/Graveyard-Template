use std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / PI
}

pub fn clamp_radians(radians: f64) -> f64 {
    let x = (radians + PI).rem_euclid(2.0 * PI) - PI;
    if x <= -PI {PI} else {x}
}


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
    fn test_clamp_radians() {
        let radians = 8.0 * PI / 3.0;
        let clamped_radians = clamp_radians(radians);
        let expected = 2.0 * PI / 3.0;
        assert!((clamped_radians - expected).abs() < 1e-10);
    }
}