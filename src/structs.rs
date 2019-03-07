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

pub fn update_syntax_user(){
    let user = User{
        name: String::from("Max"),
        email: String::from("email@mail.it"),
        age: 30,
    };

    let user1 = User{
        name: String::from("Tom"),
        ..user //this syntax let user1 inherit the missing fields from user, initializing them with the same values.
    };
    println!("user age: {}, user1 age: {}",user.age,user1.age);
}

//making structs using tuples: without having specified names for its fields.
pub struct Color(u8,u8,u8);
pub struct Point(i32,i32,i32);

pub fn make_tuple_structs(){
    let black = Color(0,0,0);

    let position = Point(10,100,-200);

    println!("the rgb values for black are: {}, {}, {} and the startin position is x: {}, y: {} z: {}", black.0, black.1, black.2, position.0,position.1,position.2);
}