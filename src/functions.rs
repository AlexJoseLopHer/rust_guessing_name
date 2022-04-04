fn some_function(x: i32, y: i32) {
    println!("Some function print {}", x + y);

    let y = {
        let x = 1;
        x + 1
    };

    println!("{}", y);

    let z = some_other_function();

    println!("{}", z);

    println!("{}", plus_one(5));
}

fn some_other_function() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}