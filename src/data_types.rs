
//tuples have fixed number of elements but each elements can be of different types
pub fn make_tuples(){
    let tup:(u8,i32,char)=(5,10_000,'c');
    let(_x,y,_z)=tup;
    println!("second value in tup: {}",y);

    let first_value = tup.0;
    let third_value = tup.2;
    println!("first value in tuple is: {} third is: {}", first_value, third_value);
}

//unlike tuples each element of an array must be of the same type and have fixed length
pub fn make_array(){
    //array with 9 elements (from 1 to 9)
    let my_array=[1,2,3,4,5,6,7,8,9];
    println!("element 0: {}",my_array[0]);
    //we can also define an array of 5 elements of a specified type
    let my_new_array:[i32;5];
    my_new_array=[2,3,4,5,6]; //we can hen initialize the array in a second moment
    println!("element 0: {}",my_new_array[0]);
    //my_new_array[0]=5; Elements inside the array cannot be modified cause the array is not mutable

    let mut mutable_array:[i32;3];
    mutable_array=[1,2,3];
    println!("mut array element 1: {}",mutable_array[1]);
    //it can now be modified cause the array was declared as mutable
    mutable_array[1]=6;
    println!("mut array element 1 after being modified: {}",mutable_array[1]);
}