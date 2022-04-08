
mod Company {
    pub struct Company {
        pub name: String
    }

    impl Company {
        pub fn public_method(&self) {
            print!("public method");
        }

        fn private_method(&self) {
            println!("private method")
        }
    }
}


#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

impl User {
    fn hello(&self) {
        println!("public hello");
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn change_email(self, new_email: String) -> User {
        User { 
            email: new_email,
            ..self
         }
    }
}

impl User {
    fn create_username_email_string(&self) -> String {
        String::from(self.email.to_string() + " " + &self.username.to_string())

    }
}

impl User {
    //Static function
    fn create_random() -> User {
        User { username: String::from("randomUsername"), email: String::from("random@email.com"), sign_in_count: 1, active: true }
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

    println!("I user 3 active?: {}", user3.is_active());
    println!("{}", user3.create_username_email_string());
    println!("{:?}", User::create_random());

    let user4 = user3.change_email(String::from("modified@email.com"));
    //println!("{:?}", user3); //Will fail as it was borrowed
    println!("{:?}", user4);
    user4.hello();

    let company = Company::Company{
        name: String::from("company name"),
    };

    company.public_method()

}

fn build_random_user(email: String) -> User {
    User {
        email, // Here as the variable and the struct parameter have the same name, we can avoid specifying the argument
        username: String::from("random username"),
        sign_in_count: 0,
        active: false,
    }
}
