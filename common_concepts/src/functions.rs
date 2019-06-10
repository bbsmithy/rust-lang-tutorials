pub enum Examples {
    ImplicitReturn,
    TypedParamaters,
}

pub fn implicit_return(x: i32) -> i32 {
    println!("You can do implicit returns by finishing a code block with a variable e.g let y = {{ let x = 5; x + 1 }}. y is 6 in this case");
    let y = { x + 1 };
    println!("{}", y);
    y
}

pub fn typed_paramaters(x: i32) -> i32{
    println!("When creating arguments you must specify a type e.g (x:64) {}", x);
    x
}