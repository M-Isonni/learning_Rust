pub fn make_panic(){
    panic!("crash and burn");
}

pub fn out_of_index_panic(){
    let v = vec![1, 2, 3];
    v[99];
}