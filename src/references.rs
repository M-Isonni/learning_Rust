pub fn make_ref(){
    let mut s1 = String::from("hello");
    
    println!("s1 before changing it is: {}", s1);

    change(&mut s1);

    let len = calculate_length(&s1);

    // let r1 = &mut s1;
    // let r2 = &mut s1; //only one mutable reference to an object can be active at a time!
    //RECAP:
    //you CANNOT have more than one mutable reference to an object at a time.
    //you CANNOT have an immutable referene and a mutable reference active at the same time (immutable reference don't expect it's reference to be mutated)7
    //you CAN have more than one immutable reference active at a time.

    // r1.push_str("r1");
    // r2.push_str("r2");

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s:&String) -> usize {   
    s.len()
}// Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(s:&mut String){
    s.push_str(", world!");
}

//We cannot execute the following code beacuse s is defined inside a scope and at the end of the scope
//it will be deallocated while we are trying to return a reference to that object which memory has been freed
// fn dangling_references(){
//     let reference_to_nothing=dangle();
// }

// fn dangle()-> &String {
//     let s = String::from("hello");
//     &s
// }