pub struct User{
    pub name: String,
    pub email: String,
    pub age: u32,
}

pub fn build_user(email: String, username: String, age: u32) -> User {
    User {
        email: email,
        name: username,
        age: age,
    }        
}