
fun ownership() {
//String object

let mut s = String::from("some arbitrary string");
let mut other_string = "hello";

println!("{}", s);

s.push_str("pushed string");

println!("{}", s);

other_string = "other string";

println!("{}", other_string);


//Ownership

let s1 = String::from("some string");
let s2 = s1;

//println!("{}", s1); // Will fail as String doesn't implement Copy trait


let s3 = "some random string";
let s4 = s3;

println!("{}", s3); // Will not fail as  str implements Copy trait
println!("{}", s4);

let s5 = String::from("other random string");
let s6 = s5.clone();

println!("{}", s5); //will not fail as the object has been clone, creating another element in the heap
println!("{}", s6);

let some_number = 5;
let s7 = String::from("randomString");
some_random_method(&s7, some_number);

println!("{}", some_number);// Will not fail as we only passed a reference not the ownership
println!("{}", s7);

let s8 = String::from("some other string");

some_method(s8);
//println!("{}", s8);//Will fail as we passed the ownership to the method

let mut s9 = String::from("aa");
let s10 = &mut s9;
//let s11 = &mut s9;//will fail as a mutable cannot be borrowed more than once at a time

//println!("{} {}", s10, s11);


//Slice
let s11 = String::from("Some string");

let word = second_word(&s11);

println!("{}", word);

//word.clear() //this would break 


}

fn some_random_method( str: &String, some_number: i32) {

println!("From random method str1: {}", str);
println!("From random method str2: {}", some_number);
}

fn some_method(str: String) {

}

fn return_reference() -> String {
let s = String::from("something");

return s;
}


fn first_word(str: &String) -> usize {
for(i, &item) in str.as_bytes().iter().enumerate() {
    if item == b' ' {
        return i;
    }
}

return str.len();
}

fn second_word(string: &String) -> &str {
for(i, &item) in string.as_bytes().iter().enumerate() {
    if item == b' ' {
        return &string[..i];

    }
}

return &string[..]
}
