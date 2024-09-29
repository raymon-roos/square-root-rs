// Approximate square root of floating point number using Newton's Method
// Inspired by tour of Go: https://go.dev/tour/flowcontrol/8

const DELTA: f32 = 1e-8;

fn custom_sqrt(x: f32) -> f32 {
    match x {
        ..0.0 => f32::NAN,
        0.0 => 0.0_f32,
        x => {
            let mut z = 1.0_f32;
            let mut zprev = 0_f32;
            loop {
                if (zprev - z).abs() <= DELTA {
                    break z;
                }

                zprev = z;
                z -= (z * z - x) / (2.0_f32 * z)
            }
        }
    }
}

fn main() {
    println!("{}", custom_sqrt(3.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        assert_eq!(custom_sqrt(3.0), 3.0_f32.sqrt());
    }

    #[test]
    fn handles_zero() {
        assert_eq!(custom_sqrt(0.0), 0.0);
    }

    #[test]
    fn handles_negatives() {
        assert!(custom_sqrt(-3.0).is_nan());
    }
}
