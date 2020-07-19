extern crate nalgebra as na;

pub mod polyfit_rs {
    
    type MatrixNN = na::Matrix<f64, na::Dynamic, na::Dynamic, na::VecStorage<f64, na::Dynamic, na::Dynamic>>;

    /// @param x_values The x-values
    /// @param y_values The y-values
    /// @param polynomial_degree The order of the polynomial. I. e. 2 for a parabola.
    /// @return Order of monomials increases with the vector index
    pub fn polyfit<'a>(x_values : &'a [f64], y_values : &'a [f64], polynomial_degree: usize) -> Vec<f64>
    {
        let number_of_columns = polynomial_degree + 1;
        let number_of_rows = x_values.len();
        let mut a = MatrixNN::zeros(number_of_rows, number_of_columns);

        for row in 0..number_of_rows 
        {
            // First column is always 1
            a[(row, 0)] = 1.0;
            let x = x_values[row];
    
            for col in 1..number_of_columns
            {
                a[(row, col)] = x.powf(col as f64);
            }
        }

        let b = na::DVector::from_row_slice(y_values);

        let decomp = na::SVD::new(a, true, true);
        let x = decomp.solve(&b, 0.000000000000000001).unwrap();

        return x.data.into();
    }
}



