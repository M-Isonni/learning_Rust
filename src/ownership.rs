pub fn make_string(){
    //through String::from the string value is stored on the heap instead that on the stack, so we can store a value fro which we don't already know its size
    //considering we might want to change it at runtime or getting it from user's input
   let mut s = String::from("Hello");
   s.push_str(", world!");
   println!("{}",s);
   //at closing scope rust calls drop function to free the allocated memory inside the scope
   let mut s1=s;
   println!("s1= {}", s1);
   s1.push_str(" pushed");
   println!("s1= {}", s1);
   //println!("s= {}", s); //i cannot use the first variable s after using let s1 = s cause that variable hase been made unavailable in order to prevent
   //double free error since both points to the same heap memory index and at the end of the scope both would try to free the same memory

   // in order to make 2 copies and copying the heap memory as well we need to use the .clone() function
   //we would have then 2 different variables not referring to the same address

    let s2 = String::from("hello");
    let s3 = s2.clone();
    println!("s2 = {} s3 = {}", s2, s3);
}

pub fn make_ownership(){
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    //println!("{}", s); cannot call it cause drop has been called on the heap memory part that s reference at the end of the scope of takes_ownership function
    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward
    println!("{}", x); //instead i can still call x cause the x being an u32 type is passed as a copy to the function and always stored in the stack

}

fn takes_ownership(s:String){
    println!("{}", s);
}// Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(x:u32){
    println!("{}", x);
}// Here, some_integer goes out of scope. Nothing special happens.

pub fn take_and_give_ownership(){
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1
    println!("s1 value = {}", s1);
    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
    //println!("{}",s2);//cannot use s2 cause it hase been moved, but i can operate on s3 that points to the same area of s2
    println!("s3 string = {}",s3);
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    let s = String::from("some_string");
    s
}

fn takes_and_gives_back(string:String) -> String{
    string
}
