#[derive(Debug)]
struct User {
    name: String,
    age: u16,
    isFunny: bool,
}

impl User {
    fn printUserInfo(&self) {
        println!("I'm {} . I'm {} years old. Am I funny ? {}", self.name, self.age, self.isFunny);
    }
}

fn buildUser(name: String, age: u16) -> User {
    User {
        name,
        age,
        isFunny: false,
    }
}

fn main() {
    let mut user_quang = buildUser(String::from("Test"), 23);
    let user_huy = buildUser(user_quang.name, 23);

    user_quang.name = "Quang".to_owned();

    let user_destructor = User {
        isFunny: true,
        ..user_quang
    };

    user_destructor.printUserInfo();

    //user_quang.printUserInfo(); // Error here because borrow of partially moved value: `user_quang
    // println!("User info : {:?}", user_destructor)
}
