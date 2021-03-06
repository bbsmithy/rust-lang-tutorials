mod moving_ownership_example;
mod copying_example;
mod slice_type_example;

fn borrowing_example(){
    //Create new String stored on the heap
    let tasty_string = String::from("Hullo");
    //Only one variable can own a piece of data at a time, so here we are borrowing a reference to tasty_string
    let using_reference_string = &tasty_string;
    //Now using_reference_string uses the borrowed reference to access String::from("Hullo");
    
    println!("{}", using_reference_string);
}

fn literal_copying(){
    // x is allocated on the stack
    let x = 1;
    // x data is copied here and give a refrenece y
    let y = x;
    // You are allowed to do this because copying data in the stack in not ineffiecient
    // Is this beacuse you don't have to search for data, and search for space to put copied data?

    println!("{}", x + y)
}


fn main() {
    //moving_ownership_example::move_ownership();
    // copying_example::copying()
    slice_type_example::first_word("Hello world")
}
