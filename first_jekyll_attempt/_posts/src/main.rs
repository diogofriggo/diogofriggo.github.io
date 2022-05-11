macro_rules! our_vec {
    ( $constant:expr; $n:expr ) => {
        {
            let mut temp_vec = Vec::with_capacity($n);
            (0..$n).for_each(|_| temp_vec.push($constant));
            temp_vec
        }
    };
}

fn main() {
    let vec = vec![1usize; 5];
    println!("{:?}", vec); // [1, 1, 1, 1, 1]

    let vec = (0..5).map(|_| 1).collect::<Vec<usize>>();
    println!("{:?}", vec); // [1, 1, 1, 1, 1]

    let vec: Vec<usize> = Vec::with_capacity(5);
    println!("{:?}", vec); // []

    let vec: Vec<usize> = our_vec![1; 5];
    println!("{:?}", vec); // [1, 1, 1, 1, 1]

    let vec_of_vecs = our_vec![our_vec![1usize; 5]; 2];
    println!("{:?}", vec_of_vecs); // [[1, 1, 1, 1, 1], [1, 1, 1, 1, 1]]
}