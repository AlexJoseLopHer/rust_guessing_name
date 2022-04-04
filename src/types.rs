
fn types() {
    //Immutable variable
    let x = 5;
    println!("The value of x is {}", x);

    //Shadowning a variable
    let x = 6;
    println!("The value of x is {}", x);

    //Variable conversion
    let random_string_number = "46";
    let random_number: u32 = random_string_number.parse().expect("Not a number");
    println!("Converted number: {}", random_number);

    //Floats
    let random_64_floating = 3.4;
    let random_32_floating: f32 = 3.4;
    println!("Floating 64: {}", random_64_floating);
    println!("Floating 32: {}", random_32_floating);


    //Boolean
    let random_boolean = true;
    println!("Boolean {}", random_boolean);

    //Compound types
    //Tuple
    let tup: (i32, f64, char) = (1, 1.2, 'c'); //tuple creation
    let (x, y, z) = tup; //destructure tuple
    println!("{}, {}, {}", x, y, z);

    println!("3rd tup value {}", tup.2);

    //Array
    let random_array:[i32; 5] = [1,2,3,4,5];
    println!("Print array {}", random_array[0]);

    let same_value_array = [3; 5];
    println!("Print value {}", same_value_array[2]);

    //RUST safety principle, when trying to access an array position that doesn't exists, Rust will prevent accessing
    //to memory by checking the array size first
    //println!("invalid position {}", same_value_array[10]);

}