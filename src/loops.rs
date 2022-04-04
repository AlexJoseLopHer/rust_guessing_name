fn loops() {
    let mut counter = 0;

    let result = loop {
        if counter == 10 {
            break counter * 2;
        }

        counter += 1;
    };

    println!("Counter {}", result);

    let mut number = 10;
    while number != 0 {
        println!("Number {}", number);
        number = number - 1;
    }

    let some_array = [1,2,3,4,5,6];
    let mut index = 0;

    println!("Printing array with while");
    while index < some_array.len() - 1 {
        println!("{}", some_array[index]);
        index += 1;
    }

    println!("Printing array with for");
    for element in some_array.iter() {
        println!("{}", element);
    }

    println!("Print range");
    for number in (1..10).rev() {
        println!("{}", number);
    }
}