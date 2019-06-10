pub enum Examples {
    IfLet,
    Loop,
    While,
    For,
    For_Range,
}

pub fn if_let(con: bool){
    println!("Because if is an expression, we can use it on the right side of a let statement");

    let number = if con {
        20
    } else {
        30
    };

    println!("Result: {}", number);
}


pub fn loopy_job(){

    let mut test_val: i32 = 0;

    let result = loop {
        test_val += 1;
        if(test_val > 10){
            break test_val;
        }
    };
    println!("loop result {}", result);
}

pub fn while_job(){
    let mut test_val: i32 = 4;

    while test_val > 1 {
        test_val -= 1;
        println!("{}" ,test_val);
    };

    println!("Lanuch");

}

pub fn for_job(){
    let test_array = ["Hello", "There", "I", "Am", "A", "Funky", "Array"];

    for word in test_array.iter() {
        println!("{}", word)
    }

}

pub fn for_range_job(){
    for num in (1..4).rev() {
        println!("{}", num)
    } 
}