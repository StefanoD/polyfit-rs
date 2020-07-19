extern crate polyfit_rs;

#[cfg(test)]
mod tests {
    use polyfit_rs as poly;

    #[test]
    fn simple_parabola_fitting() {
        // f(x) = a*x² + b * x + c
        // c = 1.0, b = 1.1, a = 2.0
        // Order of monomials increases with the vector index
        let parameters = vec![1.0, 1.1, 2.0];
        let mut y_values: Vec<f64> = Vec::with_capacity(21);
        let mut x_values: Vec<f64> = Vec::with_capacity(21);

        for i in -10..11
        {
            let x = i as f64;
            let y: f64 = parameters[0] + parameters[1] * x + parameters[2] * x * x;
            x_values.push(x);
            y_values.push(y);
        }

        let parameters = poly::polyfit_rs::polyfit(&x_values, &y_values, 2);
        let epsilon = 0.0001;
        let mut i = 0;
        for x in x_values
        {
            let fitted_y: f64 = parameters[0] + parameters[1] * x + parameters[2] * x * x;
            let error = (fitted_y - y_values[i]).abs();
            assert!(error <= epsilon);
            i += 1;
        }
    }
}