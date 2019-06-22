fn take(vector: Vec<i32>){
    println!("We took {} and {}", vector[10], vector[100])
}

pub fn move_ownership(){
    let mut vec = Vec::new();
    for i in 1..1000 {
        vec.push(i)
    };

    take(vec);

    // This print will throw an error because ownership of vec has been transferred to the take function
    // Which means vec is no longer in scope and cannot be used once its moved
    // println!("{}", vec[10])
}