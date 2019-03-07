pub fn slice_string(){
    let s = String::from("hello world");
    let first_word=first_word(&s[..]);
    
    println!("{}",first_word);
}

fn first_word(s:&str)->&str{
    let bytes=s.as_bytes();    
    for(i,&item) in bytes.iter().enumerate(){
        if item == b' '{   
            println!("found space at index: {}", i);
            return &s[0..i];
        }       
    }
    &s[..]
}    