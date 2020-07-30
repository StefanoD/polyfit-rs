extern crate rand;

#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng};
    use polyfit_rs as poly;
    use std::io::prelude::*;
    use std::fs::File;

    #[test]
    fn simple_parabola_fitting() {
        // f(x) = a*x² + b * x + c
        // c = 1.0, b = 1.1, a = 2.0
        // Order of monomials increases with the vector index
        let parameters = vec![1.0, 1.1, 2.0];
        let mut y_values: Vec<f64> = Vec::with_capacity(210);
        let mut y_values_with_errors: Vec<f64> = Vec::with_capacity(210);
        let mut x_values: Vec<f64> = Vec::with_capacity(210);

        let mut rng = rand::rngs::StdRng::seed_from_u64(5000u64);
        let epsilon = 10.0f64;

        let mut buffer = File::create("signal.csv").ok().unwrap();

        for i in -100..110
        {
            let x = (i as f64) / 10.0;
            let y: f64 = parameters[0] + parameters[1] * x + parameters[2] * x * x;
            let error: f64 = rng.gen_range(-epsilon, epsilon);
            let y_with_error = y + error;
            y_values_with_errors.push(y + error);
            x_values.push(x);
            y_values.push(y);
            buffer.write(x.to_string().as_bytes());
            buffer.write(b";");
            buffer.write(y_with_error.to_string().as_bytes());
            buffer.write(b"\n");
        }

        buffer = File::create("fitted_signal.csv").ok().unwrap();
        let fitted_parameters = poly::polyfit_rs::polyfit(&x_values, &y_values_with_errors, 2).unwrap();
        
        let mut i = 0;
        for x in x_values
        {
            let fitted_y: f64 = fitted_parameters[0] + fitted_parameters[1] * x + fitted_parameters[2] * x * x;
            let error = (fitted_y - y_values[i]).abs();
            assert!(error <= epsilon);
            buffer.write(x.to_string().as_bytes());
            buffer.write(b";");
            buffer.write(fitted_y.to_string().as_bytes());
            buffer.write(b"\n");
            i += 1;
        }
    }
}