#[derive(Debug)]
enum IPaddrKind{
    V4,
    V6,
}
//as in other languages structures can have an enum field to specify its type
struct IPAddress{
    kind: IPaddrKind,
    address: String,
}

pub fn make_ip(){
    let home = IPAddress {
    kind: IPaddrKind::V4,
    address: String::from("127.0.0.1"),
    };

    let loopback = IPAddress {
        kind: IPaddrKind::V6,
        address: String::from("::1"),
    };
    println!("first IP: address: {} type: {:?}, second IP: address: {} type: {:?}",home.address, home.kind, loopback.address, loopback.kind);
}

//other way to create ipAddresses through enum:
//instead of having an enum inside each structure, in Rust language, we can have the enum taking parameters such as variables, tuples or even structures
//to access those parameters stored through the enum type we have "match" it
#[derive(Debug)]
struct IPv4{
    address:String,
    address_num:(u8,u8,u8,u8),
}

#[derive(Debug)]
struct IPv6{
    address:String,
}

#[derive(Debug)]
enum IPAddr{
    V4(IPv4),
    V6(IPv6),
}

pub fn make_ip_through_enum(){
    let home = IPAddr::V4(IPv4{address: String::from("127.0.0.1"), address_num: (127,0,0,1)});
    let loopback = IPAddr::V6(IPv6{address: String::from("::1")});    
//here we math the possibilities in order to print the different values for the different kinds of enum
    match home {
        IPAddr::V4(ip) => println!("address: {}", ip.address),
        IPAddr::V6(ip) => println!("address: {}", ip.address),
    }
//considering we already know that loopback is an IPv6 and that it is not mutable we can just match the V6 enum and use a placeholder to give a default matching for the other possibilities
    match loopback {        
        IPAddr::V6(ip) => println!("address: {}", ip.address),
        _=>(),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

//we are implementing a function inside the enum as we did for structures
//such method will be called by the enum and act differently based on the different matches
impl Message {
    fn call(&self) {
       match self{
           Message::Quit=>println!("quitting application"),
           Message::Move{x,y}=>println!("moving to x: {}, y:{}",x,y),
           Message::Write(m)=>println!("Message: {}",m),
           Message::ChangeColor(r,g,b)=>println!("new color = r: {}, g: {}, b:{}",r,g,b),
       }
    }
}

pub fn make_message(){
    let m = Message::Write(String::from("hello"));
    m.call();
    let q = Message::Quit;
    q.call();
    let c = Message::ChangeColor(10,5,231);
    c.call();
    let mo=Message::Move{x:20, y:88};
    mo.call();
}

//null keyword is absent in Rust to prevent errors
//instead of null we have the Option<T> enum that can take any value for Some and any for None
//Non replace the null used in most languages, everytime we use it, tho, we have to manage the case where the vriable is none through a match, cause an enum wants a match for every single 
//enumeration listed in it
//doing so everytime we use the Option::None we are sure that the null case is managed.
pub fn make_option(){
    let some_number=Some(5);
    let some_string=Some("some string");

    let absent_number: Option<i32> = None;
    let absent_string: Option<String>=None;

    println!("number is: {:?}, string is: {:?}, absent number is: {:?}, absent string is: {:?}",some_number,some_string,absent_number, absent_string);
}

pub fn add(){
    let x: i8 = 5;
    let y: Option<i8> = Some(3);
    match y{
        Option::Some(t)=> {let sum = x+t; println!("sum is: {}", sum);},
        Option::None => println!("y value is null"),
    }    
}

