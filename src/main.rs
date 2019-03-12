mod data_types;
mod functions;
mod loops;
mod ownership;
mod references;
mod slices;
mod structs;
mod rectangles;
mod enums;
mod matches;
mod collections;
mod panics;
mod generics;
mod traits;
mod lifetimes;

fn main() {
    //x cannot be changed cause it's a immutable variable
    let x = 5;
    println!("The value of x is: {}", x);
    //y can be changed cause we specify with mut that it is a mutable variable
    let mut y=x;
    println!("The value of y is: {}", y);
    y=6;
    println!("The value of y is: {}", y);

    //constants must indicate value type (u32 in this example)
    //mubers can have _ (underscores) to read them better.
    const MAX_HEIGHT: u32 = 10_000;

    println!("max height is {}", MAX_HEIGHT);

    //in this case you can change the value of x even if it is immutable 
    //cause you are shadowing it (redeclaring and redefining x) -> that means
    //you are basically creting a new x variable that will shadow the old one 
    //and its value will be the one of the old x plus 1;
    //it will be immutable, so without redeclaring it with let keyword it won't be possible to change its value.
    let x = x+1;

    println!("x shadows old x and its value is: {}",x);

    //with shadowing if we redeclare a viariable we will also be able to change its type
    //this might be very useful
    //EXAMPLE:
    //spaces here is a string of blank spaces
    let spaces = "     ";   
    //we will now shadow spaces and reassing it with the length of spaces
    let spaces = spaces.len();
    //now spaces is an int and represents the length (number of spaces) of the old spaces

    println!("the number of blank spaces is {}", spaces);

    //while if we declare a mutable variable we are allowed to change its value but not its type

    let mut new_spaces="     ";
    println!("{}",new_spaces);
    new_spaces = "string";
    println!("{}",new_spaces);

    //spaces = spaces.len(); this is not possible
    let heart_eyed_cat='😻';
    println!("cat: {}",heart_eyed_cat);
    //we can only change spaces value with a new string value since its mutable but we are not redeclaring it through let keyword
    data_types::make_tuples();
    data_types::make_array();
    functions::new_function();
    functions::new_function_with_arg(5);
    let mut x = 10;
    functions::func_with_mut_arg(x);

    //passing x to the function through mutable reference allow us to modify the variable inside the function and have it modified outside as well
    functions::func_with_ref_mut_arg(&mut x);
    
    println!("x after function is now: {}",x);

    x=functions::plus_one(x);
     println!("x has been assigned with the value returned by the function and now is: {}",x);

    //  functions::func_with_ref_mut_arg(&mut x);
    //  println!("x= {}",x);

    loops::make_loop();
    loops::make_while();
    loops::make_for();
    loops::make_rev_for();
    loops::make_range_for();
    ownership::make_string();
    ownership::make_ownership();
    ownership::take_and_give_ownership();
    references::make_ref();
    slices::slice_string();

    let mut user1 = structs::User{
        name: String::from("max"),
        email: String::from("massimoisonni@gmail.com"),
        age: 28,
    };

    user1.name=String::from("asd");
    println!("{}", user1.name);

    let user2=structs::build_user(String::from("mail@mail.com"), String::from("qwer"), 10);
    println!("user 2 age: {}", user2.age);

    structs::update_syntax_user();
    structs::make_tuple_structs();

    rectangles::make_rectangle();
    rectangles::make_rectangle_tuple();
    rectangles::make_rectangle_struct();
    rectangles::make_rect_with_area_method();
    rectangles::make_rect_constructor();

    enums::make_ip();
    enums::make_ip_through_enum();
    enums::make_message();
    enums::make_option();
    enums::add();

    matches::make_coin_match();
    matches::make_option_match();
    matches::make_placeholder();
    matches::make_if_let();

    collections::make_vector();
    collections::get_vector_elem();
    collections::iterate_and_mutate();
    collections::vector_already_borrowed();
    collections::make_hashmap();
    collections::make_hashmap_from_tuples();
    collections::make_hasmap_owning_items();
    collections::iterating_hashmap();
    collections::hashmap_managing_keys();

    //panics::make_panic();
    //panics::out_of_index_panic();
    panics::make_res_error();
    panics::make_unwrap_error();
    panics::propagating_error();

    generics::extracting_function();
    generics::make_largest();
    generics::make_generic_point();

    lifetimes::longest_string();
}

