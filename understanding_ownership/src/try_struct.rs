pub struct User {
    active: bool,
    pub username: String,
    email: String,
    sign_in_count: u64,
}

pub fn try_struct() -> User {
    let user = User {
        active: true,
        username: String::from("username123"),
        email: String::from("someone@example.com"),
        sign_in_count: 0,
    };
    user
}
