use std::collections::HashMap;

pub fn make_vector(){
    let mut vector:Vec<i32> = Vec::new();
    let vector2=vec![1,2,3,4,5];

    vector.push(8);

    println!("value: {}",&vector[0]);
    println!("value from vector2 : {}", vector2[0]);
}

pub fn get_vector_elem(){
    
    let v = vec![1, 2, 3, 4, 5];

    //we can get the element in a position of the vector through indexing
    let third: &i32 = &v[2];
    let second = v[1];
    println!("The third element is {}", third);
    println!("The second element is {}", second);

    //or through matching: matching is safest cause it manages the none case.s
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

pub fn vector_already_borrowed(){
    
    let mut v = vec![1, 2, 3, 4, 5];

    let first = v[0];

    v.push(6);

    println!("The first element is: {}", first);

    //cannot execute the code cause vector is first borrowed by &v[0] as immutable and then from v.push as mutable.
    //as we said only more immutable borrowing can exist at a time.
}

pub fn iterate_and_mutate(){
    let mut v = vec![2,3,4,5,6];
    for i in &mut v{
        println!("value= {}",i);
    }

    for i in &mut v{
        *i+=50;
        println!("new value is {}",i);
    }
}

pub fn make_hashmap(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Yellow"), 10);
    scores.insert(String::from("Blue"), 50);

    println!("yellow score: {}", scores["Yellow"]);
}

pub fn make_hashmap_from_tuples(){
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    let team_name = String::from("Blue");    
    
    let score_opt = scores.get(&team_name);

    let score = hashmap_value(&score_opt);

    println!("blue score: {}", score);
}

fn hashmap_value(hash: &Option<&&i32>) -> i32{   
    match *hash{
        Some(value)=>**value,
        None=>0,
    }
}