use linear_algebra::vector::Vector as vector;
use linear_algebra::vector::knn as knn;
fn main() {
    let k = 1;
    let emma = knn::KnnData::new(
        String::from("action"),
        vector::new([1.0,3.0,5.0,3.0,4.0,2.0])
    );
    let alex = knn::KnnData::new(
        String::from("horror"),
        vector::new([5.0,1.0,1.0,2.0,5.0,2.0])
    );
    let kate = knn::KnnData::new(
        String::from("comedy"),
        vector::new([2.0,2.0,3.0,3.0,2.0,2.0])
    );
    let carl = knn::KnnData::new(
        String::from("action"),
        vector::new([1.0,1.0,4.0,5.0,5.0,2.0])
    );
    let lily = knn::KnnData::new(
        String::from("comedy"),
        vector::new([3.0,3.0,2.0,4.0,1.0,2.0])
    );
    let labelled_data = vec![emma, alex, kate, carl, lily];
    let sean_vector = vector::new([4.0,3.0,4.0,2.0,1.0,2.0]);
    match knn::KnnData::find_knn(k, labelled_data, sean_vector.clone()){
        None => println!("Something went wrong."),
        Some(i) => {
            println!("For k of {}, and for vector : {:#?} ", k, sean_vector);
            println!("the label most appropriate for this target vector is {}", i);}
    }
    


}
