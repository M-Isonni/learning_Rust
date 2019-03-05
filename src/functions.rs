pub fn new_function(){
    println!("this is a funtion that takes no arguments");
}

pub fn new_function_with_arg(x:i32){
    println!("this is a funtion that takes arguments: the argument is x: {}",x);
    }

pub fn func_with_mut_arg(mut x:i32){
    println!("arg: {}",x);
    x=9;
    println!("arg after being modified: {}",x);
}

// pub fn func_with_ref_mut_arg(mut x:&i32){
//     println!("arg: {}",x);
//     x=&9;
//     println!("arg after being modified: {}",x);
// }

//the following function returns a i32 as indicated in the signature
pub fn plus_one(x:i32)->i32{
    x+1
}