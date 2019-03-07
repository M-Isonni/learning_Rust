#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] 
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state)=> {
            println!("quarter from {:?}", state);
            match state{                
                UsState::Alabama=>25,
                UsState::Alaska=>25,
            }
        },
    }
}

pub fn make_coin_match(){
    let coin = Coin::Penny;
    let value = value_in_cents(&coin);
    println!("value of {:?} is {}", coin,value);
    let quarter=Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(&quarter);
    println!("value of {:?} is {}", coin,value);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn make_option_match(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five = {:?}, six = {:?}, none = {:?}",five,six,none);
}

pub fn make_placeholder() {
    let some_u8_value = 1;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

pub fn make_if_let(){
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
