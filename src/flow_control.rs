
fn flow_control() {
    let number = 7;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    if number != 0 {
        println!("Number different than 0");
    }

    if number % 4 == 0 {
        println!("Divisible by 4");   
    } else if number % 3 == 0 {
        println!("Divisible by 3");
    } else if number % 2 == 0 {
        println!("Divisible by 2");
    } else {
        println!("Not divisible by 4, 3, 2");
    }

    let condition = true;

    let number = if condition {
        4
    } else {
        5
    };

    println!("{}", number);
}