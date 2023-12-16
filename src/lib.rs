mod vector {
    #[derive(Clone, Debug)]
    pub struct Vector<const SIZE: usize> {
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

    mod knn {
        use crate::vector::Vector;
        #[derive(Clone, Debug)]
        struct KnnData<const SIZE: usize>
        {
            label: String,
            data: Vector<SIZE>,
            distance_from_target: f64,
        }

        impl<const SIZE: usize> PartialEq for KnnData<SIZE> {
            fn eq(&self, other: &Self) -> bool {
                self.distance_from_target == other.distance_from_target
            }
        }

        impl<const SIZE: usize> Eq for KnnData<SIZE> {}

        impl<const SIZE: usize> PartialOrd for KnnData<SIZE> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                other
                    .distance_from_target
                    .partial_cmp(&self.distance_from_target)
            }
        }

        impl<const SIZE: usize> Ord for KnnData<SIZE> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
            }
        }
        impl<const SIZE: usize> KnnData<SIZE> {
            pub fn default() -> Self {
                let origin: [f64; SIZE] = [0.0; SIZE];
                KnnData {
                    label: String::from("Unitialized"),
                    data: Vector::new(origin),
                    distance_from_target: 0.0,
                }
            }
            pub fn new(label: String, data: Vector<SIZE>, distance_from_target: f64) -> Self {
                KnnData {
                    label,
                    data,
                    distance_from_target,
                }
            }
            pub fn find_knn(
                k: usize,
                mut labelled_data: Vec<KnnData<SIZE>>,
                target: Vector<SIZE>,
            ) -> Option<String> {
                let mut k_nearest_neighbors =
                    std::collections::BinaryHeap::<KnnData<SIZE>>::with_capacity(k);
                //First find KNN
                labelled_data.iter_mut().for_each(|current_element| {
                    let distance =
                        crate::vector::Vector::compute_distance(&current_element.data, &target);
                    current_element.distance_from_target = distance;
                    //If the heap is not filled to capacity
                    if k_nearest_neighbors.len() < k {
                        k_nearest_neighbors.push(current_element.clone());
                    }
                    //If I find something closer than max (I remove the max)
                    else if let Some(farthest_nearest_neighbor) =
                        k_nearest_neighbors.peek().clone()
                    {
                        if distance < farthest_nearest_neighbor.distance_from_target {
                            k_nearest_neighbors.pop();
                            k_nearest_neighbors.push(current_element.clone())
                        }
                    }
                });
                //Then compute the mode for KNN O(k)
                let mut k_nearest_number_mode =
                    std::collections::HashMap::<String, f64>::with_capacity(k);
                k_nearest_neighbors.iter().for_each(|nth_nearest_neighbor| {
                    let mode = k_nearest_number_mode
                        .entry(nth_nearest_neighbor.clone().label)
                        .or_insert(0.0);
                    let distance = if nth_nearest_neighbor.distance_from_target != 0.0 {
                        nth_nearest_neighbor.distance_from_target
                    } else {
                        1.0
                    };
                    *mode += 1.0 / distance;
                });

                //Finally return the label with the biggest
                //(if multiple label have the same mode (unlikely)) we just return the first
                k_nearest_number_mode
                    .iter()
                    .max_by(|key1, key2| match key1.1.partial_cmp(key2.1) {
                        Some(order) => order,
                        None => std::cmp::Ordering::Equal,
                    })
                    .map(|(label, _)| label.clone())
            }
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
