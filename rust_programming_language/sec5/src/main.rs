#[derive(Debug)]
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn initialize(username: String, email: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 0,
        }
    }

    // unmutable field
    fn getUserName(&self) -> &String {
        &self.username
    }

    // mutable field
    fn updateUserName(&mut self) {
        self.username.push_str("_new");
    }
}

fn initialize_User(username: String, email: String) -> User {
    User{
        active: true,
        username, // fast init
        email, // fast init
        sign_in_count: 1,
    }
}

/// Tuple Struct
struct Point(i32, i32, i32);
struct Color(i32, i32, i32);

fn main() {
    let mut user = User{
        active: true,
        username: String::from("uname"),
        email: String::from("a@purdue"),
        sign_in_count: 32,
    };

    user.sign_in_count = 33;

    println!("user = {:?}\n", user);

    let user2 = User{
        username: String::from("uname2"),
        ..user
    };
    // In here, we can't use user variable anymore, since its email:String is owned by user2
    println!("user = {:?}\n", user2);


    // Tuple struct
    let point = Point(1, 2, 3);

    // dbg!() will take the ownership into function, then return the ownership back
    let user = User {
        username: String::from("uname"),
        email: dbg!(String::from("@purdue")),
        active: true,
        sign_in_count: 20,
    };

    // We don't want give ownership to dbg function, so we give it a reference
    dbg!(&user);

    let mut user = User::initialize(String::from("abc"), String::from("@purdue"));
    dbg!(user.getUserName());
    user.updateUserName();
    dbg!(user.getUserName());
}
