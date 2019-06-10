pub enum Examples {
    ImmutableSuccess,
    Shadowing
}

// pub fn immutable_failed(){
//     let x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

pub fn immutable_success(){
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

pub fn shadowing(){

    println!("Shadowing is where you change a variables value by assigning a new value to a new variable using the previous variable name");
    println!("You can declare a new variable with the same name as a previous variable, and the new variable shadows the previous variable");
    println!("By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed");

    //Changing variables valuable by reassigning to x
    let x = 5;
    let x = x + 2;
    let x = x + 9;
    println!("The value of x is {}", x);

    //Changing variables type/value by using shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces {}", spaces);

}