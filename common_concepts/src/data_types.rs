

pub enum Examples {
    Integers,
    FloatingPoint,
    NumericOperations,
    BoolType,
    CharType,
    ArrayType,
    TupleType
}


pub fn integers(){
    println!("Integers Types:");

    println!("Signed integer range: -(2ⁿ⁻¹) to (2ⁿ⁻¹) - 1");
    println!("Unsigned integer range: 0 to (2ⁿ) - 1");

    println!("8-bit	i8	u8");
    let signed_var_eight_bit: i8 = i8::min_value();
    let unsigned_var_eight_bit: u8 = u8::max_value();
    println!("Signed: {}, Unsigned {}", signed_var_eight_bit, unsigned_var_eight_bit);
    
    println!("16-bit  i16	u16");
    let signed_var_sixteen_bit: i16 = i16::min_value();
    let unsigned_var_sixteen_bit: u16 = u16::max_value();
    println!("Signed {}, Unsigned {}", signed_var_sixteen_bit, unsigned_var_sixteen_bit);

    println!("32-bit  i32	u32");
    let signed_var_thirtytwo_bit: i32 = i32::min_value();
    let unsigned_var_thirtytwo_bit: u32 = u32::max_value();
    println!("Signed {}, Unsigned {}",signed_var_thirtytwo_bit, unsigned_var_thirtytwo_bit);

    println!("64-bit i64 u64");
    let signed_var_sixtyfour: i64 = i64::min_value();
    let unsigned_var_sixtyfour_bit: u64 = u64::max_value();
    println!("Signed: {}, Unsigned {}", signed_var_sixtyfour, unsigned_var_sixtyfour_bit);

    println!("Arch isize usize");
    let signed_var_pointer_sized: isize = isize::min_value();
    let unsigned_var_point_sized: usize = usize::max_value();
}

pub fn floating_point(){
    println!("Floating Point Types:");
    let x = 2.0;
    let y : f32 = 3.0;
    println!("Floating points f64 by default {}, use f32 for 32-bit floating points {}", x, y)
}


pub fn numeric_operations(){
    // addition
    let sum = 5 + 10;
    println!("Addition (+) {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("Addition (-) {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("Addition (*) {}", product);


    // division
    let quotient = 56.7 / 32.2;
    println!("Division (/) {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("Remainder (%) {}", remainder);
}


pub fn bool_type(){
    let is_true = true;
    let explicit_bool : bool = false;
    println!("bool keyword to explicitly set boolean variable that is 1 byte in size: {}", is_true)
}


pub fn char_type(){
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("char types: {}, {}, {}", c, z, heart_eyed_cat)
}

pub fn array_type(){
    let a: [i8; 4] = [1, 3, 5, 4];
    println!("An array is a collection of values that have the same type with a fixed size, they are stored on the stack.");
    println!("Implicit syntax let a = [1, 3, 5, 4]");
    println!("Explicit syntax let a: [i8; 4] = [1, 3, 5, 4]");
    println!("Use a vector if you collection will grow or shrink");
    println!("Access items a[index] e.g a[0] = {}", a[0]);

}

pub fn tuple_type(){
    println!("A tuple is a collection of values with fixed length that each can have differnet types e.g let tup: (i32, f64, u8) = (500, 6.4, 1);");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("You can deconstruct with: let (x, y, z) = tup");
    let (x, y, z) = tup;
    println!("You can access each item with: tup.0, tup.1 ....");
    println!("Index 0: {}", tup.0);
}