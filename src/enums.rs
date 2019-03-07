#[derive(Debug)]
enum IPaddrKind{
    V4,
    V6,
}

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

    match home {
        IPAddr::V4(ip) => println!("address: {}", ip.address),
        IPAddr::V6(ip) => println!("address: {}", ip.address),
    }

    match loopback {
        IPAddr::V4(ip) => println!("address: {}", ip.address),
        IPAddr::V6(ip) => println!("address: {}", ip.address),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

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

