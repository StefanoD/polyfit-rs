extern crate nalgebra as na;

pub mod polyfit_rs {
    
    type MatrixNN = na::Matrix<f64, na::Dynamic, na::Dynamic, na::VecStorage<f64, na::Dynamic, na::Dynamic>>;
    type ColumnN = na::Matrix<f64, na::Dynamic, na::U1, na::VecStorage<f64, na::Dynamic, na::U1>>;

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

        let mut b = ColumnN::zeros(y_values.len());
        for row in 0..b.len()
        {
            b[(row, 0)] = y_values[row];
        }

        let decomp = na::SVD::new(a, true, true);
        let x = decomp.solve(&b, 0.000000000000000001).unwrap();

        return x.data.into();
    }
}



