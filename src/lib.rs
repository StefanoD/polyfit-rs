extern crate nalgebra as na;

pub mod polyfit_rs {

    /// @param x_values The x-values
    /// @param y_values The y-values
    /// @param polynomial_degree The degree of the polynomial. I. e. 2 for a parabola.
    /// @return Degree of monomials increases with the vector index
    pub fn polyfit<T: na::RealField>(x_values : &[T], y_values : &[T], polynomial_degree: usize) -> Vec<T>
    {
        let number_of_columns = polynomial_degree + 1;
        let number_of_rows = x_values.len();
        let mut a = na::DMatrix::zeros(number_of_rows, number_of_columns);

        for row in 0..number_of_rows 
        {
            // First column is always 1
            a[(row, 0)] = T::one();
            let x = x_values[row];
    
            for col in 1..number_of_columns
            {
                a[(row, col)] = x.powf(na::convert(col as f64));
            }
        }

        let b = na::DVector::from_row_slice(y_values);

        let decomp = na::SVD::new(a, true, true);
        let x = decomp.solve(&b, na::convert(0.000000000000000001)).unwrap();

        return x.data.into();
    }
}



