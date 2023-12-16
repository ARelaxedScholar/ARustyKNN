mod vector {

    struct Vector<const SIZE: usize> {
        coordinates: [f64; SIZE],
        norm: f64,
    }

    impl<const SIZE: usize> Vector<SIZE> {
        //Instance methods

        /// An instance method to add another vector to the current vector (modifies it)
        ///
        /// # Arguments
        ///
        /// * `other`` -  A reference of another vector of same SIZE
        ///
        /// # Returns nothing, simply modifies the vector in place
        ///
        /// # Examples
        /// let mut v1 = Vector::new([1.0, 2.0]);
        /// let v2 = Vector::new([3.0, 4.0]);
        /// v1.add(v2);
        /// assert_eq!(v1.coordinates, [4.0, 6.0]);
        pub fn add(&mut self, other_vector: Vector<SIZE>) {
            self.coordinates
                .iter_mut()
                .zip(other_vector.coordinates.iter())
                .for_each(|(x_i, &y_i)| *x_i += y_i)
        }

        pub fn scale(&mut self, scalar: f64) {
            self.coordinates
                .iter_mut()
                .for_each(|element| *element *= scalar);
        }

        pub fn dot(&self, other_vector: &Vector<SIZE>) -> f64 {
            self.coordinates
                .iter()
                .zip(other_vector.coordinates.iter())
                .map(|(x_i, y_i)| x_i * y_i)
                .sum()
        }
    }

    impl<const SIZE: usize> Vector<SIZE> {
        //Static methods
        pub fn new(coordinates: [f64; SIZE]) -> Self {
            let norm = coordinates
                .iter()
                .map(|x_i| (x_i).powi(2))
                .sum::<f64>()
                .sqrt();
            Vector { coordinates, norm }
        }
        pub fn compute_distance(vector1: &Vector<SIZE>, vector2: &Vector<SIZE>) -> f64 {
            vector1
                .coordinates
                .iter()
                .zip(vector2.coordinates.iter())
                .map(|(x_i, y_i)| (x_i - y_i).powi(2))
                .sum::<f64>()
                .sqrt()
        }

        pub fn compute_norm(vector: Vector<SIZE>) -> f64 {
            vector
                .coordinates
                .iter()
                .map(|x_i| (x_i).powi(2))
                .sum::<f64>()
                .sqrt()
        }

        pub fn dot_product(some_vector1: &Vector<SIZE>, some_vector2: &Vector<SIZE>) -> f64 {
            some_vector1
                .coordinates
                .iter()
                .zip(some_vector2.coordinates.iter())
                .map(|(&x, &y)| x * y)
                .sum::<f64>()
        }

        pub fn hadamard_product(
            some_vector1: &Vector<SIZE>,
            some_vector2: &Vector<SIZE>,
        ) -> Vector<SIZE> {
            let mut product: [f64; SIZE] = [0.0; SIZE];
            for (i, component) in some_vector1.coordinates.iter().enumerate() {
                product[i] = component * some_vector2.coordinates[i];
            }
            Vector::new(product)
        }

        pub fn add_vectors(
            some_vector1: &Vector<SIZE>,
            some_vector2: &Vector<SIZE>,
        ) -> Vector<SIZE> {
            let mut sum: [f64; SIZE] = [0.0; SIZE];
            for (i, component) in some_vector1.coordinates.iter().enumerate() {
                sum[i] = component + some_vector2.coordinates[i];
            }
            Vector::new(sum)
        }
    }
}
// mod matrix {
//     struct Matrix{
//         number_of_rows:i32,
//         number_of_columns:i32,
//         matrix:Vec<Vec<f64>>
//     }

//     impl Matrix{
//         pub fn new()
//     }
// }
