fn cop(a:i32, b:i32){
    println!("{}", a + b)
}

pub fn copying(){
    let a = 20;
    let b = 30;

    // a and b are stored on the stack because the are primitive types
    // In this case a and b are copied not moved to the cop function because it is not ineffiecient to copy values stored on the stack
    cop(a, b);

    //Which means they can still be accessed inside this scope
    println!("Still able to access a: {} and b: {}", a, b)
}