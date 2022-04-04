fn main() {
    
    let mut s = String::from("some arbitrary string");
    let mut other_string = "hello";

    println!("{}", s);

    s.push_str("pushed string");

    println!("{}", s);

    other_string = "other string";

    println!("{}", other_string);


    
}
