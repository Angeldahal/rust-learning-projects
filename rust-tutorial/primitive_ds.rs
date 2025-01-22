fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32;
    
    println!("An integer: {}", an_integer);

    let default_float = 3.0; // 'f64'
    let default_integer = 7; // 'i32'
    
    let mut inferred_type = 12; // mut = mutable 
    inferred_type = 482974;

    let mut mutable = 12;
    mutable = 123;

    // mutable = true; fails data_type cannot be changed.

    let mutable = true; // runs shadowing: creates new variable with the same name replacing
// previous

    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    for element in my_array.iter() {
        println!("{}", element);
    }

    let my_tuple = (5u32, 1u8, true, -5.04f32);
    let (value_1, value_2, value_3, value_4) = my_tuple;
    println!("{}, {}, {}, {}", value_1, value_2, value_3, value_4);
}
