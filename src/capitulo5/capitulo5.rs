pub fn capitulo5() {
    let mut user1 = User {
        email: String::from("josemoura212@gmai.com"),
        username: String::from("José Augusto"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("projeto.jose@outlook.com");

    let _user2 = User {
        email: String::from("josemoura212@gmail.com"),
        username: String::from("josemoura212"),
        ..user1 // !active: user1.active,
                // !sign_in_count: user1.sign_in_count,
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

#[allow(unused)]
fn build_user(email: String, username: String) -> User {
    User {
        // !email: email,
        // !username: username,
        email,
        username,
        active: true,
        sign_in_count: 1,
    };
    todo!();
}

#[allow(unused)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[allow(unused)]
struct Color(i32, i32, i32);

#[allow(unused)]
struct Point(i32, i32, i32);
