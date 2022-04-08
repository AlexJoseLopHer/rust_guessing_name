
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

impl User {
    fn is_active(&self) -> bool {
        self.active
    }
}

#[derive(Debug)]
struct UnamedTuple(String, String);

fn main() {
    //Structs
    let user1 = User {
        email: String::from("some@email.com"),
        username: String::from("random username"),
        sign_in_count: 0,
        active: false,
    };

    println!("{}", user1.email);

    //user1.email = String::from("some_other@email.com"); // Will fail as is not mutable

    let mut user2 = User {
        email: String::from("some@email.com"),
        username: String::from("random username"),
        sign_in_count: 0,
        active: false,
    };

    user2.email = String::from("some_other_email@email.com"); // works as user2 is mutable
    
    println!("{}", user2.email);

    let user3 = build_random_user(String::from("some@email.es"));

    println!("{}", user3.email);

    //Update syntax

    let user3 = User {
        email: String::from("User3@email.com"),
        ..user2
    };

    println!("user3 email: {}", user3.email);
    println!("user3 email: {}", user3.username);
    println!("user3: {:#?}", user3);

    let unnamed_tuple = UnamedTuple(
        String::from("some string"),
        String::from("some other string")
    );

    println!("{:?}", unnamed_tuple);

    println!("I user 3 active?: {}", user3.is_active())
    
}

fn build_random_user(email: String) -> User {
    User {
        email, // Here as the variable and the struct parameter have the same name, we can avoid specifying the argument
        username: String::from("random username"),
        sign_in_count: 0,
        active: false,
    }
}
